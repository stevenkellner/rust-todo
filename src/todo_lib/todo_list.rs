use super::task::Task;

/// A collection of tasks with methods to manage them.
///
/// `TodoList` maintains a vector of tasks and automatically assigns unique IDs
/// to new tasks. It provides methods for adding, removing, and querying tasks.
///
/// # Examples
///
/// ```
/// use todo_manager::todo_list::TodoList;
///
/// let mut list = TodoList::new();
/// let id = list.add_task("Write tests".to_string());
/// assert_eq!(list.is_empty(), false);
/// assert_eq!(list.get_tasks().len(), 1);
/// ```
pub struct TodoList {
    /// The collection of tasks managed by this todo list.
    tasks: Vec<Task>,
    /// The next available ID to be assigned to a new task.
    /// This value is incremented each time a task is added.
    next_id: usize,
}

impl TodoList {
    /// Creates a new empty todo list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let list = TodoList::new();
    /// assert!(list.is_empty());
    /// ```
    pub fn new() -> Self {
        TodoList {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    /// Adds a new task to the list with the given description.
    ///
    /// The task is automatically assigned a unique ID and is initially marked as incomplete.
    ///
    /// # Arguments
    ///
    /// * `description` - A string describing the task
    ///
    /// # Returns
    ///
    /// The unique ID assigned to the newly created task.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Buy groceries".to_string());
    /// assert_eq!(id, 1);
    /// 
    /// let id2 = list.add_task("Walk the dog".to_string());
    /// assert_eq!(id2, 2);
    /// ```
    pub fn add_task(&mut self, description: String) -> usize {
        let task = Task::new(self.next_id, description);
        let task_id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        task_id
    }

    /// Returns a reference to the vector of all tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task("Task 1".to_string());
    /// list.add_task("Task 2".to_string());
    /// 
    /// assert_eq!(list.get_tasks().len(), 2);
    /// ```
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns `true` if the todo list contains no tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// assert!(list.is_empty());
    /// 
    /// list.add_task("First task".to_string());
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Removes a task from the list by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to remove
    ///
    /// # Returns
    ///
    /// `Some(Task)` containing the removed task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Remove this".to_string());
    /// 
    /// let removed = list.remove_task(id);
    /// assert!(removed.is_some());
    /// assert_eq!(removed.unwrap().description, "Remove this");
    /// ```
    pub fn remove_task(&mut self, id: usize) -> Option<Task> {
        if let Some(pos) = self.tasks.iter().position(|task| task.id == id) {
            Some(self.tasks.remove(pos))
        } else {
            None
        }
    }

    /// Toggles the completion status of a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to toggle
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the modified task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Toggle me".to_string());
    /// 
    /// list.toggle_task(id);
    /// let task = list.get_tasks().iter().find(|t| t.id == id).unwrap();
    /// assert!(task.is_completed());
    /// ```
    pub fn toggle_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.toggle_completion();
            Some(task)
        } else {
            None
        }
    }

    /// Returns a vector of references to all completed tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task("Completed task".to_string());
    /// let id2 = list.add_task("Pending task".to_string());
    /// 
    /// list.toggle_task(id1);
    /// let completed = list.get_completed_tasks();
    /// assert_eq!(completed.len(), 1);
    /// ```
    pub fn get_completed_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.is_completed()).collect()
    }

    /// Returns a vector of references to all pending (incomplete) tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task("Completed task".to_string());
    /// let id2 = list.add_task("Pending task".to_string());
    /// 
    /// list.toggle_task(id1);
    /// let pending = list.get_pending_tasks();
    /// assert_eq!(pending.len(), 1);
    /// ```
    pub fn get_pending_tasks(&self) -> Vec<&Task> {
        self.tasks.iter().filter(|task| !task.is_completed()).collect()
    }

    /// Marks a task as completed by its ID.
    ///
    /// If the task is already completed, this method has no effect but still returns the task.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to complete
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Finish this".to_string());
    /// 
    /// list.complete_task(id);
    /// let task = list.get_tasks().iter().find(|t| t.id == id).unwrap();
    /// assert!(task.is_completed());
    /// ```
    pub fn complete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            if !task.is_completed() {
                task.toggle_completion();
            }
            Some(task)
        } else {
            None
        }
    }

    /// Marks a task as pending (incomplete) by its ID.
    ///
    /// If the task is already pending, this method has no effect but still returns the task.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to mark as pending
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Do this again".to_string());
    /// 
    /// list.complete_task(id);
    /// assert!(list.get_tasks()[0].is_completed());
    /// 
    /// list.uncomplete_task(id);
    /// assert!(!list.get_tasks()[0].is_completed());
    /// ```
    pub fn uncomplete_task(&mut self, id: usize) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            if task.is_completed() {
                task.toggle_completion();
            }
            Some(task)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_todo_list() {
        let todo_list = TodoList::new();
        assert!(todo_list.is_empty());
        assert_eq!(todo_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new();
        let task_id = todo_list.add_task("Test task".to_string());
        
        assert_eq!(task_id, 1);
        assert!(!todo_list.is_empty());
        assert_eq!(todo_list.get_tasks().len(), 1);
        
        let task = &todo_list.get_tasks()[0];
        assert_eq!(task.description, "Test task");
        assert_eq!(task.completed, false);
    }

    #[test]
    fn test_add_multiple_tasks() {
        let mut todo_list = TodoList::new();
        let task1_id = todo_list.add_task("First task".to_string());
        let task2_id = todo_list.add_task("Second task".to_string());
        
        assert_eq!(task1_id, 1);
        assert_eq!(task2_id, 2);
        assert_eq!(todo_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let task_id = todo_list.add_task("Test task".to_string());
        
        let removed_task = todo_list.remove_task(task_id);
        assert!(removed_task.is_some());
        assert_eq!(removed_task.unwrap().description, "Test task");
        assert!(todo_list.is_empty());
    }

    #[test]
    fn test_remove_nonexistent_task() {
        let mut todo_list = TodoList::new();
        let removed_task = todo_list.remove_task(999);
        assert!(removed_task.is_none());
    }

    #[test]
    fn test_toggle_task() {
        let mut todo_list = TodoList::new();
        let task_id = todo_list.add_task("Test task".to_string());
        
        let task = todo_list.toggle_task(task_id);
        assert!(task.is_some());
        assert!(task.unwrap().is_completed());
        
        let task = todo_list.toggle_task(task_id);
        assert!(task.is_some());
        assert!(!task.unwrap().is_completed());
    }

    #[test]
    fn test_toggle_nonexistent_task() {
        let mut todo_list = TodoList::new();
        let result = todo_list.toggle_task(999);
        assert!(result.is_none());
    }

    #[test]
    fn test_get_completed_and_pending_tasks() {
        let mut todo_list = TodoList::new();
        let task1_id = todo_list.add_task("Completed task".to_string());
        let _task2_id = todo_list.add_task("Pending task".to_string());
        
        todo_list.toggle_task(task1_id);
        
        let completed_tasks = todo_list.get_completed_tasks();
        let pending_tasks = todo_list.get_pending_tasks();
        
        assert_eq!(completed_tasks.len(), 1);
        assert_eq!(pending_tasks.len(), 1);
        assert_eq!(completed_tasks[0].description, "Completed task");
        assert_eq!(pending_tasks[0].description, "Pending task");
    }
}