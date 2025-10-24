use crate::models::general_command::GeneralCommand;

/// Parser for general application commands.
pub struct GeneralCommandParser;

impl GeneralCommandParser {
    /// Creates a new general command parser.
    pub fn new() -> Self {
        GeneralCommandParser
    }

    /// Tries to parse a general command from the given command string.
    ///
    /// # Arguments
    ///
    /// * `command` - The command string (already lowercased)
    ///
    /// # Returns
    ///
    /// * `Some(GeneralCommand)` - Successfully parsed general command
    /// * `None` - Not a general command
    pub fn try_parse_command(&self, command: &str) -> Option<GeneralCommand> {
        match command {
            "help" | "h" => Some(GeneralCommand::ShowHelp),
            "quit" | "exit" | "q" => Some(GeneralCommand::Quit),
            "debug" => Some(GeneralCommand::ToggleDebug),
            _ => None,
        }
    }
}

impl Default for GeneralCommandParser {
    fn default() -> Self {
        Self::new()
    }
}
