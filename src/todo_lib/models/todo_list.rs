use super::task::{Task, TaskWithoutId};
use super::priority::Priority;
use super::task_filter::TaskFilter;
use super::task_status::TaskStatus;
use super::overdue_filter::OverdueFilter;
use super::task_statistics::TaskStatistics;
use serde::{Serialize, Deserialize};

/// A collection of tasks with methods to manage them.
///
/// `TodoList` maintains a vector of tasks and automatically assigns unique IDs
/// to new tasks. It provides methods for adding, removing, and querying tasks.
///
/// # Examples
///
/// ```
/// use todo_manager::models::todo_list::TodoList;
/// use todo_manager::models::task::TaskWithoutId;
///
/// let mut list = TodoList::new();
/// let id = list.add_task(TaskWithoutId::new("Write tests".to_string()));
/// assert_eq!(list.is_empty(), false);
/// assert_eq!(list.get_tasks().len(), 1);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
    /// use todo_manager::models::task::TaskWithoutId;
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

}

impl TodoList {
    // (methods continue)

    /// Adds a task to the list.
    ///
    /// Creates a Task from the provided TaskWithoutId data and assigns it the next available ID.
    ///
    /// # Arguments
    ///
    /// * `new_task` - The task data to add to the list
    ///
    /// # Returns
    ///
    /// The unique ID assigned to the newly added task.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let new_task = TaskWithoutId::new("Buy groceries".to_string());
    /// let id = list.add_task(new_task);
    /// assert_eq!(id, 1);
    /// 
    /// let new_task2 = TaskWithoutId::new("Walk the dog".to_string());
    /// let id2 = list.add_task(new_task2);
    /// assert_eq!(id2, 2);
    /// ```
    pub fn add_task(&mut self, new_task: TaskWithoutId) -> usize {
        let task = new_task.to_task(self.next_id);
        let task_id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        task_id
    }

    /// Adds a new subtask under a parent task.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent task
    /// * `description` - The description for the new subtask
    ///
    /// # Returns
    ///
    /// * `Some(usize)` - The ID of the newly created subtask if parent exists
    /// * `None` - If the parent task doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let parent_id = list.add_task(TaskWithoutId::new("Main task".to_string()));
    /// 
    /// let subtask_id = list.add_subtask(parent_id, "Subtask 1".to_string());
    /// assert!(subtask_id.is_some());
    /// 
    /// let invalid = list.add_subtask(999, "Invalid".to_string());
    /// assert!(invalid.is_none());
    /// ```
    pub fn add_subtask(&mut self, parent_id: usize, description: String) -> Option<usize> {
        // Check if parent exists
        if !self.tasks.iter().any(|t| t.id == parent_id) {
            return None;
        }

        let mut new_task = TaskWithoutId::new(description);
        new_task.parent_id = Some(parent_id);
        let task = new_task.to_task(self.next_id);
        let task_id = task.id;
        self.tasks.push(task);
        self.next_id += 1;
        Some(task_id)
    }

    /// Returns a reference to the vector of all tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// assert_eq!(list.get_tasks().len(), 2);
    /// ```
    pub fn get_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    /// Returns all subtasks of a given parent task.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent task
    ///
    /// # Returns
    ///
    /// A vector of references to subtasks
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let parent_id = list.add_task(TaskWithoutId::new("Main task".to_string()));
    /// list.add_subtask(parent_id, "Subtask 1".to_string());
    /// list.add_subtask(parent_id, "Subtask 2".to_string());
    /// 
    /// let subtasks = list.get_subtasks(parent_id);
    /// assert_eq!(subtasks.len(), 2);
    /// ```
    pub fn get_subtasks(&self, parent_id: usize) -> Vec<&Task> {
        self.tasks
            .iter()
            .filter(|t| t.parent_id == Some(parent_id))
            .collect()
    }

    /// Returns the total count of subtasks for a given parent task.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent task
    ///
    /// # Returns
    ///
    /// The number of subtasks
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let parent_id = list.add_task(TaskWithoutId::new("Main task".to_string()));
    /// list.add_subtask(parent_id, "Subtask 1".to_string());
    /// list.add_subtask(parent_id, "Subtask 2".to_string());
    /// 
    /// assert_eq!(list.get_subtask_count(parent_id), 2);
    /// ```
    pub fn get_subtask_count(&self, parent_id: usize) -> usize {
        self.tasks
            .iter()
            .filter(|t| t.parent_id == Some(parent_id))
            .count()
    }

    /// Returns the count of completed subtasks for a given parent task.
    ///
    /// # Arguments
    ///
    /// * `parent_id` - The ID of the parent task
    ///
    /// # Returns
    ///
    /// The number of completed subtasks
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let parent_id = list.add_task(TaskWithoutId::new("Main task".to_string()));
    /// let sub1 = list.add_subtask(parent_id, "Subtask 1".to_string()).unwrap();
    /// let sub2 = list.add_subtask(parent_id, "Subtask 2".to_string()).unwrap();
    /// 
    /// list.complete_task(sub1);
    /// assert_eq!(list.get_completed_subtask_count(parent_id), 1);
    /// ```
    pub fn get_completed_subtask_count(&self, parent_id: usize) -> usize {
        self.tasks
            .iter()
            .filter(|t| t.parent_id == Some(parent_id) && t.completed)
            .count()
    }

    /// Returns `true` if the todo list contains no tasks.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// assert!(list.is_empty());
    /// 
    /// list.add_task(TaskWithoutId::new("First task".to_string()));
    /// assert!(!list.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Clears all tasks from the list.
    ///
    /// This is primarily intended for debug/testing purposes.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// list.clear_all();
    /// assert!(list.is_empty());
    /// ```
    pub fn clear_all(&mut self) {
        self.tasks.clear();
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Remove this".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Toggle me".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Completed task".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Pending task".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Completed task".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Pending task".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("High priority task".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Low priority task".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::task_filter::TaskFilter;
    /// use todo_manager::models::task_status::TaskStatus;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("High priority task".to_string()));
    /// list.set_task_priority(id, Priority::High);
    /// 
    /// let filter = TaskFilter::all().with_status(TaskStatus::Pending).with_priority(Priority::High);
    /// let tasks = list.get_filtered_tasks(&filter);
    /// assert_eq!(tasks.len(), 1);
    /// ```
    pub fn get_filtered_tasks(&self, filter: &TaskFilter) -> Vec<&Task> {
        let today = chrono::Local::now().date_naive();
        
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
            
            let overdue_matches = match filter.overdue {
                OverdueFilter::All => true,
                OverdueFilter::OnlyOverdue => task.is_overdue(today),
                OverdueFilter::OnlyNotOverdue => !task.is_overdue(today),
            };
            
            let category_matches = match &filter.category {
                Some(category) => task.category.as_ref().map(|c| c == category).unwrap_or(false),
                None => true,
            };
            
            status_matches && priority_matches && overdue_matches && category_matches
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
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Important task".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Buy groceries at the store".to_string()));
    /// list.add_task(TaskWithoutId::new("Read a book".to_string()));
    /// list.add_task(TaskWithoutId::new("Buy concert tickets".to_string()));
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

    /// Edits the description of a task by its ID.
    ///
    /// If the task with the given ID exists, its description is updated.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to edit
    /// * `new_description` - The new description for the task
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the edited task if found,
    /// or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Old description".to_string()));
    /// 
    /// list.edit_task(id, "New description".to_string());
    /// assert_eq!(list.get_tasks()[0].description, "New description");
    /// ```
    pub fn edit_task(&mut self, id: usize, new_description: String) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.description = new_description;
            Some(task)
        } else {
            None
        }
    }

    /// Sets the due date for a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task to update
    /// * `due_date` - An optional date when the task is due. Pass `None` to clear the due date.
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the updated task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    /// use chrono::NaiveDate;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Submit report".to_string()));
    /// let due = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    /// 
    /// list.set_due_date(id, Some(due));
    /// assert_eq!(list.get_tasks()[0].due_date, Some(due));
    /// 
    /// list.set_due_date(id, None);
    /// assert_eq!(list.get_tasks()[0].due_date, None);
    /// ```
    pub fn set_due_date(&mut self, id: usize, due_date: Option<chrono::NaiveDate>) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.set_due_date(due_date);
            Some(task)
        } else {
            None
        }
    }

    /// Sets the category of a task by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The unique identifier of the task
    /// * `category` - An optional category name. Pass `None` to clear the category.
    ///
    /// # Returns
    ///
    /// `Some(&Task)` containing a reference to the updated task if found, or `None` if no task with the given ID exists.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Write code".to_string()));
    /// 
    /// list.set_task_category(id, Some("work".to_string()));
    /// assert_eq!(list.get_tasks()[0].category, Some("work".to_string()));
    /// 
    /// list.set_task_category(id, None);
    /// assert_eq!(list.get_tasks()[0].category, None);
    /// ```
    pub fn set_task_category(&mut self, id: usize, category: Option<String>) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.set_category(category);
            Some(task)
        } else {
            None
        }
    }

    /// Sets the recurrence pattern of a task with the given ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the task to update
    /// * `recurrence` - The new recurrence pattern (or None to clear)
    ///
    /// # Returns
    ///
    /// A reference to the updated task, or `None` if the task doesn't exist
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Daily task".to_string()));
    /// 
    /// list.set_task_recurrence(id, Some(Recurrence::Daily));
    /// assert_eq!(list.get_tasks()[0].get_recurrence(), Some(Recurrence::Daily));
    /// 
    /// list.set_task_recurrence(id, None);
    /// assert_eq!(list.get_tasks()[0].get_recurrence(), None);
    /// ```
    pub fn set_task_recurrence(&mut self, id: usize, recurrence: Option<crate::models::recurrence::Recurrence>) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id == id) {
            task.set_recurrence(recurrence);
            Some(task)
        } else {
            None
        }
    }

    /// Sets the recurrence pattern for multiple tasks by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - The IDs of the tasks to update
    /// * `recurrence` - The recurrence pattern to set (or None to clear)
    ///
    /// # Returns
    ///
    /// A tuple with (number of tasks updated, vector of IDs not found)
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (updated, not_found) = list.set_recurrence_multiple(&[id1, id2, 999], Some(Recurrence::Weekly));
    /// assert_eq!(updated, 2);
    /// assert_eq!(not_found, vec![999]);
    /// ```
    pub fn set_recurrence_multiple(&mut self, ids: &[usize], recurrence: Option<crate::models::recurrence::Recurrence>) -> (usize, Vec<usize>) {
        let mut updated_count = 0;
        let mut not_found = Vec::new();
        
        for &id in ids {
            if self.set_task_recurrence(id, recurrence).is_some() {
                updated_count += 1;
            } else {
                not_found.push(id);
            }
        }
        
        (updated_count, not_found)
    }

    /// Gets all unique categories from all tasks.
    ///
    /// # Returns
    ///
    /// A vector of unique category names sorted alphabetically.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// let id3 = list.add_task(TaskWithoutId::new("Task 3".to_string()));
    /// 
    /// list.set_task_category(id1, Some("work".to_string()));
    /// list.set_task_category(id2, Some("personal".to_string()));
    /// list.set_task_category(id3, Some("work".to_string()));
    /// 
    /// let categories = list.get_all_categories();
    /// assert_eq!(categories, vec!["personal", "work"]);
    /// ```
    pub fn get_all_categories(&self) -> Vec<String> {
        let mut categories: Vec<String> = self.tasks
            .iter()
            .filter_map(|task| task.category.clone())
            .collect();
        
        // Remove duplicates and sort
        categories.sort();
        categories.dedup();
        categories
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Finish this".to_string()));
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
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id = list.add_task(TaskWithoutId::new("Do this again".to_string()));
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

    /// Completes multiple tasks specified by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs to complete
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully completed
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (completed, not_found) = list.complete_tasks(&[id1, id2]);
    /// assert_eq!(completed, 2);
    /// assert_eq!(not_found.len(), 0);
    /// ```
    pub fn complete_tasks(&mut self, ids: &[usize]) -> (usize, Vec<usize>) {
        let mut completed_count = 0;
        let mut not_found = Vec::new();

        for &id in ids {
            if self.complete_task(id).is_some() {
                completed_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (completed_count, not_found)
    }

    /// Removes multiple tasks specified by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs to remove
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully removed
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (removed, not_found) = list.remove_tasks(&[id1, id2]);
    /// assert_eq!(removed, 2);
    /// assert_eq!(not_found.len(), 0);
    /// assert_eq!(list.get_tasks().len(), 0);
    /// ```
    pub fn remove_tasks(&mut self, ids: &[usize]) -> (usize, Vec<usize>) {
        let mut removed_count = 0;
        let mut not_found = Vec::new();

        // Sort IDs in descending order to avoid index shifting issues
        let mut sorted_ids: Vec<usize> = ids.to_vec();
        sorted_ids.sort_unstable_by(|a, b| b.cmp(a));
        sorted_ids.dedup();

        for id in sorted_ids {
            if self.remove_task(id).is_some() {
                removed_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (removed_count, not_found)
    }

    /// Completes all tasks in the list.
    ///
    /// # Returns
    ///
    /// The number of tasks that were completed
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let completed = list.complete_all_tasks();
    /// assert_eq!(completed, 2);
    /// assert!(list.get_tasks().iter().all(|t| t.is_completed()));
    /// ```
    pub fn complete_all_tasks(&mut self) -> usize {
        let ids: Vec<usize> = self.tasks.iter().map(|t| t.id).collect();
        let (completed, _) = self.complete_tasks(&ids);
        completed
    }

    /// Removes all tasks from the list.
    ///
    /// # Returns
    ///
    /// The number of tasks that were removed
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let removed = list.remove_all_tasks();
    /// assert_eq!(removed, 2);
    /// assert_eq!(list.get_tasks().len(), 0);
    /// ```
    pub fn remove_all_tasks(&mut self) -> usize {
        let count = self.tasks.len();
        self.tasks.clear();
        count
    }

    /// Marks multiple tasks as pending (incomplete) by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs to mark as pending
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully marked as pending
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// list.complete_task(id1);
    /// list.complete_task(id2);
    /// 
    /// let (uncompleted, not_found) = list.uncomplete_tasks(&[id1, id2]);
    /// assert_eq!(uncompleted, 2);
    /// assert_eq!(not_found.len(), 0);
    /// ```
    pub fn uncomplete_tasks(&mut self, ids: &[usize]) -> (usize, Vec<usize>) {
        let mut uncompleted_count = 0;
        let mut not_found = Vec::new();

        for &id in ids {
            if self.uncomplete_task(id).is_some() {
                uncompleted_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (uncompleted_count, not_found)
    }

    /// Marks all tasks as pending (incomplete).
    ///
    /// # Returns
    ///
    /// The number of tasks that were marked as pending
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// list.complete_all_tasks();
    /// 
    /// let uncompleted = list.uncomplete_all_tasks();
    /// assert_eq!(uncompleted, 2);
    /// assert!(list.get_tasks().iter().all(|t| !t.is_completed()));
    /// ```
    pub fn uncomplete_all_tasks(&mut self) -> usize {
        let ids: Vec<usize> = self.tasks.iter().map(|t| t.id).collect();
        let (uncompleted, _) = self.uncomplete_tasks(&ids);
        uncompleted
    }

    /// Toggles the completion status of multiple tasks by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs to toggle
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully toggled
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (toggled, not_found) = list.toggle_tasks(&[id1, id2]);
    /// assert_eq!(toggled, 2);
    /// assert_eq!(not_found.len(), 0);
    /// assert!(list.get_tasks().iter().all(|t| t.is_completed()));
    /// ```
    pub fn toggle_tasks(&mut self, ids: &[usize]) -> (usize, Vec<usize>) {
        let mut toggled_count = 0;
        let mut not_found = Vec::new();

        for &id in ids {
            if self.toggle_task(id).is_some() {
                toggled_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (toggled_count, not_found)
    }

    /// Toggles the completion status of all tasks.
    ///
    /// # Returns
    ///
    /// The number of tasks that were toggled
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let toggled = list.toggle_all_tasks();
    /// assert_eq!(toggled, 2);
    /// assert!(list.get_tasks().iter().all(|t| t.is_completed()));
    /// ```
    pub fn toggle_all_tasks(&mut self) -> usize {
        let ids: Vec<usize> = self.tasks.iter().map(|t| t.id).collect();
        let (toggled, _) = self.toggle_tasks(&ids);
        toggled
    }

    /// Sets the priority of multiple tasks by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs
    /// * `priority` - The priority level to set
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully updated
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (updated, not_found) = list.set_priority_multiple(&[id1, id2], Priority::High);
    /// assert_eq!(updated, 2);
    /// assert_eq!(not_found.len(), 0);
    /// ```
    pub fn set_priority_multiple(&mut self, ids: &[usize], priority: Priority) -> (usize, Vec<usize>) {
        let mut updated_count = 0;
        let mut not_found = Vec::new();

        for &id in ids {
            if self.set_task_priority(id, priority).is_some() {
                updated_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (updated_count, not_found)
    }

    /// Sets the category of multiple tasks by their IDs.
    ///
    /// # Arguments
    ///
    /// * `ids` - A slice of task IDs
    /// * `category` - The category to set, or None to clear
    ///
    /// # Returns
    ///
    /// A tuple containing:
    /// - Number of tasks successfully updated
    /// - Vector of IDs that were not found
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// 
    /// let (updated, not_found) = list.set_category_multiple(&[id1, id2], Some("work".to_string()));
    /// assert_eq!(updated, 2);
    /// assert_eq!(not_found.len(), 0);
    /// ```
    pub fn set_category_multiple(&mut self, ids: &[usize], category: Option<String>) -> (usize, Vec<usize>) {
        let mut updated_count = 0;
        let mut not_found = Vec::new();

        for &id in ids {
            if self.set_task_category(id, category.clone()).is_some() {
                updated_count += 1;
            } else {
                not_found.push(id);
            }
        }

        (updated_count, not_found)
    }

    /// Gets statistics about the tasks in the todo list.
    ///
    /// Returns a `TaskStatistics` struct containing:
    /// - Total number of tasks
    /// - Number of completed and pending tasks
    /// - Completion percentage
    /// - Task counts by priority level
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::todo_list::TodoList;
    /// use todo_manager::models::task::TaskWithoutId;
    /// use todo_manager::models::priority::Priority;
    ///
    /// let mut list = TodoList::new();
    /// let id1 = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    /// let id2 = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    /// list.set_task_priority(id1, Priority::High);
    /// list.complete_task(id1);
    ///
    /// let stats = list.get_statistics();
    /// assert_eq!(stats.total, 2);
    /// assert_eq!(stats.completed, 1);
    /// assert_eq!(stats.pending, 1);
    /// assert_eq!(stats.completion_percentage, 50.0);
    /// assert_eq!(stats.high_priority, 1);
    /// ```
    pub fn get_statistics(&self) -> TaskStatistics {
        let total = self.tasks.len();
        let completed = self.tasks.iter().filter(|t| t.is_completed()).count();
        let pending = total - completed;
        
        let completion_percentage = if total > 0 {
            (completed as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        
        let high_priority = self.tasks.iter()
            .filter(|t| t.get_priority() == Priority::High)
            .count();
        let medium_priority = self.tasks.iter()
            .filter(|t| t.get_priority() == Priority::Medium)
            .count();
        let low_priority = self.tasks.iter()
            .filter(|t| t.get_priority() == Priority::Low)
            .count();
        
        TaskStatistics {
            total,
            completed,
            pending,
            completion_percentage,
            high_priority,
            medium_priority,
            low_priority,
        }
    }
}

impl Default for TodoList {
    fn default() -> Self {
        Self::new()
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
        let task_id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        assert_eq!(task_id, 1);
        assert!(!todo_list.is_empty());
        assert_eq!(todo_list.get_tasks().len(), 1);
        
        let task = &todo_list.get_tasks()[0];
        assert_eq!(task.description, "Test task");
        assert!(!task.completed);
    }

    #[test]
    fn test_add_multiple_tasks() {
        let mut todo_list = TodoList::new();
        let task1_id = todo_list.add_task(TaskWithoutId::new("First task".to_string()));
        let task2_id = todo_list.add_task(TaskWithoutId::new("Second task".to_string()));
        
        assert_eq!(task1_id, 1);
        assert_eq!(task2_id, 2);
        assert_eq!(todo_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_remove_task() {
        let mut todo_list = TodoList::new();
        let task_id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
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
        let task_id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
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
        let task1_id = todo_list.add_task(TaskWithoutId::new("Completed task".to_string()));
        let _task2_id = todo_list.add_task(TaskWithoutId::new("Pending task".to_string()));
        
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
        todo_list.add_task(TaskWithoutId::new("Buy groceries at the store".to_string()));
        todo_list.add_task(TaskWithoutId::new("Read a book".to_string()));
        todo_list.add_task(TaskWithoutId::new("Buy concert tickets".to_string()));
        
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
        todo_list.add_task(TaskWithoutId::new("Programming homework".to_string()));
        todo_list.add_task(TaskWithoutId::new("Program the microwave".to_string()));
        todo_list.add_task(TaskWithoutId::new("Write documentation".to_string()));
        
        let results = todo_list.search_tasks("program");
        assert_eq!(results.len(), 2);
    }

    #[test]
    fn test_search_tasks_no_results() {
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Task one".to_string()));
        todo_list.add_task(TaskWithoutId::new("Task two".to_string()));
        
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
        let task1_id = todo_list.add_task(TaskWithoutId::new("Buy milk".to_string()));
        todo_list.add_task(TaskWithoutId::new("Buy bread".to_string()));
        todo_list.add_task(TaskWithoutId::new("Sell old laptop".to_string()));
        
        todo_list.complete_task(task1_id);
        
        let results = todo_list.search_tasks("buy");
        assert_eq!(results.len(), 2);
        
        // Verify both completed and pending tasks are found
        let completed_count = results.iter().filter(|t| t.is_completed()).count();
        assert_eq!(completed_count, 1);
    }
}
