use crate::models::parse_error::ParseError;
use crate::models::loop_control::LoopControl;
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

    /// Shows the welcome message.
    pub fn show_welcome(&mut self) {
        self.handler.show_welcome();
    }

    /// Prints the command prompt.
    pub fn print_prompt(&mut self) {
        self.handler.print_prompt();
    }

    /// Shows an error message.
    pub fn show_error(&mut self, message: &str) {
        self.handler.show_error(message);
    }

    /// Handles an unknown command by displaying an error message.
    pub fn handle_unknown_command(&mut self, command: &str) {
        self.handler.handle_unknown_command(command);
    }

    /// Attempts to parse and handle a general command from raw input.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string to parse
    ///
    /// # Returns
    ///
    /// * `Some(Ok(LoopControl))` - Command was successfully parsed and executed
    /// * `Some(Err(ParseError))` - Command was recognized as a general command but had an error
    /// * `None` - Not a general command, should try other parsers
    pub fn try_handle(
        &mut self,
        input: &str,
    ) -> Option<Result<LoopControl, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();

        match self.parser.try_parse_command(&command) {
            Some(general_command) => {
                let control = self.handler.handle(&general_command);
                Some(Ok(control))
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

        let result = controller.try_handle("help");
        
        assert!(result.is_some());
        let control = result.unwrap();
        assert!(control.is_ok());
        assert_eq!(control.unwrap(), LoopControl::Continue);
    }

    #[test]
    fn test_try_handle_quit_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());

        let result = controller.try_handle("quit");
        
        assert!(result.is_some());
        let control = result.unwrap();
        assert!(control.is_ok());
        assert_eq!(control.unwrap(), LoopControl::Exit);
    }

    #[test]
    fn test_try_handle_help_alias() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());

        let result = controller.try_handle("h");
        
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_try_handle_quit_alias() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());

        let result = controller.try_handle("exit");
        
        assert!(result.is_some());
        let control = result.unwrap();
        assert!(control.is_ok());
        assert_eq!(control.unwrap(), LoopControl::Exit);
    }

    #[test]
    fn test_try_handle_non_general_command() {
        let mut controller = GeneralCommandController::with_writer(Vec::new());

        let result = controller.try_handle("add");
        
        assert!(result.is_none());
    }
}
