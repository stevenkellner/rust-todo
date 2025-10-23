use crate::models::todo_list::TodoList;
use crate::models::priority::Priority;
use crate::ui::output_writer::OutputWriter;
use rand::Rng;
use std::io::Write;

/// Controller for debug commands and operations
pub struct DebugController {
    /// Flag to track if debug mode is enabled
    debug_mode: bool,
}

impl DebugController {
    /// Creates a new DebugController
    pub fn new() -> Self {
        Self {
            debug_mode: false,
        }
    }
    
    /// Checks if debug mode is currently enabled
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }
    
    /// Toggles debug mode on/off
    pub fn toggle_debug_mode<W: Write>(&mut self, output: &mut OutputWriter<W>) {
        self.debug_mode = !self.debug_mode;
        if self.debug_mode {
            output.show_success("Debug mode enabled");
        } else {
            output.show_success("Debug mode disabled");
        }
    }
    
    /// Generates random tasks for testing
    ///
    /// # Arguments
    ///
    /// * `count` - Number of random tasks to generate
    /// * `todo_list` - The todo list to add tasks to
    /// * `output` - Output writer for displaying results
    pub fn generate_random_tasks<W: Write>(
        &self,
        count: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if !self.debug_mode {
            output.show_error("Debug mode is not enabled. Use 'debug:toggle' to enable it.");
            return;
        }
        
        let mut rng = rand::rng();
        let task_templates = [
            "Buy groceries",
            "Write documentation",
            "Review pull requests",
            "Update dependencies",
            "Fix bug in authentication",
            "Implement new feature",
            "Refactor legacy code",
            "Write unit tests",
            "Deploy to production",
            "Meeting with team",
            "Code review session",
            "Update README",
            "Optimize database queries",
            "Design new UI",
            "Research new technology",
            "Client presentation",
            "Performance testing",
            "Security audit",
            "Backup database",
            "Configure CI/CD pipeline",
        ];
        
        let priorities = [Priority::High, Priority::Medium, Priority::Low];
        
        let categories = [
            "work",
            "personal",
            "urgent",
            "bug",
            "feature",
            "documentation",
            "testing",
            "deployment",
            "maintenance",
            "research",
        ];
        
        for _ in 0..count {
            let template_idx = rng.random_range(0..task_templates.len());
            let description = task_templates[template_idx].to_string();
            let priority_idx = rng.random_range(0..priorities.len());
            let priority = priorities[priority_idx];
            
            let task_id = todo_list.add_task(description.clone());
            let _ = todo_list.set_task_priority(task_id, priority);
            
            // Randomly complete some tasks (~30% chance)
            if rng.random_bool(0.3) {
                let _ = todo_list.complete_task(task_id);
            }
            
            // Randomly add due dates (~60% chance)
            if rng.random_bool(0.6) {
                let today = chrono::Local::now().date_naive();
                // Generate due dates from -7 days to +30 days
                let days_offset = rng.random_range(-7..31);
                let due_date = today + chrono::Duration::days(days_offset);
                let _ = todo_list.set_due_date(task_id, Some(due_date));
            }
            
            // Randomly add categories (~70% chance)
            if rng.random_bool(0.7) {
                let category_idx = rng.random_range(0..categories.len());
                let category = Some(categories[category_idx].to_string());
                let _ = todo_list.set_task_category(task_id, category);
            }
        }
        
        output.show_success(&format!("Generated {} random tasks", count));
    }
    
    /// Clears all tasks from the list
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The todo list to clear
    /// * `output` - Output writer for displaying results
    pub fn clear_all_tasks<W: Write>(
        &self,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if !self.debug_mode {
            output.show_error("Debug mode is not enabled. Use 'debug:toggle' to enable it.");
            return;
        }
        
        let count = todo_list.get_tasks().len();
        todo_list.clear_all();
        output.show_success(&format!("Cleared {} tasks", count));
    }
}

impl Default for DebugController {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_new_debug_controller() {
        let controller = DebugController::new();
        assert!(!controller.is_debug_mode());
    }
    
    #[test]
    fn test_toggle_debug_mode() {
        let mut controller = DebugController::new();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        assert!(!controller.is_debug_mode());
        
        controller.toggle_debug_mode(&mut output);
        assert!(controller.is_debug_mode());
        
        controller.toggle_debug_mode(&mut output);
        assert!(!controller.is_debug_mode());
    }
    
    #[test]
    fn test_generate_random_tasks_without_debug_mode() {
        let controller = DebugController::new();
        let mut todo_list = TodoList::new();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.generate_random_tasks(5, &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Debug mode is not enabled"));
    }
    
    #[test]
    fn test_generate_random_tasks_with_debug_mode() {
        let mut controller = DebugController::new();
        let mut todo_list = TodoList::new();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.toggle_debug_mode(&mut output);
        controller.generate_random_tasks(10, &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks_without_debug_mode() {
        let controller = DebugController::new();
        let mut todo_list = TodoList::new();
        todo_list.add_task("Test task".to_string());
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.clear_all_tasks(&mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 1);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Debug mode is not enabled"));
    }
    
    #[test]
    fn test_clear_all_tasks_with_debug_mode() {
        let mut controller = DebugController::new();
        let mut todo_list = TodoList::new();
        todo_list.add_task("Test task 1".to_string());
        todo_list.add_task("Test task 2".to_string());
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.toggle_debug_mode(&mut output);
        controller.clear_all_tasks(&mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
