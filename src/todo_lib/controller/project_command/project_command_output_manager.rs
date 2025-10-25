use crate::OutputWriter;
use std::cell::RefCell;
use std::rc::Rc;

/// Handles output formatting for project-related commands.
pub struct ProjectCommandOutputManager<O: OutputWriter> {
    output_writer: Rc<RefCell<O>>,
}

impl<O: OutputWriter> ProjectCommandOutputManager<O> {
    /// Creates a new ProjectCommandOutputManager.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        Self { output_writer }
    }

    /// Displays a success message after creating a project.
    pub fn show_project_created(&mut self, name: &str) {
        self.output_writer
            .borrow_mut()
            .show_success(&format!("Created project '{}'", name));
    }

    /// Displays a success message after switching projects.
    pub fn show_switched_to_project(&mut self, name: &str) {
        self.output_writer
            .borrow_mut()
            .show_success(&format!("Switched to project '{}'", name));
    }

    /// Displays a success message after deleting a project.
    pub fn show_project_deleted(&mut self, name: &str) {
        self.output_writer
            .borrow_mut()
            .show_success(&format!("Deleted project '{}'", name));
    }

    /// Displays a success message after renaming a project.
    pub fn show_project_renamed(&mut self, old_name: &str, new_name: &str) {
        self.output_writer
            .borrow_mut()
            .show_success(&format!("Renamed project '{}' to '{}'", old_name, new_name));
    }

    /// Displays the list of projects.
    pub fn show_projects(&mut self, projects: &[String], current_project: &str) {
        let mut output = self.output_writer.borrow_mut();

        output.write_line("");
        output.write_line("--- Projects ---");

        if projects.is_empty() {
            output.write_line("No projects found.");
        } else {
            for project in projects {
                if project == current_project {
                    output.write_line(&format!("  * {} (current)", project));
                } else {
                    output.write_line(&format!("    {}", project));
                }
            }
        }

        output.write_line(&format!("\nTotal: {}", projects.len()));
        output.write_line(&"-".repeat(40));
        output.write_line("");
    }

    /// Displays an error when a project already exists.
    pub fn show_project_already_exists(&mut self, name: &str) {
        self.output_writer
            .borrow_mut()
            .show_error(&format!("Project '{}' already exists", name));
    }

    /// Displays an error when a project is not found.
    pub fn show_project_not_found(&mut self, name: &str) {
        self.output_writer
            .borrow_mut()
            .show_error(&format!("Project '{}' not found", name));
    }

    /// Displays an error when trying to delete the current project.
    pub fn show_cannot_delete_current_project(&mut self, name: &str) {
        self.output_writer.borrow_mut().show_error(&format!(
            "Cannot delete current project '{}'. Switch to another project first.",
            name
        ));
    }

    /// Displays a generic error message.
    pub fn show_error(&mut self, message: &str) {
        self.output_writer.borrow_mut().show_error(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::output::FileOutputWriter;

    #[test]
    fn test_show_project_created() {
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut manager = ProjectCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_project_created("Work");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Created project 'Work'"));
    }

    #[test]
    fn test_show_switched_to_project() {
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut manager = ProjectCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_switched_to_project("Personal");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Switched to project 'Personal'"));
    }

    #[test]
    fn test_show_projects() {
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut manager = ProjectCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        let projects = vec![
            "Work".to_string(),
            "Personal".to_string(),
            "default".to_string(),
        ];
        manager.show_projects(&projects, "Work");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Projects"));
        assert!(output.contains("Work (current)"));
        assert!(output.contains("Personal"));
        assert!(output.contains("Total: 3"));
    }

    #[test]
    fn test_show_project_already_exists() {
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut manager = ProjectCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_project_already_exists("Work");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("already exists"));
    }

    #[test]
    fn test_show_project_not_found() {
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut manager = ProjectCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_project_not_found("NonExistent");

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("not found"));
    }
}
