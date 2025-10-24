use crate::controller::command_controller::CommandController;
use crate::controller::general_command::general_command::GeneralCommand;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::ParseError;
use crate::controller::general_command::general_command_parser::GeneralCommandParser;
use crate::controller::general_command::GeneralCommandOutputWriter;
use crate::TodoList;
use std::io::Write;

/// Handler for general application commands.
///
/// `GeneralCommandController` encapsulates all operations related to general
/// application control such as showing help, handling quit, and unknown commands.
pub struct GeneralCommandController<W: Write> {
    parser: GeneralCommandParser,
    output: GeneralCommandOutputWriter<W>,
    debug_mode: bool,
}

impl GeneralCommandController<std::io::Stdout> {
    /// Creates a new general command handler with stdout.
    pub fn new() -> Self {
        GeneralCommandController {
            parser: GeneralCommandParser::new(),
            output: GeneralCommandOutputWriter::new(),
            debug_mode: false,
        }
    }
}

impl<W: Write> GeneralCommandController<W> {
    /// Creates a new general command handler with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        GeneralCommandController {
            parser: GeneralCommandParser::new(),
            output: GeneralCommandOutputWriter::with_writer(writer),
            debug_mode: false,
        }
    }

    /// Handles a general command
    fn handle_command(&mut self, command: &GeneralCommand, _todo_list: &mut TodoList) -> CommandControllerResult {
        match command {
            GeneralCommand::ShowHelp => self.show_help(),
            GeneralCommand::Quit => return self.handle_quit(),
            GeneralCommand::ToggleDebug => return self.handle_toggle_debug(),
        }
        CommandControllerResult::Continue
    }

    /// Shows the help message.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::Continue` - Always continues after showing help
    fn show_help(&mut self) {
        self.output.show_help();
    }

    /// Handles the quit command.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::ExitMainLoop` - Always exits after quit command
    fn handle_quit(&mut self) -> CommandControllerResult {
        self.output.show_goodbye();
        CommandControllerResult::ExitMainLoop
    }

    /// Handles the toggle debug command.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::EnableDebugMode` - Indicates that debug mode has been enabled
    /// * `CommandControllerResult::DisableDebugMode` - Indicates that debug mode has been disabled
    fn handle_toggle_debug(&mut self) -> CommandControllerResult {
        self.debug_mode = !self.debug_mode;
        if self.debug_mode {
            self.output.show_debug_enabled();
            CommandControllerResult::EnableDebugMode
        } else {
            self.output.show_debug_disabled();
            CommandControllerResult::DisableDebugMode
        }
    }
}

impl<W: Write> CommandController for GeneralCommandController<W> {
    fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd, todo_list);
                Some(Ok(result))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

impl Default for GeneralCommandController<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> GeneralCommandController<Vec<u8>> {
        GeneralCommandController::with_writer(Vec::new())
    }

    #[test]
    fn test_new_handler() {
        let handler = GeneralCommandController::new();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }

    #[test]
    fn test_show_help_returns_continue() {
        let mut handler = create_test_handler();
        let mut todo_list = TodoList::new();
        let result = handler.handle_command(&GeneralCommand::ShowHelp, &mut todo_list);
        assert_eq!(result, CommandControllerResult::Continue);
    }

    #[test]
    fn test_quit_returns_exit() {
        let mut handler = create_test_handler();
        let mut todo_list = TodoList::new();
        let result = handler.handle_command(&GeneralCommand::Quit, &mut todo_list);
        assert_eq!(result, CommandControllerResult::ExitMainLoop);
    }

    #[test]
    fn test_toggle_debug_returns_toggle_debug() {
        let mut handler = create_test_handler();
        let mut todo_list = TodoList::new();
        let result = handler.handle_command(&GeneralCommand::ToggleDebug, &mut todo_list);
        assert_eq!(result, CommandControllerResult::EnableDebugMode);
        let result = handler.handle_command(&GeneralCommand::ToggleDebug, &mut todo_list);
        assert_eq!(result, CommandControllerResult::DisableDebugMode);
    }

    #[test]
    fn test_default_trait() {
        let handler = GeneralCommandController::default();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }
}
