use chrono::NaiveDate;
use todo_manager::models::Priority;
use todo_manager::ui::output::OutputWriter;
use std::cell::RefCell;
use std::rc::Rc;
use todo_manager::controller::task_command::TaskCommandOutputManager;

/// Mock output writer for testing
struct MockOutputWriter {
    lines: Vec<String>,
}

impl MockOutputWriter {
    fn new() -> Self {
        MockOutputWriter { lines: Vec::new() }
    }

    fn get_lines(&self) -> &[String] {
        &self.lines
    }

    fn contains(&self, text: &str) -> bool {
        self.lines.iter().any(|line| line.contains(text))
    }
}

impl OutputWriter for MockOutputWriter {
    fn write_line(&mut self, text: &str) {
        self.lines.push(text.to_string());
    }

    fn show_success(&mut self, message: &str) {
        self.lines.push(format!("SUCCESS: {}", message));
    }

    fn show_error(&mut self, message: &str) {
        self.lines.push(format!("ERROR: {}", message));
    }

    fn write_prompt(&mut self) {
        self.lines.push("PROMPT".to_string());
    }
}

fn create_test_manager() -> (TaskCommandOutputManager<MockOutputWriter>, Rc<RefCell<MockOutputWriter>>) {
    let writer = Rc::new(RefCell::new(MockOutputWriter::new()));
    let manager = TaskCommandOutputManager::new(writer.clone());
    (manager, writer)
}

#[test]
fn test_show_task_added() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_added(1, "Test task");
    assert!(writer.borrow().contains("Task added"));
    assert!(writer.borrow().contains("ID 1"));
    assert!(writer.borrow().contains("Test task"));
}

#[test]
fn test_show_subtask_added() {
    let (mut manager, writer) = create_test_manager();
    manager.show_subtask_added(2, 1, "Subtask description");
    assert!(writer.borrow().contains("Subtask added"));
    assert!(writer.borrow().contains("ID 2"));
    assert!(writer.borrow().contains("parent task 1"));
    assert!(writer.borrow().contains("Subtask description"));
}

#[test]
fn test_show_dependency_added() {
    let (mut manager, writer) = create_test_manager();
    manager.show_dependency_added(1, 2);
    assert!(writer.borrow().contains("Task 1"));
    assert!(writer.borrow().contains("depends on task 2"));
}

#[test]
fn test_show_dependency_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_dependency_removed(1, 2);
    assert!(writer.borrow().contains("Removed dependency"));
    assert!(writer.borrow().contains("task 1"));
    assert!(writer.borrow().contains("no longer depends on task 2"));
}

#[test]
fn test_show_dependency_graph_no_deps() {
    let (mut manager, writer) = create_test_manager();
    manager.show_dependency_graph(1, "Main task", false, &[], &[]);
    
    let lines = writer.borrow();
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependency Graph for Task 1")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Main task")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependencies: None")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependents: None")));
}

#[test]
fn test_show_dependency_graph_with_dependencies() {
    let (mut manager, writer) = create_test_manager();
    let dependencies = vec![
        (2, "Dep task 1".to_string(), false),
        (3, "Dep task 2".to_string(), true),
    ];
    manager.show_dependency_graph(1, "Main task", false, &dependencies, &[]);
    
    let lines = writer.borrow();
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependencies (2)")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dep task 1")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dep task 2")));
}

#[test]
fn test_show_dependency_graph_with_dependents() {
    let (mut manager, writer) = create_test_manager();
    let dependents = vec![
        (4, "Dependent task 1".to_string(), false),
        (5, "Dependent task 2".to_string(), true),
    ];
    manager.show_dependency_graph(1, "Main task", false, &[], &dependents);
    
    let lines = writer.borrow();
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependents (2)")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependent task 1")));
    assert!(lines.get_lines().iter().any(|l| l.contains("Dependent task 2")));
}

#[test]
fn test_show_task_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_removed("Task to remove");
    assert!(writer.borrow().contains("Task removed"));
    assert!(writer.borrow().contains("Task to remove"));
}

#[test]
fn test_show_task_not_found() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_not_found(99);
    assert!(writer.borrow().contains("Task with ID 99 not found"));
}

#[test]
fn test_show_task_completed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_completed("Completed task");
    assert!(writer.borrow().contains("Completed task"));
    assert!(writer.borrow().contains("Completed task"));
}

#[test]
fn test_show_task_uncompleted() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_uncompleted("Uncompleted task");
    assert!(writer.borrow().contains("marked as pending"));
    assert!(writer.borrow().contains("Uncompleted task"));
}

#[test]
fn test_show_task_toggled_completed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_toggled("Toggled task", true);
    assert!(writer.borrow().contains("Toggled task"));
    assert!(writer.borrow().contains("completed"));
}

#[test]
fn test_show_task_toggled_pending() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_toggled("Toggled task", false);
    assert!(writer.borrow().contains("Toggled task"));
    assert!(writer.borrow().contains("pending"));
}

#[test]
fn test_show_priority_set() {
    let (mut manager, writer) = create_test_manager();
    manager.show_priority_set("High priority task", Priority::High);
    assert!(writer.borrow().contains("High priority task"));
    assert!(writer.borrow().contains("priority"));
}

#[test]
fn test_show_due_date_set() {
    let (mut manager, writer) = create_test_manager();
    let date = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
    manager.show_due_date_set("Task with due date", Some(date));
    assert!(writer.borrow().contains("Task with due date"));
    assert!(writer.borrow().contains("due date"));
}

#[test]
fn test_show_due_date_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_due_date_set("Task without due date", None);
    assert!(writer.borrow().contains("Task without due date"));
    assert!(writer.borrow().contains("removed") || writer.borrow().contains("cleared"));
}

#[test]
fn test_show_category_set() {
    let (mut manager, writer) = create_test_manager();
    manager.show_category_set("Categorized task", Some("Work".to_string()));
    assert!(writer.borrow().contains("Category set"));
    assert!(writer.borrow().contains("Categorized task"));
}

#[test]
fn test_show_category_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_category_set("Uncategorized task", None);
    assert!(writer.borrow().contains("Uncategorized task"));
    assert!(writer.borrow().contains("cleared") || writer.borrow().contains("Category cleared"));
}

#[test]
fn test_show_multiple_tasks_completed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_completed(3, &[]);
    assert!(writer.borrow().contains("Completed 3 tasks"));
}

#[test]
fn test_show_multiple_tasks_completed_singular() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_completed(1, &[]);
    assert!(writer.borrow().contains("Completed 1 task"));
}

#[test]
fn test_show_multiple_tasks_completed_with_not_found() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_completed(2, &[99, 100]);
    assert!(writer.borrow().contains("Completed 2 tasks"));
    assert!(writer.borrow().contains("99, 100 not found"));
}

#[test]
fn test_show_all_tasks_completed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_all_tasks_completed(5);
    assert!(writer.borrow().contains("Completed all 5 tasks"));
}

#[test]
fn test_show_all_tasks_completed_empty() {
    let (mut manager, writer) = create_test_manager();
    manager.show_all_tasks_completed(0);
    assert!(writer.borrow().contains("No tasks to complete"));
}

#[test]
fn test_show_multiple_tasks_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_removed(3, &[]);
    assert!(writer.borrow().contains("Removed 3 tasks"));
}

#[test]
fn test_show_multiple_tasks_removed_with_not_found() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_removed(2, &[88]);
    assert!(writer.borrow().contains("Removed 2 tasks"));
    assert!(writer.borrow().contains("88 not found"));
}

#[test]
fn test_show_all_tasks_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_all_tasks_removed(7);
    assert!(writer.borrow().contains("Removed all 7 tasks"));
}

#[test]
fn test_show_multiple_tasks_uncompleted() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_uncompleted(4, &[]);
    assert!(writer.borrow().contains("Marked 4 tasks as pending"));
}

#[test]
fn test_show_all_tasks_uncompleted() {
    let (mut manager, writer) = create_test_manager();
    manager.show_all_tasks_uncompleted(6);
    assert!(writer.borrow().contains("Marked all 6 tasks as pending"));
}

#[test]
fn test_show_multiple_tasks_toggled() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_tasks_toggled(3, &[]);
    assert!(writer.borrow().contains("Toggled 3 tasks"));
}

#[test]
fn test_show_all_tasks_toggled() {
    let (mut manager, writer) = create_test_manager();
    manager.show_all_tasks_toggled(8);
    assert!(writer.borrow().contains("Toggled all 8 tasks"));
}

#[test]
fn test_show_multiple_priorities_set() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_priorities_set(3, Priority::High, &[]);
    assert!(writer.borrow().contains("Set priority"));
    assert!(writer.borrow().contains("3 tasks"));
}

#[test]
fn test_show_multiple_priorities_set_with_not_found() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_priorities_set(2, Priority::Low, &[77]);
    assert!(writer.borrow().contains("Set priority"));
    assert!(writer.borrow().contains("2 tasks"));
    assert!(writer.borrow().contains("77 not found"));
}

#[test]
fn test_show_multiple_categories_set() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_categories_set(4, Some("Work"), &[]);
    assert!(writer.borrow().contains("Set category"));
    assert!(writer.borrow().contains("4 tasks"));
}

#[test]
fn test_show_multiple_categories_removed() {
    let (mut manager, writer) = create_test_manager();
    manager.show_multiple_categories_set(2, None, &[]);
    assert!(writer.borrow().contains("Cleared category"));
    assert!(writer.borrow().contains("2 tasks"));
}

#[test]
fn test_show_error() {
    let (mut manager, writer) = create_test_manager();
    manager.show_error("Something went wrong");
    assert!(writer.borrow().contains("ERROR"));
    assert!(writer.borrow().contains("Something went wrong"));
}

#[test]
fn test_show_recurring_task_created() {
    let (mut manager, writer) = create_test_manager();
    manager.show_recurring_task_created(5, "Daily standup");
    assert!(writer.borrow().contains("recurring task"));
    assert!(writer.borrow().contains("ID 5"));
    assert!(writer.borrow().contains("Daily standup"));
}

#[test]
fn test_show_task_edited() {
    let (mut manager, writer) = create_test_manager();
    manager.show_task_edited("Old description", "New description");
    assert!(writer.borrow().contains("updated"));
    assert!(writer.borrow().contains("Old description") || writer.borrow().contains("New description"));
}
