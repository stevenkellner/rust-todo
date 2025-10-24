use crate::ui::input_reader::InputReader;
use crate::models::ui_event::UiEvent;
use crate::models::filter_builder::FilterBuilder;
use crate::models::priority::Priority;
use crate::models::task_command::TaskCommand;
use crate::models::debug_command::DebugCommand;
use crate::models::general_command::GeneralCommand;
use crate::models::parse_error::ParseError;
use std::io::Read;

/// Parses user input into UI events.
///
/// `CommandParser` is responsible for reading input using InputReader
/// and parsing commands into events that can be processed by the application.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::ui::input_reader::InputReader;
/// use todo_manager::ui::command_parser::CommandParser;
///
/// let input = InputReader::new();
/// let mut parser = CommandParser::new(input);
/// let event = parser.read_event();
/// ```
pub struct CommandParser<R: Read> {
    input: InputReader<R>,
}

impl CommandParser<std::io::Stdin> {
    /// Creates a new command parser that reads from stdin.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::command_parser::CommandParser;
    ///
    /// let mut parser = CommandParser::new_stdin();
    /// ```
    pub fn new_stdin() -> Self {
        CommandParser {
            input: InputReader::new(),
        }
    }
}

impl<R: Read> CommandParser<R> {
    /// Creates a new command parser with a custom input reader for testing.
    ///
    /// # Arguments
    ///
    /// * `input` - An InputReader instance
    pub fn new(input: InputReader<R>) -> Self {
        CommandParser { input }
    }

    /// Reads user input and parses it into a UI event.
    ///
    /// # Returns
    ///
    /// A `Result` containing either a `UiEvent` or a `ParseError`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::input_reader::InputReader;
    /// use todo_manager::ui::command_parser::CommandParser;
    ///
    /// let input = InputReader::new();
    /// let mut parser = CommandParser::new(input);
    /// let result = parser.read_event();
    /// ```
    pub fn read_event(&mut self) -> Result<UiEvent, ParseError> {
        let input = self.input.read_input();
        self.parse_command(&input)
    }

    /// Parses a command string into a UI event.
    fn parse_command(&self, input: &str) -> Result<UiEvent, ParseError> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return Err(ParseError::EmptyCommand("Please enter a command. Type 'help' for available commands.".to_string()));
        }

        let command = parts[0].to_lowercase();
        let args: Vec<&str> = parts[1..].to_vec();

        match command.as_str() {
            "add" => self.parse_add_command(&args),
            "list" => self.parse_list_command(&args),
            "remove" | "delete" | "rm" => self.parse_remove_command(&args),
            "complete" | "done" => self.parse_complete_command(&args),
            "uncomplete" | "undo" => self.parse_uncomplete_command(&args),
            "toggle" => self.parse_toggle_command(&args),
            "priority" | "pri" => self.parse_priority_command(&args),
            "set-due" | "due" => self.parse_set_due_command(&args),
            "set-category" | "category" | "cat" => self.parse_set_category_command(&args),
            "categories" | "list-categories" => Ok(UiEvent::Task(TaskCommand::ListCategories)),
            "edit" => self.parse_edit_command(&args),
            "search" | "find" => self.parse_search_command(&args),
            "statistics" | "stats" => Ok(UiEvent::Task(TaskCommand::ShowStatistics)),
            "debug" => self.parse_debug_command(&args),
            "debug:gen" => self.parse_debug_generate_command(&args),
            "debug:clear" => Ok(UiEvent::Debug(DebugCommand::ClearAll)),
            "help" | "h" => Ok(UiEvent::General(GeneralCommand::ShowHelp)),
            "quit" | "exit" | "q" => Ok(UiEvent::General(GeneralCommand::Quit)),
            _ => Ok(UiEvent::General(GeneralCommand::Unknown(command))),
        }
    }

    /// Parses the 'add' command and validates the task description.
    fn parse_add_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
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
                Ok(UiEvent::Task(TaskCommand::Add(description)))
            }
        }
    }

    /// Parses the 'list' command with optional filter arguments.
    fn parse_list_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
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
        
        Ok(UiEvent::Task(TaskCommand::List(filter_builder.build())))
    }

    /// Parses the 'remove' command with task ID validation.
    fn parse_remove_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "remove".to_string(), 
                usage: "remove <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(UiEvent::Task(TaskCommand::Remove(id)))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'complete' command.
    fn parse_complete_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "complete".to_string(), 
                usage: "complete <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(UiEvent::Task(TaskCommand::Complete(id)))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'uncomplete' command.
    fn parse_uncomplete_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "uncomplete".to_string(), 
                usage: "uncomplete <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(UiEvent::Task(TaskCommand::Uncomplete(id)))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'toggle' command.
    fn parse_toggle_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments { 
                command: "toggle".to_string(), 
                usage: "toggle <task id>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(UiEvent::Task(TaskCommand::Toggle(id)))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'priority' command with priority level validation.
    fn parse_priority_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "priority".to_string(), 
                usage: "priority <task id> <priority level (high/h, medium/med/m, low/l)>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let priority_str = args[1].to_lowercase();
            match Priority::from_str(&priority_str) {
                Some(priority) => Ok(UiEvent::Task(TaskCommand::SetPriority(id, priority))),
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
    fn parse_set_due_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "set-due".to_string(), 
                usage: "set-due <task id> <date (DD.MM.YYYY) or 'none' to clear>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let date_str = args[1];
            
            if date_str.to_lowercase() == "none" {
                return Ok(UiEvent::Task(TaskCommand::SetDueDate(id, None)));
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
                use chrono::NaiveDate;
                match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(date) => Ok(UiEvent::Task(TaskCommand::SetDueDate(id, Some(date)))),
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
    fn parse_set_category_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments { 
                command: "set-category".to_string(), 
                usage: "set-category <task id> <category name or 'none' to clear>".to_string() 
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let category = args[1..].join(" ");
            
            if category.to_lowercase() == "none" {
                Ok(UiEvent::Task(TaskCommand::SetCategory(id, None)))
            } else if category.is_empty() {
                Err(ParseError::EmptyInput("Category name".to_string()))
            } else {
                Ok(UiEvent::Task(TaskCommand::SetCategory(id, Some(category))))
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'search' command.
    fn parse_search_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
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
                Ok(UiEvent::Task(TaskCommand::Search(keyword)))
            }
        }
    }

    /// Parses the 'edit' command.
    fn parse_edit_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
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
                Ok(UiEvent::Task(TaskCommand::Edit(id, description)))
            }
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'debug' command.
    fn parse_debug_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
        if args.is_empty() || args[0].to_lowercase() == "on" {
            Ok(UiEvent::Debug(DebugCommand::Toggle))
        } else if args[0].to_lowercase() == "off" {
            Ok(UiEvent::Debug(DebugCommand::Toggle))
        } else {
            Err(ParseError::InvalidValue { 
                field: "debug mode".to_string(), 
                value: args[0].to_string(), 
                allowed: "on or off".to_string() 
            })
        }
    }

    /// Parses the 'debug:gen' command to generate random tasks.
    fn parse_debug_generate_command(&self, args: &[&str]) -> Result<UiEvent, ParseError> {
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
                Ok(UiEvent::Debug(DebugCommand::GenerateTasks(count)))
            }
        } else {
            Err(ParseError::InvalidId("Invalid count. Please provide a number.".to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::control;

    fn setup() {
        control::set_override(false);
    }

    #[test]
    fn test_new_command_parser() {
        let input = InputReader::with_reader("add test".as_bytes());
        let _parser = CommandParser::new(input);
    }

    #[test]
    fn test_parse_add_command_valid() {
        setup();
        let input = InputReader::with_reader("add Buy groceries\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, Ok(UiEvent::Task(TaskCommand::Add(_)))));
    }

    #[test]
    fn test_parse_add_command_multiple_words() {
        setup();
        let input = InputReader::with_reader("add Buy groceries and cook dinner\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let result = parser.read_event();
        if let Ok(UiEvent::Task(TaskCommand::Add(desc))) = result {
            assert_eq!(desc, "Buy groceries and cook dinner");
        } else {
            panic!("Expected Add command");
        }
    }

    #[test]
    fn test_parse_add_command_empty() {
        setup();
        let input = InputReader::with_reader("add\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let result = parser.read_event();
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_list_command() {
        setup();
        let input = InputReader::with_reader("list\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, Ok(UiEvent::Task(TaskCommand::List(_)))));
    }

    #[test]
    fn test_parse_remove_command_valid() {
        setup();
        let input = InputReader::with_reader("remove 1\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let result = parser.read_event();
        if let Ok(UiEvent::Task(TaskCommand::Remove(id))) = result {
            assert_eq!(id, 1);
        } else {
            panic!("Expected Remove command");
        }
    }

    #[test]
    fn test_parse_complete_command() {
        setup();
        let input = InputReader::with_reader("complete 2\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let result = parser.read_event();
        if let Ok(UiEvent::Task(TaskCommand::Complete(id))) = result {
            assert_eq!(id, 2);
        } else {
            panic!("Expected Complete command");
        }
    }

    #[test]
    fn test_parse_priority_command_high() {
        setup();
        let input = InputReader::with_reader("priority 1 high\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let result = parser.read_event();
        if let Ok(UiEvent::Task(TaskCommand::SetPriority(id, priority))) = result {
            assert_eq!(id, 1);
            assert_eq!(priority, Priority::High);
        } else {
            panic!("Expected SetPriority command");
        }
    }

    #[test]
    fn test_parse_unknown_command() {
        setup();
        let input = InputReader::with_reader("invalid\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, Ok(UiEvent::General(GeneralCommand::Unknown(_)))));
    }

    #[test]
    fn test_parse_help_command() {
        setup();
        let input = InputReader::with_reader("help\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, Ok(UiEvent::General(GeneralCommand::ShowHelp))));
    }

    #[test]
    fn test_parse_quit_command() {
        setup();
        let input = InputReader::with_reader("quit\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, Ok(UiEvent::General(GeneralCommand::Quit))));
    }
}

