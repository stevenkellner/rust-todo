use crate::ui::output::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Output writer specifically for debug commands.
///
/// Handles all output operations related to debug commands
/// like generating random tasks, toggling debug mode, etc.
pub struct DebugCommandOutputManager<O: OutputWriter> {
    output_writer: Rc<RefCell<O>>,
}
impl<O: OutputWriter> DebugCommandOutputManager<O> {
    /// Creates a new DebugCommandOutputManager with stdout.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        DebugCommandOutputManager { output_writer }
    }

    /// Displays a message when debug mode is enabled.
    pub fn show_debug_mode_enabled(&mut self) {
        self.output_writer.borrow_mut().show_success("Debug mode enabled");
    }

    /// Displays a message when debug mode is disabled.
    pub fn show_debug_mode_disabled(&mut self) {
        self.output_writer.borrow_mut().show_success("Debug mode disabled");
    }

    /// Displays a message when debug mode is not enabled.
    pub fn show_debug_mode_not_enabled(&mut self) {
        self.output_writer.borrow_mut().show_error("Debug mode is not enabled. Use 'debug' to enable it.");
    }

    /// Displays a message after generating random tasks.
    pub fn show_random_tasks_generated(&mut self, count: usize) {
        self.output_writer.borrow_mut().show_success(&format!("Generated {} random tasks", count));
    }

    /// Displays a message after clearing all tasks.
    pub fn show_all_tasks_cleared(&mut self, count: usize) {
        self.output_writer.borrow_mut().show_success(&format!("Cleared {} tasks", count));
    }

    /// Displays a generic success message.
    pub fn show_success(&mut self, message: &str) {
        self.output_writer.borrow_mut().show_success(message);
    }

    /// Displays a generic error message.
    pub fn show_error(&mut self, message: &str) {
        self.output_writer.borrow_mut().show_error(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::output::FileOutputWriter;

    // Disable colors for all tests
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_debug_output_writer_enabled() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_debug_mode_enabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode enabled"));
    }

    #[test]
    fn test_debug_output_writer_disabled() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_debug_mode_disabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode disabled"));
    }

    #[test]
    fn test_debug_output_writer_not_enabled() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_debug_mode_not_enabled();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Debug mode is not enabled"));
    }

    #[test]
    fn test_debug_output_writer_random_tasks_generated() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_random_tasks_generated(10);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Generated 10 random tasks"));
    }

    #[test]
    fn test_debug_output_writer_all_tasks_cleared() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_all_tasks_cleared(25);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Cleared 25 tasks"));
    }

    #[test]
    fn test_debug_output_writer_success() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_success("Operation successful");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Operation successful"));
    }

    #[test]
    fn test_debug_output_writer_show_error() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = DebugCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_error("An error occurred");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("An error occurred"));
    }
}
