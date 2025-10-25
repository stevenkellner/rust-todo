//! Subtask integration tests
//! 
//! Tests for subtask operations including parent-child relationships and cascade deletion

use todo_manager::models::task::TaskWithoutId;
use todo_manager::models::todo_list::TodoList;

/// Test subtask functionality
#[test]
fn test_subtask_workflow() {
    let mut todo_list = TodoList::new();
    
    // Add a parent task
    let parent_id = todo_list.add_task(TaskWithoutId::new("Complete project".to_string()));
    assert_eq!(parent_id, 1);
    
    // Add subtasks
    let subtask1_id = todo_list.add_subtask(parent_id, "Write code".to_string());
    assert_eq!(subtask1_id, Some(2));
    
    let subtask2_id = todo_list.add_subtask(parent_id, "Write tests".to_string());
    assert_eq!(subtask2_id, Some(3));
    
    let subtask3_id = todo_list.add_subtask(parent_id, "Write documentation".to_string());
    assert_eq!(subtask3_id, Some(4));
    
    // Test subtask count
    assert_eq!(todo_list.get_subtask_count(parent_id), 3);
    
    // Test completed subtask count (initially 0)
    assert_eq!(todo_list.get_completed_subtask_count(parent_id), 0);
    
    // Complete one subtask
    todo_list.toggle_task(subtask1_id.unwrap());
    assert_eq!(todo_list.get_completed_subtask_count(parent_id), 1);
    
    // Complete another subtask
    todo_list.toggle_task(subtask2_id.unwrap());
    assert_eq!(todo_list.get_completed_subtask_count(parent_id), 2);
    
    // Get all subtasks
    let subtasks = todo_list.get_subtasks(parent_id);
    assert_eq!(subtasks.len(), 3);
    assert_eq!(subtasks[0].description, "Write code");
    assert_eq!(subtasks[1].description, "Write tests");
    assert_eq!(subtasks[2].description, "Write documentation");
    
    // Verify subtasks have correct parent_id
    for subtask in &subtasks {
        assert_eq!(subtask.get_parent_id(), Some(parent_id));
        assert!(subtask.is_subtask());
    }
    
    // Verify parent task is not a subtask
    let parent_task = todo_list.get_tasks().iter().find(|t| t.id == parent_id).unwrap();
    assert!(!parent_task.is_subtask());
    assert_eq!(parent_task.get_parent_id(), None);
}

/// Test adding subtask to non-existent parent
#[test]
fn test_add_subtask_invalid_parent() {
    let mut todo_list = TodoList::new();
    
    // Try to add subtask to non-existent parent
    let result = todo_list.add_subtask(999, "Invalid subtask".to_string());
    assert_eq!(result, None);
}

/// Test subtask hierarchy with multiple parents
#[test]
fn test_multiple_parent_subtasks() {
    let mut todo_list = TodoList::new();
    
    // Add two parent tasks
    let parent1_id = todo_list.add_task(TaskWithoutId::new("Project A".to_string()));
    let parent2_id = todo_list.add_task(TaskWithoutId::new("Project B".to_string()));
    
    // Add subtasks to first parent
    todo_list.add_subtask(parent1_id, "Task A1".to_string());
    todo_list.add_subtask(parent1_id, "Task A2".to_string());
    
    // Add subtasks to second parent
    todo_list.add_subtask(parent2_id, "Task B1".to_string());
    todo_list.add_subtask(parent2_id, "Task B2".to_string());
    todo_list.add_subtask(parent2_id, "Task B3".to_string());
    
    // Verify counts
    assert_eq!(todo_list.get_subtask_count(parent1_id), 2);
    assert_eq!(todo_list.get_subtask_count(parent2_id), 3);
    
    // Verify subtask retrieval
    let parent1_subtasks = todo_list.get_subtasks(parent1_id);
    let parent2_subtasks = todo_list.get_subtasks(parent2_id);
    
    assert_eq!(parent1_subtasks.len(), 2);
    assert_eq!(parent2_subtasks.len(), 3);
    
    // Verify subtasks belong to correct parent
    for subtask in parent1_subtasks {
        assert_eq!(subtask.get_parent_id(), Some(parent1_id));
    }
    
    for subtask in parent2_subtasks {
        assert_eq!(subtask.get_parent_id(), Some(parent2_id));
    }
}

/// Test edge case: removing a parent task also removes its subtasks (cascade deletion)
#[test]
fn test_remove_parent_removes_subtasks() {
    let mut todo_list = TodoList::new();
    
    let parent_id = todo_list.add_task(TaskWithoutId::new("Parent task".to_string()));
    let _subtask1_id = todo_list.add_subtask(parent_id, "Subtask 1".to_string()).unwrap();
    let _subtask2_id = todo_list.add_subtask(parent_id, "Subtask 2".to_string()).unwrap();
    
    assert_eq!(todo_list.get_tasks().len(), 3);
    
    // Remove parent task
    let removed = todo_list.remove_task(parent_id);
    assert!(removed.is_some());
    
    // Subtasks should be removed as well (cascade deletion)
    assert_eq!(todo_list.get_tasks().len(), 0);
}

/// Test edge case: completing parent doesn't complete subtasks
#[test]
fn test_complete_parent_independent_of_subtasks() {
    let mut todo_list = TodoList::new();
    
    let parent_id = todo_list.add_task(TaskWithoutId::new("Parent task".to_string()));
    let subtask1_id = todo_list.add_subtask(parent_id, "Subtask 1".to_string()).unwrap();
    let subtask2_id = todo_list.add_subtask(parent_id, "Subtask 2".to_string()).unwrap();
    
    // Complete parent
    todo_list.toggle_task(parent_id);
    
    // Verify parent is complete but subtasks are not
    let parent = todo_list.get_tasks().iter().find(|t| t.id == parent_id).unwrap();
    assert!(parent.is_completed());
    
    let subtasks = todo_list.get_subtasks(parent_id);
    assert!(!subtasks.iter().any(|t| t.id == subtask1_id && t.is_completed()));
    assert!(!subtasks.iter().any(|t| t.id == subtask2_id && t.is_completed()));
    
    // Complete one subtask
    todo_list.toggle_task(subtask1_id);
    
    // Parent should still be complete
    let parent = todo_list.get_tasks().iter().find(|t| t.id == parent_id).unwrap();
    assert!(parent.is_completed());
}

/// Test edge case: many subtasks under one parent with cascade deletion
#[test]
fn test_many_subtasks_single_parent() {
    let mut todo_list = TodoList::new();
    
    let parent_id = todo_list.add_task(TaskWithoutId::new("Main project".to_string()));
    
    // Add 20 subtasks
    for i in 1..=20 {
        let subtask_id = todo_list.add_subtask(parent_id, format!("Subtask {}", i));
        assert!(subtask_id.is_some());
    }
    
    assert_eq!(todo_list.get_subtask_count(parent_id), 20);
    assert_eq!(todo_list.get_tasks().len(), 21); // 1 parent + 20 subtasks
    
    // Complete every third subtask
    let subtask_ids: Vec<usize> = todo_list.get_subtasks(parent_id)
        .iter()
        .map(|t| t.id)
        .collect();
    
    for (i, &subtask_id) in subtask_ids.iter().enumerate() {
        if (i + 1) % 3 == 0 {
            todo_list.toggle_task(subtask_id);
        }
    }
    
    assert_eq!(todo_list.get_completed_subtask_count(parent_id), 6);
    
    // Remove parent - should remove all subtasks too (cascade deletion)
    todo_list.remove_task(parent_id);
    assert_eq!(todo_list.get_tasks().len(), 0);
}

/// Test edge case: subtasks of different parents with cascade deletion
#[test]
fn test_subtask_removal_edge_cases() {
    let mut todo_list = TodoList::new();
    
    // Create parent1 with subtasks
    let parent1 = todo_list.add_task(TaskWithoutId::new("Parent 1".to_string()));
    let sub1_1 = todo_list.add_subtask(parent1, "Sub 1.1".to_string()).unwrap();
    let sub1_2 = todo_list.add_subtask(parent1, "Sub 1.2".to_string()).unwrap();
    
    // Create parent2 with subtasks
    let parent2 = todo_list.add_task(TaskWithoutId::new("Parent 2".to_string()));
    let _sub2_1 = todo_list.add_subtask(parent2, "Sub 2.1".to_string()).unwrap();
    
    assert_eq!(todo_list.get_tasks().len(), 5);
    
    // Remove a subtask from parent1
    todo_list.remove_task(sub1_1);
    
    assert_eq!(todo_list.get_tasks().len(), 4);
    assert_eq!(todo_list.get_subtask_count(parent1), 1);
    assert_eq!(todo_list.get_subtask_count(parent2), 1);
    
    // Remove parent2 - should remove all its subtasks too (cascade deletion)
    todo_list.remove_task(parent2);
    
    assert_eq!(todo_list.get_tasks().len(), 2); // parent1 and sub1_2
    
    // Verify parent1 still has its remaining subtask
    assert_eq!(todo_list.get_subtask_count(parent1), 1);
    let remaining_subtasks = todo_list.get_subtasks(parent1);
    assert_eq!(remaining_subtasks.len(), 1);
    assert_eq!(remaining_subtasks[0].id, sub1_2);
}
