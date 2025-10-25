/// Task completion status for filtering.
///
/// Represents whether a task is completed or still pending.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// Show only completed tasks
    Completed,

    /// Show only pending (incomplete) tasks
    Pending,
}
