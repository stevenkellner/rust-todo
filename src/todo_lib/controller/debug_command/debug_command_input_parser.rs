use crate::controller::debug_command::DebugCommand;
use crate::models::parse_error::ParseError;

/// Parser for debug-related commands.
pub struct DebugCommandInputParser;

impl DebugCommandInputParser {
    /// Creates a new debug command parser.
    pub fn new() -> Self {
        DebugCommandInputParser
    }

    /// Tries to parse a debug command from the given command string and arguments.
    ///
    /// # Arguments
    ///
    /// * `command` - The command string (already lowercased)
    /// * `args` - The command arguments
    ///
    /// # Returns
    ///
    /// * `Some(Ok(DebugCommand))` - Successfully parsed debug command
    /// * `Some(Err(ParseError))` - Recognized as debug command but has errors
    /// * `None` - Not a debug command
    pub fn try_parse(&self, command: &str, args: &[&str]) -> Option<Result<DebugCommand, ParseError>> {
        match command {
            "debug:gen" => Some(self.parse_debug_generate_command(args)),
            "debug:clear" => Some(Ok(DebugCommand::ClearAll)),
            _ => None,
        }
    }

    /// Parses the 'debug:gen' command to generate random tasks.
    fn parse_debug_generate_command(&self, args: &[&str]) -> Result<DebugCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "debug:gen".to_string(), 
                usage: "debug:gen <count>".to_string() 
            })
        } else if let Ok(count) = args[0].parse::<usize>() {
            if count == 0 {
                Err(ParseError::OutOfRange { 
                    field: "count".to_string(), 
                    value: count.to_string(), 
                    range: "Must be greater than 0".to_string() 
                })
            } else if count > 1000 {
                Err(ParseError::OutOfRange { 
                    field: "count".to_string(), 
                    value: count.to_string(), 
                    range: "Cannot exceed 1000".to_string() 
                })
            } else {
                Ok(DebugCommand::GenerateTasks(count))
            }
        } else {
            Err(ParseError::InvalidId("Invalid count. Please provide a number.".to_string()))
        }
    }
}

impl Default for DebugCommandInputParser {
    fn default() -> Self {
        Self::new()
    }
}
