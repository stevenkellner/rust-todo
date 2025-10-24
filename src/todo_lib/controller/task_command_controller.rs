use crate::models::todo_list::TodoList;
use crate::models::parse_error::ParseError;
use crate::controller::task_command_handler::TaskCommandHandler;
use crate::controller::command_controller::CommandController;
use crate::ui::task_command_parser::TaskCommandParser;
use std::io::Write;

/// Controller that handles task command parsing and execution.
///
/// `TaskCommandController` combines the task command parser and handler,
/// providing a unified interface for processing task-related commands.
pub struct TaskCommandController<W: Write> {
    handler: TaskCommandHandler<W>,
    parser: TaskCommandParser,
}

impl TaskCommandController<std::io::Stdout> {
    /// Creates a new task command controller with stdout.
    pub fn new() -> Self {
        TaskCommandController {
            handler: TaskCommandHandler::new(),
            parser: TaskCommandParser::new(),
        }
    }
}

impl<W: Write> TaskCommandController<W> {
    /// Creates a new task command controller with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        TaskCommandController {
            handler: TaskCommandHandler::with_writer(writer),
            parser: TaskCommandParser::new(),
        }
    }

    /// Attempts to parse and handle a task command from raw input.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string to parse
    /// * `todo_list` - The todo list to operate on
    ///
    /// # Returns
    ///
    /// * `Some(Ok(()))` - Command was successfully parsed and executed
    /// * `Some(Err(ParseError))` - Command was recognized as a task command but had an error
    /// * `None` - Not a task command, should try other parsers
    pub fn try_handle(
        &mut self,
        input: &str,
        todo_list: &mut TodoList,
    ) -> Option<Result<(), ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.parser.try_parse_command(&command, args) {
            Some(Ok(task_command)) => {
                self.handler.handle(&task_command, todo_list);
                Some(Ok(()))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

impl<W: Write> Default for TaskCommandController<W>
where
    W: Write + Default,
{
    fn default() -> Self {
        Self::with_writer(W::default())
    }
}

impl CommandController for TaskCommandController<std::io::Stdout> {
    fn try_handle(
        &mut self,
        input: &str,
        todo_list: &mut TodoList,
    ) -> Option<Result<(), ParseError>> {
        TaskCommandController::try_handle(self, input, todo_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let _controller = TaskCommandController::new();
    }

    #[test]
    fn test_try_handle_add_command() {
        let mut controller = TaskCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("add Test task", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_try_handle_list_command() {
        let mut controller = TaskCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("list", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_try_handle_non_task_command() {
        let mut controller = TaskCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("help", &mut todo_list);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_try_handle_invalid_task_command() {
        let mut controller = TaskCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("add", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_try_handle_remove_command() {
        let mut controller = TaskCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        todo_list.add_task(crate::models::task::TaskWithoutId::new("Test".to_string()));

        let result = controller.try_handle("remove 1", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
