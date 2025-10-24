//! User interface layer for the todo list application.
//!
//! This module handles all user interactions including reading input
//! and writing output to the terminal.

pub mod command_parser;
pub mod debug_command_output_writer;
pub mod formatters;
pub mod general_command_output_writer;
pub mod input_reader;
pub mod interactive_task_properties_prompt;
pub mod output_writer;
pub mod task_command_output_writer;
pub mod task_command_parser;
pub mod debug_command_parser;
pub mod general_command_parser;

// Re-export commonly used types
pub use debug_command_output_writer::DebugCommandOutputWriter;
pub use general_command_output_writer::GeneralCommandOutputWriter;
pub use input_reader::InputReader;
pub use interactive_task_properties_prompt::InteractiveTaskPropertiesPrompt;
pub use output_writer::OutputWriter;
pub use task_command_output_writer::TaskCommandOutputWriter;
