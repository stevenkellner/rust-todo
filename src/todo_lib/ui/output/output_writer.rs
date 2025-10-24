/// Trait for writing output to a stream.
///
/// `OutputWriter` provides a unified interface for writing output to various destinations.
pub trait OutputWriter {
    /// Displays an error message.
    fn show_error(&mut self, message: &str);
    
    /// Displays a success message.
    fn show_success(&mut self, message: &str);
    
    /// Writes a line of text.
    fn write_line(&mut self, text: &str);
    
    /// Displays the command prompt.
    fn write_prompt(&mut self);
}
