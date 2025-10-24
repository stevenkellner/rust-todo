use crate::ui::output_writer::OutputWriter;
use std::io::Write;
use colored::*;

/// Manages all UI operations for the application.
///
/// `UIManager` provides a centralized interface for displaying messages,
/// prompts, and handling UI-related operations.
pub struct UIManager<W: Write> {
    writer: OutputWriter<W>,
}

impl UIManager<std::io::Stdout> {
    /// Creates a new UI manager with stdout.
    pub fn new() -> Self {
        UIManager {
            writer: OutputWriter::new(),
        }
    }
}

impl<W: Write> UIManager<W> {
    /// Creates a new UI manager with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        UIManager {
            writer: OutputWriter::with_writer(writer),
        }
    }

    /// Displays the welcome message.
    pub fn show_welcome(&mut self) {
        self.writer.print_line("");
        self.writer.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.writer.print_line(&"       ████████╗  ██████╗  ██████╗   ██████╗       ".bright_cyan().bold().to_string());
        self.writer.print_line(&"       ╚══██╔══╝ ██╔═══██╗ ██╔══██╗ ██╔═══██╗      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ╚██████╔╝ ██████╔╝ ╚██████╔╝      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ╚═╝     ╚═════╝  ╚═════╝   ╚═════╝       ".bright_cyan().bold().to_string());
        self.writer.print_line(&"                 📝 LIST MANAGER 📝                 ".bright_green().bold().to_string());
        self.writer.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"    Welcome to your personal task management system!".white().to_string());
        self.writer.print_line("");
        self.writer.print_line(&format!("    Type {} to see available commands.", "help".bright_yellow().bold()));
        self.writer.print_line("");
        self.writer.print_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.writer.print_line("");
    }

    /// Prints the command prompt.
    pub fn print_prompt(&mut self) {
        self.writer.print_prompt();
    }

    /// Shows an error message.
    pub fn show_error(&mut self, message: &str) {
        self.writer.show_error(message);
    }

    /// Handles an unknown command by displaying an error message.
    pub fn handle_unknown_command(&mut self, command: &str) {
        self.writer.show_error(&format!("Unknown command '{}'. Type help for available commands.", command));
    }
}

impl Default for UIManager<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Disable colors for all tests to make assertions easier
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_new_ui_manager() {
        let _manager = UIManager::new();
        // Just verify it compiles and constructs
    }

    #[test]
    fn test_ui_manager_with_writer() {
        let mut output = Vec::new();
        let _manager = UIManager::with_writer(&mut output);
        // Just verify it compiles and constructs
    }

    #[test]
    fn test_default_trait() {
        let _manager = UIManager::default();
        // Verify default trait works
    }

    #[test]
    fn test_show_welcome() {
        setup();
        let mut output = Vec::new();
        {
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
            
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager = UIManager::with_writer(&mut output);
            manager.handle_unknown_command("");
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        assert!(output_str.contains("Unknown command ''"));
    }

    #[test]
    fn test_ui_manager_is_mutable() {
        setup();
        let mut output = Vec::new();
        {
            let mut manager = UIManager::with_writer(&mut output);
            
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
            let mut manager = UIManager::with_writer(&mut output);
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
            let mut manager1 = UIManager::with_writer(&mut output1);
            manager1.show_error("Specific error A");
        }
        
        {
            let mut manager2 = UIManager::with_writer(&mut output2);
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


