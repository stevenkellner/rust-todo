use super::priority::Priority;
use super::recurrence::Recurrence;
use chrono::{NaiveDate, Datelike};
use serde::{Serialize, Deserialize};

/// Represents task data without an ID.
///
/// This is used for creating new tasks before they are assigned an ID by the TodoList.
///
/// # Examples
///
/// ```
/// use todo_manager::models::task::TaskWithoutId;
/// use todo_manager::models::priority::Priority;
///
/// let task = TaskWithoutId::new("Write documentation".to_string());
/// assert_eq!(task.description, "Write documentation");
/// assert_eq!(task.completed, false);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskWithoutId {
    /// A textual description of the task
    pub description: String,
    /// Whether the task has been completed
    pub completed: bool,
    /// The priority level of the task
    pub priority: Priority,
    /// The optional due date for the task
    pub due_date: Option<NaiveDate>,
    /// The optional category/tag for the task
    pub category: Option<String>,
    /// The optional parent task ID for subtasks
    pub parent_id: Option<usize>,
    /// The optional recurrence pattern for the task
    pub recurrence: Option<Recurrence>,
    /// The list of task IDs that this task depends on
    pub depends_on: Vec<usize>,
}

impl TaskWithoutId {
    /// Creates a new task data with the given description.
    ///
    /// The task is initialized with `completed` set to `false` and priority set to `Medium`.
    ///
    /// # Arguments
    ///
    /// * `description` - A description of what needs to be done
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let task = TaskWithoutId::new("Buy milk".to_string());
    /// assert_eq!(task.description, "Buy milk");
    /// assert!(!task.completed);
    /// ```
    pub fn new(description: String) -> Self {
        TaskWithoutId {
            description,
            completed: false,
            priority: Priority::default(),
            due_date: None,
            category: None,
            parent_id: None,
            recurrence: None,
            depends_on: Vec::new(),
        }
    }

    /// Converts this TaskWithoutId into a Task with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier to assign to the task
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let task_without_id = TaskWithoutId::new("Buy milk".to_string());
    /// let task = task_without_id.to_task(1);
    /// assert_eq!(task.id, 1);
    /// assert_eq!(task.description, "Buy milk");
    /// ```
    pub fn to_task(self, id: usize) -> Task {
        Task {
            id,
            description: self.description,
            completed: self.completed,
            priority: self.priority,
            due_date: self.due_date,
            category: self.category,
            parent_id: self.parent_id,
            recurrence: self.recurrence,
            depends_on: self.depends_on,
        }
    }
}

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
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    /// The unique identifier for this task
    pub id: usize,
    /// A textual description of the task
    pub description: String,
    /// Whether the task has been completed
    pub completed: bool,
    /// The priority level of the task
    pub priority: Priority,
    /// The optional due date for the task
    pub due_date: Option<NaiveDate>,
    /// The optional category/tag for the task
    pub category: Option<String>,
    /// The optional parent task ID for subtasks
    pub parent_id: Option<usize>,
    /// The optional recurrence pattern for the task
    pub recurrence: Option<Recurrence>,
    /// The list of task IDs that this task depends on
    pub depends_on: Vec<usize>,
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
            due_date: None,
            category: None,
            parent_id: None,
            recurrence: None,
            depends_on: Vec::new(),
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

    /// Sets the due date for the task.
    ///
    /// # Arguments
    ///
    /// * `due_date` - An optional date when the task is due
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use chrono::NaiveDate;
    ///
    /// let mut task = Task::new(1, "Submit report".to_string());
    /// let due = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    /// task.set_due_date(Some(due));
    /// assert_eq!(task.due_date, Some(due));
    /// ```
    pub fn set_due_date(&mut self, due_date: Option<NaiveDate>) {
        self.due_date = due_date;
    }

    /// Returns the due date of the task.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let task = Task::new(1, "Write code".to_string());
    /// assert_eq!(task.get_due_date(), None);
    /// ```
    pub fn get_due_date(&self) -> Option<NaiveDate> {
        self.due_date
    }

    /// Checks if the task is overdue based on the current date.
    ///
    /// # Arguments
    ///
    /// * `today` - The current date to compare against
    ///
    /// # Returns
    ///
    /// `true` if the task has a due date and it's before today, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use chrono::NaiveDate;
    ///
    /// let mut task = Task::new(1, "Old task".to_string());
    /// let past = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    /// let today = NaiveDate::from_ymd_opt(2025, 10, 23).unwrap();
    /// task.set_due_date(Some(past));
    /// assert!(task.is_overdue(today));
    /// ```
    pub fn is_overdue(&self, today: NaiveDate) -> bool {
        if let Some(due) = self.due_date {
            due < today
        } else {
            false
        }
    }

    /// Sets the category of the task.
    ///
    /// # Arguments
    ///
    /// * `category` - The category/tag to assign (None to clear)
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(1, "Task".to_string());
    /// task.set_category(Some("work".to_string()));
    /// assert_eq!(task.get_category(), Some(&"work".to_string()));
    /// ```
    pub fn set_category(&mut self, category: Option<String>) {
        self.category = category;
    }

    /// Gets the category of the task.
    ///
    /// # Returns
    ///
    /// A reference to the optional category string
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let task = Task::new(1, "Task".to_string());
    /// assert_eq!(task.get_category(), None);
    /// ```
    pub fn get_category(&self) -> Option<&String> {
        self.category.as_ref()
    }

    /// Sets the parent task ID, making this task a subtask.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent task
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Subtask".to_string());
    /// task.set_parent_id(1);
    /// assert_eq!(task.get_parent_id(), Some(1));
    /// assert!(task.is_subtask());
    /// ```
    pub fn set_parent_id(&mut self, parent_id: usize) {
        self.parent_id = Some(parent_id);
    }

    /// Gets the parent task ID if this is a subtask.
    ///
    /// # Returns
    ///
    /// The parent task ID, or None if this is not a subtask
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let task = Task::new(1, "Main task".to_string());
    /// assert_eq!(task.get_parent_id(), None);
    /// ```
    pub fn get_parent_id(&self) -> Option<usize> {
        self.parent_id
    }

    /// Checks if this task is a subtask (has a parent).
    ///
    /// # Returns
    ///
    /// `true` if this task has a parent, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut main_task = Task::new(1, "Main task".to_string());
    /// assert!(!main_task.is_subtask());
    ///
    /// let mut subtask = Task::new(2, "Subtask".to_string());
    /// subtask.set_parent_id(1);
    /// assert!(subtask.is_subtask());
    /// ```
    pub fn is_subtask(&self) -> bool {
        self.parent_id.is_some()
    }

    /// Clears the parent task ID, making this task a top-level task.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Task".to_string());
    /// task.set_parent_id(1);
    /// assert!(task.is_subtask());
    ///
    /// task.clear_parent();
    /// assert!(!task.is_subtask());
    /// ```
    pub fn clear_parent(&mut self) {
        self.parent_id = None;
    }

    /// Sets the recurrence pattern for this task.
    ///
    /// # Arguments
    ///
    /// * `recurrence` - The recurrence pattern (Daily, Weekly, or Monthly), or None to clear
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// let mut task = Task::new(1, "Recurring task".to_string());
    /// task.set_recurrence(Some(Recurrence::Daily));
    /// assert_eq!(task.get_recurrence(), Some(Recurrence::Daily));
    /// assert!(task.is_recurring());
    /// ```
    pub fn set_recurrence(&mut self, recurrence: Option<Recurrence>) {
        self.recurrence = recurrence;
    }

    /// Gets the recurrence pattern for this task.
    ///
    /// # Returns
    ///
    /// The recurrence pattern, or None if the task is not recurring
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// let mut task = Task::new(1, "Recurring task".to_string());
    /// assert_eq!(task.get_recurrence(), None);
    ///
    /// task.set_recurrence(Some(Recurrence::Weekly));
    /// assert_eq!(task.get_recurrence(), Some(Recurrence::Weekly));
    /// ```
    pub fn get_recurrence(&self) -> Option<Recurrence> {
        self.recurrence
    }

    /// Checks if this task is recurring.
    ///
    /// # Returns
    ///
    /// `true` if the task has a recurrence pattern, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// let mut task = Task::new(1, "Task".to_string());
    /// assert!(!task.is_recurring());
    ///
    /// task.set_recurrence(Some(Recurrence::Monthly));
    /// assert!(task.is_recurring());
    /// ```
    pub fn is_recurring(&self) -> bool {
        self.recurrence.is_some()
    }

    /// Calculates the next due date based on the recurrence pattern.
    ///
    /// # Returns
    ///
    /// The next due date, or None if the task has no due date or no recurrence pattern
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    /// use todo_manager::models::recurrence::Recurrence;
    /// use chrono::NaiveDate;
    ///
    /// let mut task = Task::new(1, "Recurring task".to_string());
    /// task.set_due_date(Some(NaiveDate::from_ymd_opt(2025, 10, 25).unwrap()));
    /// task.set_recurrence(Some(Recurrence::Daily));
    ///
    /// let next_date = task.calculate_next_due_date();
    /// assert_eq!(next_date, Some(NaiveDate::from_ymd_opt(2025, 10, 26).unwrap()));
    /// ```
    pub fn calculate_next_due_date(&self) -> Option<NaiveDate> {
        match (self.due_date, self.recurrence) {
            (Some(due_date), Some(recurrence)) => {
                match recurrence {
                    Recurrence::Daily => Some(due_date + chrono::Duration::days(1)),
                    Recurrence::Weekly => Some(due_date + chrono::Duration::weeks(1)),
                    Recurrence::Monthly => {
                        // Add one month, handling month-end edge cases
                        let mut year = due_date.year();
                        let mut month = due_date.month() + 1;
                        if month > 12 {
                            month = 1;
                            year += 1;
                        }
                        // Handle day-of-month edge cases (e.g., Jan 31 -> Feb 28/29)
                        let day = due_date.day();
                        let max_day = match month {
                            2 => if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) { 29 } else { 28 },
                            4 | 6 | 9 | 11 => 30,
                            _ => 31,
                        };
                        let actual_day = day.min(max_day);
                        NaiveDate::from_ymd_opt(year, month, actual_day)
                    }
                }
            }
            _ => None,
        }
    }

    /// Adds a dependency to this task.
    ///
    /// # Arguments
    ///
    /// * `dependency_id` - The ID of the task that this task depends on
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Complete report".to_string());
    /// task.add_dependency(1);
    /// assert!(task.has_dependency(1));
    /// ```
    pub fn add_dependency(&mut self, dependency_id: usize) {
        if !self.depends_on.contains(&dependency_id) {
            self.depends_on.push(dependency_id);
        }
    }

    /// Removes a dependency from this task.
    ///
    /// # Arguments
    ///
    /// * `dependency_id` - The ID of the dependency to remove
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Complete report".to_string());
    /// task.add_dependency(1);
    /// task.remove_dependency(1);
    /// assert!(!task.has_dependency(1));
    /// ```
    pub fn remove_dependency(&mut self, dependency_id: usize) {
        self.depends_on.retain(|&id| id != dependency_id);
    }

    /// Gets the list of task IDs that this task depends on.
    ///
    /// # Returns
    ///
    /// A reference to the vector of dependency IDs
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Complete report".to_string());
    /// task.add_dependency(1);
    /// assert_eq!(task.get_dependencies(), &vec![1]);
    /// ```
    pub fn get_dependencies(&self) -> &Vec<usize> {
        &self.depends_on
    }

    /// Checks if this task has a specific dependency.
    ///
    /// # Arguments
    ///
    /// * `dependency_id` - The ID of the dependency to check
    ///
    /// # Returns
    ///
    /// `true` if the task depends on the specified task, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Complete report".to_string());
    /// task.add_dependency(1);
    /// assert!(task.has_dependency(1));
    /// assert!(!task.has_dependency(3));
    /// ```
    pub fn has_dependency(&self, dependency_id: usize) -> bool {
        self.depends_on.contains(&dependency_id)
    }

    /// Checks if this task has any dependencies.
    ///
    /// # Returns
    ///
    /// `true` if the task has at least one dependency, `false` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::task::Task;
    ///
    /// let mut task = Task::new(2, "Complete report".to_string());
    /// assert!(!task.has_dependencies());
    /// task.add_dependency(1);
    /// assert!(task.has_dependencies());
    /// ```
    pub fn has_dependencies(&self) -> bool {
        !self.depends_on.is_empty()
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
        assert!(!task.completed);
    }

    #[test]
    fn test_toggle_completion() {
        let mut task = Task::new(1, "Test task".to_string());
        assert!(!task.is_completed());
        
        task.toggle_completion();
        assert!(task.is_completed());
        
        task.toggle_completion();
        assert!(!task.is_completed());
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