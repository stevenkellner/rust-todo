use crate::models::task_command::TaskCommand;
use crate::models::filter_builder::FilterBuilder;
use crate::models::priority::Priority;
use crate::models::parse_error::ParseError;
use chrono::NaiveDate;

/// Parser for task-related commands.
pub struct TaskCommandParser;

impl TaskCommandParser {
    /// Creates a new task command parser.
    pub fn new() -> Self {
        TaskCommandParser
    }

    /// Tries to parse a task command from the given command string and arguments.
    ///
    /// # Arguments
    ///
    /// * `command` - The command string (already lowercased)
    /// * `args` - The command arguments
    ///
    /// # Returns
    ///
    /// * `Some(Ok(TaskCommand))` - Successfully parsed task command
    /// * `Some(Err(ParseError))` - Recognized as task command but has errors
    /// * `None` - Not a task command
    pub fn try_parse(&self, command: &str, args: &[&str]) -> Option<Result<TaskCommand, ParseError>> {
        match command {
            "add" => Some(self.parse_add_command(args)),
            "list" => Some(self.parse_list_command(args)),
            "remove" | "delete" | "rm" => Some(self.parse_remove_command(args)),
            "complete" | "done" => Some(self.parse_complete_command(args)),
            "uncomplete" | "undo" => Some(self.parse_uncomplete_command(args)),
            "toggle" => Some(self.parse_toggle_command(args)),
            "priority" | "pri" => Some(self.parse_priority_command(args)),
            "set-due" | "due" => Some(self.parse_set_due_command(args)),
            "set-category" | "category" | "cat" => Some(self.parse_set_category_command(args)),
            "categories" | "list-categories" => Some(Ok(TaskCommand::ListCategories)),
            "edit" => Some(self.parse_edit_command(args)),
            "search" | "find" => Some(self.parse_search_command(args)),
            "statistics" | "stats" => Some(Ok(TaskCommand::ShowStatistics)),
            _ => None,
        }
    }

    /// Parses the 'add' command and validates the task description.
    fn parse_add_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "add".to_string(), 
                usage: "add <task description>".to_string() 
            })
        } else {
            let description = args.join(" ");
            if description.is_empty() {
                Err(ParseError::EmptyInput("Task description".to_string()))
            } else {
                Ok(TaskCommand::Add(description))
            }
        }
    }

    /// Parses the 'list' command with optional filter arguments.
    fn parse_list_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let mut filter_builder = FilterBuilder::new();
        
        for arg in args {
            let lower = arg.to_lowercase();
            filter_builder = match filter_builder.parse_argument(&lower) {
                Ok(builder) => builder,
                Err(err) => return Err(ParseError::InvalidFormat { 
                    field: "filter".to_string(),
                    expected: "status (completed/pending/overdue), priority (high/medium/low), or category:name".to_string(),
                    actual: err 
                }),
            };
        }
        
        Ok(TaskCommand::List(filter_builder.build()))
    }

    /// Parses the 'remove' command with task ID validation.
    fn parse_remove_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "remove".to_string(), 
                usage: "remove <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(TaskCommand::Remove(id))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'complete' command.
    fn parse_complete_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "complete".to_string(), 
                usage: "complete <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(TaskCommand::Complete(id))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'uncomplete' command.
    fn parse_uncomplete_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "uncomplete".to_string(), 
                usage: "uncomplete <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(TaskCommand::Uncomplete(id))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'toggle' command.
    fn parse_toggle_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "toggle".to_string(), 
                usage: "toggle <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(TaskCommand::Toggle(id))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'priority' command with priority level validation.
    fn parse_priority_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "priority".to_string(), 
                usage: "priority <task id> <priority level (high/h, medium/med/m, low/l)>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let priority_str = args[1].to_lowercase();
            match Priority::from_str(&priority_str) {
                Some(priority) => Ok(TaskCommand::SetPriority(id, priority)),
                None => Err(ParseError::InvalidValue { 
                    field: "priority level".to_string(), 
                    value: priority_str, 
                    allowed: "high/h, medium/med/m, or low/l".to_string() 
                }),
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'set-due' command with date validation.
    fn parse_set_due_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "set-due".to_string(), 
                usage: "set-due <task id> <date (DD.MM.YYYY) or 'none' to clear>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let date_str = args[1];
            
            if date_str.to_lowercase() == "none" {
                return Ok(TaskCommand::SetDueDate(id, None));
            }
            
            let parts: Vec<&str> = date_str.split('.').collect();
            if parts.len() != 3 {
                return Err(ParseError::InvalidFormat { 
                    field: "date".to_string(), 
                    expected: "DD.MM.YYYY (e.g., 31.12.2024)".to_string(), 
                    actual: date_str.to_string() 
                });
            }

            if let (Ok(day), Ok(month), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<i32>(),
            ) {
                match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(date) => Ok(TaskCommand::SetDueDate(id, Some(date))),
                    None => Err(ParseError::InvalidDate("Invalid date. Please check the date is valid.".to_string())),
                }
            } else {
                Err(ParseError::InvalidFormat { 
                    field: "date values".to_string(), 
                    expected: "DD.MM.YYYY".to_string(), 
                    actual: date_str.to_string() 
                })
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'set-category' command.
    fn parse_set_category_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "set-category".to_string(), 
                usage: "set-category <task id> <category name or 'none' to clear>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let category = args[1..].join(" ");
            
            if category.to_lowercase() == "none" {
                Ok(TaskCommand::SetCategory(id, None))
            } else if category.is_empty() {
                Err(ParseError::EmptyInput("Category name".to_string()))
            } else {
                Ok(TaskCommand::SetCategory(id, Some(category)))
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'search' command.
    fn parse_search_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "search".to_string(), 
                usage: "search <keyword>".to_string() 
            })
        } else {
            let keyword = args.join(" ");
            if keyword.is_empty() {
                Err(ParseError::EmptyInput("Search keyword".to_string()))
            } else {
                Ok(TaskCommand::Search(keyword))
            }
        }
    }

    /// Parses the 'edit' command.
    fn parse_edit_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "edit".to_string(), 
                usage: "edit <task id> <new description>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let description = args[1..].join(" ");
            if description.is_empty() {
                Err(ParseError::EmptyInput("Task description".to_string()))
            } else {
                Ok(TaskCommand::Edit(id, description))
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }
}

impl Default for TaskCommandParser {
    fn default() -> Self {
        Self::new()
    }
}
