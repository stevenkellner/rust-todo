use crate::controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::todo_list::TodoList;
use crate::controller::debug_command::debug_command::DebugCommand;
use crate::controller::debug_command::RandomTaskGenerator;
use crate::models::ParseError;
use crate::controller::debug_command::debug_command_input_parser::DebugCommandInputParser;
use crate::controller::debug_command::DebugCommandOutputManager;
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Handler for debug commands and operations
pub struct DebugCommandController<O: OutputWriter> {
    input_parser: DebugCommandInputParser,
    /// Output writer for displaying results
    output_manager: DebugCommandOutputManager<O>,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
}

impl<O: OutputWriter> DebugCommandController<O> {
    /// Creates a new DebugCommandController with a custom output writer
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            input_parser: DebugCommandInputParser::new(),
            output_manager: DebugCommandOutputManager::new(output_writer),
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
        
        self.output_manager.show_success(&format!("Generated {} random tasks", count));
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
        self.output_manager.show_success(&format!("Cleared {} tasks", count));
    }
}

impl<O: OutputWriter> CommandController for DebugCommandController<O> {
    fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.input_parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd, todo_list);
                Some(Ok(result))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::task::TaskWithoutId;
    use crate::ui::output::FileOutputWriter;
    
    #[test]
    fn test_generate_random_tasks() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::new(RefCell::new(output_writer)));
        let mut todo_list = TodoList::new();
        
        controller.generate_random_tasks(10, &mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks() {
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::new(RefCell::new(output_writer)));
        let mut todo_list = TodoList::new();
        todo_list.add_task(TaskWithoutId::new("Test task 1".to_string()));
        todo_list.add_task(TaskWithoutId::new("Test task 2".to_string()));
        
        controller.clear_all_tasks(&mut todo_list);
        
        assert_eq!(todo_list.get_tasks().len(), 0);
    }
}
