use crate::controller::CommandControllerRegistry;
use crate::models::todo_list::TodoList;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::loop_control::LoopControl;
use crate::ui::{InputStream, OutputManager};
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Controls the todo list application by coordinating specialized controllers.
///
/// and delegating command processing to specialized controllers that can be added or removed dynamically.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::controller::todo_manager::TodoManager;
/// use todo_manager::ui::input::FileInputStream;
/// use todo_manager::ui::output::FileOutputWriter;
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let input_stream = FileInputStream::new(std::io::stdin());
/// let output_writer = FileOutputWriter::new(std::io::stdout());
/// let mut manager = TodoManager::new(
///     Rc::new(RefCell::new(input_stream)),
///     Rc::new(RefCell::new(output_writer))
/// );
/// manager.run();
/// ```
pub struct TodoManager<I: InputStream, O: OutputWriter> {
    #[cfg(test)]
    todo_list: Rc<RefCell<TodoList>>,
    input_stream: Rc<RefCell<I>>,
    output_manager: OutputManager<O>,
    command_controller_registry: CommandControllerRegistry<O>,
}

impl<I: InputStream, O: OutputWriter> TodoManager<I, O> {

    /// Creates a new manager with an empty todo list and new UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::todo_manager::TodoManager;
    /// use todo_manager::ui::input::FileInputStream;
    /// use todo_manager::ui::output::FileOutputWriter;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let input_stream = FileInputStream::new(std::io::stdin());
    /// let output_writer = FileOutputWriter::new(std::io::stdout());
    /// let manager = TodoManager::new(
    ///     Rc::new(RefCell::new(input_stream)),
    ///     Rc::new(RefCell::new(output_writer))
    /// );
    /// ```
    pub fn new(input_stream: Rc<RefCell<I>>, output_writer: Rc<RefCell<O>>) -> Self {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        Self {
            #[cfg(test)]
            todo_list: Rc::clone(&todo_list),
            input_stream,
            output_manager: OutputManager::new(Rc::clone(&output_writer)),
            command_controller_registry: CommandControllerRegistry::new(Rc::clone(&todo_list), Rc::clone(&output_writer)),
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
    /// use todo_manager::ui::input::FileInputStream;
    /// use todo_manager::ui::output::FileOutputWriter;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let input_stream = FileInputStream::new(std::io::stdin());
    /// let output_writer = FileOutputWriter::new(std::io::stdout());
    /// let mut manager = TodoManager::new(
    ///     Rc::new(RefCell::new(input_stream)),
    ///     Rc::new(RefCell::new(output_writer))
    /// );
    /// manager.run();
    /// ```
    pub fn run(&mut self) {
        self.output_manager.show_welcome();

        loop {
            self.output_manager.print_prompt();
            let input = self.input_stream.borrow_mut().get_next_input();
            
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

        if let Some(result) = self.command_controller_registry.try_execute(trimmed) {
            match result {
                Ok(CommandControllerResult::Continue) => return LoopControl::Continue,
                Ok(CommandControllerResult::ExitMainLoop) => return LoopControl::Exit,
                Ok(CommandControllerResult::EnableDebugMode) => {
                    self.command_controller_registry.enable_debug();
                    return LoopControl::Continue;
                }
                Ok(CommandControllerResult::DisableDebugMode) => {
                    self.command_controller_registry.disable_debug();
                    return LoopControl::Continue;
                }
                Err(err) => {
                    self.output_manager.show_error(&err.message());
                    return LoopControl::Continue;
                }
            }
        }

        // Unknown command
        self.output_manager.handle_unknown_command(trimmed);
        LoopControl::Continue
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::input::FileInputStream;
    use crate::ui::output::FileOutputWriter;

    #[test]
    fn test_new_controller() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let manager = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        assert!(manager.todo_list.borrow().is_empty());
    }

    #[test]
    fn test_handle_add_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut manager = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        manager.handle_input("add Test task");
        
        assert_eq!(manager.todo_list.borrow().get_tasks().len(), 1);
        assert_eq!(manager.todo_list.borrow().get_tasks()[0].description, "Test task");
    }

    #[test]
    fn test_handle_add_multiple_tasks() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 3);
    }

    #[test]
    fn test_handle_remove_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task to remove");
        let task_id = controller.todo_list.borrow().get_tasks()[0].id;
        
        controller.handle_input(&format!("remove {}", task_id));
        
        assert!(controller.todo_list.borrow().is_empty());
    }

    #[test]
    fn test_handle_remove_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Test task");
        controller.handle_input("remove 999");
        
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task to complete");
        let task_id = controller.todo_list.borrow().get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        
        assert!(controller.todo_list.borrow().get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.borrow().get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Test task");
        controller.handle_input("complete 999");
        
        assert!(!controller.todo_list.borrow().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_uncomplete_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task to uncomplete");
        let task_id = controller.todo_list.borrow().get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        assert!(controller.todo_list.borrow().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("uncomplete {}", task_id));
        assert!(!controller.todo_list.borrow().get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.borrow().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_uncomplete_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Test task");
        controller.handle_input("uncomplete 999");
        
        assert!(!controller.todo_list.borrow().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task to toggle");
        let task_id = controller.todo_list.borrow().get_tasks()[0].id;
        
        assert!(!controller.todo_list.borrow().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(controller.todo_list.borrow().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(!controller.todo_list.borrow().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Test task");
        let initial_status = controller.todo_list.borrow().get_tasks()[0].is_completed();
        
        controller.handle_input("toggle 999");
        
        assert_eq!(controller.todo_list.borrow().get_tasks()[0].is_completed(), initial_status);
    }

    #[test]
    fn test_handle_list_tasks_all() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list");
        
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 2);
    }

    #[test]
    fn test_handle_list_tasks_completed() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list completed");
        
        assert_eq!(controller.todo_list.borrow().get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_list_tasks_pending() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list pending");
        
        assert_eq!(controller.todo_list.borrow().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_quit_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        let control = controller.handle_input("quit");
        
        assert_eq!(control, LoopControl::Exit);
    }

    #[test]
    fn test_handle_help_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        let control = controller.handle_input("help");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_add_command_returns_continue() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        let control = controller.handle_input("add New task");
        
        assert_eq!(control, LoopControl::Continue);
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_empty_input() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        let control = controller.handle_input("");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_unknown_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        let control = controller.handle_input("invalidcommand");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_complex_workflow() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        // Add multiple tasks
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 3);
        
        // Complete some tasks
        controller.handle_input("complete 1");
        controller.handle_input("complete 2");
        
        assert_eq!(controller.todo_list.borrow().get_completed_tasks().len(), 2);
        assert_eq!(controller.todo_list.borrow().get_pending_tasks().len(), 1);
        
        // Remove a task
        controller.handle_input("remove 3");
        
        assert_eq!(controller.todo_list.borrow().get_tasks().len(), 2);
        
        // Toggle a task
        controller.handle_input("toggle 1");
        
        assert_eq!(controller.todo_list.borrow().get_completed_tasks().len(), 1);
        assert_eq!(controller.todo_list.borrow().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_search_tasks() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Buy groceries");
        controller.handle_input("add Read a book");
        controller.handle_input("add Buy concert tickets");
        
        controller.handle_input("search buy");
        
        // No panic means search executed successfully
        // The actual display is tested in OutputWriter tests
    }

    #[test]
    fn test_handle_search_tasks_no_results() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("add Task one");
        controller.handle_input("add Task two");
        
        controller.handle_input("search nonexistent");
        
        // Should handle gracefully with no results
    }

    #[test]
    fn test_handle_search_tasks_empty_list() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)));
        
        controller.handle_input("search anything");
        
        // Should handle gracefully with empty list
    }
}
