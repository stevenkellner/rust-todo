use crate::models::task_filter::TaskFilter;
use crate::models::priority::Priority;
use chrono::NaiveDate;

/// Represents task-related commands.
///
/// These commands handle all operations on tasks including CRUD operations,
/// filtering, searching, and statistics.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskCommand {
    /// Add a new task with the given description
    Add(String),
    
    /// Add a subtask to a parent task
    AddSubtask(usize, String),
    
    /// List tasks with an optional filter
    List(Option<TaskFilter>),
    
    /// Remove a task by ID
    Remove(usize),
    
    /// Mark a task as completed
    Complete(usize),
    
    /// Mark a task as pending (incomplete)
    Uncomplete(usize),
    
    /// Toggle a task's completion status
    Toggle(usize),
    
    /// Set the priority of a task
    SetPriority(usize, Priority),
    
    /// Set the due date of a task
    SetDueDate(usize, Option<NaiveDate>),
    
    /// Edit a task's description
    Edit(usize, String),
    
    /// Set the category of a task
    SetCategory(usize, Option<String>),
    
    /// List all categories
    ListCategories,
    
    /// Search for tasks by keyword
    Search(String),
    
    /// Show task statistics
    ShowStatistics,
}
