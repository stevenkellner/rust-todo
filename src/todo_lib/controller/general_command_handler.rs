use crate::models::general_command::GeneralCommand;
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

    /// Handles a general command and returns the loop control state.
    ///
    /// # Arguments
    ///
    /// * `command` - The general command to handle
    ///
    /// # Returns
    ///
    /// * `LoopControl` - Whether to continue or exit the application loop
    pub fn handle(
        &mut self,
        command: &GeneralCommand,
    ) -> LoopControl {
        match command {
            GeneralCommand::ShowHelp => self.show_help(),
            GeneralCommand::Quit => self.handle_quit(),
            GeneralCommand::Unknown(cmd) => self.handle_unknown_command(cmd),
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

    /// Handles an unknown command.
    ///
    /// # Arguments
    ///
    /// * `cmd` - The unknown command string
    ///
    /// # Returns
    ///
    /// * `LoopControl::Continue` - Always continues after unknown command
    fn handle_unknown_command(
        &mut self,
        cmd: &str,
    ) -> LoopControl {
        self.output.show_unknown_command(cmd);
        LoopControl::Continue
    }

    /// Displays the welcome message.
    pub fn show_welcome(&mut self) {
        self.output.show_welcome();
    }

    /// Prints the command prompt.
    pub fn print_prompt(&mut self) {
        self.output.print_prompt();
    }

    /// Shows an error message.
    pub fn show_error(&mut self, message: &str) {
        self.output.show_error(message);
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
        assert_eq!(result, LoopControl::Continue);
    }

    #[test]
    fn test_quit_returns_exit() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::Quit);
        assert_eq!(result, LoopControl::Exit);
    }

    #[test]
    fn test_unknown_command_returns_continue() {
        let mut handler = create_test_handler();
        let result = handler.handle(&GeneralCommand::Unknown("invalid".to_string()));
        assert_eq!(result, LoopControl::Continue);
    }

    #[test]
    fn test_default_trait() {
        let handler = GeneralCommandHandler::default();
        // Handler now has output field
        assert!(std::mem::size_of_val(&handler) > 0);
    }
}
