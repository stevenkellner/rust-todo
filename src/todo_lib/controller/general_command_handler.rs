use crate::models::general_command::GeneralCommand;
use crate::models::command_controller_result::CommandControllerResult;
use crate::ui::GeneralCommandOutputWriter;
use std::io::Write;

/// Handler for general application commands.
///
/// `GeneralCommandHandler` encapsulates all operations related to general
/// application control such as showing help, handling quit, and unknown commands.
pub struct GeneralCommandHandler<W: Write> {
    output: GeneralCommandOutputWriter<W>,
    debug_mode: bool,
}

impl GeneralCommandHandler<std::io::Stdout> {
    /// Creates a new general command handler with stdout.
    pub fn new() -> Self {
        GeneralCommandHandler {
            output: GeneralCommandOutputWriter::new(),
            debug_mode: false,
        }
    }
}

impl<W: Write> GeneralCommandHandler<W> {
    /// Creates a new general command handler with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        GeneralCommandHandler {
            output: GeneralCommandOutputWriter::with_writer(writer),
            debug_mode: false,
        }
    }

    /// Handles a general command and returns the result.
    ///
    /// # Arguments
    ///
    /// * `command` - The general command to handle
    ///
    /// # Returns
    ///
    /// * `GeneralCommandResult` - Result indicating action to take
    pub fn handle(
        &mut self,
        command: &GeneralCommand,
    ) -> CommandControllerResult {
        match command {
            GeneralCommand::ShowHelp => self.show_help(),
            GeneralCommand::Quit => self.handle_quit(),
            GeneralCommand::ToggleDebug => self.handle_toggle_debug(),
        }
    }

    /// Shows the help message.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::Continue` - Always continues after showing help
    fn show_help(
        &mut self,
    ) -> CommandControllerResult {
        self.output.show_help();
        CommandControllerResult::Continue
    }

    /// Handles the quit command.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::ExitMainLoop` - Always exits after quit command
    fn handle_quit(
        &mut self,
    ) -> CommandControllerResult {
        self.output.show_goodbye();
        CommandControllerResult::ExitMainLoop
    }

    /// Handles the toggle debug command.
    ///
    /// # Returns
    ///
    /// * `CommandControllerResult::EnableDebugMode` - Indicates that debug mode has been enabled
    /// * `CommandControllerResult::DisableDebugMode` - Indicates that debug mode has been disabled
    fn handle_toggle_debug(
        &mut self,
    ) -> CommandControllerResult {
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

impl Default for GeneralCommandHandler<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> GeneralCommandHandler<Vec<u8>> {
        GeneralCommandHandler::with_writer(Vec::new())
    }

    #[test]
    fn test_new_handler() {
        let handler = GeneralCommandHandler::new();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }

    #[test]
    fn test_show_help_returns_continue() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::ShowHelp);
        assert_eq!(result, CommandControllerResult::Continue);
    }

    #[test]
    fn test_quit_returns_exit() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::Quit);
        assert_eq!(result, CommandControllerResult::ExitMainLoop);
    }

    #[test]
    fn test_toggle_debug_returns_toggle_debug() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::ToggleDebug);
        assert_eq!(result, CommandControllerResult::EnableDebugMode);
        let result = handler.handle(&GeneralCommand::ToggleDebug);
        assert_eq!(result, CommandControllerResult::DisableDebugMode);
    }

    #[test]
    fn test_default_trait() {
        let handler = GeneralCommandHandler::default();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }
}
