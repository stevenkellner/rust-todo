//! User interface layer for the todo list application.
//!
//! This module handles all user interactions including reading input
//! and writing output to the terminal.

pub mod formatters;
pub mod input;
pub mod output;

// Re-export commonly used types
pub use input::{InputStream, FileInputStream};
pub use output::{OutputWriter, FileOutputWriter, OutputManager};
