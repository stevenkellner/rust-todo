//! Integration tests for the Todo List application
//! 
//! These tests verify that all components work together correctly
//! and test the complete user workflows.

use todo_manager::models::task::{TaskWithoutId, Task};
use todo_manager::models::todo_list::TodoList;

/// Test the complete workflow: add, list, complete, and remove tasks
#[test]
fn test_complete_workflow() {
    let mut todo_list = TodoList::new();
    
    // Test adding tasks
    let task1_id = todo_list.add_task(TaskWithoutId::new("Buy groceries".to_string()));
    let task2_id = todo_list.add_task(TaskWithoutId::new("Walk the dog".to_string()));
    let task3_id = todo_list.add_task(TaskWithoutId::new("Finish homework".to_string()));
    
    assert_eq!(task1_id, 1);
    assert_eq!(task2_id, 2);
    assert_eq!(task3_id, 3);
    assert_eq!(todo_list.get_tasks().len(), 3);
    
    // Test listing all tasks
    let all_tasks = todo_list.get_tasks();
    assert_eq!(all_tasks.len(), 3);
    assert_eq!(all_tasks[0].description, "Buy groceries");
    assert_eq!(all_tasks[1].description, "Walk the dog");
    assert_eq!(all_tasks[2].description, "Finish homework");
    
    // Test completing a task
    let completed_task = todo_list.toggle_task(task2_id);
    assert!(completed_task.is_some());
    assert!(completed_task.unwrap().is_completed());
    
    // Test filtering completed and pending tasks
    let completed_tasks = todo_list.get_completed_tasks();
    let pending_tasks = todo_list.get_pending_tasks();
    
    assert_eq!(completed_tasks.len(), 1);
    assert_eq!(pending_tasks.len(), 2);
    assert_eq!(completed_tasks[0].description, "Walk the dog");
    
    // Test removing a task
    let removed_task = todo_list.remove_task(task1_id);
    assert!(removed_task.is_some());
    assert_eq!(removed_task.unwrap().description, "Buy groceries");
    assert_eq!(todo_list.get_tasks().len(), 2);
    
    // Test that the remaining tasks are correct
    let remaining_tasks = todo_list.get_tasks();
    assert_eq!(remaining_tasks.len(), 2);
    assert_eq!(remaining_tasks[0].description, "Walk the dog");
    assert_eq!(remaining_tasks[1].description, "Finish homework");
}

/// Test that task IDs remain sequential even after removing tasks
#[test]
fn test_task_id_persistence_after_removal() {
    let mut todo_list = TodoList::new();
    
    // Add multiple tasks
    let _task1_id = todo_list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = todo_list.add_task(TaskWithoutId::new("Task 2".to_string()));
    let _task3_id = todo_list.add_task(TaskWithoutId::new("Task 3".to_string()));
    
    // Remove middle task
    todo_list.remove_task(task2_id);
    
    // Add new task - should get next sequential ID
    let task4_id = todo_list.add_task(TaskWithoutId::new("Task 4".to_string()));
    assert_eq!(task4_id, 4);
    
    // Verify tasks have correct IDs
    let tasks = todo_list.get_tasks();
    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 3);
    assert_eq!(tasks[2].id, 4);
}

/// Test operations on an empty todo list
#[test]
fn test_empty_todo_list_operations() {
    let mut todo_list = TodoList::new();
    
    // Test operations on empty list
    assert!(todo_list.is_empty());
    assert_eq!(todo_list.get_completed_tasks().len(), 0);
    assert_eq!(todo_list.get_pending_tasks().len(), 0);
    
    // Test removing from empty list
    let result = todo_list.remove_task(1);
    assert!(result.is_none());
    
    // Test toggling non-existent task
    let result = todo_list.toggle_task(1);
    assert!(result.is_none());
}

/// Test task completion edge cases
#[test]
fn test_task_completion_edge_cases() {
    let mut todo_list = TodoList::new();
    let task_id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
    
    // Complete the task
    let task = todo_list.toggle_task(task_id).unwrap();
    assert!(task.is_completed());
    
    // Try to complete again (should uncomplete)
    let task = todo_list.toggle_task(task_id).unwrap();
    assert!(!task.is_completed());
    
    // Verify the task is in pending list
    let pending_tasks = todo_list.get_pending_tasks();
    assert_eq!(pending_tasks.len(), 1);
    assert_eq!(pending_tasks[0].id, task_id);
}

/// Test handling a large number of tasks
#[test]
fn test_large_number_of_tasks() {
    let mut todo_list = TodoList::new();
    let num_tasks = 100;
    
    // Add many tasks
    for i in 1..=num_tasks {
        let task_id = todo_list.add_task(TaskWithoutId::new(format!("Task {}", i)));
        assert_eq!(task_id, i);
    }
    
    assert_eq!(todo_list.get_tasks().len(), num_tasks);
    
    // Complete every other task
    for i in (2..=num_tasks).step_by(2) {
        todo_list.toggle_task(i);
    }
    
    let completed_tasks = todo_list.get_completed_tasks();
    let pending_tasks = todo_list.get_pending_tasks();
    
    assert_eq!(completed_tasks.len(), num_tasks / 2);
    assert_eq!(pending_tasks.len(), num_tasks / 2);
    
    // Remove all completed tasks
    let completed_ids: Vec<usize> = completed_tasks.iter().map(|task| task.id).collect();
    for id in completed_ids {
        todo_list.remove_task(id);
    }
    
    assert_eq!(todo_list.get_tasks().len(), num_tasks / 2);
    assert_eq!(todo_list.get_completed_tasks().len(), 0);
    assert_eq!(todo_list.get_pending_tasks().len(), num_tasks / 2);
}

/// Test concurrent completion and removal operations
#[test]
fn test_concurrent_operations() {
    let mut todo_list = TodoList::new();
    
    // Add several tasks
    let task_ids: Vec<usize> = (1..=5)
        .map(|i| todo_list.add_task(TaskWithoutId::new(format!("Task {}", i))))
        .collect();
    
    // Complete some tasks
    todo_list.toggle_task(task_ids[1]);
    todo_list.toggle_task(task_ids[3]);
    
    // Remove a completed task
    let removed = todo_list.remove_task(task_ids[1]);
    assert!(removed.is_some());
    assert!(removed.unwrap().is_completed());
    
    // Remove a pending task
    let removed = todo_list.remove_task(task_ids[0]);
    assert!(removed.is_some());
    assert!(!removed.unwrap().is_completed());
    
    // Verify final state
    assert_eq!(todo_list.get_tasks().len(), 3);
    assert_eq!(todo_list.get_completed_tasks().len(), 1);
    assert_eq!(todo_list.get_pending_tasks().len(), 2);
}

/// Test edge case with empty task descriptions
#[test]
fn test_empty_task_description() {
    let mut todo_list = TodoList::new();
    
    // Add task with empty description (should still work at the data layer)
    let task_id = todo_list.add_task(TaskWithoutId::new("".to_string()));
    assert_eq!(task_id, 1);
    
    let task = &todo_list.get_tasks()[0];
    assert_eq!(task.description, "");
    assert!(!task.is_completed());
}

/// Test task status symbols
#[test]
fn test_task_status_symbols() {
    let mut task = Task::new(1, "Test task".to_string());
    
    // Test pending symbol
    assert_eq!(task.get_status_symbol(), " ");
    
    // Test completed symbol
    task.toggle_completion();
    assert_eq!(task.get_status_symbol(), "✓");
}

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
