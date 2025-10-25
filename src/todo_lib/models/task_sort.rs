use std::cmp::Ordering;
use std::str::FromStr;

/// Defines the field to sort tasks by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    /// Sort by task ID (default)
    Id,
    /// Sort by priority (High > Medium > Low)
    Priority,
    /// Sort by due date (earliest first, tasks without due date last)
    DueDate,
    /// Sort by category name (alphabetically, tasks without category last)
    Category,
    /// Sort by completion status (pending first, completed last)
    Status,
}

impl Default for SortBy {
    /// Returns the default sort order (by ID, ascending)
    fn default() -> Self {
        SortBy::Id
    }
}

impl FromStr for SortBy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "id" => Ok(SortBy::Id),
            "priority" | "pri" => Ok(SortBy::Priority),
            "due" | "due-date" | "duedate" => Ok(SortBy::DueDate),
            "category" | "cat" => Ok(SortBy::Category),
            "status" => Ok(SortBy::Status),
            _ => Err(format!(
                "Invalid sort option: '{}'. Valid options: id, priority, due, category, status",
                s
            )),
        }
    }
}

/// Defines the sort order direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOrder {
    /// Ascending order (default)
    Ascending,
    /// Descending order
    Descending,
}

impl Default for SortOrder {
    /// Returns the default sort order (ascending)
    fn default() -> Self {
        SortOrder::Ascending
    }
}

impl SortOrder {
    /// Reverses the ordering based on the sort order.
    pub fn apply(&self, ordering: Ordering) -> Ordering {
        match self {
            SortOrder::Ascending => ordering,
            SortOrder::Descending => ordering.reverse(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_by_from_str_id() {
        assert_eq!(SortBy::from_str("id").unwrap(), SortBy::Id);
        assert_eq!(SortBy::from_str("ID").unwrap(), SortBy::Id);
    }

    #[test]
    fn test_sort_by_from_str_priority() {
        assert_eq!(SortBy::from_str("priority").unwrap(), SortBy::Priority);
        assert_eq!(SortBy::from_str("pri").unwrap(), SortBy::Priority);
        assert_eq!(SortBy::from_str("PRIORITY").unwrap(), SortBy::Priority);
    }

    #[test]
    fn test_sort_by_from_str_due_date() {
        assert_eq!(SortBy::from_str("due").unwrap(), SortBy::DueDate);
        assert_eq!(SortBy::from_str("due-date").unwrap(), SortBy::DueDate);
        assert_eq!(SortBy::from_str("duedate").unwrap(), SortBy::DueDate);
    }

    #[test]
    fn test_sort_by_from_str_category() {
        assert_eq!(SortBy::from_str("category").unwrap(), SortBy::Category);
        assert_eq!(SortBy::from_str("cat").unwrap(), SortBy::Category);
    }

    #[test]
    fn test_sort_by_from_str_status() {
        assert_eq!(SortBy::from_str("status").unwrap(), SortBy::Status);
    }

    #[test]
    fn test_sort_by_from_str_invalid() {
        assert!(SortBy::from_str("invalid").is_err());
        assert!(SortBy::from_str("xyz").is_err());
    }

    #[test]
    fn test_sort_by_default() {
        assert_eq!(SortBy::default(), SortBy::Id);
    }

    #[test]
    fn test_sort_order_default() {
        assert_eq!(SortOrder::default(), SortOrder::Ascending);
    }

    #[test]
    fn test_sort_order_apply_ascending() {
        let order = SortOrder::Ascending;
        assert_eq!(order.apply(Ordering::Less), Ordering::Less);
        assert_eq!(order.apply(Ordering::Equal), Ordering::Equal);
        assert_eq!(order.apply(Ordering::Greater), Ordering::Greater);
    }

    #[test]
    fn test_sort_order_apply_descending() {
        let order = SortOrder::Descending;
        assert_eq!(order.apply(Ordering::Less), Ordering::Greater);
        assert_eq!(order.apply(Ordering::Equal), Ordering::Equal);
        assert_eq!(order.apply(Ordering::Greater), Ordering::Less);
    }
}
