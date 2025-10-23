//! Integration tests for the Todo List application
//! 
//! These tests verify that all components work together correctly
//! and test the complete user workflows.

use todo_manager::task::Task;
use todo_manager::todo_list::TodoList;

/// Test the complete workflow: add, list, complete, and remove tasks
#[test]
fn test_complete_workflow() {
    let mut todo_list = TodoList::new();
    
    // Test adding tasks
    let task1_id = todo_list.add_task("Buy groceries".to_string());
    let task2_id = todo_list.add_task("Walk the dog".to_string());
    let task3_id = todo_list.add_task("Finish homework".to_string());
    
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
    let _task1_id = todo_list.add_task("Task 1".to_string());
    let task2_id = todo_list.add_task("Task 2".to_string());
    let _task3_id = todo_list.add_task("Task 3".to_string());
    
    // Remove middle task
    todo_list.remove_task(task2_id);
    
    // Add new task - should get next sequential ID
    let task4_id = todo_list.add_task("Task 4".to_string());
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
    let task_id = todo_list.add_task("Test task".to_string());
    
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
        let task_id = todo_list.add_task(format!("Task {}", i));
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
        .map(|i| todo_list.add_task(format!("Task {}", i)))
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
    let task_id = todo_list.add_task("".to_string());
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