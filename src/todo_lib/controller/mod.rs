//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod command_controller;
pub mod command_controller_type;
pub mod debug_command_controller;
pub mod general_command_controller;
pub mod task_command_controller;
pub mod task_generator;
pub mod todo_manager;

// Re-export commonly used types
pub use command_controller::CommandController;
pub use command_controller_type::CommandControllerType;
pub use debug_command_controller::DebugCommandController;
pub use general_command_controller::GeneralCommandController;
// generic_command_controller exports removed - internal use only
pub use task_command_controller::TaskCommandController;
pub use task_generator::RandomTaskGenerator;
pub use todo_manager::TodoManager;