use crate::controller::command_controller::CommandController;
use crate::controller::general_command::GeneralCommand;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::ParseError;
use crate::controller::general_command::GeneralCommandInputParser;
use crate::controller::general_command::GeneralCommandOutputManager;
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Handler for general application commands.
///
/// `GeneralCommandController` encapsulates all operations related to general
/// application control such as showing help, handling quit, and unknown commands.
pub struct GeneralCommandController<O: OutputWriter> {
    input_parser: GeneralCommandInputParser,
    output_manager: GeneralCommandOutputManager<O>,
    debug_mode: bool,
}

impl<O: OutputWriter> GeneralCommandController<O> {
    /// Creates a new general command handler with a custom output writer.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            input_parser: GeneralCommandInputParser::new(),
            output_manager: GeneralCommandOutputManager::new(output_writer),
            debug_mode: false,
        }
    }

    /// Handles a general command
    fn handle_command(&mut self, command: &GeneralCommand) -> CommandControllerResult {
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
        self.output_manager.show_help();
    }

    /// Handles the quit command.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::ExitMainLoop` - Always exits after quit command
    fn handle_quit(&mut self) -> CommandControllerResult {
        self.output_manager.show_goodbye();
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
            self.output_manager.show_debug_enabled();
            CommandControllerResult::EnableDebugMode
        } else {
            self.output_manager.show_debug_disabled();
            CommandControllerResult::DisableDebugMode
        }
    }
}

impl<O: OutputWriter> CommandController for GeneralCommandController<O> {
    fn try_execute(&mut self, input: &str) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.input_parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd);
                Some(Ok(result))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::output::FileOutputWriter;

    #[test]
    fn test_new_handler() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let handler = GeneralCommandController::new(Rc::new(RefCell::new(output_writer)));
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }

    #[test]
    fn test_show_help_returns_continue() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut handler = GeneralCommandController::new(Rc::new(RefCell::new(output_writer)));
        let result = handler.handle_command(&GeneralCommand::ShowHelp);
        assert_eq!(result, CommandControllerResult::Continue);
    }

    #[test]
    fn test_quit_returns_exit() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut handler = GeneralCommandController::new(Rc::new(RefCell::new(output_writer)));
        let result = handler.handle_command(&GeneralCommand::Quit);
        assert_eq!(result, CommandControllerResult::ExitMainLoop);
    }

    #[test]
    fn test_toggle_debug_returns_toggle_debug() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut handler = GeneralCommandController::new(Rc::new(RefCell::new(output_writer)));
        let result = handler.handle_command(&GeneralCommand::ToggleDebug);
        assert_eq!(result, CommandControllerResult::EnableDebugMode);
        let result = handler.handle_command(&GeneralCommand::ToggleDebug);
        assert_eq!(result, CommandControllerResult::DisableDebugMode);
    }
}
