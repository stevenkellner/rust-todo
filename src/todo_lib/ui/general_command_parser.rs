use crate::models::general_command::GeneralCommand;
use crate::models::parse_error::ParseError;

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
    pub fn try_parse(&self, command: &str, _args: &[&str]) -> Option<Result<GeneralCommand, ParseError>> {
        match command {
            "help" | "h" => Some(Ok(GeneralCommand::ShowHelp)),
            "quit" | "exit" | "q" => Some(Ok(GeneralCommand::Quit)),
            "debug" => Some(Ok(GeneralCommand::ToggleDebug)),
            _ => None,
        }
    }
}

impl Default for GeneralCommandParser {
    fn default() -> Self {
        Self::new()
    }
}
