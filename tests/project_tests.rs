//! Project management integration tests
//! 
//! Tests for project operations: create, switch, rename, delete, and isolation

use todo_manager::models::task::TaskWithoutId;
use todo_manager::controller::project_command::ProjectManager;

/// Test complete project workflow: create multiple projects, switch between them,
/// add tasks to each, verify isolation
#[test]
fn test_complete_project_workflow() {
    let mut project_manager = ProjectManager::new();
    
    // Verify we start with the default project
    assert_eq!(project_manager.project_count(), 1);
    assert_eq!(project_manager.get_current_project_name(), "default");
    
    // Add tasks to default project
    let _default_task1 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Default task 1".to_string()));
    let _default_task2 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Default task 2".to_string()));
    
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 2);
    
    // Create Work project
    assert!(project_manager.create_project("Work".to_string()).is_some());
    assert_eq!(project_manager.project_count(), 2);
    
    // Switch to Work project
    assert!(project_manager.switch_project("Work".to_string()).is_some());
    assert_eq!(project_manager.get_current_project_name(), "Work");
    
    // Work project should be empty
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 0);
    
    // Add tasks to Work project
    let _work_task1 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Work task 1".to_string()));
    let work_task2 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Work task 2".to_string()));
    let _work_task3 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Work task 3".to_string()));
    
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 3);
    assert_eq!(project_manager.get_current_todo_list().get_tasks()[0].description, "Work task 1");
    
    // Create Personal project
    assert!(project_manager.create_project("Personal".to_string()).is_some());
    assert_eq!(project_manager.project_count(), 3);
    
    // Switch to Personal project
    assert!(project_manager.switch_project("Personal".to_string()).is_some());
    assert_eq!(project_manager.get_current_project_name(), "Personal");
    
    // Personal project should be empty
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 0);
    
    // Add tasks to Personal project
    let personal_task1 = project_manager.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Personal task 1".to_string()));
    project_manager.get_current_todo_list_mut()
        .toggle_task(personal_task1);
    
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 1);
    assert!(project_manager.get_current_todo_list().get_tasks()[0].is_completed());
    
    // Switch back to default and verify tasks are still there
    assert!(project_manager.switch_project("default".to_string()).is_some());
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 2);
    assert_eq!(project_manager.get_current_todo_list().get_tasks()[0].description, "Default task 1");
    assert_eq!(project_manager.get_current_todo_list().get_tasks()[1].description, "Default task 2");
    
    // Switch back to Work and verify tasks are still there
    assert!(project_manager.switch_project("Work".to_string()).is_some());
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 3);
    assert_eq!(project_manager.get_current_todo_list().get_tasks()[0].description, "Work task 1");
    assert_eq!(project_manager.get_current_todo_list().get_tasks()[2].description, "Work task 3");
    
    // Verify all Work tasks are pending (not completed)
    for task in project_manager.get_current_todo_list().get_tasks() {
        assert!(!task.is_completed());
    }
    
    // Complete a Work task
    project_manager.get_current_todo_list_mut().toggle_task(work_task2);
    assert_eq!(project_manager.get_current_todo_list().get_completed_tasks().len(), 1);
    assert_eq!(project_manager.get_current_todo_list().get_pending_tasks().len(), 2);
    
    // We're currently on Work project, delete Personal project
    assert_eq!(project_manager.get_current_project_name(), "Work");
    assert!(project_manager.delete_project("Personal".to_string()).is_some());
    assert_eq!(project_manager.project_count(), 2);
    
    // Should still be on Work after deletion (we didn't delete current project)
    assert_eq!(project_manager.get_current_project_name(), "Work");
    
    // Rename Work project
    assert!(project_manager.rename_project("Work".to_string(), "Office".to_string()).is_some());
    
    // Switch to renamed project
    assert!(project_manager.switch_project("Office".to_string()).is_some());
    assert_eq!(project_manager.get_current_todo_list().get_tasks().len(), 3);
    
    // Verify we can't delete current project
    assert!(project_manager.delete_project("Office".to_string()).is_none());
    
    // List all projects
    let projects = project_manager.list_projects();
    assert_eq!(projects.len(), 2);
    assert!(projects.contains(&"default".to_string()));
    assert!(projects.contains(&"Office".to_string()));
}

/// Test project isolation: tasks don't leak between projects
#[test]
fn test_project_task_isolation() {
    let mut pm = ProjectManager::new();
    
    // Add tasks to default
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("Default 1".to_string()));
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("Default 2".to_string()));
    
    // Create and switch to Project A
    pm.create_project("ProjectA".to_string());
    pm.switch_project("ProjectA".to_string());
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("A1".to_string()));
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("A2".to_string()));
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("A3".to_string()));
    
    // Create and switch to Project B
    pm.create_project("ProjectB".to_string());
    pm.switch_project("ProjectB".to_string());
    pm.get_current_todo_list_mut().add_task(TaskWithoutId::new("B1".to_string()));
    
    // Verify isolation
    assert_eq!(pm.get_current_todo_list().get_tasks().len(), 1);
    
    pm.switch_project("ProjectA".to_string());
    assert_eq!(pm.get_current_todo_list().get_tasks().len(), 3);
    
    pm.switch_project("default".to_string());
    assert_eq!(pm.get_current_todo_list().get_tasks().len(), 2);
    
    // Delete ProjectA
    pm.delete_project("ProjectA".to_string());
    
    // Verify ProjectB and default are unaffected
    pm.switch_project("ProjectB".to_string());
    assert_eq!(pm.get_current_todo_list().get_tasks().len(), 1);
    
    pm.switch_project("default".to_string());
    assert_eq!(pm.get_current_todo_list().get_tasks().len(), 2);
}

/// Test edge case: rapid project switching
#[test]
fn test_rapid_project_switching() {
    let mut pm = ProjectManager::new();
    
    // Create multiple projects
    for i in 1..=5 {
        pm.create_project(format!("Project{}", i));
    }
    
    // Add unique tasks to each project
    for i in 0..=5 {
        let project_name = if i == 0 {
            "default".to_string()
        } else {
            format!("Project{}", i)
        };
        
        pm.switch_project(project_name.clone());
        pm.get_current_todo_list_mut()
            .add_task(TaskWithoutId::new(format!("{} task", project_name)));
    }
    
    // Rapidly switch and verify each project has correct task
    for i in 0..=5 {
        let project_name = if i == 0 {
            "default".to_string()
        } else {
            format!("Project{}", i)
        };
        
        pm.switch_project(project_name.clone());
        let tasks = pm.get_current_todo_list().get_tasks();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, format!("{} task", project_name));
    }
}

/// Test combined operations: projects with subtasks
#[test]
fn test_projects_with_subtasks() {
    let mut pm = ProjectManager::new();
    
    // Add parent with subtasks to default
    let default_parent = pm.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Default parent".to_string()));
    pm.get_current_todo_list_mut().add_subtask(default_parent, "Default sub1".to_string());
    pm.get_current_todo_list_mut().add_subtask(default_parent, "Default sub2".to_string());
    
    // Create Work project with parent and subtasks
    pm.create_project("Work".to_string());
    pm.switch_project("Work".to_string());
    
    let work_parent = pm.get_current_todo_list_mut()
        .add_task(TaskWithoutId::new("Work parent".to_string()));
    let work_sub1 = pm.get_current_todo_list_mut()
        .add_subtask(work_parent, "Work sub1".to_string()).unwrap();
    let _work_sub2 = pm.get_current_todo_list_mut()
        .add_subtask(work_parent, "Work sub2".to_string()).unwrap();
    let work_sub3 = pm.get_current_todo_list_mut()
        .add_subtask(work_parent, "Work sub3".to_string()).unwrap();
    
    // Complete some subtasks
    pm.get_current_todo_list_mut().toggle_task(work_sub1);
    pm.get_current_todo_list_mut().toggle_task(work_sub3);
    
    assert_eq!(pm.get_current_todo_list().get_subtask_count(work_parent), 3);
    assert_eq!(pm.get_current_todo_list().get_completed_subtask_count(work_parent), 2);
    
    // Switch back to default and verify its subtasks are intact
    pm.switch_project("default".to_string());
    assert_eq!(pm.get_current_todo_list().get_subtask_count(default_parent), 2);
    assert_eq!(pm.get_current_todo_list().get_completed_subtask_count(default_parent), 0);
    
    // Switch to Work and verify again
    pm.switch_project("Work".to_string());
    assert_eq!(pm.get_current_todo_list().get_subtask_count(work_parent), 3);
    assert_eq!(pm.get_current_todo_list().get_completed_subtask_count(work_parent), 2);
}

/// Test edge case: attempting invalid operations
#[test]
fn test_invalid_operations() {
    let mut pm = ProjectManager::new();
    
    // Try to create duplicate project
    pm.create_project("Test".to_string());
    assert!(pm.create_project("Test".to_string()).is_none());
    
    // Try to switch to non-existent project
    assert!(pm.switch_project("NonExistent".to_string()).is_none());
    
    // Try to delete non-existent project
    assert!(pm.delete_project("NonExistent".to_string()).is_none());
    
    // Try to delete current project
    assert!(pm.delete_project("default".to_string()).is_none());
    
    // Try to rename to existing name
    pm.create_project("Another".to_string());
    assert!(pm.rename_project("Test".to_string(), "Another".to_string()).is_none());
    
    // Try to rename non-existent project
    assert!(pm.rename_project("DoesNotExist".to_string(), "NewName".to_string()).is_none());
}

/// Test combined operations with projects: create, populate, rename, switch
#[test]
fn test_complex_project_operations() {
    let mut pm = ProjectManager::new();
    
    // Create projects A, B, C
    pm.create_project("A".to_string());
    pm.create_project("B".to_string());
    pm.create_project("C".to_string());
    
    // Add tasks to each
    for project in &["default", "A", "B", "C"] {
        pm.switch_project(project.to_string());
        for i in 1..=3 {
            pm.get_current_todo_list_mut()
                .add_task(TaskWithoutId::new(format!("{}-{}", project, i)));
        }
    }
    
    // Rename B to Beta
    pm.rename_project("B".to_string(), "Beta".to_string());
    
    // Delete A
    pm.delete_project("A".to_string());
    
    // Verify remaining projects
    let projects = pm.list_projects();
    assert_eq!(projects.len(), 3);
    assert!(projects.contains(&"default".to_string()));
    assert!(projects.contains(&"Beta".to_string()));
    assert!(projects.contains(&"C".to_string()));
    assert!(!projects.contains(&"A".to_string()));
    assert!(!projects.contains(&"B".to_string()));
    
    // Verify Beta has its tasks
    pm.switch_project("Beta".to_string());
    let tasks = pm.get_current_todo_list().get_tasks();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].description, "B-1");
    assert_eq!(tasks[1].description, "B-2");
    assert_eq!(tasks[2].description, "B-3");
}

/// Test edge case: empty project name handling
#[test]
fn test_empty_project_name() {
    let mut pm = ProjectManager::new();
    
    // Try to create project with empty name
    let result = pm.create_project("".to_string());
    
    // Should succeed (data layer allows it, validation would be at UI layer)
    assert!(result.is_some());
    
    // Can switch to it
    assert!(pm.switch_project("".to_string()).is_some());
    assert_eq!(pm.get_current_project_name(), "");
}
