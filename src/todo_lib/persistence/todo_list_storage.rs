use crate::controller::project_command::ProjectManager;
use crate::models::TodoList;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Handles persistence operations for TodoList.
///
/// This struct encapsulates all file I/O operations for saving and loading
/// todo lists, keeping storage concerns separate from business logic.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::persistence::TodoListStorage;
/// use todo_manager::models::TodoList;
///
/// let storage = TodoListStorage::new("tasks.json");
/// let todo_list = TodoList::new();
///
/// // Save to file
/// storage.save(&todo_list).unwrap();
///
/// // Load from file
/// let loaded_list = storage.load().unwrap();
/// ```
pub struct TodoListStorage {
    storage_path: PathBuf,
}

impl TodoListStorage {
    /// Creates a new TodoListStorage instance with the specified file path.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path where the TodoList will be saved/loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::persistence::TodoListStorage;
    ///
    /// let storage = TodoListStorage::new("tasks.json");
    /// ```
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        TodoListStorage {
            storage_path: path.as_ref().to_path_buf(),
        }
    }

    /// Saves a TodoList to a JSON file at the configured storage path.
    ///
    /// Creates parent directories if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The TodoList to save
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(String)` with an error message on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::persistence::TodoListStorage;
    /// use todo_manager::models::TodoList;
    ///
    /// let storage = TodoListStorage::new("tasks.json");
    /// let todo_list = TodoList::new();
    /// storage.save(&todo_list).unwrap();
    /// ```
    pub fn save(&self, todo_list: &TodoList) -> Result<(), String> {
        let path = &self.storage_path;

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
        }

        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(&todo_list)
            .map_err(|e| format!("Failed to serialize TodoList: {}", e))?;

        // Write to file
        let mut file =
            fs::File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }

    /// Loads a TodoList from a JSON file at the configured storage path.
    ///
    /// # Returns
    ///
    /// `Ok(TodoList)` on success, or `Err(String)` with an error message on failure.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::persistence::TodoListStorage;
    ///
    /// let storage = TodoListStorage::new("tasks.json");
    /// let todo_list = storage.load().unwrap();
    /// ```
    pub fn load(&self) -> Result<TodoList, String> {
        let path = &self.storage_path;

        // Read file contents
        let contents =
            fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

        // Deserialize from JSON
        let todo_list: TodoList =
            serde_json::from_str(&contents).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        Ok(todo_list)
    }

    /// Saves a ProjectManager to a JSON file at the configured storage path.
    ///
    /// Creates parent directories if they don't exist.
    ///
    /// # Arguments
    ///
    /// * `project_manager` - The ProjectManager to save
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or `Err(String)` with an error message on failure.
    pub fn save_projects(&self, project_manager: &ProjectManager) -> Result<(), String> {
        let path = &self.storage_path;

        // Create parent directories if they don't exist
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("Failed to create parent directory: {}", e))?;
            }
        }

        // Serialize to pretty JSON
        let json = serde_json::to_string_pretty(&project_manager)
            .map_err(|e| format!("Failed to serialize ProjectManager: {}", e))?;

        // Write to file
        let mut file =
            fs::File::create(path).map_err(|e| format!("Failed to create file: {}", e))?;

        file.write_all(json.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        Ok(())
    }

    /// Loads a ProjectManager from a JSON file at the configured storage path.
    ///
    /// For backward compatibility, if the file contains an old TodoList format,
    /// it will be loaded into the default project of a new ProjectManager.
    ///
    /// # Returns
    ///
    /// `Ok(ProjectManager)` on success, or `Err(String)` with an error message on failure.
    pub fn load_projects(&self) -> Result<ProjectManager, String> {
        let path = &self.storage_path;

        // Read file contents
        let contents =
            fs::read_to_string(path).map_err(|e| format!("Failed to read file: {}", e))?;

        // Try to deserialize as ProjectManager first
        if let Ok(project_manager) = serde_json::from_str::<ProjectManager>(&contents) {
            return Ok(project_manager);
        }

        // Fall back to old TodoList format for backward compatibility
        if let Ok(todo_list) = serde_json::from_str::<TodoList>(&contents) {
            let mut project_manager = ProjectManager::new();
            // Replace the default project's todo list with the loaded one
            *project_manager.get_current_todo_list_mut() = todo_list;
            return Ok(project_manager);
        }

        Err("Failed to parse file as either ProjectManager or TodoList".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;

    #[test]
    fn test_new_storage() {
        let storage = TodoListStorage::new("test.json");
        assert!(std::mem::size_of_val(&storage) > 0);
    }

    #[test]
    fn test_save_and_load_empty_list() {
        let temp_path = "test_empty_list.json";
        let storage = TodoListStorage::new(temp_path);
        let todo_list = TodoList::new();

        // Save
        storage.save(&todo_list).unwrap();

        // Load
        let loaded_list = storage.load().unwrap();
        assert_eq!(loaded_list.get_tasks().len(), 0);

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_save_and_load_with_tasks() {
        let temp_path = "test_with_tasks.json";
        let storage = TodoListStorage::new(temp_path);
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Task 1".to_string()));
        todo_list.add_task(TaskWithoutId::new("Task 2".to_string()));

        // Save
        storage.save(&todo_list).unwrap();

        // Load
        // Load
        let loaded_list = storage.load().unwrap();
        assert_eq!(loaded_list.get_tasks().len(), 2);
        assert_eq!(loaded_list.get_tasks()[0].description, "Task 1");
        assert_eq!(loaded_list.get_tasks()[1].description, "Task 2");

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let storage = TodoListStorage::new("nonexistent_file.json");
        let result = storage.load();
        assert!(result.is_err());
    }

    #[test]
    fn test_save_creates_parent_directories() {
        let temp_path = "test_dir/nested_dir/test.json";
        let storage = TodoListStorage::new(temp_path);
        let todo_list = TodoList::new();

        // Save (should create directories)
        storage.save(&todo_list).unwrap();

        // Verify file exists
        assert!(Path::new(temp_path).exists());

        // Cleanup
        let _ = fs::remove_dir_all("test_dir");
    }

    #[test]
    fn test_save_and_load_projects_empty() {
        let temp_path = "test_projects_empty.json";
        let storage = TodoListStorage::new(temp_path);
        let project_manager = ProjectManager::new();

        // Save
        storage.save_projects(&project_manager).unwrap();

        // Load
        let loaded_manager = storage.load_projects().unwrap();
        assert_eq!(loaded_manager.project_count(), 1); // Default project
        assert_eq!(loaded_manager.get_current_todo_list().get_tasks().len(), 0);

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_save_and_load_projects_with_tasks() {
        let temp_path = "test_projects_with_tasks.json";
        let storage = TodoListStorage::new(temp_path);
        let mut project_manager = ProjectManager::new();

        // Add tasks to default project
        project_manager
            .get_current_todo_list_mut()
            .add_task(TaskWithoutId::new("Task 1".to_string()));
        project_manager
            .get_current_todo_list_mut()
            .add_task(TaskWithoutId::new("Task 2".to_string()));

        // Save
        storage.save_projects(&project_manager).unwrap();

        // Load
        let loaded_manager = storage.load_projects().unwrap();
        assert_eq!(loaded_manager.get_current_todo_list().get_tasks().len(), 2);
        assert_eq!(
            loaded_manager.get_current_todo_list().get_tasks()[0].description,
            "Task 1"
        );
        assert_eq!(
            loaded_manager.get_current_todo_list().get_tasks()[1].description,
            "Task 2"
        );

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_save_and_load_multiple_projects() {
        let temp_path = "test_multiple_projects.json";
        let storage = TodoListStorage::new(temp_path);
        let mut project_manager = ProjectManager::new();

        // Create additional projects
        project_manager.create_project("Work".to_string()).unwrap();
        project_manager
            .create_project("Personal".to_string())
            .unwrap();

        // Add tasks to default project
        project_manager
            .get_current_todo_list_mut()
            .add_task(TaskWithoutId::new("Default task".to_string()));

        // Switch to Work and add tasks
        project_manager.switch_project("Work".to_string()).unwrap();
        project_manager
            .get_current_todo_list_mut()
            .add_task(TaskWithoutId::new("Work task".to_string()));

        // Switch to Personal and add tasks
        project_manager
            .switch_project("Personal".to_string())
            .unwrap();
        project_manager
            .get_current_todo_list_mut()
            .add_task(TaskWithoutId::new("Personal task".to_string()));

        // Save
        storage.save_projects(&project_manager).unwrap();

        // Load and verify
        let loaded_manager = storage.load_projects().unwrap();
        assert_eq!(loaded_manager.project_count(), 3);

        // Check Personal project (current)
        assert_eq!(loaded_manager.get_current_todo_list().get_tasks().len(), 1);
        assert_eq!(
            loaded_manager.get_current_todo_list().get_tasks()[0].description,
            "Personal task"
        );

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_backward_compatibility_load_old_todolist() {
        let temp_path = "test_backward_compat.json";
        let storage = TodoListStorage::new(temp_path);

        // Save old TodoList format
        let mut old_todo_list = TodoList::new();
        old_todo_list.add_task(TaskWithoutId::new("Old task 1".to_string()));
        old_todo_list.add_task(TaskWithoutId::new("Old task 2".to_string()));
        storage.save(&old_todo_list).unwrap();

        // Load as ProjectManager (should work with backward compatibility)
        let loaded_manager = storage.load_projects().unwrap();
        assert_eq!(loaded_manager.project_count(), 1); // Default project
        assert_eq!(loaded_manager.get_current_todo_list().get_tasks().len(), 2);
        assert_eq!(
            loaded_manager.get_current_todo_list().get_tasks()[0].description,
            "Old task 1"
        );
        assert_eq!(
            loaded_manager.get_current_todo_list().get_tasks()[1].description,
            "Old task 2"
        );

        // Cleanup
        let _ = fs::remove_file(temp_path);
    }
}
