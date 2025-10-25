//! Domain models for the todo list application.
//!
//! This module contains all data structures, events, and business logic
//! related to tasks, priorities, filtering, and control flow.

pub mod command_controller_result;
pub mod filter_builder;
pub mod id_parser;
pub mod loop_control;
pub mod overdue_filter;
pub mod parse_error;
pub mod priority;
pub mod task;
pub mod task_filter;
pub mod task_status;
pub mod todo_list;
pub mod task_statistics;

// Re-export commonly used types
pub use filter_builder::FilterBuilder;
pub use id_parser::parse_ids;
pub use loop_control::LoopControl;
pub use parse_error::ParseError;
pub use priority::Priority;
pub use task::Task;
pub use task_filter::TaskFilter;
pub use task_status::TaskStatus;
pub use todo_list::TodoList;
pub use task_statistics::TaskStatistics;
