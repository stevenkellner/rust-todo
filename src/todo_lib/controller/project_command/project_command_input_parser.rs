use crate::controller::project_command::ProjectCommand;
use crate::models::parse_error::ParseError;

/// Parser for project-related commands.
pub struct ProjectCommandInputParser;

impl ProjectCommandInputParser {
    /// Creates a new project command parser.
    pub fn new() -> Self {
        ProjectCommandInputParser
    }

    /// Tries to parse a project command from the given command string and arguments.
    ///
    /// # Arguments
    ///
    /// * `command` - The command string (already lowercased)
    /// * `args` - The command arguments
    ///
    /// # Returns
    ///
    /// * `Some(Ok(ProjectCommand))` - Successfully parsed project command
    /// * `Some(Err(ParseError))` - Recognized as project command but has errors
    /// * `None` - Not a project command
    pub fn try_parse(&self, command: &str, args: &[&str]) -> Option<Result<ProjectCommand, ParseError>> {
        match command {
            "new-project" | "new-proj" | "create-project" => Some(self.parse_new_project_command(args)),
            "switch-project" | "switch" | "use-project" => Some(self.parse_switch_project_command(args)),
            "list-projects" | "projects" => Some(Ok(ProjectCommand::ListProjects)),
            "delete-project" | "remove-project" | "rm-project" => Some(self.parse_delete_project_command(args)),
            "rename-project" | "mv-project" => Some(self.parse_rename_project_command(args)),
            _ => None,
        }
    }

    /// Parses the 'new-project' command.
    fn parse_new_project_command(&self, args: &[&str]) -> Result<ProjectCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "new-project".to_string(), 
                usage: "new-project <name>".to_string() 
            })
        } else {
            let name = args.join(" ");
            if name.trim().is_empty() {
                Err(ParseError::EmptyInput("Project name".to_string()))
            } else {
                Ok(ProjectCommand::NewProject(name))
            }
        }
    }

    /// Parses the 'switch-project' command.
    fn parse_switch_project_command(&self, args: &[&str]) -> Result<ProjectCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "switch-project".to_string(), 
                usage: "switch-project <name>".to_string() 
            })
        } else {
            let name = args.join(" ");
            if name.trim().is_empty() {
                Err(ParseError::EmptyInput("Project name".to_string()))
            } else {
                Ok(ProjectCommand::SwitchProject(name))
            }
        }
    }

    /// Parses the 'delete-project' command.
    fn parse_delete_project_command(&self, args: &[&str]) -> Result<ProjectCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "delete-project".to_string(), 
                usage: "delete-project <name>".to_string() 
            })
        } else {
            let name = args.join(" ");
            if name.trim().is_empty() {
                Err(ParseError::EmptyInput("Project name".to_string()))
            } else {
                Ok(ProjectCommand::DeleteProject(name))
            }
        }
    }

    /// Parses the 'rename-project' command.
    fn parse_rename_project_command(&self, args: &[&str]) -> Result<ProjectCommand, ParseError> {
        if args.len() < 2 {
            return Err(ParseError::MissingArguments { 
                command: "rename-project".to_string(), 
                usage: "rename-project <old_name> <new_name>".to_string() 
            });
        }

        let old_name = args[0].to_string();
        let new_name = args[1..].join(" ");

        if old_name.trim().is_empty() || new_name.trim().is_empty() {
            return Err(ParseError::EmptyInput("Project name".to_string()));
        }

        Ok(ProjectCommand::RenameProject(old_name, new_name))
    }
}

impl Default for ProjectCommandInputParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_new_project() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("new-project", &["Work"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_parse_new_project_missing_args() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("new-project", &[]);
        assert!(result.is_some());
        assert!(result.unwrap().is_err());
    }

    #[test]
    fn test_parse_switch_project() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("switch", &["Personal"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_parse_list_projects() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("projects", &[]);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_parse_delete_project() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("delete-project", &["Old"]);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_parse_rename_project() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("rename-project", &["old", "new"]);
        assert!(result.is_some());
        match result.unwrap() {
            Ok(ProjectCommand::RenameProject(old, new)) => {
                assert_eq!(old, "old");
                assert_eq!(new, "new");
            }
            _ => panic!("Expected RenameProject command"),
        }
    }

    #[test]
    fn test_parse_rename_project_multi_word() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("rename-project", &["old", "new", "name"]);
        assert!(result.is_some());
        match result.unwrap() {
            Ok(ProjectCommand::RenameProject(old, new)) => {
                assert_eq!(old, "old");
                assert_eq!(new, "new name");
            }
            _ => panic!("Expected RenameProject command"),
        }
    }

    #[test]
    fn test_unrecognized_command() {
        let parser = ProjectCommandInputParser::new();
        let result = parser.try_parse("unknown", &[]);
        assert!(result.is_none());
    }
}
