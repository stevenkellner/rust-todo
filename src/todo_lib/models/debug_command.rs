/// Represents debug-related commands.
///
/// These commands are used for development and testing purposes.
#[derive(Debug, Clone, PartialEq)]
pub enum DebugCommand {
    /// Generate random tasks for testing
    GenerateTasks(usize),
    
    /// Clear all tasks
    ClearAll,
    
    /// Toggle debug mode on/off
    Toggle,
}
