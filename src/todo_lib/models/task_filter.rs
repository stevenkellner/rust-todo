use super::overdue_filter::OverdueFilter;
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

    /// Filter by overdue status
    pub overdue: OverdueFilter,

    /// Filter by category (None means all categories)
    pub category: Option<String>,
}

impl TaskFilter {
    /// Creates a new task filter with the specified criteria.
    ///
    /// # Arguments
    ///
    /// * `status` - Filter by completion status (None means all statuses)
    /// * `priority` - Filter by priority level (None means all priorities)
    /// * `overdue` - Filter by overdue status (defaults to All if not specified)
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task_filter::TaskFilter;
    /// use todo_manager::models::task_status::TaskStatus;
    /// use todo_manager::models::priority::Priority;
    /// use todo_manager::models::overdue_filter::OverdueFilter;
    ///
    /// // All tasks
    /// let filter = TaskFilter::new(None, None, OverdueFilter::All);
    ///
    /// // Only pending tasks
    /// let filter = TaskFilter::new(Some(TaskStatus::Pending), None, OverdueFilter::All);
    ///
    /// // Only high priority tasks
    /// let filter = TaskFilter::new(None, Some(Priority::High), OverdueFilter::All);
    ///
    /// // Pending high priority overdue tasks
    /// let filter = TaskFilter::new(
    ///     Some(TaskStatus::Pending),
    ///     Some(Priority::High),
    ///     OverdueFilter::OnlyOverdue
    /// );
    /// ```
    pub fn new(
        status: Option<TaskStatus>,
        priority: Option<Priority>,
        overdue: OverdueFilter,
    ) -> Self {
        TaskFilter {
            status,
            priority,
            overdue,
            category: None,
        }
    }

    /// Creates a filter that shows all tasks.
    pub fn all() -> Self {
        Self::new(None, None, OverdueFilter::All)
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

    /// Sets the overdue filter.
    pub fn with_overdue(mut self, overdue: OverdueFilter) -> Self {
        self.overdue = overdue;
        self
    }

    /// Sets the category filter.
    pub fn with_category(mut self, category: String) -> Self {
        self.category = Some(category);
        self
    }
}
