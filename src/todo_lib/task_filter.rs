/// Filter options for listing tasks.
///
/// Determines which tasks should be displayed when listing.
#[derive(Debug, Clone, PartialEq)]
pub enum TaskFilter {
    /// Show all tasks regardless of completion status
    All,
    
    /// Show only completed tasks
    Completed,
    
    /// Show only pending (incomplete) tasks
    Pending,
}
