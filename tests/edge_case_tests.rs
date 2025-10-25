//! Edge case and stress testing integration tests
//!
//! Tests for boundary conditions, large data sets, and unusual scenarios

use todo_manager::models::task::TaskWithoutId;
use todo_manager::models::todo_list::TodoList;

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

/// Test edge case: clearing all tasks and re-adding
#[test]
fn test_clear_and_repopulate() {
    let mut todo_list = TodoList::new();

    // Add initial tasks
    for i in 1..=5 {
        todo_list.add_task(TaskWithoutId::new(format!("Task {}", i)));
    }

    assert_eq!(todo_list.get_tasks().len(), 5);

    // Clear all
    todo_list.clear_all();
    assert_eq!(todo_list.get_tasks().len(), 0);
    assert!(todo_list.is_empty());

    // Add new tasks - IDs should continue from where they left off
    let id1 = todo_list.add_task(TaskWithoutId::new("New task 1".to_string()));
    let id2 = todo_list.add_task(TaskWithoutId::new("New task 2".to_string()));

    assert_eq!(id1, 6);
    assert_eq!(id2, 7);
    assert_eq!(todo_list.get_tasks().len(), 2);
}

/// Test stress scenario: many operations in sequence
#[test]
fn test_stress_many_operations() {
    let mut todo_list = TodoList::new();

    // Add 50 tasks
    let mut ids = Vec::new();
    for i in 1..=50 {
        let id = todo_list.add_task(TaskWithoutId::new(format!("Task {}", i)));
        ids.push(id);
    }

    // Add subtasks to first 10 tasks
    for &id in ids.iter().take(10) {
        for j in 1..=3 {
            todo_list.add_subtask(id, format!("Subtask {}.{}", id, j));
        }
    }

    // Complete every 5th task
    for (i, &id) in ids.iter().enumerate() {
        if (i + 1) % 5 == 0 {
            todo_list.toggle_task(id);
        }
    }

    // Remove every 7th task (including cascade deletion of subtasks)
    for (i, &id) in ids.iter().enumerate() {
        if (i + 1) % 7 == 0 {
            todo_list.remove_task(id);
        }
    }

    // Verify counts (50 tasks + 30 subtasks - removed tasks and their subtasks)
    let remaining = todo_list.get_tasks().len();
    assert!(remaining > 0);
    assert!(remaining < 80);
}
