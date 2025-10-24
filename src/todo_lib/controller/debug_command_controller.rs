use crate::models::todo_list::TodoList;
use crate::models::parse_error::ParseError;
use crate::controller::debug_command_handler::DebugCommandHandler;
use crate::controller::command_controller::CommandController;
use crate::ui::debug_command_parser::DebugCommandParser;
use std::io::Write;

/// Controller that handles debug command parsing and execution.
///
/// `DebugCommandController` combines the debug command parser and handler,
/// providing a unified interface for processing debug-related commands.
pub struct DebugCommandController<W: Write> {
    handler: DebugCommandHandler<W>,
    parser: DebugCommandParser,
}

impl DebugCommandController<std::io::Stdout> {
    /// Creates a new debug command controller with stdout.
    pub fn new() -> Self {
        DebugCommandController {
            handler: DebugCommandHandler::new(),
            parser: DebugCommandParser::new(),
        }
    }
}

impl<W: Write> DebugCommandController<W> {
    /// Creates a new debug command controller with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        DebugCommandController {
            handler: DebugCommandHandler::with_writer(writer),
            parser: DebugCommandParser::new(),
        }
    }

    /// Checks if debug mode is currently enabled.
    pub fn is_debug_mode(&self) -> bool {
        self.handler.is_debug_mode()
    }

    /// Attempts to parse and handle a debug command from raw input.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string to parse
    /// * `todo_list` - The todo list to operate on
    ///
    /// # Returns
    ///
    /// * `Some(Ok(()))` - Command was successfully parsed and executed
    /// * `Some(Err(ParseError))` - Command was recognized as a debug command but had an error
    /// * `None` - Not a debug command, should try other parsers
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
            Some(Ok(debug_command)) => {
                self.handler.handle(&debug_command, todo_list);
                Some(Ok(()))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

impl<W: Write> Default for DebugCommandController<W>
where
    W: Write + Default,
{
    fn default() -> Self {
        Self::with_writer(W::default())
    }
}

impl CommandController for DebugCommandController<std::io::Stdout> {
    fn try_handle(
        &mut self,
        input: &str,
        todo_list: &mut TodoList,
    ) -> Option<Result<(), ParseError>> {
        DebugCommandController::try_handle(self, input, todo_list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let _controller = DebugCommandController::new();
    }

    #[test]
    fn test_try_handle_debug_toggle() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        assert!(!controller.is_debug_mode());
        
        let result = controller.try_handle("debug", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert!(controller.is_debug_mode());
    }

    #[test]
    fn test_try_handle_debug_generate() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        // Enable debug mode first
        controller.try_handle("debug", &mut todo_list);

        let result = controller.try_handle("debug:gen 5", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 5);
    }

    #[test]
    fn test_try_handle_non_debug_command() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("help", &mut todo_list);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_try_handle_invalid_debug_command() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("debug:gen", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_try_handle_debug_clear() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        
        // Add some tasks and enable debug mode
        todo_list.add_task(crate::models::task::TaskWithoutId::new("Test".to_string()));
        controller.try_handle("debug", &mut todo_list);

        let result = controller.try_handle("debug:clear", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
