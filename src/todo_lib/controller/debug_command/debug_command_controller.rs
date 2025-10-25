use crate::controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_result::CommandControllerResultAction;
use crate::models::todo_list::TodoList;
use crate::controller::debug_command::DebugCommand;
use crate::controller::debug_command::RandomTaskGenerator;
use crate::models::ParseError;
use crate::controller::debug_command::DebugCommandInputParser;
use crate::controller::debug_command::DebugCommandOutputManager;
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;

/// Handler for debug commands and operations
pub struct DebugCommandController<O: OutputWriter> {
    todo_list: Rc<RefCell<TodoList>>,
    input_parser: DebugCommandInputParser,
    /// Output writer for displaying results
    output_manager: DebugCommandOutputManager<O>,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
}

impl<O: OutputWriter> DebugCommandController<O> {
    /// Creates a new DebugCommandController with a custom output writer
    pub fn new(todo_list: Rc<RefCell<TodoList>>, output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            todo_list,
            input_parser: DebugCommandInputParser::new(),
            output_manager: DebugCommandOutputManager::new(output_writer),
            task_generator: RandomTaskGenerator::new(), 
        }
    }
    
    /// Handles a debug command
    fn handle_command(&mut self, command: &DebugCommand) -> CommandControllerResult {
        match command {
            DebugCommand::GenerateTasks(count) => self.generate_random_tasks(*count),
            DebugCommand::ClearAll => self.clear_all_tasks(),
        }
    }
    
    /// Generates random tasks for testing
    ///
    /// # Arguments
    ///
    /// * `count` - Number of random tasks to generate
    /// * `todo_list` - The todo list to add tasks to
    fn generate_random_tasks(&mut self, count: usize) -> CommandControllerResult {
        // Generate random tasks
        let new_tasks = self.task_generator.generate(count);
        
        // Add each generated task to the todo list
        for new_task in new_tasks {
            let _ = self.todo_list.borrow_mut().add_task(new_task);
        }
        
        self.output_manager.show_success(&format!("Generated {} random tasks", count));
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }
    
    /// Clears all tasks from the todo list
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The todo list to clear
    fn clear_all_tasks(&mut self) -> CommandControllerResult {
        let count = self.todo_list.borrow().get_tasks().len();
        self.todo_list.borrow_mut().clear_all();
        self.output_manager.show_success(&format!("Cleared {} tasks", count));
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }
}

impl<O: OutputWriter> CommandController for DebugCommandController<O> {
    fn try_execute(&mut self, input: &str) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.input_parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd);
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
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));

        controller.generate_random_tasks(10);

        assert_eq!(todo_list.borrow().get_tasks().len(), 10);
    }
    
    #[test]
    fn test_clear_all_tasks() {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        todo_list.borrow_mut().add_task(TaskWithoutId::new("Test task 1".to_string()));
        todo_list.borrow_mut().add_task(TaskWithoutId::new("Test task 2".to_string()));
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));

        controller.clear_all_tasks();
        
        assert_eq!(todo_list.borrow().get_tasks().len(), 0);
    }
}
