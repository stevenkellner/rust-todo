use crate::models::todo_list::TodoList;
use crate::models::priority::Priority;
use crate::ui::output_writer::OutputWriter;
use chrono::NaiveDate;
use std::io::Write;

/// Handles task-related commands and operations.
///
/// `TaskCommandHandler` encapsulates all operations related to individual tasks
/// such as completing, editing, setting priority, due date, and category.
pub struct TaskCommandHandler {
}

impl TaskCommandHandler {
    /// Creates a new task command handler.
    pub fn new() -> Self {
        TaskCommandHandler {}
    }

    /// Removes a task by ID.
    pub fn remove_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.remove_task(id) {
            output.show_task_removed(&task.description);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Marks a task as completed.
    pub fn complete_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.complete_task(id) {
            if task.is_completed() {
                output.show_task_completed(&task.description);
            }
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Marks a task as not completed.
    pub fn uncomplete_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.uncomplete_task(id) {
            if !task.is_completed() {
                output.show_task_uncompleted(&task.description);
            }
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Toggles a task's completion status.
    pub fn toggle_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.toggle_task(id) {
            output.show_task_toggled(&task.description, task.is_completed());
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the priority of a task.
    pub fn set_priority<W: Write>(
        &self,
        id: usize,
        priority: Priority,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_task_priority(id, priority) {
            output.show_priority_set(&task.description, priority);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the due date of a task.
    pub fn set_due_date<W: Write>(
        &self,
        id: usize,
        due_date: Option<NaiveDate>,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_due_date(id, due_date) {
            output.show_due_date_set(&task.description, due_date);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the category of a task.
    pub fn set_category<W: Write>(
        &self,
        id: usize,
        category: Option<String>,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_task_category(id, category.clone()) {
            output.show_category_set(&task.description, category);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Edits a task's description.
    pub fn edit_task<W: Write>(
        &self,
        id: usize,
        new_description: &str,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if new_description.trim().is_empty() {
            output.show_error("Task description cannot be empty.");
            return;
        }

        // Get the old description before editing
        let old_description = todo_list.get_tasks()
            .iter()
            .find(|task| task.id == id)
            .map(|task| task.description.clone());

        if let Some(_task) = todo_list.edit_task(id, new_description.to_string()) {
            if let Some(old_desc) = old_description {
                output.show_task_edited(&old_desc, new_description);
            }
        } else {
            output.show_task_not_found(id);
        }
    }
}

impl Default for TaskCommandHandler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> (TodoList, OutputWriter) {
        let todo_list = TodoList::new();
        let output = OutputWriter::new();
        (todo_list, output)
    }

    #[test]
    fn test_remove_task_existing() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task("Test task".to_string());
        
        let handler = TaskCommandHandler::new();
        handler.remove_task(id, &mut todo_list, &mut output);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).is_none());
    }

    #[test]
    fn test_complete_task() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task("Test task".to_string());
        
        let handler = TaskCommandHandler::new();
        handler.complete_task(id, &mut todo_list, &mut output);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().is_completed());
    }

    #[test]
    fn test_set_priority() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task("Test task".to_string());
        
        let handler = TaskCommandHandler::new();
        handler.set_priority(id, Priority::High, &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().priority, Priority::High);
    }

    #[test]
    fn test_edit_task() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task("Old description".to_string());
        
        let handler = TaskCommandHandler::new();
        handler.edit_task(id, "New description", &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().description, "New description");
    }
}
