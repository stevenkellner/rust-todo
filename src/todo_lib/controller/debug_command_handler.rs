use crate::models::todo_list::TodoList;
use crate::models::debug_command::DebugCommand;
use crate::controller::task_generator::RandomTaskGenerator;
use crate::ui::DebugCommandOutputWriter;
use std::io::Write;

/// Handler for debug commands and operations
pub struct DebugCommandHandler<W: Write> {
    /// Flag to track if debug mode is enabled
    debug_mode: bool,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
    /// Output writer for displaying results
    output: DebugCommandOutputWriter<W>,
}

impl DebugCommandHandler<std::io::Stdout> {
    /// Creates a new DebugCommandHandler with stdout
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            task_generator: RandomTaskGenerator::new(),
            output: DebugCommandOutputWriter::new(),
        }
    }
}

impl<W: Write> DebugCommandHandler<W> {
    /// Creates a new DebugCommandHandler with a custom output writer
    pub fn with_writer(writer: W) -> Self {
        Self {
            debug_mode: false,
            task_generator: RandomTaskGenerator::new(),
            output: DebugCommandOutputWriter::with_writer(writer),
        }
    }
    
    /// Checks if debug mode is currently enabled
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }
    
    /// Handles a debug command
    pub fn handle(
        &mut self,
        command: &DebugCommand,
        todo_list: &mut TodoList,
    ) {
        match command {
            DebugCommand::GenerateTasks(count) => self.generate_random_tasks(*count, todo_list),
            DebugCommand::ClearAll => self.clear_all_tasks(todo_list),
            DebugCommand::Toggle => self.toggle_debug_mode(),
        }
    }
    
    /// Toggles debug mode on/off
    fn toggle_debug_mode(&mut self) {
        self.debug_mode = !self.debug_mode;
        if self.debug_mode {
            self.output.show_success("Debug mode enabled");
        } else {
            self.output.show_success("Debug mode disabled");
        }
    }
    
    /// Generates random tasks for testing
    ///
    /// # Arguments
    ///
    /// * `count` - Number of random tasks to generate
    /// * `todo_list` - The todo list to add tasks to
    fn generate_random_tasks(
        &mut self,
        count: usize,
        todo_list: &mut TodoList,
    ) {
        if !self.debug_mode {
            self.output.show_error("Debug mode is not enabled. Use 'debug:toggle' to enable it.");
            return;
        }
        
        // Generate random tasks
        let new_tasks = self.task_generator.generate(count);
        
        // Add each generated task to the todo list
        for new_task in new_tasks {
            let _ = todo_list.add_task(new_task);
        }
        
        self.output.show_success(&format!("Generated {} random tasks", count));
    }
    
    /// Clears all tasks from the todo list
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The todo list to clear
    fn clear_all_tasks(
        &mut self,
        todo_list: &mut TodoList,
    ) {
        if !self.debug_mode {
            self.output.show_error("Debug mode is not enabled. Use 'debug:toggle' to enable it.");
            return;
        }
        
        let count = todo_list.get_tasks().len();
        todo_list.clear_all();
        self.output.show_success(&format!("Cleared {} tasks", count));
    }
}

impl Default for DebugCommandHandler<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;
    
    #[test]
    fn test_new_debug_controller() {
        let controller = DebugCommandHandler::new();
        assert!(!controller.is_debug_mode());
    }
    
    #[test]
    fn test_toggle_debug_mode() {
        let mut controller = DebugCommandHandler::with_writer(Vec::new());
        
        assert!(!controller.is_debug_mode());
        
        controller.toggle_debug_mode();
        assert!(controller.is_debug_mode());
        
        controller.toggle_debug_mode();
        assert!(!controller.is_debug_mode());
    }
    
    #[test]
    fn test_generate_random_tasks_without_debug_mode() {
        let mut controller = DebugCommandHandler::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        
        controller.generate_random_tasks(5, &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
    
    #[test]
    fn test_generate_random_tasks_with_debug_mode() {
        let mut controller = DebugCommandHandler::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        
        controller.toggle_debug_mode();
        controller.generate_random_tasks(10, &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks_without_debug_mode() {
        let mut controller = DebugCommandHandler::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        
        controller.clear_all_tasks(&mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 1);
    }
    
    #[test]
    fn test_clear_all_tasks_with_debug_mode() {
        let mut controller = DebugCommandHandler::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task 1".to_string()));
        todo_list.add_task(TaskWithoutId::new("Test task 2".to_string()));
        
        controller.toggle_debug_mode();
        controller.clear_all_tasks(&mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
