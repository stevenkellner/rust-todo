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
//! The library follows an event-driven architecture with clear separation of concerns:
//! - `task`: Data structure for individual tasks
//! - `todo_list`: Business logic for managing tasks
//! - `input_reader`: Reading and parsing user input into events
//! - `output_writer`: Displaying messages and formatting output
//! - `ui_event`: Event definitions for UI interactions
//! - `task_filter`: Filter options for listing tasks
//! - `loop_control`: Control flow for the main event loop
//! - `todo_controller`: Controller that reacts to UI events and manages state
//!
//! ## Examples
//!
//! ```no_run
//! use todo_manager::todo_controller::TodoController;
//!
//! let mut controller = TodoController::new();
//! controller.run();  // Starts the interactive application
//! ```

pub mod priority;
pub mod task_status;
pub mod task;
pub mod todo_list;
pub mod input_reader;
pub mod output_writer;
pub mod ui_event;
pub mod task_filter;
pub mod loop_control;
pub mod todo_controller;
