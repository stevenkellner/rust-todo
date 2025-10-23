/// Represents a single task in the todo list.
///
/// A task contains a unique identifier, a description of what needs to be done,
/// and a completion status indicating whether the task has been finished.
///
/// # Examples
///
/// ```
/// use todo_manager::task::Task;
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
}

impl Task {
    /// Creates a new task with the given ID and description.
    ///
    /// The task is initialized with `completed` set to `false`.
    ///
    /// # Arguments
    ///
    /// * `id` - A unique identifier for the task
    /// * `description` - A description of what needs to be done
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::task::Task;
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
        }
    }

    /// Toggles the completion status of the task.
    ///
    /// If the task is completed, it becomes pending. If it's pending, it becomes completed.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::task::Task;
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
    /// use todo_manager::task::Task;
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
    /// use todo_manager::task::Task;
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
}