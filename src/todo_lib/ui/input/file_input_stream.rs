use std::io::{BufRead, BufReader, Read};
use super::input_stream::InputStream;

/// File-based implementation of InputStream for the command-line interface.
///
/// `FileInputStream` reads user input from the terminal or any Read source.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::{FileInputStream, InputStream};
///
/// let mut input = FileInputStream::new(std::io::stdin());
/// let line = input.get_next_input();
/// ```
pub struct FileInputStream<R: Read> {
    reader: BufReader<R>,
}

impl<R: Read> FileInputStream<R> {
    /// Creates a new input stream with a custom reader for testing.
    ///
    /// # Arguments
    ///
    /// * `reader` - Any type implementing Read
    pub fn new(reader: R) -> Self {
        FileInputStream {
            reader: BufReader::new(reader),
        }
    }
}

impl<R: Read> InputStream for FileInputStream<R> {
    /// Gets the next line of input from the stream.
    ///
    /// # Returns
    ///
    /// A `String` containing the trimmed input line.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::{FileInputStream, InputStream};
    ///
    /// let mut input = FileInputStream::new(std::io::stdin());
    /// let user_input = input.get_next_input();
    /// println!("You entered: {}", user_input);
    /// ```
    fn get_next_input(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}
