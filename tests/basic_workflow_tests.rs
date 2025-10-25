//! Basic workflow integration tests
//! 
//! Tests for fundamental task operations: add, list, complete, remove

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

/// Test combined operations: add multiple tasks, complete some, remove others
#[test]
fn test_combined_operations_workflow() {
    let mut todo_list = TodoList::new();
    
    // Add multiple tasks
    let ids: Vec<usize> = (1..=10)
        .map(|i| todo_list.add_task(TaskWithoutId::new(format!("Task {}", i))))
        .collect();
    
    assert_eq!(todo_list.get_tasks().len(), 10);
    
    // Complete tasks 2, 4, 6, 8
    for &id in &ids[1..9].iter().step_by(2).collect::<Vec<_>>() {
        todo_list.toggle_task(*id);
    }
    
    assert_eq!(todo_list.get_completed_tasks().len(), 4);
    assert_eq!(todo_list.get_pending_tasks().len(), 6);
    
    // Remove tasks 1, 3, 5
    for &id in &[ids[0], ids[2], ids[4]] {
        let removed = todo_list.remove_task(id);
        assert!(removed.is_some());
    }
    
    assert_eq!(todo_list.get_tasks().len(), 7);
    assert_eq!(todo_list.get_completed_tasks().len(), 4);
    assert_eq!(todo_list.get_pending_tasks().len(), 3);
}

/// Test combined operations: multiple toggles and removals
#[test]
fn test_multiple_toggle_and_remove() {
    let mut todo_list = TodoList::new();
    
    let id1 = todo_list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let id2 = todo_list.add_task(TaskWithoutId::new("Task 2".to_string()));
    let _id3 = todo_list.add_task(TaskWithoutId::new("Task 3".to_string()));
    
    // Toggle multiple times
    todo_list.toggle_task(id1);
    todo_list.toggle_task(id1);
    todo_list.toggle_task(id2);
    
    assert_eq!(todo_list.get_completed_tasks().len(), 1);
    assert!(!todo_list.get_tasks().iter().find(|t| t.id == id1).unwrap().is_completed());
    assert!(todo_list.get_tasks().iter().find(|t| t.id == id2).unwrap().is_completed());
    
    // Remove a task
    todo_list.remove_task(id2);
    
    assert_eq!(todo_list.get_tasks().len(), 2);
    assert_eq!(todo_list.get_completed_tasks().len(), 0);
}
