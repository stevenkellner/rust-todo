use crate::models::project::Project;
use crate::models::todo_list::TodoList;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Manages multiple projects and tracks the current active project.
///
/// The ProjectManager allows users to organize tasks into separate projects,
/// switch between them, and manage them independently.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectManager {
    /// Map of project names to Project instances
    projects: HashMap<String, Project>,

    /// Name of the currently active project
    current_project: String,
}

impl ProjectManager {
    /// Creates a new ProjectManager with a default project.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let manager = ProjectManager::new();
    /// assert_eq!(manager.get_current_project_name(), "default");
    /// ```
    pub fn new() -> Self {
        let default_project_name = "default".to_string();
        let mut projects = HashMap::new();
        projects.insert(
            default_project_name.clone(),
            Project::new(default_project_name.clone()),
        );

        Self {
            projects,
            current_project: default_project_name,
        }
    }

    /// Creates a new project with the given name.
    ///
    /// Returns `Some(())` if the project was created successfully,
    /// or `None` if a project with that name already exists.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project to create
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// assert!(manager.create_project("Work".to_string()).is_some());
    /// assert!(manager.create_project("Work".to_string()).is_none()); // Already exists
    /// ```
    pub fn create_project(&mut self, name: String) -> Option<()> {
        if self.projects.contains_key(&name) {
            None
        } else {
            self.projects.insert(name.clone(), Project::new(name));
            Some(())
        }
    }

    /// Deletes a project by name.
    ///
    /// Returns `Some(())` if the project was deleted successfully,
    /// or `None` if the project doesn't exist or is the current project.
    ///
    /// Cannot delete the current project - switch to another first.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project to delete
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// manager.create_project("Work".to_string());
    /// assert!(manager.delete_project("Work".to_string()).is_some());
    /// assert!(manager.delete_project("Work".to_string()).is_none()); // Already deleted
    /// ```
    pub fn delete_project(&mut self, name: String) -> Option<()> {
        // Can't delete current project
        if name == self.current_project {
            return None;
        }

        // Can't delete if doesn't exist
        if !self.projects.contains_key(&name) {
            return None;
        }

        self.projects.remove(&name);
        Some(())
    }

    /// Switches to a different project.
    ///
    /// Returns `Some(())` if the switch was successful,
    /// or `None` if the project doesn't exist.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the project to switch to
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// manager.create_project("Work".to_string());
    /// assert!(manager.switch_project("Work".to_string()).is_some());
    /// assert_eq!(manager.get_current_project_name(), "Work");
    /// ```
    pub fn switch_project(&mut self, name: String) -> Option<()> {
        if self.projects.contains_key(&name) {
            self.current_project = name;
            Some(())
        } else {
            None
        }
    }

    /// Renames a project.
    ///
    /// Returns `Some(())` if the rename was successful,
    /// or `None` if the old name doesn't exist or the new name already exists.
    ///
    /// # Arguments
    ///
    /// * `old_name` - The current name of the project
    /// * `new_name` - The new name for the project
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// assert!(manager.rename_project("default".to_string(), "personal".to_string()).is_some());
    /// assert_eq!(manager.get_current_project_name(), "personal");
    /// ```
    pub fn rename_project(&mut self, old_name: String, new_name: String) -> Option<()> {
        // Can't rename to a name that already exists
        if self.projects.contains_key(&new_name) {
            return None;
        }

        // Can't rename if old name doesn't exist
        if !self.projects.contains_key(&old_name) {
            return None;
        }

        // Remove the old entry and create a new one with updated name
        if let Some(mut project) = self.projects.remove(&old_name) {
            project.name = new_name.clone();
            self.projects.insert(new_name.clone(), project);

            // Update current project if it was the renamed one
            if self.current_project == old_name {
                self.current_project = new_name;
            }

            Some(())
        } else {
            None
        }
    }

    /// Returns the name of the current project.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let manager = ProjectManager::new();
    /// assert_eq!(manager.get_current_project_name(), "default");
    /// ```
    pub fn get_current_project_name(&self) -> &str {
        &self.current_project
    }

    /// Returns a reference to the current project's TodoList.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let manager = ProjectManager::new();
    /// let todo_list = manager.get_current_todo_list();
    /// assert_eq!(todo_list.get_tasks().len(), 0);
    /// ```
    pub fn get_current_todo_list(&self) -> &TodoList {
        &self.projects.get(&self.current_project).unwrap().todo_list
    }

    /// Returns a mutable reference to the current project's TodoList.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    /// use todo_manager::models::task::TaskWithoutId;
    ///
    /// let mut manager = ProjectManager::new();
    /// let task = TaskWithoutId::new("Task 1".to_string());
    /// manager.get_current_todo_list_mut().add_task(task);
    /// assert_eq!(manager.get_current_todo_list().get_tasks().len(), 1);
    /// ```
    pub fn get_current_todo_list_mut(&mut self) -> &mut TodoList {
        &mut self
            .projects
            .get_mut(&self.current_project)
            .unwrap()
            .todo_list
    }

    /// Returns a list of all project names.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// manager.create_project("Work".to_string());
    /// manager.create_project("Personal".to_string());
    ///
    /// let projects = manager.list_projects();
    /// assert_eq!(projects.len(), 3);
    /// assert!(projects.contains(&"default".to_string()));
    /// assert!(projects.contains(&"Work".to_string()));
    /// assert!(projects.contains(&"Personal".to_string()));
    /// ```
    pub fn list_projects(&self) -> Vec<String> {
        let mut names: Vec<String> = self.projects.keys().cloned().collect();
        names.sort();
        names
    }

    /// Returns the number of projects.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::project_command::ProjectManager;
    ///
    /// let mut manager = ProjectManager::new();
    /// assert_eq!(manager.project_count(), 1); // default project
    ///
    /// manager.create_project("Work".to_string());
    /// assert_eq!(manager.project_count(), 2);
    /// ```
    pub fn project_count(&self) -> usize {
        self.projects.len()
    }
}

impl Default for ProjectManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;

    #[test]
    fn test_new_project_manager() {
        let manager = ProjectManager::new();
        assert_eq!(manager.get_current_project_name(), "default");
        assert_eq!(manager.project_count(), 1);
    }

    #[test]
    fn test_create_project() {
        let mut manager = ProjectManager::new();
        assert!(manager.create_project("Work".to_string()).is_some());
        assert_eq!(manager.project_count(), 2);

        // Can't create duplicate
        assert!(manager.create_project("Work".to_string()).is_none());
    }

    #[test]
    fn test_delete_project() {
        let mut manager = ProjectManager::new();
        manager.create_project("Work".to_string());
        manager.create_project("Personal".to_string());

        // Can delete non-current project
        assert!(manager.delete_project("Work".to_string()).is_some());
        assert_eq!(manager.project_count(), 2);

        // Can't delete current project
        assert!(manager.delete_project("default".to_string()).is_none());

        // Can't delete non-existent project
        assert!(manager.delete_project("Work".to_string()).is_none());
    }

    #[test]
    fn test_switch_project() {
        let mut manager = ProjectManager::new();
        manager.create_project("Work".to_string());

        assert!(manager.switch_project("Work".to_string()).is_some());
        assert_eq!(manager.get_current_project_name(), "Work");

        // Can't switch to non-existent project
        assert!(manager.switch_project("NonExistent".to_string()).is_none());
    }

    #[test]
    fn test_rename_project() {
        let mut manager = ProjectManager::new();

        assert!(manager
            .rename_project("default".to_string(), "personal".to_string())
            .is_some());
        assert_eq!(manager.get_current_project_name(), "personal");
        assert_eq!(manager.project_count(), 1);

        // Can't rename to existing name
        manager.create_project("Work".to_string());
        assert!(manager
            .rename_project("personal".to_string(), "Work".to_string())
            .is_none());

        // Can't rename non-existent project
        assert!(manager
            .rename_project("NonExistent".to_string(), "New".to_string())
            .is_none());
    }

    #[test]
    fn test_get_current_todo_list() {
        let manager = ProjectManager::new();
        let todo_list = manager.get_current_todo_list();
        assert_eq!(todo_list.get_tasks().len(), 0);
    }

    #[test]
    fn test_get_current_todo_list_mut() {
        let mut manager = ProjectManager::new();
        let task = TaskWithoutId::new("Task 1".to_string());
        manager.get_current_todo_list_mut().add_task(task);
        assert_eq!(manager.get_current_todo_list().get_tasks().len(), 1);
    }

    #[test]
    fn test_list_projects() {
        let mut manager = ProjectManager::new();
        manager.create_project("Work".to_string());
        manager.create_project("Personal".to_string());

        let projects = manager.list_projects();
        assert_eq!(projects.len(), 3);
        assert_eq!(projects, vec!["Personal", "Work", "default"]); // Sorted
    }

    #[test]
    fn test_projects_are_independent() {
        let mut manager = ProjectManager::new();
        manager.create_project("Work".to_string());

        // Add task to default project
        let task1 = TaskWithoutId::new("Default Task".to_string());
        manager.get_current_todo_list_mut().add_task(task1);

        // Switch to Work and add task
        manager.switch_project("Work".to_string());
        let task2 = TaskWithoutId::new("Work Task".to_string());
        manager.get_current_todo_list_mut().add_task(task2);

        // Work should have 1 task
        assert_eq!(manager.get_current_todo_list().get_tasks().len(), 1);

        // Switch back to default, should still have 1 task
        manager.switch_project("default".to_string());
        assert_eq!(manager.get_current_todo_list().get_tasks().len(), 1);
    }
}
