use std::io::{self, Write};
use colored::*;
use super::formatters::MessageFormatter;

/// Handles core output operations for the command-line interface.
///
/// `OutputWriter` provides only the fundamental output methods.
/// Specialized output writers for different command types build upon these methods.
///
/// # Examples
///
/// ```
/// use todo_manager::ui::output_writer::OutputWriter;
///
/// let mut output = OutputWriter::new();
/// output.print_line("Hello, World!");
/// ```
pub struct OutputWriter<W: Write = io::Stdout> {
    writer: W,
}

impl OutputWriter<io::Stdout> {
    /// Creates a new output writer that writes to stdout.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let output = OutputWriter::new();
    /// ```
    pub fn new() -> Self {
        OutputWriter {
            writer: io::stdout(),
        }
    }
}

impl<W: Write> OutputWriter<W> {
    /// Creates a new output writer with a custom writer (for testing).
    pub fn with_writer(writer: W) -> Self {
        OutputWriter { writer }
    }

    /// Displays an error message for invalid input.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    pub fn show_error(&mut self, message: &str) {
        self.print_line(&MessageFormatter::error(message));
    }

    /// Displays a success message to the user.
    ///
    /// # Arguments
    ///
    /// * `message` - The success message to display
    pub fn show_success(&mut self, message: &str) {
        self.print_line(&MessageFormatter::success(message));
    }

    /// Prints a line of text to the output.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to print
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let mut output = OutputWriter::new();
    /// output.print_line("Task added successfully!");
    /// ```
    pub fn print_line(&mut self, text: &str) {
        writeln!(self.writer, "{}", text).unwrap();
    }

    /// Displays the command prompt without a newline.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let mut output = OutputWriter::new();
    /// output.print_prompt();
    /// ```
    pub fn print_prompt(&mut self) {
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
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("Test message");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Test message\n");
    }

    #[test]
    fn test_show_error() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_error("Invalid input");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Invalid input\n");
    }

    #[test]
    fn test_show_error_different_message() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_error("Task ID must be a number");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Task ID must be a number\n");
    }

    #[test]
    fn test_show_success() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_success("Operation completed");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Operation completed\n");
    }

    #[test]
    fn test_print_line_empty_string() {
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_print_line_multiline_string() {
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("Line 1\nLine 2\nLine 3");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_print_prompt() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_prompt();
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "> ");
    }

    #[test]
    fn test_multiple_operations() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_success("First operation");
        output.show_success("Second operation");
        output.show_error("An error occurred");
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("✓ First operation"));
        assert!(result.contains("✓ Second operation"));
        assert!(result.contains("✗ An error occurred"));
    }
}
