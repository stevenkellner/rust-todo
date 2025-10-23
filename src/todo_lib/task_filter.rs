use super::priority::Priority;
use super::task_status::TaskStatus;

/// Filter options for listing tasks.
///
/// Determines which tasks should be displayed when listing.
#[derive(Debug, Clone, PartialEq)]
pub struct TaskFilter {
    /// Filter by completion status (None means all statuses)
    pub status: Option<TaskStatus>,
    
    /// Filter by priority level (None means all priorities)
    pub priority: Option<Priority>,
}

impl TaskFilter {
    /// Creates a filter that shows all tasks.
    pub fn all() -> Self {
        TaskFilter {
            status: None,
            priority: None,
        }
    }

    /// Creates a filter for completed tasks.
    pub fn completed() -> Self {
        TaskFilter {
            status: Some(TaskStatus::Completed),
            priority: None,
        }
    }

    /// Creates a filter for pending tasks.
    pub fn pending() -> Self {
        TaskFilter {
            status: Some(TaskStatus::Pending),
            priority: None,
        }
    }

    /// Creates a filter for tasks with a specific priority.
    pub fn by_priority(priority: Priority) -> Self {
        TaskFilter {
            status: None,
            priority: Some(priority),
        }
    }

    /// Creates a filter for completed tasks with a specific priority.
    pub fn completed_with_priority(priority: Priority) -> Self {
        TaskFilter {
            status: Some(TaskStatus::Completed),
            priority: Some(priority),
        }
    }

    /// Creates a filter for pending tasks with a specific priority.
    pub fn pending_with_priority(priority: Priority) -> Self {
        TaskFilter {
            status: Some(TaskStatus::Pending),
            priority: Some(priority),
        }
    }

    /// Sets the status filter.
    pub fn with_status(mut self, status: TaskStatus) -> Self {
        self.status = Some(status);
        self
    }

    /// Sets the priority filter.
    pub fn with_priority(mut self, priority: Priority) -> Self {
        self.priority = Some(priority);
        self
    }
}
