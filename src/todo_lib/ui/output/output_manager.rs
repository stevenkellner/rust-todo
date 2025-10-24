use crate::ui::output::OutputWriter;
use colored::*;
use std::rc::Rc;
use std::cell::RefCell;

/// Manages all UI operations for the application.
///
/// `OutputManager` provides a centralized interface for displaying messages,
/// prompts, and handling UI-related operations.
pub struct OutputManager<O: OutputWriter> {
    output_writer: Rc<RefCell<O>>,
}

impl<O: OutputWriter> OutputManager<O> {
    /// Creates a new UI manager with a custom output writer.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            output_writer
        }
    }

    /// Displays the welcome message.
    pub fn show_welcome(&mut self) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"       ████████╗  ██████╗  ██████╗   ██████╗       ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"       ╚══██╔══╝ ██╔═══██╗ ██╔══██╗ ██╔═══██╗      ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"          ██║    ╚██████╔╝ ██████╔╝ ╚██████╔╝      ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"          ╚═╝     ╚═════╝  ╚═════╝   ╚═════╝       ".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"                 📝 LIST MANAGER 📝                 ".bright_green().bold().to_string());
        self.output_writer.borrow_mut().write_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&"    Welcome to your personal task management system!".white().to_string());
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&format!("    Type {} to see available commands.", "help".bright_yellow().bold()));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.output_writer.borrow_mut().write_line("");
    }

    /// Prints the command prompt.
    pub fn print_prompt(&mut self) {
        self.output_writer.borrow_mut().write_prompt();
    }

    /// Shows an error message.
    pub fn show_error(&mut self, message: &str) {
        self.output_writer.borrow_mut().show_error(message);
    }

    /// Handles an unknown command by displaying an error message.
    pub fn handle_unknown_command(&mut self, command: &str) {
        self.output_writer.borrow_mut().show_error(&format!("Unknown command '{}'. Type help for available commands.", command));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::output::FileOutputWriter;

    // Disable colors for all tests to make assertions easier
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_new_output_manager() {
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let _manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
        // Just verify it compiles and constructs
    }

    #[test]
    fn test_output_manager_with_writer() {
        let mut output = Vec::new();
        let output_writer = FileOutputWriter::new(&mut output);
        let _manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
        // Just verify it compiles and constructs
    }

    #[test]
    fn test_show_welcome() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.show_welcome();
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Check that welcome message contains key elements
        assert!(output_str.contains("TODO") || output_str.contains("LIST MANAGER"));
        assert!(output_str.contains("Welcome to your personal task management system!"));
        assert!(output_str.contains("Type help to see available commands"));
    }

    #[test]
    fn test_show_error() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.show_error("Test error message");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Check that error message is displayed
        assert!(output_str.contains("Test error message"));
    }

    #[test]
    fn test_show_error_with_special_characters() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.show_error("Error: File 'test.txt' not found!");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        assert!(output_str.contains("Error: File 'test.txt' not found!"));
    }

    #[test]
    fn test_handle_unknown_command() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.handle_unknown_command("invalidcmd");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Check that unknown command message is displayed
        assert!(output_str.contains("Unknown command 'invalidcmd'"));
        assert!(output_str.contains("Type help for available commands"));
    }

    #[test]
    fn test_handle_unknown_command_with_arguments() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.handle_unknown_command("badcommand arg1 arg2");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        assert!(output_str.contains("Unknown command 'badcommand arg1 arg2'"));
    }

    #[test]
    fn test_multiple_operations() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            
            manager.show_error("First error");
            manager.show_error("Second error");
            manager.handle_unknown_command("unknown");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // All messages should be in output
        assert!(output_str.contains("First error"));
        assert!(output_str.contains("Second error"));
        assert!(output_str.contains("Unknown command 'unknown'"));
    }

    #[test]
    fn test_show_error_empty_message() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.show_error("");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Output should exist even with empty message
        assert!(!output_str.is_empty());
    }

    #[test]
    fn test_handle_unknown_command_empty_string() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.handle_unknown_command("");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        assert!(output_str.contains("Unknown command ''"));
    }

    #[test]
    fn test_output_manager_is_mutable() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            
            // Verify we can call multiple mutable methods
            manager.show_error("Error 1");
            manager.handle_unknown_command("cmd1");
            manager.show_error("Error 2");
        }
        
        // Should compile and work
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Error 1"));
        assert!(output_str.contains("cmd1"));
        assert!(output_str.contains("Error 2"));
    }

    #[test]
    fn test_show_welcome_contains_branding() {
        setup();
        let mut output = Vec::new();
        {
            let output_writer = FileOutputWriter::new(&mut output);
            let mut manager = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager.show_welcome();
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Verify branding elements
        assert!(output_str.len() > 100); // Welcome message should be substantial
    }

    #[test]
    fn test_error_messages_are_distinct() {
        setup();
        let mut output1 = Vec::new();
        let mut output2 = Vec::new();
        
        {
            let output_writer = FileOutputWriter::new(&mut output1);
            let mut manager1 = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager1.show_error("Specific error A");
        }
        
        {
            let output_writer = FileOutputWriter::new(&mut output2);
            let mut manager2 = OutputManager::new(Rc::new(RefCell::new(output_writer)));
            manager2.show_error("Specific error B");
        }
        
        let output_str1 = String::from_utf8(output1).unwrap();
        let output_str2 = String::from_utf8(output2).unwrap();
        
        assert!(output_str1.contains("Specific error A"));
        assert!(!output_str1.contains("Specific error B"));
        
        assert!(output_str2.contains("Specific error B"));
        assert!(!output_str2.contains("Specific error A"));
    }
}


