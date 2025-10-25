use crate::models::task::Task;
use crate::models::priority::Priority;
use colored::*;
use chrono::NaiveDate;

/// Handles formatting of tasks for display
pub struct TaskFormatter;

impl TaskFormatter {
    /// Formats a task for display with colored output
    ///
    /// # Arguments
    ///
    /// * `task` - The task to format
    /// * `id_width` - The width to use for ID alignment
    ///
    /// # Returns
    ///
    /// A formatted string ready for display
    pub fn format_task(task: &Task, id_width: usize) -> String {
        let status_symbol = Self::format_status_symbol(task.is_completed());
        let colored_priority = Self::format_priority(task.priority);
        let task_id_formatted = format!("{:>width$}", task.id, width = id_width);
        
        let description_color = if task.is_completed() {
            task.description.bright_black()
        } else {
            task.description.white()
        };
        
        let due_date_str = if let Some(due_date) = task.due_date {
            format!(" {}", Self::format_due_date(due_date))
        } else {
            String::new()
        };
        
        let category_str = if let Some(category) = &task.category {
            format!(" {}", Self::format_category(category))
        } else {
            String::new()
        };
        
        let recurrence_str = if let Some(recurrence) = task.recurrence {
            format!(" {}", Self::format_recurrence(recurrence))
        } else {
            String::new()
        };
        
        let dependency_str = if !task.depends_on.is_empty() {
            format!(" {}", Self::format_dependencies(&task.depends_on))
        } else {
            String::new()
        };
        
        format!("{}. {} {} {}{}{}{}{}", 
            task_id_formatted.bright_blue(), 
            status_symbol, 
            colored_priority,
            description_color,
            due_date_str,
            category_str,
            recurrence_str,
            dependency_str)
    }
    
    /// Formats dependency information for display
    fn format_dependencies(dependencies: &[usize]) -> ColoredString {
        let dep_str = if dependencies.len() == 1 {
            format!("🔒 depends on: {}", dependencies[0])
        } else {
            format!("🔒 depends on: {}", dependencies.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", "))
        };
        dep_str.bright_yellow()
    }
    
    /// Formats a category badge for display
    fn format_category(category: &str) -> ColoredString {
        format!("[{}]", category).bright_magenta()
    }
    
    /// Formats a recurrence indicator for display
    fn format_recurrence(recurrence: crate::models::recurrence::Recurrence) -> ColoredString {
        format!("{} {}", crate::models::recurrence::Recurrence::icon(), recurrence.as_str()).bright_cyan()
    }
    
    /// Formats a due date with color coding based on how soon it's due
    fn format_due_date(due_date: NaiveDate) -> ColoredString {
        let today = chrono::Local::now().date_naive();
        let days_until_due = (due_date - today).num_days();
        
        let date_str = format!("(due: {})", due_date.format("%d.%m.%Y"));
        
        if days_until_due < 0 {
            // Overdue - red
            date_str.bright_red().bold()
        } else if days_until_due == 0 {
            // Due today - yellow
            date_str.bright_yellow().bold()
        } else if days_until_due <= 3 {
            // Due soon (within 3 days) - orange/yellow
            date_str.yellow()
        } else {
            // Future due date - cyan
            date_str.cyan()
        }
    }
    
    /// Formats the status symbol for a task
    fn format_status_symbol(is_completed: bool) -> String {
        if is_completed {
            format!("[{}]", "✓".bright_green().bold())
        } else {
            format!("[{}]", " ".white())
        }
    }
    
    /// Formats a priority with appropriate color
    pub fn format_priority(priority: Priority) -> ColoredString {
        let symbol = priority.symbol();
        match priority {
            Priority::High => symbol.bright_red().bold(),
            Priority::Medium => symbol.bright_yellow().bold(),
            Priority::Low => symbol.bright_blue().bold(),
        }
    }
    
    /// Formats a priority with its name and color
    pub fn format_priority_with_name(priority: Priority) -> ColoredString {
        let formatted = format!("{} {}", priority.symbol(), priority.as_str());
        match priority {
            Priority::High => formatted.bright_red().bold(),
            Priority::Medium => formatted.bright_yellow().bold(),
            Priority::Low => formatted.bright_blue().bold(),
        }
    }
    
    /// Calculates the maximum ID width for a list of tasks
    pub fn calculate_max_id_width(tasks: &[&Task]) -> usize {
        tasks.iter()
            .map(|t| t.id.to_string().len())
            .max()
            .unwrap_or(1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup() {
        colored::control::set_override(false);
    }
    
    #[test]
    fn test_format_status_symbol_completed() {
        setup();
        let result = TaskFormatter::format_status_symbol(true);
        assert!(result.contains("✓"));
    }
    
    #[test]
    fn test_format_status_symbol_pending() {
        setup();
        let result = TaskFormatter::format_status_symbol(false);
        assert!(result.contains("[ ]"));
    }
    
    #[test]
    fn test_calculate_max_id_width_single_digit() {
        let task1 = Task { id: 1, description: "Test".to_string(), completed: false, priority: Priority::Medium, due_date: None, category: None, parent_id: None, recurrence: None, depends_on: Vec::new() };
        let task2 = Task { id: 5, description: "Test".to_string(), completed: false, priority: Priority::Medium, due_date: None, category: None, parent_id: None, recurrence: None, depends_on: Vec::new() };
        let tasks = vec![&task1, &task2];
        
        assert_eq!(TaskFormatter::calculate_max_id_width(&tasks), 1);
    }
    
    #[test]
    fn test_calculate_max_id_width_mixed() {
        let task1 = Task { id: 9, description: "Test".to_string(), completed: false, priority: Priority::Medium, due_date: None, category: None, parent_id: None, recurrence: None, depends_on: Vec::new() };
        let task2 = Task { id: 10, description: "Test".to_string(), completed: false, priority: Priority::Medium, due_date: None, category: None, parent_id: None, recurrence: None, depends_on: Vec::new() };
        let task3 = Task { id: 100, description: "Test".to_string(), completed: false, priority: Priority::Medium, due_date: None, category: None, parent_id: None, recurrence: None, depends_on: Vec::new() };
        let tasks = vec![&task1, &task2, &task3];
        
        assert_eq!(TaskFormatter::calculate_max_id_width(&tasks), 3);
    }
}
