use super::TaskSelection;
use crate::models::priority::Priority;
use crate::models::recurrence::Recurrence;
use crate::models::task_filter::TaskFilter;
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

    /// Remove task(s) by ID, IDs, or all
    Remove(TaskSelection),

    /// Mark task(s) as completed
    Complete(TaskSelection),

    /// Mark task(s) as pending (incomplete)
    Uncomplete(TaskSelection),

    /// Toggle task(s) completion status
    Toggle(TaskSelection),

    /// Set the priority of task(s)
    SetPriority(TaskSelection, Priority),

    /// Set the due date of a task
    SetDueDate(usize, Option<NaiveDate>),

    /// Edit a task's description
    Edit(usize, String),

    /// Set the category of task(s)
    SetCategory(TaskSelection, Option<String>),

    /// Set the recurrence pattern for task(s)
    SetRecurring(TaskSelection, Option<Recurrence>),

    /// Add a dependency to a task
    AddDependency(usize, usize),

    /// Remove a dependency from a task
    RemoveDependency(usize, usize),

    /// Show the dependency graph for a task
    ShowDependencyGraph(usize),

    /// List all categories
    ListCategories,

    /// Search for tasks by keyword
    Search(String),

    /// Show task statistics
    ShowStatistics,
}
