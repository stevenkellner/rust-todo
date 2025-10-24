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
pub struct TaskCommandHandler {
}

impl TaskCommandHandler {
    /// Creates a new task command handler.
    pub fn new() -> Self {
        TaskCommandHandler {}
    }

    /// Handles a task command
    pub fn handle<W: Write>(
        &self,
        command: &TaskCommand,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        match command {
            TaskCommand::Add(description) => {
                let new_task = TaskWithoutId::new(description.clone());
                let task_id = todo_list.add_task(new_task);
                output.show_task_added(task_id, description);
            }
            TaskCommand::List(filter) => self.list_tasks(filter, todo_list, output),
            TaskCommand::Remove(id) => self.remove_task(*id, todo_list, output),
            TaskCommand::Complete(id) => self.complete_task(*id, todo_list, output),
            TaskCommand::Uncomplete(id) => self.uncomplete_task(*id, todo_list, output),
            TaskCommand::Toggle(id) => self.toggle_task(*id, todo_list, output),
            TaskCommand::SetPriority(id, priority) => self.set_priority(*id, *priority, todo_list, output),
            TaskCommand::SetDueDate(id, due_date) => self.set_due_date(*id, *due_date, todo_list, output),
            TaskCommand::SetCategory(id, category) => self.set_category(*id, category.clone(), todo_list, output),
            TaskCommand::ListCategories => {
                output.show_categories(todo_list);
            }
            TaskCommand::Edit(id, new_description) => self.edit_task(*id, new_description, todo_list, output),
            TaskCommand::Search(keyword) => {
                output.show_search_results(todo_list, keyword);
            }
            TaskCommand::ShowStatistics => {
                output.show_statistics(todo_list);
            }
        }
    }

    /// Lists tasks with optional filtering
    fn list_tasks<W: Write>(
        &self,
        filter: &Option<TaskFilter>,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        match filter {
            None => output.show_all_tasks(todo_list.get_tasks()),
            Some(task_filter) => {
                let filtered_tasks = todo_list.get_filtered_tasks(task_filter);
                
                if task_filter.status == Some(TaskStatus::Completed) && task_filter.priority.is_none() {
                    output.show_completed_tasks(&filtered_tasks);
                } else if task_filter.status == Some(TaskStatus::Pending) && task_filter.priority.is_none() {
                    output.show_pending_tasks(&filtered_tasks);
                } else if let Some(priority) = task_filter.priority {
                    output.show_tasks_by_priority(&filtered_tasks, priority);
                } else {
                    output.show_filtered_tasks(&filtered_tasks, task_filter);
                }
            }
        }
    }

    /// Removes a task by ID.
    fn remove_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        if let Some(task) = todo_list.remove_task(id) {
            output.show_task_removed(&task.description);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Marks a task as completed.
    fn complete_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
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
    fn uncomplete_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
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
    fn toggle_task<W: Write>(
        &self,
        id: usize,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        if let Some(task) = todo_list.toggle_task(id) {
            output.show_task_toggled(&task.description, task.is_completed());
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the priority of a task.
    fn set_priority<W: Write>(
        &self,
        id: usize,
        priority: Priority,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_task_priority(id, priority) {
            output.show_priority_set(&task.description, priority);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the due date of a task.
    fn set_due_date<W: Write>(
        &self,
        id: usize,
        due_date: Option<NaiveDate>,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_due_date(id, due_date) {
            output.show_due_date_set(&task.description, due_date);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Sets the category of a task.
    fn set_category<W: Write>(
        &self,
        id: usize,
        category: Option<String>,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
    ) {
        if let Some(task) = todo_list.set_task_category(id, category.clone()) {
            output.show_category_set(&task.description, category);
        } else {
            output.show_task_not_found(id);
        }
    }

    /// Edits a task's description.
    fn edit_task<W: Write>(
        &self,
        id: usize,
        new_description: &str,
        todo_list: &mut TodoList,
        output: &mut TaskCommandOutputWriter<W>
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

    fn create_test_handler() -> (TodoList, TaskCommandOutputWriter<Vec<u8>>) {
        let todo_list = TodoList::new();
        let output = TaskCommandOutputWriter::with_writer(Vec::new());
        (todo_list, output)
    }

    #[test]
    fn test_remove_task_existing() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        let handler = TaskCommandHandler::new();
        handler.remove_task(id, &mut todo_list, &mut output);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).is_none());
    }

    #[test]
    fn test_complete_task() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        let handler = TaskCommandHandler::new();
        handler.complete_task(id, &mut todo_list, &mut output);
        
        assert!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().is_completed());
    }

    #[test]
    fn test_set_priority() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        let handler = TaskCommandHandler::new();
        handler.set_priority(id, Priority::High, &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().priority, Priority::High);
    }

    #[test]
    fn test_edit_task() {
        let (mut todo_list, mut output) = create_test_handler();
        let id = todo_list.add_task(TaskWithoutId::new("Old description".to_string()));
        
        let handler = TaskCommandHandler::new();
        handler.edit_task(id, "New description", &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().iter().find(|t| t.id == id).unwrap().description, "New description");
    }
}
