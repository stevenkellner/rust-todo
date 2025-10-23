//! User interface layer for the todo list application.
//!
//! This module handles all user interactions including reading input
//! and writing output to the terminal.

pub mod formatters;
pub mod input_reader;
pub mod interactive_prompt;
pub mod output_writer;

// Re-export commonly used types
pub use input_reader::InputReader;
pub use interactive_prompt::InteractivePrompt;
pub use output_writer::OutputWriter;
