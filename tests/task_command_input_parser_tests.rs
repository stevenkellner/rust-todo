use chrono::NaiveDate;
use todo_manager::controller::task_command::{TaskCommand, TaskCommandInputParser, TaskSelection};
use todo_manager::models::priority::Priority;
use todo_manager::models::recurrence::Recurrence;

#[test]
fn test_parse_priority_command_single_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("priority", &["1", "high"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetPriority(TaskSelection::Single(id), priority) => {
            assert_eq!(id, 1);
            assert_eq!(priority, Priority::High);
        }
        _ => panic!("Expected SetPriority command"),
    }
}

#[test]
fn test_parse_priority_command_all() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("priority", &["all", "low"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetPriority(TaskSelection::All, priority) => {
            assert_eq!(priority, Priority::Low);
        }
        _ => panic!("Expected SetPriority command with All selection"),
    }
}

#[test]
fn test_parse_priority_command_range() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("priority", &["1-3", "medium"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetPriority(TaskSelection::Multiple(ids), priority) => {
            assert_eq!(ids, vec![1, 2, 3]);
            assert_eq!(priority, Priority::Medium);
        }
        _ => panic!("Expected SetPriority command with Multiple selection"),
    }
}

#[test]
fn test_parse_priority_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("priority", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_priority_command_invalid_priority() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("priority", &["1", "urgent"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_priority_command_aliases() {
    let parser = TaskCommandInputParser::new();

    // Test "pri" alias
    let result = parser.try_parse("pri", &["1", "h"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    // Test short priority names
    let result = parser.try_parse("priority", &["1", "m"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("priority", &["1", "l"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_set_due_command_valid_date() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["1", "31.12.2025"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetDueDate(id, Some(date)) => {
            assert_eq!(id, 1);
            assert_eq!(date, NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
        }
        _ => panic!("Expected SetDueDate command with date"),
    }
}

#[test]
fn test_parse_set_due_command_none() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["1", "none"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetDueDate(id, None) => {
            assert_eq!(id, 1);
        }
        _ => panic!("Expected SetDueDate command with None"),
    }
}

#[test]
fn test_parse_set_due_command_invalid_date_format() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["1", "2025/12/31"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_due_command_invalid_date() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["1", "32.13.2025"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_due_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_due_command_invalid_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-due", &["abc", "31.12.2025"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_due_command_alias() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("due", &["1", "31.12.2025"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_set_category_command_single() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-category", &["1", "work"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetCategory(TaskSelection::Single(id), Some(category)) => {
            assert_eq!(id, 1);
            assert_eq!(category, "work");
        }
        _ => panic!("Expected SetCategory command"),
    }
}

#[test]
fn test_parse_set_category_command_multi_word() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-category", &["1", "personal", "projects"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetCategory(TaskSelection::Single(id), Some(category)) => {
            assert_eq!(id, 1);
            assert_eq!(category, "personal projects");
        }
        _ => panic!("Expected SetCategory command with multi-word category"),
    }
}

#[test]
fn test_parse_set_category_command_none() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-category", &["1", "none"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetCategory(TaskSelection::Single(id), None) => {
            assert_eq!(id, 1);
        }
        _ => panic!("Expected SetCategory command with None"),
    }
}

#[test]
fn test_parse_set_category_command_all() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-category", &["all", "urgent"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetCategory(TaskSelection::All, Some(category)) => {
            assert_eq!(category, "urgent");
        }
        _ => panic!("Expected SetCategory command with All selection"),
    }
}

#[test]
fn test_parse_set_category_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-category", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_category_command_aliases() {
    let parser = TaskCommandInputParser::new();

    let result = parser.try_parse("category", &["1", "work"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("cat", &["1", "work"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_set_recurring_command_daily() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["1", "daily"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetRecurring(TaskSelection::Single(id), Some(recurrence)) => {
            assert_eq!(id, 1);
            assert_eq!(recurrence, Recurrence::Daily);
        }
        _ => panic!("Expected SetRecurring command"),
    }
}

#[test]
fn test_parse_set_recurring_command_weekly() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["1", "weekly"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetRecurring(TaskSelection::Single(id), Some(recurrence)) => {
            assert_eq!(id, 1);
            assert_eq!(recurrence, Recurrence::Weekly);
        }
        _ => panic!("Expected SetRecurring command with Weekly"),
    }
}

#[test]
fn test_parse_set_recurring_command_monthly() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["1", "monthly"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetRecurring(TaskSelection::Single(id), Some(recurrence)) => {
            assert_eq!(id, 1);
            assert_eq!(recurrence, Recurrence::Monthly);
        }
        _ => panic!("Expected SetRecurring command with Monthly"),
    }
}

#[test]
fn test_parse_set_recurring_command_none() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["1", "none"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetRecurring(TaskSelection::Single(id), None) => {
            assert_eq!(id, 1);
        }
        _ => panic!("Expected SetRecurring command with None"),
    }
}

#[test]
fn test_parse_set_recurring_command_invalid_frequency() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["1", "yearly"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_set_recurring_command_all() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("set-recurring", &["all", "daily"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::SetRecurring(TaskSelection::All, Some(recurrence)) => {
            assert_eq!(recurrence, Recurrence::Daily);
        }
        _ => panic!("Expected SetRecurring command with All selection"),
    }
}

#[test]
fn test_parse_set_recurring_command_aliases() {
    let parser = TaskCommandInputParser::new();

    let result = parser.try_parse("recurring", &["1", "daily"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("recur", &["1", "weekly"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_add_dependency_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("add-dependency", &["2", "1"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::AddDependency(task_id, depends_on_id) => {
            assert_eq!(task_id, 2);
            assert_eq!(depends_on_id, 1);
        }
        _ => panic!("Expected AddDependency command"),
    }
}

#[test]
fn test_parse_add_dependency_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("add-dependency", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_add_dependency_command_invalid_task_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("add-dependency", &["abc", "1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_add_dependency_command_invalid_depends_on_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("add-dependency", &["1", "xyz"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_add_dependency_command_aliases() {
    let parser = TaskCommandInputParser::new();

    let result = parser.try_parse("add-dep", &["2", "1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("depends-on", &["2", "1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_remove_dependency_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("remove-dependency", &["2", "1"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::RemoveDependency(task_id, depends_on_id) => {
            assert_eq!(task_id, 2);
            assert_eq!(depends_on_id, 1);
        }
        _ => panic!("Expected RemoveDependency command"),
    }
}

#[test]
fn test_parse_remove_dependency_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("remove-dependency", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_remove_dependency_command_invalid_ids() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("remove-dependency", &["abc", "xyz"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_remove_dependency_command_aliases() {
    let parser = TaskCommandInputParser::new();

    let result = parser.try_parse("remove-dep", &["2", "1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("rm-dep", &["2", "1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_show_dependency_graph_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("show-dependencies", &["1"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::ShowDependencyGraph(task_id) => {
            assert_eq!(task_id, 1);
        }
        _ => panic!("Expected ShowDependencyGraph command"),
    }
}

#[test]
fn test_parse_show_dependency_graph_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("show-dependencies", &[]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_show_dependency_graph_command_invalid_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("show-dependencies", &["abc"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_show_dependency_graph_command_aliases() {
    let parser = TaskCommandInputParser::new();

    let result = parser.try_parse("dependencies", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("deps", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("dep-graph", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());

    let result = parser.try_parse("dependency-graph", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_list_categories_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("categories", &[]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::ListCategories => {}
        _ => panic!("Expected ListCategories command"),
    }
}

#[test]
fn test_parse_list_categories_command_alias() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("list-categories", &[]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_search_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("search", &["test"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::Search(keyword) => {
            assert_eq!(keyword, "test");
        }
        _ => panic!("Expected Search command"),
    }
}

#[test]
fn test_parse_search_command_multi_word() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("search", &["buy", "groceries"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::Search(keyword) => {
            assert_eq!(keyword, "buy groceries");
        }
        _ => panic!("Expected Search command with multi-word keyword"),
    }
}

#[test]
fn test_parse_search_command_missing_args() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("search", &[]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_search_command_alias() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("find", &["keyword"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_edit_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("edit", &["1", "Updated", "description"]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::Edit(id, description) => {
            assert_eq!(id, 1);
            assert_eq!(description, "Updated description");
        }
        _ => panic!("Expected Edit command"),
    }
}

#[test]
fn test_parse_edit_command_missing_description() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("edit", &["1"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_edit_command_invalid_id() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("edit", &["abc", "description"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_statistics_command() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("statistics", &[]);
    assert!(result.is_some());
    let cmd = result.unwrap().unwrap();
    match cmd {
        TaskCommand::ShowStatistics => {}
        _ => panic!("Expected ShowStatistics command"),
    }
}

#[test]
fn test_parse_statistics_command_alias() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("stats", &[]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}

#[test]
fn test_parse_task_selection_invalid_format() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("complete", &["not-a-number"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_err());
}

#[test]
fn test_parse_list_command_with_invalid_filter() {
    let parser = TaskCommandInputParser::new();
    let result = parser.try_parse("list", &["invalidfilter"]);
    assert!(result.is_some());
    // Should return error for invalid filter
    assert!(result.unwrap().is_err());
}

#[test]
fn test_default_parser() {
    let parser: TaskCommandInputParser = Default::default();
    let result = parser.try_parse("add", &["Test", "task"]);
    assert!(result.is_some());
    assert!(result.unwrap().is_ok());
}
