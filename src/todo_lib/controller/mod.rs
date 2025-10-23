//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod debug_controller;
pub mod task_command_handler;
pub mod todo_controller;

// Re-export commonly used types
pub use debug_controller::DebugController;
pub use task_command_handler::TaskCommandHandler;
pub use todo_controller::TodoController;
