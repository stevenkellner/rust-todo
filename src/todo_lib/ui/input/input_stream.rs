/// Trait for reading input from a stream.
///
/// `InputStream` provides a unified interface for reading input from various sources.
pub trait InputStream {
    /// Gets the next line of input from the stream.
    ///
    /// # Returns
    ///
    /// A `String` containing the trimmed input line.
    fn get_next_input(&mut self) -> String;
}
