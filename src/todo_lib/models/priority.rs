/// Priority level for a task.
///
/// Tasks can be assigned different priority levels to help organize
/// and focus on what's most important.
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
pub enum Priority {
    /// Low priority - can be done later
    Low,
    /// Medium priority - normal importance (default)
    #[default]
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
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        s.parse().ok()
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

impl std::str::FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "low" | "l" => Ok(Priority::Low),
            "medium" | "med" | "m" => Ok(Priority::Medium),
            "high" | "h" => Ok(Priority::High),
            _ => Err(()),
        }
    }
}
