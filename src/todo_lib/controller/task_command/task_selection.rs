/// Represents how tasks are specified for bulk operations.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskSelection {
    /// Single task by ID
    Single(usize),

    /// Multiple tasks by IDs
    Multiple(Vec<usize>),

    /// All tasks
    All,
}
