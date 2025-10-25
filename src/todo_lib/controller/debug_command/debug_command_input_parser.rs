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
            "debug:gen-projects" => Some(self.parse_debug_generate_projects_command(args)),
            "debug:clear-projects" => Some(Ok(DebugCommand::ClearAllProjects)),
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

    /// Parses the 'debug:gen-projects' command to generate random projects with tasks.
    fn parse_debug_generate_projects_command(&self, args: &[&str]) -> Result<DebugCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "debug:gen-projects".to_string(), 
                usage: "debug:gen-projects <project_count> <tasks_per_project>".to_string() 
            })
        } else if let (Ok(project_count), Ok(tasks_per_project)) = 
            (args[0].parse::<usize>(), args[1].parse::<usize>()) {
            
            if project_count == 0 {
                Err(ParseError::OutOfRange { 
                    field: "project_count".to_string(), 
                    value: project_count.to_string(), 
                    range: "Must be greater than 0".to_string() 
                })
            } else if project_count > 50 {
                Err(ParseError::OutOfRange { 
                    field: "project_count".to_string(), 
                    value: project_count.to_string(), 
                    range: "Cannot exceed 50".to_string() 
                })
            } else if tasks_per_project > 100 {
                Err(ParseError::OutOfRange { 
                    field: "tasks_per_project".to_string(), 
                    value: tasks_per_project.to_string(), 
                    range: "Cannot exceed 100".to_string() 
                })
            } else {
                Ok(DebugCommand::GenerateProjects { project_count, tasks_per_project })
            }
        } else {
            Err(ParseError::InvalidId("Invalid count. Please provide numbers.".to_string()))
        }
    }
}

impl Default for DebugCommandInputParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_debug_gen() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:gen", &["10"]);
        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), DebugCommand::GenerateTasks(10));
    }

    #[test]
    fn test_parse_debug_clear() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:clear", &[]);
        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), DebugCommand::ClearAll);
    }

    #[test]
    fn test_parse_debug_gen_projects() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:gen-projects", &["5", "10"]);
        assert!(result.is_some());
        assert_eq!(
            result.unwrap().unwrap(), 
            DebugCommand::GenerateProjects { project_count: 5, tasks_per_project: 10 }
        );
    }

    #[test]
    fn test_parse_debug_clear_projects() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:clear-projects", &[]);
        assert!(result.is_some());
        assert_eq!(result.unwrap().unwrap(), DebugCommand::ClearAllProjects);
    }

    #[test]
    fn test_parse_debug_gen_projects_missing_args() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:gen-projects", &["5"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_parse_debug_gen_projects_invalid_count() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:gen-projects", &["0", "10"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_parse_debug_gen_projects_exceeds_limit() {
        let parser = DebugCommandInputParser::new();
        let result = parser.try_parse("debug:gen-projects", &["100", "10"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }
}
