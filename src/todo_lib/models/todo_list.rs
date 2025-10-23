use super::task::Task;
use super::priority::Priority;
use super::task_filter::TaskFilter;
use super::task_status::TaskStatus;

/// A collection of tasks with methods to manage them.
///
/// `TodoList` maintains a vector of tasks and automatically assigns unique IDs
/// to new tasks. It provides methods for adding, removing, and querying tasks.
///
/// # Examples
///
/// ```
/// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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

    /// Returns a vector of references to all tasks with the specified priority.
    ///
    /// # Arguments
    ///
    /// * `priority` - The priority level to filter by
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task("High priority task".to_string());
    /// let id2 = list.add_task("Low priority task".to_string());
    /// 
    /// list.set_task_priority(id1, Priority::High);
    /// let high_tasks = list.get_tasks_by_priority(Priority::High);
    /// assert_eq!(high_tasks.len(), 1);
    /// ```
    pub fn get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.priority == priority).collect()
    }

    /// Gets tasks filtered by both status and priority.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter criteria (status and/or priority)
    ///
    /// # Returns
    ///
    /// A vector of references to tasks that match the filter criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task_filter::TaskFilter;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("High priority task".to_string());
    /// list.set_task_priority(id, Priority::High);
    /// 
    /// let filter = TaskFilter::pending_with_priority(Priority::High);
    /// let tasks = list.get_filtered_tasks(&filter);
    /// assert_eq!(tasks.len(), 1);
    /// ```
    pub fn get_filtered_tasks(&self, filter: &TaskFilter) -> Vec<&Task> {
        self.tasks.iter().filter(|task| {
            let status_matches = match filter.status {
                Some(TaskStatus::Completed) => task.is_completed(),
                Some(TaskStatus::Pending) => !task.is_completed(),
                None => true,
            };
            
            let priority_matches = match filter.priority {
                Some(priority) => task.priority == priority,
                None => true,
            };
            
            status_matches && priority_matches
        }).collect()
    }

    /// Sets the priority of a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task
    /// * `priority` - The new priority level to set
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task("Important task".to_string());
    /// 
    /// let task = list.set_task_priority(id, Priority::High);
    /// assert!(task.is_some());
    /// assert_eq!(task.unwrap().priority, Priority::High);
    /// ```
    pub fn set_task_priority(&mut self, id: usize, priority: Priority) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.set_priority(priority);
            return self.tasks.iter().find(|t| t.id == id);
        }
        None
    }

    /// Searches for tasks containing the given keyword in their description.
    ///
    /// The search is case-insensitive and matches partial strings.
    ///
    /// # Arguments
    ///
    /// * `keyword` - The search term to look for in task descriptions
    ///
    /// # Returns
    ///
    /// A vector of references to tasks that match the search criteria.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task("Buy groceries at the store".to_string());
    /// list.add_task("Read a book".to_string());
    /// list.add_task("Buy concert tickets".to_string());
    /// 
    /// let results = list.search_tasks("buy");
    /// assert_eq!(results.len(), 2);
    /// ```
    pub fn search_tasks(&self, keyword: &str) -> Vec<&Task> {
        let keyword_lower = keyword.to_lowercase();
        self.tasks.iter()
            .filter(|task| task.description.to_lowercase().contains(&keyword_lower))
            .collect()
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
    /// use todo_manager::models::todo_list::TodoList;
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
    /// use todo_manager::models::todo_list::TodoList;
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

    #[test]
    fn test_search_tasks_case_insensitive() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Buy groceries at the store".to_string());
        todo_list.add_task("Read a book".to_string());
        todo_list.add_task("Buy concert tickets".to_string());
        
        let results = todo_list.search_tasks("buy");
        assert_eq!(results.len(), 2);
        
        let results = todo_list.search_tasks("BUY");
        assert_eq!(results.len(), 2);
        
        let results = todo_list.search_tasks("Buy");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_tasks_partial_match() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Programming homework".to_string());
        todo_list.add_task("Program the microwave".to_string());
        todo_list.add_task("Write documentation".to_string());
        
        let results = todo_list.search_tasks("program");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_tasks_no_results() {
        let mut todo_list = TodoList::new();
        todo_list.add_task("Task one".to_string());
        todo_list.add_task("Task two".to_string());
        
        let results = todo_list.search_tasks("nonexistent");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_tasks_empty_list() {
        let todo_list = TodoList::new();
        
        let results = todo_list.search_tasks("anything");
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_search_tasks_with_completed_and_pending() {
        let mut todo_list = TodoList::new();
        let task1_id = todo_list.add_task("Buy milk".to_string());
        todo_list.add_task("Buy bread".to_string());
        todo_list.add_task("Sell old laptop".to_string());
        
        todo_list.complete_task(task1_id);
        
        let results = todo_list.search_tasks("buy");
        assert_eq!(results.len(), 2);
        
        // Verify both completed and pending tasks are found
        let completed_count = results.iter().filter(|t| t.is_completed()).count();
        assert_eq!(completed_count, 1);
    }
}