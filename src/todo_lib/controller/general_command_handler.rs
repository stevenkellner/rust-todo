use crate::models::general_command::GeneralCommand;
use crate::models::general_command_result::GeneralCommandResult;
use crate::models::loop_control::LoopControl;
use crate::ui::GeneralCommandOutputWriter;
use std::io::Write;

/// Handler for general application commands.
///
/// `GeneralCommandHandler` encapsulates all operations related to general
/// application control such as showing help, handling quit, and unknown commands.
pub struct GeneralCommandHandler<W: Write> {
    output: GeneralCommandOutputWriter<W>,
}

impl GeneralCommandHandler<std::io::Stdout> {
    /// Creates a new general command handler with stdout.
    pub fn new() -> Self {
        GeneralCommandHandler {
            output: GeneralCommandOutputWriter::new(),
        }
    }
}

impl<W: Write> GeneralCommandHandler<W> {
    /// Creates a new general command handler with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        GeneralCommandHandler {
            output: GeneralCommandOutputWriter::with_writer(writer),
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
    ) -> GeneralCommandResult {
        match command {
            GeneralCommand::ShowHelp => GeneralCommandResult::Continue(self.show_help()),
            GeneralCommand::Quit => GeneralCommandResult::Continue(self.handle_quit()),
            GeneralCommand::ToggleDebug => GeneralCommandResult::ToggleDebug,
        }
    }

    /// Shows the help message.
    ///
    /// # Returns
    ///
    /// * `LoopControl::Continue` - Always continues after showing help
    fn show_help(
        &mut self,
    ) -> LoopControl {
        self.output.show_help();
        LoopControl::Continue
    }

    /// Handles the quit command.
    ///
    /// # Returns
    ///
    /// * `LoopControl::Exit` - Always exits after quit command
    fn handle_quit(
        &mut self,
    ) -> LoopControl {
        self.output.show_goodbye();
        LoopControl::Exit
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
        assert_eq!(result, GeneralCommandResult::Continue(LoopControl::Continue));
    }

    #[test]
    fn test_quit_returns_exit() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::Quit);
        assert_eq!(result, GeneralCommandResult::Continue(LoopControl::Exit));
    }

    #[test]
    fn test_toggle_debug_returns_toggle_debug() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::ToggleDebug);
        assert_eq!(result, GeneralCommandResult::ToggleDebug);
    }

    #[test]
    fn test_default_trait() {
        let handler = GeneralCommandHandler::default();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }
}
