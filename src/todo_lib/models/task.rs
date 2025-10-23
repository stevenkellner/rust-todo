use super::priority::Priority;

/// Represents a single task in the todo list.
///
/// A task contains a unique identifier, a description of what needs to be done,
/// a completion status indicating whether the task has been finished, and a priority level.
///
/// # Examples
///
/// ```
/// use todo_manager::models::task::Task;
///
/// let task = Task::new(1, "Write documentation".to_string());
/// assert_eq!(task.id, 1);
/// assert_eq!(task.is_completed(), false);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    /// The unique identifier for this task
    pub id: usize,
    /// A textual description of the task
    pub description: String,
    /// Whether the task has been completed
    pub completed: bool,
    /// The priority level of the task
    pub priority: Priority,
}

impl Task {
    /// Creates a new task with the given ID and description.
    ///
    /// The task is initialized with `completed` set to `false` and priority set to `Medium`.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the task
    /// * `description` - A description of what needs to be done
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let task = Task::new(1, "Buy milk".to_string());
    /// assert_eq!(task.id, 1);
    /// assert_eq!(task.description, "Buy milk");
    /// assert!(!task.completed);
    /// ```
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
            priority: Priority::default(),
        }
    }

    /// Sets the priority of the task.
    ///
    /// # Arguments
    ///
    /// * `priority` - The new priority level for the task
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut task = Task::new(1, "Important meeting".to_string());
    /// task.set_priority(Priority::High);
    /// assert_eq!(task.priority, Priority::High);
    /// ```
    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
    }

    /// Gets the priority of the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let task = Task::new(1, "Regular task".to_string());
    /// assert_eq!(task.get_priority(), Priority::Medium);
    /// ```
    pub fn get_priority(&self) -> Priority {
        self.priority
    }
    /// Toggles the completion status of the task.
    ///
    /// If the task is completed, it becomes pending. If it's pending, it becomes completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(1, "Do laundry".to_string());
    /// assert!(!task.is_completed());
    /// 
    /// task.toggle_completion();
    /// assert!(task.is_completed());
    /// 
    /// task.toggle_completion();
    /// assert!(!task.is_completed());
    /// ```
    pub fn toggle_completion(&mut self) {
        self.completed = !self.completed;
    }

    /// Returns whether the task is completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let task = Task::new(1, "Read a book".to_string());
    /// assert_eq!(task.is_completed(), false);
    /// ```
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Returns a visual symbol representing the task's completion status.
    ///
    /// Returns "✓" for completed tasks and " " (space) for pending tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(1, "Clean room".to_string());
    /// assert_eq!(task.get_status_symbol(), " ");
    /// 
    /// task.toggle_completion();
    /// assert_eq!(task.get_status_symbol(), "✓");
    /// ```
    pub fn get_status_symbol(&self) -> &str {
        if self.completed { "✓" } else { " " }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_task_creation() {
        let task = Task::new(1, "Test task".to_string());
        assert_eq!(task.id, 1);
        assert_eq!(task.description, "Test task");
        assert_eq!(task.completed, false);
    }

    #[test]
    fn test_toggle_completion() {
        let mut task = Task::new(1, "Test task".to_string());
        assert_eq!(task.is_completed(), false);
        
        task.toggle_completion();
        assert_eq!(task.is_completed(), true);
        
        task.toggle_completion();
        assert_eq!(task.is_completed(), false);
    }

    #[test]
    fn test_status_symbol() {
        let mut task = Task::new(1, "Test task".to_string());
        assert_eq!(task.get_status_symbol(), " ");
        
        task.toggle_completion();
        assert_eq!(task.get_status_symbol(), "✓");
    }

    #[test]
    fn test_priority_default() {
        let task = Task::new(1, "Test task".to_string());
        assert_eq!(task.get_priority(), Priority::Medium);
    }

    #[test]
    fn test_set_priority() {
        let mut task = Task::new(1, "Test task".to_string());
        
        task.set_priority(Priority::High);
        assert_eq!(task.get_priority(), Priority::High);
        
        task.set_priority(Priority::Low);
        assert_eq!(task.get_priority(), Priority::Low);
    }

    #[test]
    fn test_priority_from_str() {
        assert_eq!(Priority::from_str("high"), Some(Priority::High));
        assert_eq!(Priority::from_str("h"), Some(Priority::High));
        assert_eq!(Priority::from_str("HIGH"), Some(Priority::High));
        
        assert_eq!(Priority::from_str("medium"), Some(Priority::Medium));
        assert_eq!(Priority::from_str("med"), Some(Priority::Medium));
        assert_eq!(Priority::from_str("m"), Some(Priority::Medium));
        
        assert_eq!(Priority::from_str("low"), Some(Priority::Low));
        assert_eq!(Priority::from_str("l"), Some(Priority::Low));
        
        assert_eq!(Priority::from_str("invalid"), None);
    }

    #[test]
    fn test_priority_ordering() {
        assert!(Priority::High > Priority::Medium);
        assert!(Priority::Medium > Priority::Low);
        assert!(Priority::High > Priority::Low);
    }
}