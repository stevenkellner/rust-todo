use crate::models::todo_list::TodoList;
use crate::ui::command_parser::CommandParser;
use crate::ui::{TaskCommandOutputWriter, DebugCommandOutputWriter, GeneralCommandOutputWriter};
use crate::models::ui_event::UiEvent;
use crate::models::loop_control::LoopControl;
use crate::controller::debug_command_handler::DebugCommandHandler;
use crate::controller::task_command_handler::TaskCommandHandler;

/// Controls the todo list application by reacting to UI events.
///
/// `TodoController` acts as the controller layer, handling events from the UI
/// and managing the todo list state. It processes events and delegates
/// display responsibilities back to the UI.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::controller::todo_controller::TodoController;
///
/// let mut controller = TodoController::new();
/// controller.run();
/// ```
pub struct TodoController {
    todo_list: TodoList,
    parser: CommandParser<std::io::Stdin>,
    task_output: TaskCommandOutputWriter<std::io::Stdout>,
    debug_output: DebugCommandOutputWriter<std::io::Stdout>,
    general_output: GeneralCommandOutputWriter<std::io::Stdout>,
    task_handler: TaskCommandHandler,
    debug_handler: DebugCommandHandler,
}

impl TodoController {
    /// Creates a new controller with an empty todo list and new UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::todo_controller::TodoController;
    ///
    /// let controller = TodoController::new();
    /// ```
    pub fn new() -> Self {
        TodoController {
            todo_list: TodoList::new(),
            parser: CommandParser::new_stdin(),
            task_output: TaskCommandOutputWriter::new(),
            debug_output: DebugCommandOutputWriter::new(),
            general_output: GeneralCommandOutputWriter::new(),
            task_handler: TaskCommandHandler::new(),
            debug_handler: DebugCommandHandler::new(),
        }
    }

    /// Starts the interactive command loop.
    ///
    /// This method displays a welcome message and enters an event loop
    /// that processes UI events until the user quits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::controller::todo_controller::TodoController;
    ///
    /// let mut controller = TodoController::new();
    /// controller.run();
    /// ```
    pub fn run(&mut self) {
        self.general_output.show_welcome();

        loop {
            self.general_output.print_prompt();
            match self.parser.read_event() {
                Ok(event) => {
                    if self.handle_event(&event) == LoopControl::Exit {
                        break;
                    }
                }
                Err(err) => {
                    self.general_output.show_error(&err.message());
                }
            }
        }
    }

    /// Handles a UI event and returns the loop control decision.
    ///
    /// # Arguments
    ///
    /// * `event` - The UI event to handle
    ///
    /// # Returns
    ///
    /// `LoopControl::Continue` to continue the event loop, `LoopControl::Exit` to quit
    fn handle_event(&mut self, event: &UiEvent) -> LoopControl {
        match event {
            UiEvent::Task(task_command) => {
                self.task_handler.handle(task_command, &mut self.todo_list, &mut self.task_output);
            }
            UiEvent::Debug(debug_command) => {
                self.debug_handler.handle(debug_command, &mut self.todo_list, &mut self.debug_output);
            }
            UiEvent::General(general_command) => {
                return self.handle_general_command(general_command);
            }
        }
        
        LoopControl::Continue
    }

    /// Handles general application commands.
    fn handle_general_command(&mut self, command: &crate::models::general_command::GeneralCommand) -> LoopControl {
        use crate::models::general_command::GeneralCommand;
        
        match command {
            GeneralCommand::ShowHelp => {
                self.general_output.show_help();
                LoopControl::Continue
            }
            GeneralCommand::Quit => self.handle_quit(),
            GeneralCommand::Unknown(cmd) => {
                self.general_output.show_unknown_command(cmd);
                LoopControl::Continue
            }
        }
    }

    /// Handles the Quit event.
    fn handle_quit(&mut self) -> LoopControl {
        self.general_output.show_goodbye();
        LoopControl::Exit
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task_filter::TaskFilter;
    use crate::models::task_status::TaskStatus;
    use crate::models::task_command::TaskCommand;

    #[test]
    fn test_new_controller() {
        let controller = TodoController::new();
        assert!(controller.todo_list.is_empty());
    }

    #[test]
    fn test_handle_add_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Test task".to_string())));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
        assert_eq!(controller.todo_list.get_tasks()[0].description, "Test task");
    }

    #[test]
    fn test_handle_add_multiple_tasks() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 1".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 2".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 3".to_string())));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
    }

    #[test]
    fn test_handle_remove_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task to remove".to_string())));
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Remove(task_id)));
        
        assert!(controller.todo_list.is_empty());
    }

    #[test]
    fn test_handle_remove_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Test task".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Remove(999)));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task to complete".to_string())));
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(task_id)));
        
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Test task".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(999)));
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_uncomplete_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task to uncomplete".to_string())));
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(task_id)));
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Uncomplete(task_id)));
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_uncomplete_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Test task".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Uncomplete(999)));
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task to toggle".to_string())));
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Toggle(task_id)));
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Toggle(task_id)));
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Test task".to_string())));
        let initial_status = controller.todo_list.get_tasks()[0].is_completed();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Toggle(999)));
        
        assert_eq!(controller.todo_list.get_tasks()[0].is_completed(), initial_status);
    }

    #[test]
    fn test_handle_list_tasks_all() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 1".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 2".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(1)));
        
        controller.handle_event(&UiEvent::Task(TaskCommand::List(None)));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_handle_list_tasks_completed() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 1".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 2".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(1)));
        
        controller.handle_event(&UiEvent::Task(TaskCommand::List(Some(TaskFilter::all().with_status(TaskStatus::Completed)))));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_list_tasks_pending() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 1".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 2".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(1)));
        
        controller.handle_event(&UiEvent::Task(TaskCommand::List(Some(TaskFilter::all().with_status(TaskStatus::Pending)))));
        
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_event_quit() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::General(crate::models::general_command::GeneralCommand::Quit));
        
        assert_eq!(control, LoopControl::Exit);
    }

    #[test]
    fn test_handle_event_non_quit() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::General(crate::models::general_command::GeneralCommand::ShowHelp));
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_event_add_task() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::Task(TaskCommand::Add("New task".to_string())));
        
        assert_eq!(control, LoopControl::Continue);
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_event_unknown_command() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::General(crate::models::general_command::GeneralCommand::Unknown("invalid".to_string())));
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_complex_workflow() {
        let mut controller = TodoController::new();
        
        // Add multiple tasks
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 1".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 2".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task 3".to_string())));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
        
        // Complete some tasks
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(1)));
        controller.handle_event(&UiEvent::Task(TaskCommand::Complete(2)));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 2);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
        
        // Remove a task
        controller.handle_event(&UiEvent::Task(TaskCommand::Remove(3)));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
        
        // Toggle a task
        controller.handle_event(&UiEvent::Task(TaskCommand::Toggle(1)));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_search_tasks() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Buy groceries".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Read a book".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Buy concert tickets".to_string())));
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Search("buy".to_string())));
        
        // No panic means search executed successfully
        // The actual display is tested in OutputWriter tests
    }

    #[test]
    fn test_handle_search_tasks_no_results() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task one".to_string())));
        controller.handle_event(&UiEvent::Task(TaskCommand::Add("Task two".to_string())));
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Search("nonexistent".to_string())));
        
        // Should handle gracefully with no results
    }

    #[test]
    fn test_handle_search_tasks_empty_list() {
        let mut controller = TodoController::new();
        
        controller.handle_event(&UiEvent::Task(TaskCommand::Search("anything".to_string())));
        
        // Should handle gracefully with empty list
    }
}

