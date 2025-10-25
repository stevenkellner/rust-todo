/// Represents project-related commands.
///
/// These commands handle operations on projects including creation,
/// deletion, switching, and listing.
#[derive(Debug, Clone, PartialEq)]
pub enum ProjectCommand {
    /// Create a new project with the given name
    NewProject(String),

    /// Switch to an existing project
    SwitchProject(String),

    /// List all projects
    ListProjects,

    /// Delete a project by name
    DeleteProject(String),

    /// Rename a project
    RenameProject(String, String), // (old_name, new_name)
}
