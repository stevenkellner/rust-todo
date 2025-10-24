//! User interface layer for the todo list application.
//!
//! This module handles all user interactions including reading input
//! and writing output to the terminal.

pub mod formatters;
pub mod input_reader;
pub mod interactive_task_properties_prompt;
pub mod output_writer;
pub mod ui_manager;

// Re-export commonly used types
pub use input_reader::InputReader;
pub use interactive_task_properties_prompt::InteractiveTaskPropertiesPrompt;
pub use output_writer::OutputWriter;
pub use ui_manager::UIManager;
