use crate::ui::input_reader::InputReader;
use crate::models::ui_event::UiEvent;
use crate::models::filter_builder::FilterBuilder;
use crate::models::priority::Priority;
use crate::models::task_command::TaskCommand;
use crate::models::debug_command::DebugCommand;
use crate::models::general_command::GeneralCommand;
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
    /// A `UiEvent` representing the user's command.
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
    pub fn read_event(&mut self) -> UiEvent {
        let input = self.input.read_input();
        self.parse_command(&input)
    }

    /// Parses a command string into a UI event.
    fn parse_command(&self, input: &str) -> UiEvent {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return UiEvent::General(GeneralCommand::InvalidInput("Please enter a command. Type 'help' for available commands.".to_string()));
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
            "categories" | "list-categories" => UiEvent::Task(TaskCommand::ListCategories),
            "edit" => self.parse_edit_command(&args),
            "search" | "find" => self.parse_search_command(&args),
            "statistics" | "stats" => UiEvent::Task(TaskCommand::ShowStatistics),
            "debug" => self.parse_debug_command(&args),
            "debug:gen" => self.parse_debug_generate_command(&args),
            "debug:clear" => UiEvent::Debug(DebugCommand::ClearAll),
            "help" | "h" => UiEvent::General(GeneralCommand::ShowHelp),
            "quit" | "exit" | "q" => UiEvent::General(GeneralCommand::Quit),
            _ => UiEvent::General(GeneralCommand::Unknown(command)),
        }
    }

    /// Parses the 'add' command and validates the task description.
    fn parse_add_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: add <task description>".to_string()))
        } else {
            let description = args.join(" ");
            if description.is_empty() {
                UiEvent::General(GeneralCommand::InvalidInput("Task description cannot be empty.".to_string()))
            } else {
                UiEvent::Task(TaskCommand::Add(description))
            }
        }
    }

    /// Parses the 'list' command with optional filter arguments.
    fn parse_list_command(&self, args: &[&str]) -> UiEvent {
        let mut filter_builder = FilterBuilder::new();
        
        for arg in args {
            let lower = arg.to_lowercase();
            filter_builder = match filter_builder.parse_argument(&lower) {
                Ok(builder) => builder,
                Err(err) => return UiEvent::General(GeneralCommand::InvalidInput(err)),
            };
        }
        
        UiEvent::Task(TaskCommand::List(filter_builder.build()))
    }

    /// Parses the 'remove' command with task ID validation.
    fn parse_remove_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: remove <task id>".to_string()))
        } else if let Ok(id) = args[0].parse::<usize>() {
            UiEvent::Task(TaskCommand::Remove(id))
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'complete' command.
    fn parse_complete_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: complete <task id>".to_string()))
        } else if let Ok(id) = args[0].parse::<usize>() {
            UiEvent::Task(TaskCommand::Complete(id))
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'uncomplete' command.
    fn parse_uncomplete_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: uncomplete <task id>".to_string()))
        } else if let Ok(id) = args[0].parse::<usize>() {
            UiEvent::Task(TaskCommand::Uncomplete(id))
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'toggle' command.
    fn parse_toggle_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: toggle <task id>".to_string()))
        } else if let Ok(id) = args[0].parse::<usize>() {
            UiEvent::Task(TaskCommand::Toggle(id))
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'priority' command with priority level validation.
    fn parse_priority_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            UiEvent::General(GeneralCommand::InvalidInput(
                "Usage: priority <task id> <priority level (high/h, medium/med/m, low/l)>".to_string(),
            ))
        } else if let Ok(id) = args[0].parse::<usize>() {
            let priority_str = args[1].to_lowercase();
            match Priority::from_str(&priority_str) {
                Some(priority) => UiEvent::Task(TaskCommand::SetPriority(id, priority)),
                None => UiEvent::General(GeneralCommand::InvalidInput(
                    "Invalid priority level. Use: high/h, medium/med/m, or low/l".to_string(),
                )),
            }
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'set-due' command with date validation.
    fn parse_set_due_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            UiEvent::General(GeneralCommand::InvalidInput(
                "Usage: set-due <task id> <date (DD.MM.YYYY) or 'none' to clear>".to_string(),
            ))
        } else if let Ok(id) = args[0].parse::<usize>() {
            let date_str = args[1];
            
            if date_str.to_lowercase() == "none" {
                return UiEvent::Task(TaskCommand::SetDueDate(id, None));
            }
            
            let parts: Vec<&str> = date_str.split('.').collect();
            if parts.len() != 3 {
                return UiEvent::General(GeneralCommand::InvalidInput(
                    "Invalid date format. Use DD.MM.YYYY (e.g., 31.12.2024)".to_string(),
                ));
            }

            if let (Ok(day), Ok(month), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<i32>(),
            ) {
                use chrono::NaiveDate;
                match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(date) => UiEvent::Task(TaskCommand::SetDueDate(id, Some(date))),
                    None => UiEvent::General(GeneralCommand::InvalidInput("Invalid date. Please check the date is valid.".to_string())),
                }
            } else {
                UiEvent::General(GeneralCommand::InvalidInput("Invalid date values. Use DD.MM.YYYY".to_string()))
            }
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'set-category' command.
    fn parse_set_category_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            UiEvent::General(GeneralCommand::InvalidInput(
                "Usage: set-category <task id> <category name or 'none' to clear>".to_string(),
            ))
        } else if let Ok(id) = args[0].parse::<usize>() {
            let category = args[1..].join(" ");
            
            if category.to_lowercase() == "none" {
                UiEvent::Task(TaskCommand::SetCategory(id, None))
            } else if category.is_empty() {
                UiEvent::General(GeneralCommand::InvalidInput("Category name cannot be empty.".to_string()))
            } else {
                UiEvent::Task(TaskCommand::SetCategory(id, Some(category)))
            }
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'search' command.
    fn parse_search_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: search <keyword>".to_string()))
        } else {
            let keyword = args.join(" ");
            if keyword.is_empty() {
                UiEvent::General(GeneralCommand::InvalidInput("Search keyword cannot be empty.".to_string()))
            } else {
                UiEvent::Task(TaskCommand::Search(keyword))
            }
        }
    }

    /// Parses the 'edit' command.
    fn parse_edit_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: edit <task id> <new description>".to_string()))
        } else if let Ok(id) = args[0].parse::<usize>() {
            let description = args[1..].join(" ");
            if description.is_empty() {
                UiEvent::General(GeneralCommand::InvalidInput("Task description cannot be empty.".to_string()))
            } else {
                UiEvent::Task(TaskCommand::Edit(id, description))
            }
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid task ID. Please provide a number.".to_string()))
        }
    }

    /// Parses the 'debug' command.
    fn parse_debug_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() || args[0].to_lowercase() == "on" {
            UiEvent::Debug(DebugCommand::Toggle)
        } else if args[0].to_lowercase() == "off" {
            UiEvent::Debug(DebugCommand::Toggle)
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: debug [on|off]".to_string()))
        }
    }

    /// Parses the 'debug:gen' command to generate random tasks.
    fn parse_debug_generate_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::General(GeneralCommand::InvalidInput("Usage: debug:gen <count>".to_string()))
        } else if let Ok(count) = args[0].parse::<usize>() {
            if count == 0 {
                UiEvent::General(GeneralCommand::InvalidInput("Count must be greater than 0".to_string()))
            } else if count > 1000 {
                UiEvent::General(GeneralCommand::InvalidInput("Count cannot exceed 1000".to_string()))
            } else {
                UiEvent::Debug(DebugCommand::GenerateTasks(count))
            }
        } else {
            UiEvent::General(GeneralCommand::InvalidInput("Invalid count. Please provide a number.".to_string()))
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
        assert!(matches!(event, UiEvent::Task(TaskCommand::Add(_))));
    }

    #[test]
    fn test_parse_add_command_multiple_words() {
        setup();
        let input = InputReader::with_reader("add Buy groceries and cook dinner\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        if let UiEvent::Task(TaskCommand::Add(desc)) = event {
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
        
        let event = parser.read_event();
        assert!(matches!(event, UiEvent::General(GeneralCommand::InvalidInput(_))));
    }

    #[test]
    fn test_parse_list_command() {
        setup();
        let input = InputReader::with_reader("list\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, UiEvent::Task(TaskCommand::List(_))));
    }

    #[test]
    fn test_parse_remove_command_valid() {
        setup();
        let input = InputReader::with_reader("remove 1\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        if let UiEvent::Task(TaskCommand::Remove(id)) = event {
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
        
        let event = parser.read_event();
        if let UiEvent::Task(TaskCommand::Complete(id)) = event {
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
        
        let event = parser.read_event();
        if let UiEvent::Task(TaskCommand::SetPriority(id, priority)) = event {
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
        assert!(matches!(event, UiEvent::General(GeneralCommand::Unknown(_))));
    }

    #[test]
    fn test_parse_help_command() {
        setup();
        let input = InputReader::with_reader("help\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, UiEvent::General(GeneralCommand::ShowHelp)));
    }

    #[test]
    fn test_parse_quit_command() {
        setup();
        let input = InputReader::with_reader("quit\n".as_bytes());
        let mut parser = CommandParser::new(input);
        
        let event = parser.read_event();
        assert!(matches!(event, UiEvent::General(GeneralCommand::Quit)));
    }
}
