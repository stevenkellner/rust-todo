/// Priority level for a task.
///
/// Tasks can be assigned different priority levels to help organize
/// and focus on what's most important.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Low priority - can be done later
    Low,
    /// Medium priority - normal importance (default)
    Medium,
    /// High priority - important and urgent
    High,
}

impl Priority {
    /// Creates a Priority from a string representation.
    ///
    /// # Arguments
    ///
    /// * `s` - String slice representing the priority ("low", "medium", "high")
    ///
    /// # Returns
    ///
    /// `Some(Priority)` if the string is valid, `None` otherwise
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Some(Priority::Low),
            "medium" | "med" | "m" => Some(Priority::Medium),
            "high" | "h" => Some(Priority::High),
            _ => None,
        }
    }

    /// Returns a string representation of the priority.
    pub fn as_str(&self) -> &str {
        match self {
            Priority::Low => "Low",
            Priority::Medium => "Medium",
            Priority::High => "High",
        }
    }

    /// Returns a symbol representing the priority.
    pub fn symbol(&self) -> &str {
        match self {
            Priority::Low => "▼",
            Priority::Medium => "■",
            Priority::High => "▲",
        }
    }
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Medium
    }
}
