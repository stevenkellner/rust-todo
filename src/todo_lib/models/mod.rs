//! Domain models for the todo list application.
//!
//! This module contains all data structures, events, and business logic
//! related to tasks, priorities, filtering, and control flow.

pub mod debug_command;
pub mod filter_builder;
pub mod general_command;
pub mod loop_control;
pub mod overdue_filter;
pub mod priority;
pub mod task;
pub mod task_command;
pub mod task_filter;
pub mod task_status;
pub mod todo_list;
pub mod ui_event;

// Re-export commonly used types
pub use debug_command::DebugCommand;
pub use filter_builder::FilterBuilder;
pub use general_command::GeneralCommand;
pub use loop_control::LoopControl;
pub use priority::Priority;
pub use task::Task;
pub use task_command::TaskCommand;
pub use task_filter::TaskFilter;
pub use task_status::TaskStatus;
pub use todo_list::{TodoList, TaskStatistics};
pub use ui_event::UiEvent;
