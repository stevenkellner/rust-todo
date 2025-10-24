use crate::models::parse_error::ParseError;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::todo_list::TodoList;
use crate::controller::command_controller::CommandController;
use crate::controller::general_command_handler::GeneralCommandHandler;
use crate::ui::general_command_parser::GeneralCommandParser;
use std::io::Write;

/// Controller that handles general command parsing and execution.
///
/// `GeneralCommandController` combines the general command parser and handler,
/// providing a unified interface for processing general application commands.
pub struct GeneralCommandController<W: Write> {
    handler: GeneralCommandHandler<W>,
    parser: GeneralCommandParser,
}

impl GeneralCommandController<std::io::Stdout> {
    /// Creates a new general command controller with stdout.
    pub fn new() -> Self {
        GeneralCommandController {
            handler: GeneralCommandHandler::new(),
            parser: GeneralCommandParser::new(),
        }
    }
}

impl<W: Write> GeneralCommandController<W> {
    /// Creates a new general command controller with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        GeneralCommandController {
            handler: GeneralCommandHandler::with_writer(writer),
            parser: GeneralCommandParser::new(),
        }
    }
}

impl<W: Write> CommandController for GeneralCommandController<W> {
    /// Attempts to parse and handle a general command from raw input.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string to parse
    ///
    /// # Returns
    ///
    /// * `Some(Ok(GeneralCommandResult))` - Command was successfully parsed and executed
    /// * `Some(Err(ParseError))` - Command was recognized as a general command but had an error
    /// * `None` - Not a general command, should try other parsers
    fn try_handle(&mut self, input: &str, _todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();

        match self.parser.try_parse_command(&command) {
            Some(general_command) => {
                let result = self.handler.handle(&general_command);
                Some(Ok(result))
            }
            None => None,
        }
    }
}

impl<W: Write> Default for GeneralCommandController<W>
where
    W: Write + Default,
{
    fn default() -> Self {
        Self::with_writer(W::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let _controller = GeneralCommandController::new();
    }

    #[test]
    fn test_try_handle_help_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("help", &mut todo_list);
        
        assert!(result.is_some());
        let cmd_result = result.unwrap();
        assert!(cmd_result.is_ok());
        assert_eq!(cmd_result.unwrap(), CommandControllerResult::Continue);
    }

    #[test]
    fn test_try_handle_quit_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("quit", &mut todo_list);
        
        assert!(result.is_some());
        let cmd_result = result.unwrap();
        assert!(cmd_result.is_ok());
        assert_eq!(cmd_result.unwrap(), CommandControllerResult::ExitMainLoop);
    }

    #[test]
    fn test_try_handle_help_alias() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("h", &mut todo_list);
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_try_handle_quit_alias() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("exit", &mut todo_list);
        
        assert!(result.is_some());
        let cmd_result = result.unwrap();
        assert!(cmd_result.is_ok());
        assert_eq!(cmd_result.unwrap(), CommandControllerResult::ExitMainLoop);
    }

    #[test]
    fn test_try_handle_debug_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("debug", &mut todo_list);
        
        assert!(result.is_some());
        let cmd_result = result.unwrap();
        assert!(cmd_result.is_ok());
        assert_eq!(cmd_result.unwrap(), CommandControllerResult::ToggleDebug);
    }

    #[test]
    fn test_try_handle_non_general_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();

        let result = controller.try_handle("add", &mut todo_list);
        
        assert!(result.is_none());
    }
}
