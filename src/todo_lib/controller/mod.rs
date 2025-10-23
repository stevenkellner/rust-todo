//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod todo_controller;

// Re-export commonly used types
pub use todo_controller::TodoController;
