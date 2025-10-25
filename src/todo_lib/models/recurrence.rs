use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Represents the recurrence pattern for a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Recurrence {
    /// Task recurs daily
    Daily,
    /// Task recurs weekly
    Weekly,
    /// Task recurs monthly
    Monthly,
}

impl Recurrence {
    /// Returns the string representation of the recurrence
    pub fn as_str(&self) -> &str {
        match self {
            Recurrence::Daily => "Daily",
            Recurrence::Weekly => "Weekly",
            Recurrence::Monthly => "Monthly",
        }
    }

    /// Returns the emoji icon for recurring tasks
    pub fn icon() -> &'static str {
        "🔄"
    }
}

impl FromStr for Recurrence {
    type Err = ();

    /// Parses a recurrence from a string
    ///
    /// # Arguments
    ///
    /// * `s` - The string to parse (case-insensitive)
    ///
    /// # Returns
    ///
    /// `Ok(Recurrence)` if valid, `Err(())` otherwise
    ///
    /// # Examples
    ///
    /// ```
    /// use std::str::FromStr;
    /// use todo_manager::models::recurrence::Recurrence;
    ///
    /// assert_eq!(Recurrence::from_str("daily"), Ok(Recurrence::Daily));
    /// assert_eq!(Recurrence::from_str("WEEKLY"), Ok(Recurrence::Weekly));
    /// assert_eq!(Recurrence::from_str("monthly"), Ok(Recurrence::Monthly));
    /// assert!(Recurrence::from_str("invalid").is_err());
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "daily" | "d" => Ok(Recurrence::Daily),
            "weekly" | "w" => Ok(Recurrence::Weekly),
            "monthly" | "m" => Ok(Recurrence::Monthly),
            _ => Err(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recurrence_as_str() {
        assert_eq!(Recurrence::Daily.as_str(), "Daily");
        assert_eq!(Recurrence::Weekly.as_str(), "Weekly");
        assert_eq!(Recurrence::Monthly.as_str(), "Monthly");
    }
    #[test]
    fn test_recurrence_from_str() {
        assert_eq!(Recurrence::from_str("daily"), Ok(Recurrence::Daily));
        assert_eq!(Recurrence::from_str("DAILY"), Ok(Recurrence::Daily));
        assert_eq!(Recurrence::from_str("d"), Ok(Recurrence::Daily));
        
        assert_eq!(Recurrence::from_str("weekly"), Ok(Recurrence::Weekly));
        assert_eq!(Recurrence::from_str("WEEKLY"), Ok(Recurrence::Weekly));
        assert_eq!(Recurrence::from_str("w"), Ok(Recurrence::Weekly));
        
        assert_eq!(Recurrence::from_str("monthly"), Ok(Recurrence::Monthly));
        assert_eq!(Recurrence::from_str("MONTHLY"), Ok(Recurrence::Monthly));
        assert_eq!(Recurrence::from_str("m"), Ok(Recurrence::Monthly));
        
        assert!(Recurrence::from_str("invalid").is_err());
        assert!(Recurrence::from_str("").is_err());
    }

    #[test]
    fn test_recurrence_serialization() {
        let daily = Recurrence::Daily;
        let json = serde_json::to_string(&daily).unwrap();
        let deserialized: Recurrence = serde_json::from_str(&json).unwrap();
        assert_eq!(daily, deserialized);
    }
}
