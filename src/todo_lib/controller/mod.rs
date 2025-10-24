//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod command_controller;
pub mod debug_command;
pub mod general_command;
pub mod task_command;
pub mod todo_manager;

// Re-export commonly used types
pub use command_controller::CommandController;
pub use debug_command::DebugCommandController;
pub use general_command::GeneralCommandController;
pub use task_command::TaskCommandController;
pub use debug_command::RandomTaskGenerator;
pub use todo_manager::TodoManager;