//! Controller layer for the todo list application.
//!
//! This module contains the application controller that orchestrates
//! the interaction between UI and model layers.

pub mod command_controller;
pub mod command_controller_registry;
pub mod debug_command;
pub mod general_command;
pub mod project_command;
pub mod task_command;
pub mod application_controller;

// Re-export commonly used types
pub use command_controller::CommandController;
pub use command_controller_registry::CommandControllerRegistry;
pub use debug_command::DebugCommandController;
pub use general_command::GeneralCommandController;
pub use project_command::ProjectCommandController;
pub use project_command::ProjectManager;
pub use task_command::TaskCommandController;
pub use debug_command::RandomTaskGenerator;
pub use application_controller::ApplicationController;