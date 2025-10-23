//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod todo_controller;
pub mod debug_controller;

// Re-export commonly used types
pub use todo_controller::TodoController;
pub use debug_controller::DebugController;
