use crate::models::todo_list::TodoList;
use crate::models::task_command::TaskCommand;
use crate::models::task::TaskWithoutId;
use crate::models::priority::Priority;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use crate::ui::TaskCommandOutputWriter;
use chrono::NaiveDate;
use std::io::Write;

/// Handles task-related commands and operations.
///
/// `TaskCommandHandler` encapsulates all operations related to individual tasks
/// such as completing, editing, setting priority, due date, and category.
pub struct TaskCommandHandler<W: Write> {
    output: TaskCommandOutputWriter<W>,
}

impl TaskCommandHandler<std::io::Stdout> {
    /// Creates a new task command handler with stdout.
    pub fn new() -> Self {
        TaskCommandHandler {
            output: TaskCommandOutputWriter::new(),
        }
    }
}

impl<W: Write> TaskCommandHandler<W> {
    /// Creates a new task command handler with a custom output writer.
    pub fn with_writer(writer: W) -> Self {
        TaskCommandHandler {
            output: TaskCommandOutputWriter::with_writer(writer),
        }
    }

    /// Handles a task command
    pub fn handle(
        &mut self,
        command: &TaskCommand,
        todo_list: &mut TodoList,
    ) {
        match command {
            TaskCommand::Add(description) => {
                let new_task = TaskWithoutId::new(description.clone());
                let task_id = todo_list.add_task(new_task);
                self.output.show_task_added(task_id, description);
            }
            TaskCommand::List(filter) => self.list_tasks(filter, todo_list),
            TaskCommand::Remove(id) => self.remove_task(*id, todo_list),
            TaskCommand::Complete(id) => self.complete_task(*id, todo_list),
            TaskCommand::Uncomplete(id) => self.uncomplete_task(*id, todo_list),
            TaskCommand::Toggle(id) => self.toggle_task(*id, todo_list),
            TaskCommand::SetPriority(id, priority) => self.set_priority(*id, *priority, todo_list),
            TaskCommand::SetDueDate(id, due_date) => self.set_due_date(*id, *due_date, todo_list),
            TaskCommand::SetCategory(id, category) => self.set_category(*id, category.clone(), todo_list),
            TaskCommand::ListCategories => {
                self.output.show_categories(todo_list);
            }
            TaskCommand::Edit(id, new_description) => self.edit_task(*id, new_description, todo_list),
            TaskCommand::Search(keyword) => {
                self.output.show_search_results(todo_list, keyword);
            }
            TaskCommand::ShowStatistics => {
                self.output.show_statistics(todo_list);
            }
        }
    }

    /// Lists tasks with optional filtering
    fn list_tasks(
        &mut self,
        filter: &Option<TaskFilter>,
        todo_list: &mut TodoList,
    ) {
        match filter {
            None => self.output.show_all_tasks(todo_list.get_tasks()),
            Some(task_filter) => {
                let filtered_tasks = todo_list.get_filtered_tasks(task_filter);
                
                if task_filter.status == Some(TaskStatus::Completed) && task_filter.priority.is_none() {
                    self.output.show_completed_tasks(&filtered_tasks);
                } else if task_filter.status == Some(TaskStatus::Pending) && task_filter.priority.is_none() {
                    self.output.show_pending_tasks(&filtered_tasks);
                } else if let Some(priority) = task_filter.priority {
                    self.output.show_tasks_by_priority(&filtered_tasks, priority);
                } else {
                    self.output.show_filtered_tasks(&filtered_tasks, task_filter);
                }
            }
        }
    }

    /// Removes a task by ID.
    fn remove_task(
        &mut self,
        id: usize,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.remove_task(id) {
            self.output.show_task_removed(&task.description);
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Marks a task as completed.
    fn complete_task(
        &mut self,
        id: usize,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.complete_task(id) {
            if task.is_completed() {
                self.output.show_task_completed(&task.description);
            }
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Marks a task as not completed.
    fn uncomplete_task(
        &mut self,
        id: usize,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.uncomplete_task(id) {
            if !task.is_completed() {
                self.output.show_task_uncompleted(&task.description);
            }
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Toggles a task's completion status.
    fn toggle_task(
        &mut self,
        id: usize,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.toggle_task(id) {
            self.output.show_task_toggled(&task.description, task.is_completed());
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Sets the priority of a task.
    fn set_priority(
        &mut self,
        id: usize,
        priority: Priority,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.set_task_priority(id, priority) {
            self.output.show_priority_set(&task.description, priority);
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Sets the due date of a task.
    fn set_due_date(
        &mut self,
        id: usize,
        due_date: Option<NaiveDate>,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.set_due_date(id, due_date) {
            self.output.show_due_date_set(&task.description, due_date);
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Sets the category of a task.
    fn set_category(
        &mut self,
        id: usize,
        category: Option<String>,
        todo_list: &mut TodoList,
    ) {
        if let Some(task) = todo_list.set_task_category(id, category.clone()) {
            self.output.show_category_set(&task.description, category);
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Edits a task's description.
    fn edit_task(
        &mut self,
        id: usize,
        new_description: &str,
        todo_list: &mut TodoList,
    ) {
        if new_description.trim().is_empty() {
            self.output.show_error("Task description cannot be empty.");
            return;
        }

        // Get the old description before editing
        let old_description = todo_list.get_tasks()
            .iter()
            .find(|task| task.id == id)
            .map(|task| task.description.clone());

        if let Some(_task) = todo_list.edit_task(id, new_description.to_string()) {
            if let Some(old_desc) = old_description {
                self.output.show_task_edited(&old_desc, new_description);
            }
        } else {
            self.output.show_task_not_found(id);
        }
    }
}

impl Default for TaskCommandHandler<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_handler() -> (TodoList, TaskCommandHandler<Vec<u8>>) {
        let todo_list = TodoList::new();
        let handler = TaskCommandHandler::with_writer(Vec::new());
        (todo_list, handler)
    }

    #[test]
    fn test_remove_task_existing() {
        let (mut todo_list, mut handler) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        handler.remove_task(id, &mut todo_list);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).is_none());
    }

    #[test]
    fn test_complete_task() {
        let (mut todo_list, mut handler) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        handler.complete_task(id, &mut todo_list);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().is_completed());
    }

    #[test]
    fn test_set_priority() {
        let (mut todo_list, mut handler) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        handler.set_priority(id, Priority::High, &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().priority, Priority::High);
    }

    #[test]
    fn test_edit_task() {
        let (mut todo_list, mut handler) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Old description".to_string()));
        
        handler.edit_task(id, "New description", &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().description, "New description");
    }
}
