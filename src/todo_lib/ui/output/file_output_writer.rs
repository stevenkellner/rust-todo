use std::io::Write;
use colored::*;
use crate::ui::formatters::MessageFormatter;
use super::output_writer::OutputWriter;

/// File-based implementation of OutputWriter for the command-line interface.
///
/// `FileOutputWriter` provides the fundamental output methods for CLI.
/// Specialized output writers for different command types build upon these methods.
///
/// # Examples
///
/// ```
/// use todo_manager::ui::output::{FileOutputWriter, OutputWriter};
///
/// let mut output = FileOutputWriter::new(std::io::stdout());
/// output.write_line("Hello, World!");
/// ```
pub struct FileOutputWriter<W: Write> {
    writer: W,
}

impl<W: Write> FileOutputWriter<W> {
    /// Creates a new output writer with a custom writer (for testing).
    pub fn new(writer: W) -> Self {
        FileOutputWriter { writer }
    }
}

impl<W: Write> OutputWriter for FileOutputWriter<W> {
    /// Displays an error message for invalid input.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    fn show_error(&mut self, message: &str) {
        self.write_line(&MessageFormatter::error(message));
    }

    /// Displays a success message to the user.
    ///
    /// # Arguments
    ///
    /// * `message` - The success message to display
    fn show_success(&mut self, message: &str) {
        self.write_line(&MessageFormatter::success(message));
    }

    /// Writes a line of text to the output.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to write
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::output::{FileOutputWriter, OutputWriter};
    ///
    /// let mut output = FileOutputWriter::new(std::io::stdout());
    /// output.write_line("Task added successfully!");
    /// ```
    fn write_line(&mut self, text: &str) {
        writeln!(self.writer, "{}", text).unwrap();
    }

    /// Displays the command prompt without a newline.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::output::{FileOutputWriter, OutputWriter};
    ///
    /// let mut output = FileOutputWriter::new(std::io::stdout());
    /// output.write_prompt();
    /// ```
    fn write_prompt(&mut self) {
        write!(self.writer, "{}", "> ".bright_green().bold()).unwrap();
        self.writer.flush().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Disable colors for all tests
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_new_output_writer() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.write_line("Test message");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Test message\n");
    }

    #[test]
    fn test_show_error() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.show_error("Invalid input");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Invalid input\n");
    }

    #[test]
    fn test_show_error_different_message() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.show_error("Task ID must be a number");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Task ID must be a number\n");
    }

    #[test]
    fn test_show_success() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.show_success("Operation completed");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Operation completed\n");
    }

    #[test]
    fn test_write_line_empty_string() {
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.write_line("");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_write_line_multiline_string() {
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.write_line("Line 1\nLine 2\nLine 3");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_write_prompt() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.write_prompt();
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "> ");
    }

    #[test]
    fn test_multiple_operations() {
        setup();
        let mut buffer = Vec::new();
        let mut output = FileOutputWriter::new(&mut buffer);
        output.show_success("First operation");
        output.show_success("Second operation");
        output.show_error("An error occurred");
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("✓ First operation"));
        assert!(result.contains("✓ Second operation"));
        assert!(result.contains("✗ An error occurred"));
    }
}
