use crate::ui::output_writer::OutputWriter;
use std::io::Write;

/// Output writer specifically for debug commands.
///
/// Handles all output operations related to debug commands
/// like generating random tasks, toggling debug mode, etc.
pub struct DebugCommandOutputWriter<W: Write> {
    writer: OutputWriter<W>,
}

impl DebugCommandOutputWriter<std::io::Stdout> {
    /// Creates a new DebugCommandOutputWriter with stdout.
    pub fn new() -> Self {
        DebugCommandOutputWriter {
            writer: OutputWriter::new(),
        }
    }
}

impl<W: Write> DebugCommandOutputWriter<W> {
    /// Creates a new DebugCommandOutputWriter with a custom writer.
    pub fn with_writer(writer: W) -> Self {
        DebugCommandOutputWriter {
            writer: OutputWriter::with_writer(writer),
        }
    }

    /// Displays a message when debug mode is enabled.
    pub fn show_debug_mode_enabled(&mut self) {
        self.writer.show_success("Debug mode enabled");
    }

    /// Displays a message when debug mode is disabled.
    pub fn show_debug_mode_disabled(&mut self) {
        self.writer.show_success("Debug mode disabled");
    }

    /// Displays a message when debug mode is not enabled.
    pub fn show_debug_mode_not_enabled(&mut self) {
        self.writer.show_error("Debug mode is not enabled. Use 'debug' to enable it.");
    }

    /// Displays a message after generating random tasks.
    pub fn show_random_tasks_generated(&mut self, count: usize) {
        self.writer.show_success(&format!("Generated {} random tasks", count));
    }

    /// Displays a message after clearing all tasks.
    pub fn show_all_tasks_cleared(&mut self, count: usize) {
        self.writer.show_success(&format!("Cleared {} tasks", count));
    }

    /// Displays a generic success message.
    pub fn show_success(&mut self, message: &str) {
        self.writer.show_success(message);
    }

    /// Displays a generic error message.
    pub fn show_error(&mut self, message: &str) {
        self.writer.show_error(message);
    }
}

impl Default for DebugCommandOutputWriter<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
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
    fn test_debug_output_writer_enabled() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_debug_mode_enabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode enabled"));
    }

    #[test]
    fn test_debug_output_writer_disabled() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_debug_mode_disabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode disabled"));
    }

    #[test]
    fn test_debug_output_writer_not_enabled() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_debug_mode_not_enabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode is not enabled"));
    }

    #[test]
    fn test_debug_output_writer_random_tasks_generated() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_random_tasks_generated(10);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Generated 10 random tasks"));
    }

    #[test]
    fn test_debug_output_writer_all_tasks_cleared() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_all_tasks_cleared(25);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Cleared 25 tasks"));
    }

    #[test]
    fn test_debug_output_writer_success() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_success("Operation successful");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Operation successful"));
    }

    #[test]
    fn test_debug_output_writer_show_error() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = DebugCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_error("An error occurred");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("An error occurred"));
    }
}
