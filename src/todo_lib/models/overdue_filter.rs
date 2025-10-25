/// Overdue status filter for tasks with due dates.
///
/// Determines which tasks to show based on their due date status.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverdueFilter {
    /// Show all tasks regardless of due date status
    All,

    /// Show only overdue tasks (due date in the past)
    OnlyOverdue,

    /// Show only tasks that are not overdue (no due date or future due date)
    OnlyNotOverdue,
}
