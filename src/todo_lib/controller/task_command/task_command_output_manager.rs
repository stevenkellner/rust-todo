use crate::ui::output::OutputWriter;
use crate::ui::formatters::{TaskFormatter, MessageFormatter};
use crate::models::todo_list::TodoList;
use crate::models::task::Task;
use crate::models::priority::Priority;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use chrono::NaiveDate;
use colored::*;
use std::rc::Rc;
use std::cell::RefCell;

/// Output manager specifically for task commands.
///
/// Handles all output operations related to task management commands
/// like add, remove, complete, edit, etc.
pub struct TaskCommandOutputManager<O: OutputWriter> {
    output_writer: Rc<RefCell<O>>,
}

impl<O: OutputWriter> TaskCommandOutputManager<O> {
    /// Creates a new TaskCommandOutputManager with a custom writer.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        TaskCommandOutputManager { output_writer }
    }

    /// Displays a success message after adding a task.
    pub fn show_task_added(&mut self, id: usize, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Task added with ID {}: '{}'", id, description));
    }

    /// Displays a success message after adding a subtask.
    pub fn show_subtask_added(&mut self, subtask_id: usize, parent_id: usize, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Subtask added with ID {} under parent task {}: '{}'", subtask_id, parent_id, description));
    }

    /// Displays a success message after removing a task.
    pub fn show_task_removed(&mut self, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Task removed: '{}'", description));
    }

    /// Displays a success message after completing a task.
    pub fn show_task_completed(&mut self, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Task '{}' marked as completed.", description));
    }

    /// Displays a success message after creating a recurring task instance.
    pub fn show_recurring_task_created(&mut self, id: usize, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Created recurring task with ID {}: '{}'", id, description));
    }

    /// Displays a success message after uncompleting a task.
    pub fn show_task_uncompleted(&mut self, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Task '{}' marked as pending.", description));
    }

    /// Displays a success message after completing multiple tasks.
    pub fn show_multiple_tasks_completed(&mut self, completed_count: usize, not_found: &[usize]) {
        if completed_count > 0 {
            let message = if completed_count == 1 {
                "Completed 1 task.".to_string()
            } else {
                format!("Completed {} tasks.", completed_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if completed_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to complete.");
        }
    }

    /// Displays a success message after completing all tasks.
    pub fn show_all_tasks_completed(&mut self, count: usize) {
        if count > 0 {
            let message = if count == 1 {
                "Completed 1 task.".to_string()
            } else {
                format!("Completed all {} tasks.", count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        } else {
            self.output_writer.borrow_mut().show_error("No tasks to complete.");
        }
    }

    /// Displays a success message after removing multiple tasks.
    pub fn show_multiple_tasks_removed(&mut self, removed_count: usize, not_found: &[usize]) {
        if removed_count > 0 {
            let message = if removed_count == 1 {
                "Removed 1 task.".to_string()
            } else {
                format!("Removed {} tasks.", removed_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if removed_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to remove.");
        }
    }

    /// Displays a success message after removing all tasks.
    pub fn show_all_tasks_removed(&mut self, count: usize) {
        if count > 0 {
            let message = if count == 1 {
                "Removed 1 task.".to_string()
            } else {
                format!("Removed all {} tasks.", count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        } else {
            self.output_writer.borrow_mut().show_error("No tasks to remove.");
        }
    }

    /// Displays a success message after marking multiple tasks as pending.
    pub fn show_multiple_tasks_uncompleted(&mut self, uncompleted_count: usize, not_found: &[usize]) {
        if uncompleted_count > 0 {
            let message = if uncompleted_count == 1 {
                "Marked 1 task as pending.".to_string()
            } else {
                format!("Marked {} tasks as pending.", uncompleted_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if uncompleted_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to mark as pending.");
        }
    }

    /// Displays a success message after marking all tasks as pending.
    pub fn show_all_tasks_uncompleted(&mut self, count: usize) {
        if count > 0 {
            let message = if count == 1 {
                "Marked 1 task as pending.".to_string()
            } else {
                format!("Marked all {} tasks as pending.", count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        } else {
            self.output_writer.borrow_mut().show_error("No tasks to mark as pending.");
        }
    }

    /// Displays a success message after toggling multiple tasks.
    pub fn show_multiple_tasks_toggled(&mut self, toggled_count: usize, not_found: &[usize]) {
        if toggled_count > 0 {
            let message = if toggled_count == 1 {
                "Toggled 1 task.".to_string()
            } else {
                format!("Toggled {} tasks.", toggled_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if toggled_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to toggle.");
        }
    }

    /// Displays a success message after toggling all tasks.
    pub fn show_all_tasks_toggled(&mut self, count: usize) {
        if count > 0 {
            let message = if count == 1 {
                "Toggled 1 task.".to_string()
            } else {
                format!("Toggled all {} tasks.", count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        } else {
            self.output_writer.borrow_mut().show_error("No tasks to toggle.");
        }
    }

    /// Displays a success message after setting priority for multiple tasks.
    pub fn show_multiple_priorities_set(&mut self, updated_count: usize, priority: Priority, not_found: &[usize]) {
        use crate::ui::formatters::TaskFormatter;
        let colored_priority = TaskFormatter::format_priority_with_name(priority);
        
        if updated_count > 0 {
            let message = if updated_count == 1 {
                format!("Set priority to {} for 1 task.", colored_priority)
            } else {
                format!("Set priority to {} for {} tasks.", colored_priority, updated_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if updated_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to update.");
        }
    }

    /// Displays a success message after setting category for multiple tasks.
    pub fn show_multiple_categories_set(&mut self, updated_count: usize, category: Option<&str>, not_found: &[usize]) {
        if updated_count > 0 {
            let message = if let Some(cat) = category {
                if updated_count == 1 {
                    format!("Set category to '{}' for 1 task.", cat)
                } else {
                    format!("Set category to '{}' for {} tasks.", cat, updated_count)
                }
            } else if updated_count == 1 {
                "Cleared category for 1 task.".to_string()
            } else {
                format!("Cleared category for {} tasks.", updated_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if updated_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to update.");
        }
    }

    /// Displays a success message after updating a task.
    pub fn show_task_updated(&mut self, old_desc: &str, new_desc: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Task '{}' updated to '{}'.", old_desc, new_desc));
    }

    /// Displays a success message after updating a task description.
    pub fn show_task_edited(&mut self, old_desc: &str, new_desc: &str) {
        self.show_task_updated(old_desc, new_desc);
    }

    /// Displays a success message after toggling a task.
    pub fn show_task_toggled(&mut self, description: &str, is_completed: bool) {
        if is_completed {
            self.show_task_completed(description);
        } else {
            self.show_task_uncompleted(description);
        }
    }

    /// Displays a success message after setting priority.
    pub fn show_priority_set(&mut self, description: &str, priority: Priority) {
        use crate::ui::formatters::TaskFormatter;
        let colored_priority = TaskFormatter::format_priority_with_name(priority);
        self.output_writer.borrow_mut().show_success(&format!("Priority set to {} for task: '{}'", colored_priority, description));
    }

    /// Displays a success message after setting due date.
    pub fn show_due_date_set(&mut self, description: &str, due_date: Option<NaiveDate>) {
        let message = if let Some(date) = due_date {
            format!("Due date set to {} for task: '{}'", date.format("%d.%m.%Y"), description)
        } else {
            format!("Due date cleared for task: '{}'", description)
        };
        self.output_writer.borrow_mut().show_success(&message);
    }

    /// Displays a success message after clearing due date.
    pub fn show_due_date_cleared(&mut self, description: &str) {
        self.output_writer.borrow_mut().show_success(&format!("Due date cleared for task: '{}'", description));
    }

    /// Displays a success message after setting category.
    pub fn show_category_set(&mut self, description: &str, category: Option<String>) {
        let message = if let Some(cat) = category {
            format!("Category set to '{}' for task: '{}'", cat, description)
        } else {
            format!("Category cleared for task: '{}'", description)
        };
        self.output_writer.borrow_mut().show_success(&message);
    }

    /// Displays a success message after clearing category.
    pub fn show_category_cleared(&mut self, description: &str) {
        self.show_category_set(description, None);
    }

    /// Displays a success message after setting recurrence.
    pub fn show_recurrence_set(&mut self, description: &str, recurrence: Option<crate::models::Recurrence>) {
        let message = if let Some(rec) = recurrence {
            format!("Recurrence set to '{}' for task: '{}'", rec.as_str(), description)
        } else {
            format!("Recurrence cleared for task: '{}'", description)
        };
        self.output_writer.borrow_mut().show_success(&message);
    }

    /// Displays a success message after setting recurrence for multiple tasks.
    pub fn show_multiple_recurrences_set(&mut self, updated_count: usize, recurrence: Option<crate::models::Recurrence>, not_found: &[usize]) {
        if updated_count > 0 {
            let message = if let Some(rec) = recurrence {
                if updated_count == 1 {
                    format!("Set recurrence to '{}' for 1 task.", rec.as_str())
                } else {
                    format!("Set recurrence to '{}' for {} tasks.", rec.as_str(), updated_count)
                }
            } else if updated_count == 1 {
                "Cleared recurrence for 1 task.".to_string()
            } else {
                format!("Cleared recurrence for {} tasks.", updated_count)
            };
            self.output_writer.borrow_mut().show_success(&message);
        }
        
        if !not_found.is_empty() {
            let ids = not_found.iter()
                .map(|id| id.to_string())
                .collect::<Vec<_>>()
                .join(", ");
            self.output_writer.borrow_mut().show_error(&format!("Tasks with IDs {} not found.", ids));
        }
        
        if updated_count == 0 && not_found.is_empty() {
            self.output_writer.borrow_mut().show_error("No tasks to update.");
        }
    }

    /// Displays an error when a task is not found.
    pub fn show_task_not_found(&mut self, id: usize) {
        self.output_writer.borrow_mut().show_error(&format!("Task with ID {} not found.", id));
    }

    /// Displays an error for invalid priority.
    pub fn show_invalid_priority(&mut self) {
        self.output_writer.borrow_mut().show_error("Invalid priority. Use: high, medium, or low");
    }

    /// Displays an error for invalid date format.
    pub fn show_invalid_date_format(&mut self) {
        self.output_writer.borrow_mut().show_error("Invalid date format. Use DD.MM.YYYY");
    }

    /// Displays a generic error message.
    pub fn show_error(&mut self, message: &str) {
        self.output_writer.borrow_mut().show_error(message);
    }

    /// Displays a list of tasks.
    pub fn show_task_list(&mut self, todo_list: &TodoList, filter: &TaskFilter) {
        let tasks = todo_list.get_filtered_tasks(filter);
        self.show_filtered_tasks(&tasks, filter);
    }

    /// Displays search results.
    pub fn show_search_results(&mut self, todo_list: &TodoList, keyword: &str) {
        let tasks = todo_list.search_tasks(keyword);
        self.show_search_results_internal(&tasks, keyword);
    }

    /// Displays all categories.
    pub fn show_categories(&mut self, todo_list: &TodoList) {
        let categories = todo_list.get_all_categories();
        self.show_categories_internal(&categories);
    }

    /// Displays task statistics.
    pub fn show_statistics(&mut self, todo_list: &TodoList) {
        let stats = todo_list.get_statistics();
        self.show_statistics_internal(&stats);
    }

    /// Helper method to display a list of tasks with a given title.
    fn show_task_list_internal(&mut self, title: &str, tasks: Vec<&Task>) {
        let separator_length = title.len() + 8;
        let max_id_width = TaskFormatter::calculate_max_id_width(&tasks);
        
        self.output_writer.borrow_mut().write_line(&format!("\n{}", MessageFormatter::section_title(title)));
        
        for task in &tasks {
            let formatted_task = TaskFormatter::format_task(task, max_id_width);
            self.output_writer.borrow_mut().write_line(&formatted_task);
        }
        
        self.output_writer.borrow_mut().write_line(&format!("{}\n", MessageFormatter::separator(separator_length)));
    }

    /// Displays a list of all tasks.
    pub fn show_all_tasks(&mut self, tasks: &[Task]) {
        if tasks.is_empty() {
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning("No tasks found. Use 'add <description>' to create a task."));
            return;
        }

        self.show_task_list_internal("All Tasks", tasks.iter().collect());
    }

    /// Displays all tasks hierarchically with subtasks indented under their parents.
    pub fn show_all_tasks_hierarchical(&mut self, todo_list: &TodoList) {
        let tasks = todo_list.get_tasks();
        
        if tasks.is_empty() {
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning("No tasks found. Use 'add <description>' to create a task."));
            return;
        }

        let max_id_width = TaskFormatter::calculate_max_id_width(&tasks.iter().collect::<Vec<_>>());
        
        self.output_writer.borrow_mut().write_line(&format!("\n{}", MessageFormatter::section_title("All Tasks")));
        
        // Display only top-level tasks (tasks without a parent)
        for task in tasks {
            if !task.is_subtask() {
                // Display parent task
                let formatted_task = TaskFormatter::format_task(task, max_id_width);
                
                // Add subtask progress if task has subtasks
                let subtask_count = todo_list.get_subtask_count(task.id);
                if subtask_count > 0 {
                    let completed_count = todo_list.get_completed_subtask_count(task.id);
                    let progress = format!(" ({}/{} subtasks)", completed_count, subtask_count).bright_cyan();
                    self.output_writer.borrow_mut().write_line(&format!("{}{}", formatted_task, progress));
                } else {
                    self.output_writer.borrow_mut().write_line(&formatted_task);
                }
                
                // Display subtasks indented
                let subtasks = todo_list.get_subtasks(task.id);
                for subtask in subtasks {
                    let formatted_subtask = TaskFormatter::format_task(subtask, max_id_width);
                    self.output_writer.borrow_mut().write_line(&format!("  ↳ {}", formatted_subtask));
                }
            }
        }
        
        self.output_writer.borrow_mut().write_line(&format!("{}\n", MessageFormatter::separator(18)));
    }

    /// Displays a list of completed tasks.
    pub fn show_completed_tasks(&mut self, tasks: &[&Task]) {
        if tasks.is_empty() {
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning("No completed tasks found."));
            return;
        }

        self.show_task_list_internal("Completed Tasks", tasks.to_vec());
    }

    /// Displays a list of pending tasks.
    pub fn show_pending_tasks(&mut self, tasks: &[&Task]) {
        if tasks.is_empty() {
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning("No pending tasks found."));
            return;
        }

        self.show_task_list_internal("Pending Tasks", tasks.to_vec());
    }

    /// Displays tasks filtered by priority level.
    pub fn show_tasks_by_priority(&mut self, tasks: &[&Task], priority: Priority) {
        if tasks.is_empty() {
            let message = format!("No {} priority tasks found.", priority.as_str());
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning(&message));
            return;
        }

        let title = format!("{} Priority Tasks", priority.as_str());
        self.show_task_list_internal(&title, tasks.to_vec());
    }

    /// Displays tasks filtered by both status and priority.
    pub fn show_filtered_tasks(&mut self, tasks: &[&Task], filter: &TaskFilter) {
        if tasks.is_empty() {
            let status_str = match filter.status {
                Some(TaskStatus::Completed) => "completed ",
                Some(TaskStatus::Pending) => "pending ",
                None => "",
            };
            let priority_str = match filter.priority {
                Some(priority) => format!("{} priority ", priority.as_str()),
                None => String::new(),
            };
            let message = format!("No {}{}tasks found.", status_str, priority_str);
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning(&message));
            return;
        }

        // Build title based on filter
        let mut title_parts = Vec::new();
        if let Some(priority) = filter.priority {
            title_parts.push(format!("{} Priority", priority.as_str()));
        }
        if let Some(status) = filter.status {
            match status {
                TaskStatus::Completed => title_parts.push("Completed".to_string()),
                TaskStatus::Pending => title_parts.push("Pending".to_string()),
            }
        }
        
        let title = if title_parts.is_empty() {
            "Tasks".to_string()
        } else {
            format!("{} Tasks", title_parts.join(" "))
        };
        
        self.show_task_list_internal(&title, tasks.to_vec());
    }

    /// Displays search results for tasks matching a keyword.
    fn show_search_results_internal(&mut self, tasks: &[&Task], keyword: &str) {
        if tasks.is_empty() {
            let message = format!("No tasks found matching '{}'.", keyword);
            self.output_writer.borrow_mut().write_line(&MessageFormatter::warning(&message));
            return;
        }

        let title = format!("Search Results for '{}'", keyword);
        self.show_task_list_internal(&title, tasks.to_vec());
    }

    /// Displays task statistics.
    fn show_statistics_internal(&mut self, stats: &crate::models::TaskStatistics) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&MessageFormatter::section_title("Task Statistics"));
        self.output_writer.borrow_mut().write_line("");
        
        // Overall statistics
        self.output_writer.borrow_mut().write_line(&format!("  {}: {}", 
            "Total Tasks".bright_white().bold(), 
            stats.total));
        self.output_writer.borrow_mut().write_line(&format!("  {}: {}", 
            "Completed".bright_white().bold(), 
            stats.completed.to_string().green()));
        self.output_writer.borrow_mut().write_line(&format!("  {}: {}", 
            "Pending".bright_white().bold(), 
            stats.pending.to_string().yellow()));
        self.output_writer.borrow_mut().write_line(&format!("  {}: {:.1}%", 
            "Completion".bright_white().bold(), 
            stats.completion_percentage));
        
        // Priority breakdown
        if stats.total > 0 {
            self.output_writer.borrow_mut().write_line("");
            self.output_writer.borrow_mut().write_line(&format!("  {}", "By Priority:".bright_cyan()));
            self.output_writer.borrow_mut().write_line(&format!("    {} {}", 
                "▲ High:  ".red(), 
                stats.high_priority));
            self.output_writer.borrow_mut().write_line(&format!("    {} {}", 
                "■ Medium:".yellow(), 
                stats.medium_priority));
            self.output_writer.borrow_mut().write_line(&format!("    {} {}", 
                "▼ Low:   ".blue(), 
                stats.low_priority));
        }
        
        self.output_writer.borrow_mut().write_line("");
    }

    /// Displays all unique categories.
    fn show_categories_internal(&mut self, categories: &[String]) {
        self.output_writer.borrow_mut().write_line("");
        if categories.is_empty() {
            self.output_writer.borrow_mut().write_line(&"↻ No categories found.".bright_yellow().to_string());
        } else {
            self.output_writer.borrow_mut().write_line(&"--- All Categories ---".bright_cyan().bold().to_string());
            self.output_writer.borrow_mut().write_line("");
            for category in categories {
                self.output_writer.borrow_mut().write_line(&format!("  {}", category.bright_magenta()));
            }
            self.output_writer.borrow_mut().write_line("");
            self.output_writer.borrow_mut().write_line(&format!("Total: {} categories", categories.len()).bright_black().to_string());
        }
        self.output_writer.borrow_mut().write_line("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::TaskStatistics;
    use crate::models::overdue_filter::OverdueFilter;
    use crate::ui::output::FileOutputWriter;

    // Disable colors for all tests
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_task_output_writer_task_added() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_added(1, "Test task");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Task added with ID 1"));
        assert!(output.contains("Test task"));
    }

    #[test]
    fn test_task_output_writer_task_removed() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_removed("Completed task");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Task removed"));
        assert!(output.contains("Completed task"));
    }

    #[test]
    fn test_task_output_writer_task_completed() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_completed("Finish homework");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("marked as completed"));
        assert!(output.contains("Finish homework"));
    }

    #[test]
    fn test_task_output_writer_task_uncompleted() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_uncompleted("Review code");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("marked as pending"));
        assert!(output.contains("Review code"));
    }

    #[test]
    fn test_task_output_writer_task_edited() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_edited("Old description", "New description");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Old description"));
        assert!(output.contains("New description"));
        assert!(output.contains("updated"));
    }

    #[test]
    fn test_task_output_writer_task_toggled_completed() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_toggled("Test task", true);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("marked as completed"));
    }

    #[test]
    fn test_task_output_writer_task_toggled_pending() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_toggled("Test task", false);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("marked as pending"));
    }

    #[test]
    fn test_task_output_writer_priority_set() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_priority_set("Important task", Priority::High);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Priority set"));
        assert!(output.contains("Important task"));
    }

    #[test]
    fn test_task_output_writer_due_date_set() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        let date = chrono::NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
        writer.show_due_date_set("Task with deadline", Some(date));
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Due date set"));
        assert!(output.contains("Task with deadline"));
        assert!(output.contains("31.12.2024"));
    }

    #[test]
    fn test_task_output_writer_due_date_cleared() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_due_date_set("Task without deadline", None);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Due date cleared"));
        assert!(output.contains("Task without deadline"));
    }

    #[test]
    fn test_task_output_writer_category_set() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_category_set("Work task", Some("Work".to_string()));
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Category set"));
        assert!(output.contains("Work"));
        assert!(output.contains("Work task"));
    }

    #[test]
    fn test_task_output_writer_category_cleared() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_category_set("Uncategorized task", None);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Category cleared"));
        assert!(output.contains("Uncategorized task"));
    }

    #[test]
    fn test_task_output_writer_task_not_found() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_task_not_found(42);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Task with ID 42 not found"));
    }

    #[test]
    fn test_task_output_writer_invalid_priority() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_invalid_priority();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Invalid priority"));
    }

    #[test]
    fn test_task_output_writer_invalid_date_format() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_invalid_date_format();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Invalid date format"));
    }

    #[test]
    fn test_task_output_writer_show_error() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_error("Custom error message");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Custom error message"));
    }

    #[test]
    fn test_task_output_writer_all_tasks_empty() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_all_tasks(&[]);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("No tasks found"));
    }

    #[test]
    fn test_task_output_writer_all_tasks_with_data() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        let tasks = vec![
            Task::new(1, "Task 1".to_string()),
            Task::new(2, "Task 2".to_string()),
        ];
        
        writer.show_all_tasks(&tasks);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("All Tasks"));
        assert!(output.contains("Task 1"));
        assert!(output.contains("Task 2"));
    }

    #[test]
    fn test_task_output_writer_filtered_tasks_empty() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        let filter = TaskFilter::new(None, None, OverdueFilter::All);
        writer.show_filtered_tasks(&[], &filter);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("No") && output.contains("tasks found"));
    }

    #[test]
    fn test_task_output_writer_statistics() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        let stats = TaskStatistics {
            total: 10,
            completed: 3,
            pending: 7,
            completion_percentage: 30.0,
            high_priority: 2,
            medium_priority: 5,
            low_priority: 3,
        };
        
        writer.show_statistics_internal(&stats);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Task Statistics"));
        assert!(output.contains("Total Tasks"));
        assert!(output.contains("10"));
        assert!(output.contains("Completed"));
        assert!(output.contains("3"));
        assert!(output.contains("Pending"));
        assert!(output.contains("7"));
        assert!(output.contains("30.0"));
    }

    #[test]
    fn test_task_output_writer_categories_empty() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        writer.show_categories_internal(&[]);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("No categories found"));
    }

    #[test]
    fn test_task_output_writer_categories_with_data() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = TaskCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));
        
        let categories = vec!["Work".to_string(), "Personal".to_string(), "Shopping".to_string()];
        writer.show_categories_internal(&categories);
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("All Categories"));
        assert!(output.contains("Work"));
        assert!(output.contains("Personal"));
        assert!(output.contains("Shopping"));
        assert!(output.contains("Total: 3 categories"));
    }
}
