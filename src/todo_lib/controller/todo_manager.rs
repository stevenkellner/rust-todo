use crate::models::todo_list::TodoList;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_type::CommandControllerType;
use crate::models::loop_control::LoopControl;
use crate::ui::{InputReader, UIManager};
use crate::controller::command_controller::CommandController;
use crate::controller::task_command::TaskCommandController;
use crate::controller::general_command::GeneralCommandController;
use crate::controller::debug_command::DebugCommandController;
use std::collections::HashMap;

/// Controls the todo list application by coordinating specialized controllers.
///
/// and delegating command processing to specialized controllers that can be added or removed dynamically.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::controller::todo_manager::TodoManager;
///
/// let mut manager = TodoManager::new();
/// manager.run();
/// ```
pub struct TodoManager {
    todo_list: TodoList,
    input_reader: InputReader,
    ui_manager: UIManager<std::io::Stdout>,
    command_controllers: HashMap<CommandControllerType, Box<dyn CommandController>>,
}

impl TodoManager {

    /// Creates a new manager with an empty todo list and new UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::todo_manager::TodoManager;
    ///
    /// let manager = TodoManager::new();
    /// ```
    pub fn new() -> Self {
        let command_controllers = HashMap::from([
            (CommandControllerType::Task, Box::new(TaskCommandController::new()) as Box<dyn CommandController>),
            (CommandControllerType::General, Box::new(GeneralCommandController::new()) as Box<dyn CommandController>),
        ]);
        
        TodoManager {
            todo_list: TodoList::new(),
            input_reader: InputReader::new(),
            ui_manager: UIManager::new(),
            command_controllers,
        }
    }

    /// Starts the interactive command loop.
    ///
    /// This method displays a welcome message and enters an event loop
    /// that processes user input until the user quits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::controller::todo_manager::TodoManager;
    ///
    /// let mut manager = TodoManager::new();
    /// manager.run();
    /// ```
    pub fn run(&mut self) {
        self.ui_manager.show_welcome();

        loop {
            self.ui_manager.print_prompt();
            let input = self.input_reader.read_input();
            
            if self.handle_input(&input) == LoopControl::Exit {
                break;
            }
        }
    }

    /// Handles user input by trying each specialized controller in turn.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string from the user
    ///
    /// # Returns
    ///
    /// `LoopControl::Continue` to continue the event loop, `LoopControl::Exit` to quit
    fn handle_input(&mut self, input: &str) -> LoopControl {
        let trimmed = input.trim();
        
        // Empty input
        if trimmed.is_empty() {
            return LoopControl::Continue;
        }

        // Try dynamic command controllers (task and optionally debug)
        for controller in self.command_controllers.values_mut() {
            if let Some(result) = controller.try_execute(trimmed, &mut self.todo_list) {
                match result {
                    Ok(CommandControllerResult::Continue) => return LoopControl::Continue,
                    Ok(CommandControllerResult::ExitMainLoop) => return LoopControl::Exit,
                    Ok(CommandControllerResult::EnableDebugMode) => {
                        self.command_controllers.insert(CommandControllerType::Debug, Box::new(DebugCommandController::new()));
                        return LoopControl::Continue;
                    }
                    Ok(CommandControllerResult::DisableDebugMode) => {
                        self.command_controllers.remove(&CommandControllerType::Debug);
                        return LoopControl::Continue;
                    }
                    Err(err) => {
                        self.ui_manager.show_error(&err.message());
                        return LoopControl::Continue;
                    }
                }
            }
        }

        // Unknown command
        self.ui_manager.handle_unknown_command(trimmed);
        LoopControl::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let manager = TodoManager::new();
        assert!(manager.todo_list.is_empty());
    }

    #[test]
    fn test_handle_add_task() {
        let mut manager = TodoManager::new();
        
        manager.handle_input("add Test task");
        
        assert_eq!(manager.todo_list.get_tasks().len(), 1);
        assert_eq!(manager.todo_list.get_tasks()[0].description, "Test task");
    }

    #[test]
    fn test_handle_add_multiple_tasks() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
    }

    #[test]
    fn test_handle_remove_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task to remove");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_input(&format!("remove {}", task_id));
        
        assert!(controller.todo_list.is_empty());
    }

    #[test]
    fn test_handle_remove_nonexistent_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Test task");
        controller.handle_input("remove 999");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task to complete");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_nonexistent_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Test task");
        controller.handle_input("complete 999");
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_uncomplete_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task to uncomplete");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("uncomplete {}", task_id));
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_uncomplete_nonexistent_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Test task");
        controller.handle_input("uncomplete 999");
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task to toggle");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_nonexistent_task() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Test task");
        let initial_status = controller.todo_list.get_tasks()[0].is_completed();
        
        controller.handle_input("toggle 999");
        
        assert_eq!(controller.todo_list.get_tasks()[0].is_completed(), initial_status);
    }

    #[test]
    fn test_handle_list_tasks_all() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_handle_list_tasks_completed() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list completed");
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_list_tasks_pending() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list pending");
        
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_quit_command() {
        let mut controller = TodoManager::new();
        
        let control = controller.handle_input("quit");
        
        assert_eq!(control, LoopControl::Exit);
    }

    #[test]
    fn test_handle_help_command() {
        let mut controller = TodoManager::new();
        
        let control = controller.handle_input("help");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_add_command_returns_continue() {
        let mut controller = TodoManager::new();
        
        let control = controller.handle_input("add New task");
        
        assert_eq!(control, LoopControl::Continue);
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_empty_input() {
        let mut controller = TodoManager::new();
        
        let control = controller.handle_input("");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_unknown_command() {
        let mut controller = TodoManager::new();
        
        let control = controller.handle_input("invalidcommand");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_complex_workflow() {
        let mut controller = TodoManager::new();
        
        // Add multiple tasks
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
        
        // Complete some tasks
        controller.handle_input("complete 1");
        controller.handle_input("complete 2");
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 2);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
        
        // Remove a task
        controller.handle_input("remove 3");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
        
        // Toggle a task
        controller.handle_input("toggle 1");
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_search_tasks() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Buy groceries");
        controller.handle_input("add Read a book");
        controller.handle_input("add Buy concert tickets");
        
        controller.handle_input("search buy");
        
        // No panic means search executed successfully
        // The actual display is tested in OutputWriter tests
    }

    #[test]
    fn test_handle_search_tasks_no_results() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("add Task one");
        controller.handle_input("add Task two");
        
        controller.handle_input("search nonexistent");
        
        // Should handle gracefully with no results
    }

    #[test]
    fn test_handle_search_tasks_empty_list() {
        let mut controller = TodoManager::new();
        
        controller.handle_input("search anything");
        
        // Should handle gracefully with empty list
    }
}
