/// Represents debug-related commands.
///
/// These commands are used for development and testing purposes.
#[derive(Debug, Clone, PartialEq)]
pub enum DebugCommand {
    /// Generate random tasks for testing
    GenerateTasks(usize),

    /// Clear all tasks
    ClearAll,

    /// Generate random projects with random tasks
    GenerateProjects {
        project_count: usize,
        tasks_per_project: usize,
    },

    /// Clear and delete all projects (keeping empty default project)
    ClearAllProjects,
}
