use crate::controller::project_command::ProjectCommand;
use crate::controller::project_command::ProjectCommandInputParser;
use crate::controller::project_command::ProjectCommandOutputManager;
use crate::controller::project_command::ProjectManager;
use crate::controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_result::CommandControllerResultAction;
use crate::models::parse_error::ParseError;
use crate::OutputWriter;
use std::cell::RefCell;
use std::rc::Rc;

/// Handler for project commands and operations.
pub struct ProjectCommandController<O: OutputWriter> {
    project_manager: Rc<RefCell<ProjectManager>>,
    input_parser: ProjectCommandInputParser,
    output_manager: ProjectCommandOutputManager<O>,
}

impl<O: OutputWriter> ProjectCommandController<O> {
    /// Creates a new ProjectCommandController.
    pub fn new(
        project_manager: Rc<RefCell<ProjectManager>>,
        output_writer: Rc<RefCell<O>>,
    ) -> Self {
        Self {
            project_manager,
            input_parser: ProjectCommandInputParser::new(),
            output_manager: ProjectCommandOutputManager::new(output_writer),
        }
    }

    /// Handles a project command.
    fn handle_command(&mut self, command: &ProjectCommand) -> CommandControllerResult {
        match command {
            ProjectCommand::NewProject(name) => self.new_project(name),
            ProjectCommand::SwitchProject(name) => self.switch_project(name),
            ProjectCommand::ListProjects => self.list_projects(),
            ProjectCommand::DeleteProject(name) => self.delete_project(name),
            ProjectCommand::RenameProject(old_name, new_name) => {
                self.rename_project(old_name, new_name)
            }
        }
    }

    fn new_project(&mut self, name: &str) -> CommandControllerResult {
        match self
            .project_manager
            .borrow_mut()
            .create_project(name.to_string())
        {
            Some(()) => {
                self.output_manager.show_project_created(name);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            }
            None => {
                self.output_manager.show_project_already_exists(name);
                CommandControllerResult::default()
            }
        }
    }

    fn switch_project(&mut self, name: &str) -> CommandControllerResult {
        match self
            .project_manager
            .borrow_mut()
            .switch_project(name.to_string())
        {
            Some(()) => {
                self.output_manager.show_switched_to_project(name);
                CommandControllerResult::default()
            }
            None => {
                self.output_manager.show_project_not_found(name);
                CommandControllerResult::default()
            }
        }
    }

    fn list_projects(&mut self) -> CommandControllerResult {
        let project_manager = self.project_manager.borrow();
        let projects = project_manager.list_projects();
        let current_project = project_manager.get_current_project_name();
        self.output_manager
            .show_projects(&projects, current_project);
        CommandControllerResult::default()
    }

    fn delete_project(&mut self, name: &str) -> CommandControllerResult {
        let current_project = self
            .project_manager
            .borrow()
            .get_current_project_name()
            .to_string();

        if name == current_project {
            self.output_manager.show_cannot_delete_current_project(name);
            return CommandControllerResult::default();
        }

        match self
            .project_manager
            .borrow_mut()
            .delete_project(name.to_string())
        {
            Some(()) => {
                self.output_manager.show_project_deleted(name);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            }
            None => {
                self.output_manager.show_project_not_found(name);
                CommandControllerResult::default()
            }
        }
    }

    fn rename_project(&mut self, old_name: &str, new_name: &str) -> CommandControllerResult {
        match self
            .project_manager
            .borrow_mut()
            .rename_project(old_name.to_string(), new_name.to_string())
        {
            Some(()) => {
                self.output_manager.show_project_renamed(old_name, new_name);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            }
            None => {
                if self
                    .project_manager
                    .borrow()
                    .list_projects()
                    .contains(&new_name.to_string())
                {
                    self.output_manager.show_project_already_exists(new_name);
                } else {
                    self.output_manager.show_project_not_found(old_name);
                }
                CommandControllerResult::default()
            }
        }
    }
}

impl<O: OutputWriter> CommandController for ProjectCommandController<O> {
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
    use crate::ui::output::FileOutputWriter;

    #[test]
    fn test_new_project() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        let result = controller.new_project("Work");
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
        assert_eq!(project_manager.borrow().project_count(), 2);
    }

    #[test]
    fn test_new_project_duplicate() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        project_manager
            .borrow_mut()
            .create_project("Work".to_string());

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        let result = controller.new_project("Work");
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_switch_project() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        project_manager
            .borrow_mut()
            .create_project("Work".to_string());

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        controller.switch_project("Work");
        assert_eq!(project_manager.borrow().get_current_project_name(), "Work");
    }

    #[test]
    fn test_list_projects() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        project_manager
            .borrow_mut()
            .create_project("Work".to_string());
        project_manager
            .borrow_mut()
            .create_project("Personal".to_string());

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        controller.list_projects();
        // Should not crash and should display projects
    }

    #[test]
    fn test_delete_project() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        project_manager
            .borrow_mut()
            .create_project("Work".to_string());

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        let result = controller.delete_project("Work");
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
        assert_eq!(project_manager.borrow().project_count(), 1);
    }

    #[test]
    fn test_delete_current_project() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        let result = controller.delete_project("default");
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_rename_project() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));

        let mut buffer = Vec::new();
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(&mut buffer)));
        let mut controller =
            ProjectCommandController::new(Rc::clone(&project_manager), output_writer);

        let result = controller.rename_project("default", "personal");
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
        assert_eq!(
            project_manager.borrow().get_current_project_name(),
            "personal"
        );
    }
}
