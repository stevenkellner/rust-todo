// Integration tests for task sorting functionality

use chrono::NaiveDate;
use todo_manager::models::priority::Priority;
use todo_manager::models::task::TaskWithoutId;
use todo_manager::models::task_filter::TaskFilter;
use todo_manager::models::task_sort::{SortBy, SortOrder};
use todo_manager::models::todo_list::TodoList;

#[test]
fn test_sort_by_id_ascending() {
    let mut list = TodoList::new();
    list.add_task(TaskWithoutId::new("Task 3".to_string()));
    list.add_task(TaskWithoutId::new("Task 1".to_string()));
    list.add_task(TaskWithoutId::new("Task 2".to_string()));

    let filter = TaskFilter::all().with_sort_by(SortBy::Id);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[2].id, 3);
}

#[test]
fn test_sort_by_id_descending() {
    let mut list = TodoList::new();
    list.add_task(TaskWithoutId::new("Task 1".to_string()));
    list.add_task(TaskWithoutId::new("Task 2".to_string()));
    list.add_task(TaskWithoutId::new("Task 3".to_string()));

    let filter = TaskFilter::all()
        .with_sort_by(SortBy::Id)
        .with_sort_order(SortOrder::Descending);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 3);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[2].id, 1);
}

#[test]
fn test_sort_by_priority_ascending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Low priority task".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("High priority task".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Medium priority task".to_string()));

    list.set_task_priority(id1, Priority::Low);
    list.set_task_priority(id2, Priority::High);
    list.set_task_priority(id3, Priority::Medium);

    let filter = TaskFilter::all().with_sort_by(SortBy::Priority);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Priority sort: High > Medium > Low (descending priority value)
    assert_eq!(tasks[0].priority, Priority::High);
    assert_eq!(tasks[1].priority, Priority::Medium);
    assert_eq!(tasks[2].priority, Priority::Low);
}

#[test]
fn test_sort_by_priority_descending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Low priority task".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("High priority task".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Medium priority task".to_string()));

    list.set_task_priority(id1, Priority::Low);
    list.set_task_priority(id2, Priority::High);
    list.set_task_priority(id3, Priority::Medium);

    let filter = TaskFilter::all()
        .with_sort_by(SortBy::Priority)
        .with_sort_order(SortOrder::Descending);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Descending order: Low < Medium < High
    assert_eq!(tasks[0].priority, Priority::Low);
    assert_eq!(tasks[1].priority, Priority::Medium);
    assert_eq!(tasks[2].priority, Priority::High);
}

#[test]
fn test_sort_by_due_date_ascending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Task due tomorrow".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("Task due today".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Task with no due date".to_string()));

    let today = NaiveDate::from_ymd_opt(2025, 10, 25).unwrap();
    let tomorrow = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();

    list.set_due_date(id1, Some(tomorrow));
    list.set_due_date(id2, Some(today));

    let filter = TaskFilter::all().with_sort_by(SortBy::DueDate);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Tasks with due dates come first, sorted by date
    assert_eq!(tasks[0].id, id2); // Due today
    assert_eq!(tasks[1].id, id1); // Due tomorrow
    assert_eq!(tasks[2].id, id3); // No due date
}

#[test]
fn test_sort_by_due_date_descending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Task due tomorrow".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("Task due today".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Task with no due date".to_string()));

    let today = NaiveDate::from_ymd_opt(2025, 10, 25).unwrap();
    let tomorrow = NaiveDate::from_ymd_opt(2025, 10, 26).unwrap();

    list.set_due_date(id1, Some(tomorrow));
    list.set_due_date(id2, Some(today));

    let filter = TaskFilter::all()
        .with_sort_by(SortBy::DueDate)
        .with_sort_order(SortOrder::Descending);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Descending: No due date first, then latest to earliest
    assert_eq!(tasks[0].id, id3); // No due date
    assert_eq!(tasks[1].id, id1); // Due tomorrow
    assert_eq!(tasks[2].id, id2); // Due today
}

#[test]
fn test_sort_by_category_ascending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Work task".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("Personal task".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Task with no category".to_string()));

    list.set_task_category(id1, Some("work".to_string()));
    list.set_task_category(id2, Some("personal".to_string()));

    let filter = TaskFilter::all().with_sort_by(SortBy::Category);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Alphabetically: personal < work, then tasks without category
    assert_eq!(tasks[0].id, id2); // personal
    assert_eq!(tasks[1].id, id1); // work
    assert_eq!(tasks[2].id, id3); // No category
}

#[test]
fn test_sort_by_category_descending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Work task".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("Personal task".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Task with no category".to_string()));

    list.set_task_category(id1, Some("work".to_string()));
    list.set_task_category(id2, Some("personal".to_string()));

    let filter = TaskFilter::all()
        .with_sort_by(SortBy::Category)
        .with_sort_order(SortOrder::Descending);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Descending: No category first, then reverse alphabetically (work > personal)
    assert_eq!(tasks[0].id, id3); // No category
    assert_eq!(tasks[1].id, id1); // work
    assert_eq!(tasks[2].id, id2); // personal
}

#[test]
fn test_sort_by_status_ascending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Completed task".to_string()));
    list.add_task(TaskWithoutId::new("Pending task 1".to_string()));
    list.add_task(TaskWithoutId::new("Pending task 2".to_string()));

    list.complete_task(id1);

    let filter = TaskFilter::all().with_sort_by(SortBy::Status);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Pending tasks first, then completed
    assert!(!tasks[0].is_completed()); // Pending
    assert!(!tasks[1].is_completed()); // Pending
    assert!(tasks[2].is_completed()); // Completed
}

#[test]
fn test_sort_by_status_descending() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Completed task".to_string()));
    list.add_task(TaskWithoutId::new("Pending task 1".to_string()));
    list.add_task(TaskWithoutId::new("Pending task 2".to_string()));

    list.complete_task(id1);

    let filter = TaskFilter::all()
        .with_sort_by(SortBy::Status)
        .with_sort_order(SortOrder::Descending);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    // Completed tasks first, then pending
    assert!(tasks[0].is_completed()); // Completed
    assert!(!tasks[1].is_completed()); // Pending
    assert!(!tasks[2].is_completed()); // Pending
}

#[test]
fn test_default_sort_without_filter() {
    let mut list = TodoList::new();
    list.add_task(TaskWithoutId::new("Task 1".to_string()));
    list.add_task(TaskWithoutId::new("Task 2".to_string()));
    list.add_task(TaskWithoutId::new("Task 3".to_string()));

    // No filter means default sort by ID ascending
    let filter = TaskFilter::all();
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 3);
    assert_eq!(tasks[0].id, 1);
    assert_eq!(tasks[1].id, 2);
    assert_eq!(tasks[2].id, 3);
}

#[test]
fn test_combined_filter_and_sort() {
    let mut list = TodoList::new();
    let id1 = list.add_task(TaskWithoutId::new("Low priority pending".to_string()));
    let id2 = list.add_task(TaskWithoutId::new("High priority pending".to_string()));
    let id3 = list.add_task(TaskWithoutId::new("Medium priority completed".to_string()));

    list.set_task_priority(id1, Priority::Low);
    list.set_task_priority(id2, Priority::High);
    list.set_task_priority(id3, Priority::Medium);
    list.complete_task(id3);

    // Filter by pending status and sort by priority
    use todo_manager::models::task_status::TaskStatus;
    let filter = TaskFilter::all()
        .with_status(TaskStatus::Pending)
        .with_sort_by(SortBy::Priority);
    let tasks = list.get_filtered_tasks(&filter);

    assert_eq!(tasks.len(), 2);
    // Should only show pending tasks, sorted by priority (High first)
    assert_eq!(tasks[0].id, id2); // High priority
    assert_eq!(tasks[1].id, id1); // Low priority
}
