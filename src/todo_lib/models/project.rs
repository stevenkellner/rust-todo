use crate::models::todo_list::TodoList;
use serde::{Deserialize, Serialize};

/// Represents a project containing a todo list.
///
/// Projects allow organizing tasks into separate lists,
/// making it easier to manage tasks for different contexts
/// (e.g., work, personal, specific projects).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// The name of the project
    pub name: String,

    /// The todo list associated with this project
    pub todo_list: TodoList,
}

impl Project {
    /// Creates a new project with the given name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project
    ///
    /// # Returns
    ///
    /// A new `Project` instance with an empty todo list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::project::Project;
    ///
    /// let project = Project::new("Work".to_string());
    /// assert_eq!(project.name, "Work");
    /// assert_eq!(project.todo_list.get_tasks().len(), 0);
    /// ```
    pub fn new(name: String) -> Self {
        Self {
            name,
            todo_list: TodoList::new(),
        }
    }

    /// Returns a reference to the project's todo list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::project::Project;
    ///
    /// let project = Project::new("Personal".to_string());
    /// let todo_list = project.get_todo_list();
    /// assert_eq!(todo_list.get_tasks().len(), 0);
    /// ```
    pub fn get_todo_list(&self) -> &TodoList {
        &self.todo_list
    }

    /// Returns a mutable reference to the project's todo list.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::models::project::Project;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut project = Project::new("Work".to_string());
    /// let task = TaskWithoutId::new("Complete report".to_string());
    /// project.get_todo_list_mut().add_task(task);
    /// assert_eq!(project.todo_list.get_tasks().len(), 1);
    /// ```
    pub fn get_todo_list_mut(&mut self) -> &mut TodoList {
        &mut self.todo_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;

    #[test]
    fn test_new_project() {
        let project = Project::new("Test".to_string());
        assert_eq!(project.name, "Test");
        assert_eq!(project.todo_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_get_todo_list() {
        let project = Project::new("Test".to_string());
        let todo_list = project.get_todo_list();
        assert_eq!(todo_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_get_todo_list_mut() {
        let mut project = Project::new("Test".to_string());
        let task = TaskWithoutId::new("Task 1".to_string());
        project.get_todo_list_mut().add_task(task);
        assert_eq!(project.todo_list.get_tasks().len(), 1);
    }
}
