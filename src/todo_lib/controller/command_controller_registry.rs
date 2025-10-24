use crate::controller::command_controller::CommandController;
use crate::controller::task_command::TaskCommandController;
use crate::controller::general_command::GeneralCommandController;
use crate::controller::debug_command::DebugCommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::todo_list::TodoList;
use crate::models::ParseError;
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Manages the collection of command controllers and their active states.
///
/// This struct encapsulates the logic for managing different command controllers
/// using a HashMap, allowing for dynamic addition and removal of controllers.
pub struct CommandControllerRegistry<O: OutputWriter> {
    task_controller: TaskCommandController<O>,
    general_controller: GeneralCommandController<O>,
    debug_controller: DebugCommandController<O>,
    is_debug_active: bool,
}

impl<O: OutputWriter> CommandControllerRegistry<O> {
    /// Creates a new CommandControllerRegistry with task and general controllers active,
    /// and debug controller inactive by default.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            task_controller: TaskCommandController::new(Rc::clone(&output_writer)),
            general_controller: GeneralCommandController::new(Rc::clone(&output_writer)),
            debug_controller: DebugCommandController::new(Rc::clone(&output_writer)),
            is_debug_active: false,
        }
    }

    /// Enables the debug controller.
    pub fn enable_debug(&mut self) {
        self.is_debug_active = true;
    }

    /// Disables the debug controller.
    pub fn disable_debug(&mut self) {
        self.is_debug_active = false;
    }

    /// Tries to execute the input with all active controllers.
    ///
    /// This method iterates through the active controllers in a specific order
    /// (Task, General, Debug) and attempts to execute the input with each one.
    /// Returns the first successful match.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string from the user
    /// * `todo_list` - Mutable reference to the todo list
    ///
    /// # Returns
    ///
    /// `Some(Result)` if a controller handled the command, `None` if no controller recognized it
    pub fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        if let Some(result) = self.task_controller.try_execute(input, todo_list) {
            return Some(result);
        }
        if let Some(result) = self.general_controller.try_execute(input, todo_list) {
            return Some(result);
        }
        if self.is_debug_active {
            if let Some(result) = self.debug_controller.try_execute(input, todo_list) {
                return Some(result);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_execute_with_task_command() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("add Test task", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_try_execute_with_general_command() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("quit", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_try_execute_with_debug_command_inactive() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        let mut todo_list = TodoList::new();
        
        // Debug controller is not active, so debug commands should not be recognized
        let result = registry.try_execute("debug:gen 5", &mut todo_list);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_execute_with_debug_command_active() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        registry.enable_debug();
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("debug:gen 3", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 3);
    }

    #[test]
    fn test_try_execute_unknown_command() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("unknown command", &mut todo_list);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_execute_with_disabled_task_controller() {
        let output = crate::ui::output::FileOutputWriter::new(std::io::stdout());
        let mut registry = CommandControllerRegistry::new(Rc::new(RefCell::new(output)));
        registry.disable_debug();
        let mut todo_list = TodoList::new();
        
        // Task controller is still active, so this should succeed
        let result = registry.try_execute("add Test task", &mut todo_list);
        assert!(result.is_some());
    }
}
    