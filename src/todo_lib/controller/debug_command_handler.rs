use crate::models::todo_list::TodoList;
use crate::models::debug_command::DebugCommand;
use crate::controller::task_generator::RandomTaskGenerator;
use crate::ui::output_writer::OutputWriter;
use std::io::Write;

/// Handler for debug commands and operations
pub struct DebugCommandHandler {
    /// Flag to track if debug mode is enabled
    debug_mode: bool,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
}

impl DebugCommandHandler {
    /// Creates a new DebugCommandHandler
    pub fn new() -> Self {
        Self {
            debug_mode: false,
            task_generator: RandomTaskGenerator::new(),
        }
    }
    
    /// Checks if debug mode is currently enabled
    pub fn is_debug_mode(&self) -> bool {
        self.debug_mode
    }
    
    /// Handles a debug command
    pub fn handle<W: Write>(
        &mut self,
        command: &DebugCommand,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        match command {
            DebugCommand::GenerateTasks(count) => self.generate_random_tasks(*count, todo_list, output),
            DebugCommand::ClearAll => self.clear_all_tasks(todo_list, output),
            DebugCommand::Toggle => {
                self.toggle_debug_mode(output);
                if self.debug_mode {
                    output.show_debug_help();
                }
            }
        }
    }
    
    /// Toggles debug mode on/off
    fn toggle_debug_mode<W: Write>(&mut self, output: &mut OutputWriter<W>) {
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
    fn generate_random_tasks<W: Write>(
        &self,
        count: usize,
        todo_list: &mut TodoList,
        output: &mut OutputWriter<W>
    ) {
        if !self.debug_mode {
            output.show_error("Debug mode is not enabled. Use 'debug:toggle' to enable it.");
            return;
        }
        
        // Generate random tasks
        let new_tasks = self.task_generator.generate(count);
        
        // Add each generated task to the todo list
        for new_task in new_tasks {
            let _ = todo_list.add_task(new_task);
        }
        
        output.show_success(&format!("Generated {} random tasks", count));
    }
    
    /// Clears all tasks from the todo list
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The todo list to clear
    /// * `output` - Output writer for displaying results
    fn clear_all_tasks<W: Write>(
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

impl Default for DebugCommandHandler {
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
        let mut controller = DebugCommandHandler::new();
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
        let controller = DebugCommandHandler::new();
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
        let mut controller = DebugCommandHandler::new();
        let mut todo_list = TodoList::new();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.toggle_debug_mode(&mut output);
        controller.generate_random_tasks(10, &mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks_without_debug_mode() {
        let controller = DebugCommandHandler::new();
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task".to_string()));
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.clear_all_tasks(&mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 1);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Debug mode is not enabled"));
    }
    
    #[test]
    fn test_clear_all_tasks_with_debug_mode() {
        let mut controller = DebugCommandHandler::new();
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task 1".to_string()));
        todo_list.add_task(TaskWithoutId::new("Test task 2".to_string()));
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        
        controller.toggle_debug_mode(&mut output);
        controller.clear_all_tasks(&mut todo_list, &mut output);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
