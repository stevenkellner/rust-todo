//! Persistence layer for saving and loading todo lists.
//!
//! This module provides functionality to persist TodoList data to disk
//! and load it back, decoupling storage concerns from business logic.

pub mod todo_list_storage;

pub use todo_list_storage::TodoListStorage;
