//! Additional integration tests for combined filtering feature

use todo_manager::models::todo_list::TodoList;
use todo_manager::models::priority::Priority;
use todo_manager::models::task_filter::TaskFilter;
use todo_manager::models::task_status::TaskStatus;

/// Test combined status and priority filtering
#[test]
fn test_combined_filtering() {
    let mut todo_list = TodoList::new();
    
    // Create tasks with different priorities and statuses
    let id1 = todo_list.add_task("High priority urgent".to_string());
    let id2 = todo_list.add_task("High priority regular".to_string());
    let id3 = todo_list.add_task("Medium priority task".to_string());
    let id4 = todo_list.add_task("Low priority task".to_string());
    let id5 = todo_list.add_task("Another high priority".to_string());
    
    // Set priorities
    todo_list.set_task_priority(id1, Priority::High);
    todo_list.set_task_priority(id2, Priority::High);
    todo_list.set_task_priority(id3, Priority::Medium);
    todo_list.set_task_priority(id4, Priority::Low);
    todo_list.set_task_priority(id5, Priority::High);
    
    // Complete some tasks
    todo_list.complete_task(id1);
    todo_list.complete_task(id3);
    todo_list.complete_task(id4);
    
    // Test filtering by priority only
    let high_priority_filter = TaskFilter::by_priority(Priority::High);
    let high_priority_tasks = todo_list.get_filtered_tasks(&high_priority_filter);
    assert_eq!(high_priority_tasks.len(), 3, "Should have 3 high priority tasks");
    
    // Test filtering by status only
    let completed_filter = TaskFilter::completed();
    let completed_tasks = todo_list.get_filtered_tasks(&completed_filter);
    assert_eq!(completed_tasks.len(), 3, "Should have 3 completed tasks");
    
    let pending_filter = TaskFilter::pending();
    let pending_tasks = todo_list.get_filtered_tasks(&pending_filter);
    assert_eq!(pending_tasks.len(), 2, "Should have 2 pending tasks");
    
    // Test combined filtering: completed + high priority
    let completed_high = TaskFilter::completed_with_priority(Priority::High);
    let completed_high_tasks = todo_list.get_filtered_tasks(&completed_high);
    assert_eq!(completed_high_tasks.len(), 1, "Should have 1 completed high priority task");
    assert_eq!(completed_high_tasks[0].id, id1);
    
    // Test combined filtering: pending + high priority
    let pending_high = TaskFilter::pending_with_priority(Priority::High);
    let pending_high_tasks = todo_list.get_filtered_tasks(&pending_high);
    assert_eq!(pending_high_tasks.len(), 2, "Should have 2 pending high priority tasks");
    
    // Test combined filtering: completed + low priority
    let completed_low = TaskFilter::completed_with_priority(Priority::Low);
    let completed_low_tasks = todo_list.get_filtered_tasks(&completed_low);
    assert_eq!(completed_low_tasks.len(), 1, "Should have 1 completed low priority task");
    assert_eq!(completed_low_tasks[0].id, id4);
    
    // Test combined filtering: pending + medium priority
    let pending_medium = TaskFilter::pending_with_priority(Priority::Medium);
    let pending_medium_tasks = todo_list.get_filtered_tasks(&pending_medium);
    assert_eq!(pending_medium_tasks.len(), 0, "Should have 0 pending medium priority tasks");
}

/// Test filter builder methods
#[test]
fn test_filter_builder_methods() {
    let mut todo_list = TodoList::new();
    
    // Create test tasks
    let id1 = todo_list.add_task("Task 1".to_string());
    let id2 = todo_list.add_task("Task 2".to_string());
    let id3 = todo_list.add_task("Task 3".to_string());
    
    todo_list.set_task_priority(id1, Priority::High);
    todo_list.set_task_priority(id2, Priority::High);
    todo_list.set_task_priority(id3, Priority::Low);
    
    todo_list.complete_task(id1);
    
    // Test using builder pattern
    let filter = TaskFilter::all()
        .with_status(TaskStatus::Completed)
        .with_priority(Priority::High);
    
    let filtered_tasks = todo_list.get_filtered_tasks(&filter);
    assert_eq!(filtered_tasks.len(), 1);
    assert_eq!(filtered_tasks[0].id, id1);
    
    // Test partial builder (only status)
    let status_only = TaskFilter::all().with_status(TaskStatus::Pending);
    let pending_tasks = todo_list.get_filtered_tasks(&status_only);
    assert_eq!(pending_tasks.len(), 2);
    
    // Test partial builder (only priority)
    let priority_only = TaskFilter::all().with_priority(Priority::Low);
    let low_priority_tasks = todo_list.get_filtered_tasks(&priority_only);
    assert_eq!(low_priority_tasks.len(), 1);
    assert_eq!(low_priority_tasks[0].id, id3);
}

/// Test empty filter results
#[test]
fn test_empty_filter_results() {
    let mut todo_list = TodoList::new();
    
    // Create only high priority tasks
    let id1 = todo_list.add_task("High 1".to_string());
    let id2 = todo_list.add_task("High 2".to_string());
    
    todo_list.set_task_priority(id1, Priority::High);
    todo_list.set_task_priority(id2, Priority::High);
    
    // Try to filter for low priority tasks (should be empty)
    let low_filter = TaskFilter::by_priority(Priority::Low);
    let low_tasks = todo_list.get_filtered_tasks(&low_filter);
    assert_eq!(low_tasks.len(), 0, "Should have no low priority tasks");
    
    // Try to filter for completed tasks (should be empty)
    let completed_filter = TaskFilter::completed();
    let completed_tasks = todo_list.get_filtered_tasks(&completed_filter);
    assert_eq!(completed_tasks.len(), 0, "Should have no completed tasks");
    
    // Try combined filter that doesn't match anything
    let completed_low = TaskFilter::completed_with_priority(Priority::Low);
    let tasks = todo_list.get_filtered_tasks(&completed_low);
    assert_eq!(tasks.len(), 0, "Should have no completed low priority tasks");
}

/// Test filter with all tasks matching
#[test]
fn test_filter_all_matching() {
    let mut todo_list = TodoList::new();
    
    // Create all high priority pending tasks
    let id1 = todo_list.add_task("Task 1".to_string());
    let id2 = todo_list.add_task("Task 2".to_string());
    let id3 = todo_list.add_task("Task 3".to_string());
    
    todo_list.set_task_priority(id1, Priority::High);
    todo_list.set_task_priority(id2, Priority::High);
    todo_list.set_task_priority(id3, Priority::High);
    
    // Filter for pending high priority tasks (should match all)
    let filter = TaskFilter::pending_with_priority(Priority::High);
    let tasks = todo_list.get_filtered_tasks(&filter);
    assert_eq!(tasks.len(), 3, "All tasks should match the filter");
    
    // Verify the tasks are in order
    assert_eq!(tasks[0].id, id1);
    assert_eq!(tasks[1].id, id2);
    assert_eq!(tasks[2].id, id3);
}

/// Test priority filtering with mixed completion states
#[test]
fn test_priority_filter_mixed_states() {
    let mut todo_list = TodoList::new();
    
    // Create tasks with same priority but different completion states
    let id1 = todo_list.add_task("Medium task 1".to_string());
    let id2 = todo_list.add_task("Medium task 2".to_string());
    let id3 = todo_list.add_task("Medium task 3".to_string());
    let id4 = todo_list.add_task("Medium task 4".to_string());
    
    todo_list.set_task_priority(id1, Priority::Medium);
    todo_list.set_task_priority(id2, Priority::Medium);
    todo_list.set_task_priority(id3, Priority::Medium);
    todo_list.set_task_priority(id4, Priority::Medium);
    
    // Complete some
    todo_list.complete_task(id1);
    todo_list.complete_task(id2);
    
    // Filter by priority only (should get all)
    let medium_filter = TaskFilter::by_priority(Priority::Medium);
    let medium_tasks = todo_list.get_filtered_tasks(&medium_filter);
    assert_eq!(medium_tasks.len(), 4, "Should get all medium priority tasks");
    
    // Filter by priority + completed
    let completed_medium = TaskFilter::completed_with_priority(Priority::Medium);
    let completed_tasks = todo_list.get_filtered_tasks(&completed_medium);
    assert_eq!(completed_tasks.len(), 2, "Should get 2 completed medium tasks");
    
    // Filter by priority + pending
    let pending_medium = TaskFilter::pending_with_priority(Priority::Medium);
    let pending_tasks = todo_list.get_filtered_tasks(&pending_medium);
    assert_eq!(pending_tasks.len(), 2, "Should get 2 pending medium tasks");
}

/// Test edge case: changing task state affects filter results
#[test]
fn test_filter_dynamic_updates() {
    let mut todo_list = TodoList::new();
    
    let id1 = todo_list.add_task("Task 1".to_string());
    let id2 = todo_list.add_task("Task 2".to_string());
    
    todo_list.set_task_priority(id1, Priority::High);
    todo_list.set_task_priority(id2, Priority::High);
    
    // Initially both are pending high priority
    let filter = TaskFilter::pending_with_priority(Priority::High);
    let tasks = todo_list.get_filtered_tasks(&filter);
    assert_eq!(tasks.len(), 2);
    
    // Complete one task
    todo_list.complete_task(id1);
    
    // Now only one should be pending high priority
    let tasks = todo_list.get_filtered_tasks(&filter);
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0].id, id2);
    
    // Check completed high priority filter
    let completed_filter = TaskFilter::completed_with_priority(Priority::High);
    let completed_tasks = todo_list.get_filtered_tasks(&completed_filter);
    assert_eq!(completed_tasks.len(), 1);
    assert_eq!(completed_tasks[0].id, id1);
}
