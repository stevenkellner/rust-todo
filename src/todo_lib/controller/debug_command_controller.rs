use crate::controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::todo_list::TodoList;
use crate::models::debug_command::DebugCommand;
use crate::controller::task_generator::RandomTaskGenerator;
use crate::models::ParseError;
use crate::ui::debug_command_parser::DebugCommandParser;
use crate::ui::DebugCommandOutputWriter;
use std::io::Write;

/// Handler for debug commands and operations
pub struct DebugCommandController<W: Write> {
    parser: DebugCommandParser,
    /// Output writer for displaying results
    output: DebugCommandOutputWriter<W>,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
}

impl DebugCommandController<std::io::Stdout> {
    /// Creates a new DebugCommandController with stdout
    pub fn new() -> Self {
        Self {
            parser: DebugCommandParser::new(),
            output: DebugCommandOutputWriter::new(), 
            task_generator: RandomTaskGenerator::new(), 
        }
    }
}

impl<W: Write> DebugCommandController<W> {
    /// Creates a new DebugCommandController with a custom output writer
    pub fn with_writer(writer: W) -> Self {
        Self {
            parser: DebugCommandParser::new(),
            output: DebugCommandOutputWriter::with_writer(writer),
            task_generator: RandomTaskGenerator::new(), 
        }
    }
    
    /// Handles a debug command
    fn handle_command(&mut self, command: &DebugCommand, todo_list: &mut TodoList) -> CommandControllerResult {
        match command {
            DebugCommand::GenerateTasks(count) => self.generate_random_tasks(*count, todo_list),
            DebugCommand::ClearAll => self.clear_all_tasks(todo_list),
        }
        CommandControllerResult::Continue
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
        let count = todo_list.get_tasks().len();
        todo_list.clear_all();
        self.output.show_success(&format!("Cleared {} tasks", count));
    }
}

impl<W: Write> CommandController for DebugCommandController<W> {
    fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd, todo_list);
                Some(Ok(result))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

impl Default for DebugCommandController<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;
    
    #[test]
    fn test_generate_random_tasks() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        
        controller.generate_random_tasks(10, &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks() {
        let mut controller = DebugCommandController::with_writer(Vec::new());
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task 1".to_string()));
        todo_list.add_task(TaskWithoutId::new("Test task 2".to_string()));
        
        controller.clear_all_tasks(&mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
