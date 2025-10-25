use chrono::NaiveDate;
use todo_manager::models::task::TaskWithoutId;
use todo_manager::models::todo_list::TodoList;

#[test]
fn test_todo_list_add_subtask_success() {
    let mut list = TodoList::new();
    let parent_id = list.add_task(TaskWithoutId::new("Parent task".to_string()));

    let subtask_id = list.add_subtask(parent_id, "Subtask".to_string());
    assert!(subtask_id.is_some());

    let subtasks = list.get_subtasks(parent_id);
    assert_eq!(subtasks.len(), 1);
}

#[test]
fn test_todo_list_add_subtask_parent_not_found() {
    let mut list = TodoList::new();

    let subtask_id = list.add_subtask(999, "Subtask".to_string());
    assert!(subtask_id.is_none());
}

#[test]
fn test_todo_list_set_due_date() {
    let mut list = TodoList::new();
    let task_id = list.add_task(TaskWithoutId::new("Task with due date".to_string()));

    let due_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    let result = list.set_due_date(task_id, Some(due_date));
    assert!(result.is_some());

    let task = list.get_tasks().iter().find(|t| t.id == task_id).unwrap();
    assert_eq!(task.get_due_date(), Some(due_date));
}

#[test]
fn test_todo_list_clear_due_date() {
    let mut list = TodoList::new();
    let task_id = list.add_task(TaskWithoutId::new("Task".to_string()));

    // Set then clear
    let due_date = NaiveDate::from_ymd_opt(2025, 12, 31).unwrap();
    list.set_due_date(task_id, Some(due_date));
    let result = list.set_due_date(task_id, None);
    assert!(result.is_some());

    let task = list.get_tasks().iter().find(|t| t.id == task_id).unwrap();
    assert_eq!(task.get_due_date(), None);
}

#[test]
fn test_todo_list_add_dependency_success() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));

    let result = list.add_task_dependency(task2_id, task1_id);
    assert!(result.is_some());

    let task2 = list.get_tasks().iter().find(|t| t.id == task2_id).unwrap();
    assert!(task2.get_dependencies().contains(&task1_id));
}

#[test]
fn test_todo_list_add_dependency_self() {
    let mut list = TodoList::new();
    let task_id = list.add_task(TaskWithoutId::new("Task".to_string()));

    let result = list.add_task_dependency(task_id, task_id);
    assert!(result.is_none());
}

#[test]
fn test_todo_list_add_dependency_task_not_found() {
    let mut list = TodoList::new();
    let task_id = list.add_task(TaskWithoutId::new("Task".to_string()));

    let result = list.add_task_dependency(task_id, 999);
    assert!(result.is_none());
}

#[test]
fn test_todo_list_add_dependency_circular() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));

    // task2 depends on task1
    list.add_task_dependency(task2_id, task1_id);

    // Try to make task1 depend on task2 (would create circular dependency)
    let result = list.add_task_dependency(task1_id, task2_id);
    assert!(result.is_none());
}

#[test]
fn test_todo_list_remove_dependency_success() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));

    list.add_task_dependency(task2_id, task1_id);

    let result = list.remove_task_dependency(task2_id, task1_id);
    assert!(result.is_some());

    let task2 = list.get_tasks().iter().find(|t| t.id == task2_id).unwrap();
    assert!(!task2.get_dependencies().contains(&task1_id));
}

#[test]
fn test_todo_list_get_dependent_tasks() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    let task3_id = list.add_task(TaskWithoutId::new("Task 3".to_string()));

    // task2 and task3 both depend on task1
    list.add_task_dependency(task2_id, task1_id);
    list.add_task_dependency(task3_id, task1_id);

    let dependents = list.get_dependent_tasks(task1_id);
    assert_eq!(dependents.len(), 2);
    assert!(dependents.contains(&task2_id));
    assert!(dependents.contains(&task3_id));
}

#[test]
fn test_todo_list_get_all_categories() {
    let mut list = TodoList::new();

    let mut task1 = TaskWithoutId::new("Task 1".to_string());
    task1.category = Some("work".to_string());
    let _task1_id = list.add_task(task1);

    let mut task2 = TaskWithoutId::new("Task 2".to_string());
    task2.category = Some("personal".to_string());
    let _task2_id = list.add_task(task2);

    let mut task3 = TaskWithoutId::new("Task 3".to_string());
    task3.category = Some("work".to_string());
    let _task3_id = list.add_task(task3);

    let categories = list.get_all_categories();
    assert_eq!(categories.len(), 2);
    assert!(categories.contains(&"work".to_string()));
    assert!(categories.contains(&"personal".to_string()));
}

#[test]
fn test_todo_list_get_statistics() {
    let mut list = TodoList::new();

    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    list.add_task(TaskWithoutId::new("Task 2".to_string()));
    list.add_task(TaskWithoutId::new("Task 3".to_string()));

    list.complete_task(task1_id);

    let stats = list.get_statistics();
    assert_eq!(stats.total, 3);
    assert_eq!(stats.completed, 1);
    assert_eq!(stats.pending, 2);
}

#[test]
fn test_todo_list_edit_task_description() {
    let mut list = TodoList::new();
    let task_id = list.add_task(TaskWithoutId::new("Old description".to_string()));

    let result = list.edit_task(task_id, "New description".to_string());
    assert!(result.is_some());

    let task = list.get_tasks().iter().find(|t| t.id == task_id).unwrap();
    assert_eq!(task.description, "New description");
}

#[test]
fn test_todo_list_edit_task_not_found() {
    let mut list = TodoList::new();

    let result = list.edit_task(999, "New description".to_string());
    assert!(result.is_none());
}

#[test]
fn test_todo_list_get_subtask_count() {
    let mut list = TodoList::new();
    let parent_id = list.add_task(TaskWithoutId::new("Parent".to_string()));

    list.add_subtask(parent_id, "Subtask 1".to_string());
    list.add_subtask(parent_id, "Subtask 2".to_string());
    list.add_subtask(parent_id, "Subtask 3".to_string());

    let count = list.get_subtask_count(parent_id);
    assert_eq!(count, 3);
}

#[test]
fn test_todo_list_remove_parent_removes_subtasks() {
    let mut list = TodoList::new();
    let parent_id = list.add_task(TaskWithoutId::new("Parent".to_string()));

    list.add_subtask(parent_id, "Subtask 1".to_string());
    list.add_subtask(parent_id, "Subtask 2".to_string());

    assert_eq!(list.get_tasks().len(), 3);

    list.remove_task(parent_id);

    // Parent and subtasks should be removed
    assert_eq!(list.get_tasks().len(), 0);
}

#[test]
fn test_todo_list_task_with_multiple_subtasks() {
    let mut list = TodoList::new();
    let parent_id = list.add_task(TaskWithoutId::new("Main project".to_string()));

    for i in 1..=5 {
        list.add_subtask(parent_id, format!("Subtask {}", i));
    }

    let subtasks = list.get_subtasks(parent_id);
    assert_eq!(subtasks.len(), 5);
}

#[test]
fn test_todo_list_complete_task_with_subtasks() {
    let mut list = TodoList::new();
    let parent_id = list.add_task(TaskWithoutId::new("Parent".to_string()));
    let subtask_id = list.add_subtask(parent_id, "Subtask".to_string()).unwrap();

    list.complete_task(parent_id);

    let parent = list.get_tasks().iter().find(|t| t.id == parent_id).unwrap();
    assert!(parent.is_completed());

    // Subtask status is independent
    let subtask = list
        .get_tasks()
        .iter()
        .find(|t| t.id == subtask_id)
        .unwrap();
    assert!(!subtask.is_completed());
}

#[test]
fn test_todo_list_nested_dependencies() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));
    let task3_id = list.add_task(TaskWithoutId::new("Task 3".to_string()));

    // Chain: task3 -> task2 -> task1
    list.add_task_dependency(task2_id, task1_id);
    list.add_task_dependency(task3_id, task2_id);

    // Should not allow task1 -> task3 (circular)
    let result = list.add_task_dependency(task1_id, task3_id);
    assert!(result.is_none());
}

#[test]
fn test_todo_list_remove_task_cleans_up_references() {
    let mut list = TodoList::new();
    let task1_id = list.add_task(TaskWithoutId::new("Task 1".to_string()));
    let task2_id = list.add_task(TaskWithoutId::new("Task 2".to_string()));

    list.add_task_dependency(task2_id, task1_id);

    // Remove task1
    list.remove_task(task1_id);

    // Verify task1 is removed
    assert!(list.get_tasks().iter().all(|t| t.id != task1_id));

    // task2 should still exist
    assert!(list.get_tasks().iter().any(|t| t.id == task2_id));
}

#[test]
fn test_todo_list_overdue_tasks() {
    let mut list = TodoList::new();

    let past_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
    let future_date = NaiveDate::from_ymd_opt(2099, 12, 31).unwrap();

    let task1_id = list.add_task(TaskWithoutId::new("Overdue task".to_string()));
    list.set_due_date(task1_id, Some(past_date));

    let task2_id = list.add_task(TaskWithoutId::new("Future task".to_string()));
    list.set_due_date(task2_id, Some(future_date));

    // Test overdue detection
    let today = chrono::Local::now().date_naive();
    let task1 = list.get_tasks().iter().find(|t| t.id == task1_id).unwrap();
    assert!(task1.is_overdue(today));

    let task2 = list.get_tasks().iter().find(|t| t.id == task2_id).unwrap();
    assert!(!task2.is_overdue(today));
}
