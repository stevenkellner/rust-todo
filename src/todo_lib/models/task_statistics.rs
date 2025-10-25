use serde::{Deserialize, Serialize};

/// Statistics about the tasks in a todo list.
///
/// This struct is used to return summarized information about a TodoList.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskStatistics {
    /// Total number of tasks
    pub total: usize,
    /// Number of completed tasks
    pub completed: usize,
    /// Number of pending tasks
    pub pending: usize,
    /// Completion percentage (0.0 to 100.0)
    pub completion_percentage: f64,
    /// Number of high priority tasks
    pub high_priority: usize,
    /// Number of medium priority tasks
    pub medium_priority: usize,
    /// Number of low priority tasks
    pub low_priority: usize,
}
