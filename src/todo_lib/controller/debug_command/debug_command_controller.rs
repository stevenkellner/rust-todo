use crate::controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_result::CommandControllerResultAction;
use crate::controller::project_command::ProjectManager;
use crate::controller::debug_command::DebugCommand;
use crate::controller::debug_command::RandomTaskGenerator;
use crate::models::ParseError;
use crate::controller::debug_command::DebugCommandInputParser;
use crate::controller::debug_command::DebugCommandOutputManager;
use crate::OutputWriter;
use std::rc::Rc;
use std::cell::RefCell;
use rand::Rng;

/// Handler for debug commands and operations
pub struct DebugCommandController<O: OutputWriter> {
    project_manager: Rc<RefCell<ProjectManager>>,
    input_parser: DebugCommandInputParser,
    /// Output writer for displaying results
    output_manager: DebugCommandOutputManager<O>,
    /// Task generator for creating random tasks
    task_generator: RandomTaskGenerator,
}

impl<O: OutputWriter> DebugCommandController<O> {
    /// Creates a new DebugCommandController with a custom output writer
    pub fn new(project_manager: Rc<RefCell<ProjectManager>>, output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            project_manager,
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
            DebugCommand::GenerateProjects { project_count, tasks_per_project } => 
                self.generate_random_projects(*project_count, *tasks_per_project),
            DebugCommand::ClearAllProjects => self.clear_all_projects(),
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
        
        let mut total_tasks = 0;
        let mut total_subtasks = 0;
        let mut task_ids = Vec::new();
        
        // Add each generated task to the todo list
        for new_task in new_tasks {
            let parent_id = self.project_manager.borrow_mut().get_current_todo_list_mut().add_task(new_task);
            task_ids.push(parent_id);
            total_tasks += 1;
            
            // Generate subtasks for some tasks (50% probability)
            let subtask_count = self.task_generator.generate_subtask_count(0.5);
            for _ in 0..subtask_count {
                let subtask = self.task_generator.generate_single_subtask(0.2);
                if self.project_manager.borrow_mut().get_current_todo_list_mut().add_subtask(parent_id, subtask.description).is_some() {
                    total_subtasks += 1;
                }
            }
        }
        
        // Add random dependencies (30% chance for each task)
        let mut rng = rand::rng();
        let mut dependencies_added = 0;
        for (idx, &task_id) in task_ids.iter().enumerate() {
            // 30% chance to add a dependency
            if rng.random_bool(0.3) && idx > 0 {
                // Pick a random earlier task to depend on
                let depends_on_idx = rng.random_range(0..idx);
                let depends_on_id = task_ids[depends_on_idx];
                if self.project_manager.borrow_mut().get_current_todo_list_mut().add_task_dependency(task_id, depends_on_id).is_some() {
                    dependencies_added += 1;
                }
            }
        }
        
        // Uncomplete tasks that have incomplete dependencies
        for &task_id in &task_ids {
            let has_incomplete_deps = !self.project_manager.borrow().get_current_todo_list().are_dependencies_completed(task_id);
            if has_incomplete_deps {
                self.project_manager.borrow_mut().get_current_todo_list_mut().uncomplete_task(task_id);
            }
        }
        
        let mut message = if total_subtasks > 0 {
            format!("Generated {} random tasks with {} subtasks", total_tasks, total_subtasks)
        } else {
            format!("Generated {} random tasks", total_tasks)
        };
        
        if dependencies_added > 0 {
            message.push_str(&format!(" and {} dependencies", dependencies_added));
        }
        
        self.output_manager.show_success(&message);
        
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }
    
    /// Clears all tasks from the todo list
    ///
    /// # Arguments
    ///
    /// * `todo_list` - The todo list to clear
    fn clear_all_tasks(&mut self) -> CommandControllerResult {
        let count = self.project_manager.borrow().get_current_todo_list().get_tasks().len();
        self.project_manager.borrow_mut().get_current_todo_list_mut().clear_all();
        self.output_manager.show_success(&format!("Cleared {} tasks", count));
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }
    
    /// Generates random projects with random tasks
    ///
    /// # Arguments
    ///
    /// * `project_count` - Number of random projects to generate
    /// * `tasks_per_project` - Number of tasks to generate per project
    fn generate_random_projects(&mut self, project_count: usize, tasks_per_project: usize) -> CommandControllerResult {
        let project_names = self.task_generator.generate_project_names(project_count);
        let mut created_projects = 0;
        let mut total_tasks = 0;
        let mut total_subtasks = 0;
        let mut total_dependencies = 0;
        
        for project_name in project_names {
            // Create the project
            if self.project_manager.borrow_mut().create_project(project_name.clone()).is_some() {
                created_projects += 1;
                
                // Switch to the new project
                self.project_manager.borrow_mut().switch_project(project_name.clone());
                
                // Generate tasks for this project
                let new_tasks = self.task_generator.generate(tasks_per_project);
                let mut task_ids = Vec::new();
                
                // Add each generated task to the project's todo list
                for new_task in new_tasks {
                    let parent_id = self.project_manager.borrow_mut().get_current_todo_list_mut().add_task(new_task);
                    task_ids.push(parent_id);
                    total_tasks += 1;
                    
                    // Generate subtasks for some tasks (50% probability)
                    let subtask_count = self.task_generator.generate_subtask_count(0.5);
                    for _ in 0..subtask_count {
                        let subtask = self.task_generator.generate_single_subtask(0.2);
                        if self.project_manager.borrow_mut().get_current_todo_list_mut().add_subtask(parent_id, subtask.description).is_some() {
                            total_subtasks += 1;
                        }
                    }
                }
                
                // Add random dependencies (30% chance for each task)
                let mut rng = rand::rng();
                for (idx, &task_id) in task_ids.iter().enumerate() {
                    // 30% chance to add a dependency
                    if rng.random_bool(0.3) && idx > 0 {
                        // Pick a random earlier task to depend on
                        let depends_on_idx = rng.random_range(0..idx);
                        let depends_on_id = task_ids[depends_on_idx];
                        if self.project_manager.borrow_mut().get_current_todo_list_mut().add_task_dependency(task_id, depends_on_id).is_some() {
                            total_dependencies += 1;
                        }
                    }
                }
                
                // Uncomplete tasks that have incomplete dependencies
                for &task_id in &task_ids {
                    let has_incomplete_deps = !self.project_manager.borrow().get_current_todo_list().are_dependencies_completed(task_id);
                    if has_incomplete_deps {
                        self.project_manager.borrow_mut().get_current_todo_list_mut().uncomplete_task(task_id);
                    }
                }
            }
        }
        
        let mut message = format!("Generated {} projects with {} tasks", created_projects, total_tasks);
        
        if total_subtasks > 0 {
            message.push_str(&format!(", {} subtasks", total_subtasks));
        }
        
        if total_dependencies > 0 {
            message.push_str(&format!(", and {} dependencies", total_dependencies));
        }
        
        self.output_manager.show_success(&message);
        
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }
    
    /// Clears and deletes all projects, keeping only an empty default project
    fn clear_all_projects(&mut self) -> CommandControllerResult {
        let all_projects = self.project_manager.borrow().list_projects();
        let mut deleted_count = 0;
        let mut total_tasks_cleared = 0;
        
        // Count total tasks before clearing
        for project_name in &all_projects {
            self.project_manager.borrow_mut().switch_project(project_name.clone());
            total_tasks_cleared += self.project_manager.borrow().get_current_todo_list().get_tasks().len();
        }
        
        // Switch to default project
        self.project_manager.borrow_mut().switch_project("default".to_string());
        
        // Delete all non-default projects
        for project_name in all_projects {
            if project_name != "default" && self.project_manager.borrow_mut().delete_project(project_name).is_some() {
                deleted_count += 1;
            }
        }
        
        // Clear all tasks from default project
        self.project_manager.borrow_mut().get_current_todo_list_mut().clear_all();
        
        let message = format!(
            "Cleared {} projects and {} tasks. Empty default project remains.",
            deleted_count,
            total_tasks_cleared
        );
        
        self.output_manager.show_success(&message);
        
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
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&project_manager), Rc::new(RefCell::new(output_writer)));

        controller.generate_random_tasks(10);

        // Should have at least 10 tasks (parents), possibly more with subtasks
        let total_tasks = project_manager.borrow().get_current_todo_list().get_tasks().len();
        assert!(total_tasks >= 10, "Expected at least 10 tasks, got {}", total_tasks);
    }
    
    #[test]
    fn test_clear_all_tasks() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        project_manager.borrow_mut().get_current_todo_list_mut().add_task(TaskWithoutId::new("Test task 1".to_string()));
        project_manager.borrow_mut().get_current_todo_list_mut().add_task(TaskWithoutId::new("Test task 2".to_string()));
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&project_manager), Rc::new(RefCell::new(output_writer)));

        controller.clear_all_tasks();
        
        assert_eq!(project_manager.borrow().get_current_todo_list().get_tasks().len(), 0);
    }
    
    #[test]
    fn test_generate_random_projects() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&project_manager), Rc::new(RefCell::new(output_writer)));

        controller.generate_random_projects(3, 5);

        // Should have 4 projects total (default + 3 new ones)
        let project_count = project_manager.borrow().project_count();
        assert_eq!(project_count, 4, "Expected 4 projects, got {}", project_count);
        
        // Each project should have tasks (we generated 5 tasks per project)
        let projects = project_manager.borrow().list_projects();
        for project_name in projects {
            if project_name != "default" {
                project_manager.borrow_mut().switch_project(project_name.clone());
                let task_count = project_manager.borrow().get_current_todo_list().get_tasks().len();
                assert!(task_count >= 5, "Project {} should have at least 5 tasks, got {}", project_name, task_count);
            }
        }
    }
    
    #[test]
    fn test_clear_all_projects() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        
        // Create some projects with tasks
        project_manager.borrow_mut().create_project("Work".to_string());
        project_manager.borrow_mut().switch_project("Work".to_string());
        project_manager.borrow_mut().get_current_todo_list_mut().add_task(TaskWithoutId::new("Work task".to_string()));
        
        project_manager.borrow_mut().create_project("Personal".to_string());
        project_manager.borrow_mut().switch_project("Personal".to_string());
        project_manager.borrow_mut().get_current_todo_list_mut().add_task(TaskWithoutId::new("Personal task".to_string()));
        
        project_manager.borrow_mut().switch_project("default".to_string());
        project_manager.borrow_mut().get_current_todo_list_mut().add_task(TaskWithoutId::new("Default task".to_string()));
        
        let buffer = Vec::new();
        let output_writer = FileOutputWriter::new(buffer);
        let mut controller = DebugCommandController::new(Rc::clone(&project_manager), Rc::new(RefCell::new(output_writer)));

        controller.clear_all_projects();
        
        // Should only have default project left
        assert_eq!(project_manager.borrow().project_count(), 1);
        assert_eq!(project_manager.borrow().get_current_project_name(), "default");
        
        // Default project should be empty
        assert_eq!(project_manager.borrow().get_current_todo_list().get_tasks().len(), 0);
    }
}
