use std::io::{self, BufRead, BufReader, Read};

/// Handles input operations for the command-line interface.
///
/// `InputReader` is responsible for reading user input from the terminal.
/// All parsing logic has been moved to CommandParser.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::ui::input_reader::InputReader;
///
/// let mut input = InputReader::new();
/// let line = input.read_input();
/// ```
pub struct InputReader<R: Read = io::Stdin> {
    reader: BufReader<R>,
}

impl InputReader<io::Stdin> {
    /// Creates a new input reader that reads from stdin.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::input_reader::InputReader;
    ///
    /// let mut input = InputReader::new();
    /// ```
    pub fn new() -> Self {
        InputReader {
            reader: BufReader::new(io::stdin()),
        }
    }
}

impl<R: Read> InputReader<R> {
    /// Creates a new input reader with a custom reader for testing.
    ///
    /// # Arguments
    ///
    /// * `reader` - Any type implementing Read
    pub fn with_reader(reader: R) -> Self {
        InputReader {
            reader: BufReader::new(reader),
        }
    }

    /// Reads a line of input from the reader.
    ///
    /// # Returns
    ///
    /// A `String` containing the trimmed input line.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::input_reader::InputReader;
    ///
    /// let mut input = InputReader::new();
    /// let user_input = input.read_input();
    /// println!("You entered: {}", user_input);
    /// ```
    pub fn read_input(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}
