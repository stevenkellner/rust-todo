use crate::ui::input_reader::InputReader;
use crate::ui::task_command_parser::TaskCommandParser;
use crate::ui::debug_command_parser::DebugCommandParser;
use crate::ui::general_command_parser::GeneralCommandParser;
use crate::models::ui_event::UiEvent;
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
    task_parser: TaskCommandParser,
    debug_parser: DebugCommandParser,
    general_parser: GeneralCommandParser,
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
            task_parser: TaskCommandParser::new(),
            debug_parser: DebugCommandParser::new(),
            general_parser: GeneralCommandParser::new(),
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
        CommandParser { 
            input,
            task_parser: TaskCommandParser::new(),
            debug_parser: DebugCommandParser::new(),
            general_parser: GeneralCommandParser::new(),
        }
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

        // Try task parser first
        if let Some(result) = self.task_parser.try_parse_command(&command, &args) {
            return result.map(UiEvent::Task);
        }

        // Try debug parser
        if let Some(result) = self.debug_parser.try_parse_command(&command, &args) {
            return result.map(UiEvent::Debug);
        }

        // Try general parser
        if let Some(general_cmd) = self.general_parser.try_parse_command(&command) {
            return Ok(UiEvent::General(general_cmd));
        }

        // Unknown command - return as UnknownCommand error
        Err(ParseError::UnknownCommand(command))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::control;
    use crate::models::task_command::TaskCommand;
    use crate::models::general_command::GeneralCommand;
    use crate::models::priority::Priority;

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
        
        let result = parser.read_event();
        assert!(matches!(result, Err(ParseError::UnknownCommand(_))));
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

