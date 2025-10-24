//! # Todo Manager
//!
//! A comprehensive todo list management library providing task tracking,
//! list management, and a user-friendly command-line interface.
//!
//! ## Features
//!
//! - Create and manage tasks with unique IDs
//! - Mark tasks as completed or pending
//! - Filter tasks by completion status
//! - Interactive command-line interface with descriptive commands
//! - Event-driven architecture with clean separation of concerns
//! - Comprehensive test coverage
//!
//! ## Architecture
//!
//! The library follows a layered architecture with clear separation of concerns:
//!
//! ### Models Layer (`models`)
//! - `task`: Data structure for individual tasks with priority and status
//! - `todo_list`: Collection and business logic for managing tasks
//! - `priority`: Task priority levels (Low, Medium, High)
//! - `task_status`: Task completion status (Pending, Completed)
//! - `task_filter`: Filter criteria for querying tasks
//! - `ui_event`: Event definitions representing user actions
//! - `loop_control`: Control flow signals for the application loop
//!
//! ### UI Layer (`ui`)
//! - `input_reader`: Reading and parsing user input into events
//! - `output_writer`: Displaying messages and formatting output
//!
//! ### Controller Layer (`controller`)
//! - `todo_controller`: Orchestrates UI events with model operations
//!
//! ## Examples
//!
//! ```no_run
//! use todo_manager::controller::TodoManager;
//! use todo_manager::ui::input::FileInputStream;
//! use todo_manager::ui::output::FileOutputWriter;
//! use std::rc::Rc;
//! use std::cell::RefCell;
//!
//! let input_stream = FileInputStream::new(std::io::stdin());
//! let output_writer = FileOutputWriter::new(std::io::stdout());
//! let mut manager = TodoManager::new(
//!     Rc::new(RefCell::new(input_stream)),
//!     Rc::new(RefCell::new(output_writer))
//! );
//! manager.run();  // Starts the interactive application
//! ```

// Module declarations
pub mod controller;
pub mod models;
pub mod ui;

// Re-export commonly used types for convenience
pub use controller::TodoManager;
pub use models::{LoopControl, Priority, Task, TaskFilter, TaskStatus, TodoList};
pub use ui::{InputStream, FileInputStream, OutputWriter, FileOutputWriter};
