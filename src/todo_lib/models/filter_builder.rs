use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use crate::models::overdue_filter::OverdueFilter;
use crate::models::priority::Priority;

/// Builds a TaskFilter from command-line arguments.
///
/// `FilterBuilder` provides a fluent interface for parsing and constructing
/// complex task filters with validation.
pub struct FilterBuilder {
    filter: TaskFilter,
    status_set: bool,
    priority_set: bool,
    category_set: bool,
}

impl FilterBuilder {
    /// Creates a new FilterBuilder with an empty filter.
    pub fn new() -> Self {
        FilterBuilder {
            filter: TaskFilter::all(),
            status_set: false,
            priority_set: false,
            category_set: false,
        }
    }

    /// Attempts to add a status filter.
    ///
    /// Returns an error if a status filter was already set.
    pub fn with_status(mut self, status: TaskStatus) -> Result<Self, String> {
        if self.status_set {
            return Err("Cannot specify multiple status filters (done/todo).".to_string());
        }
        self.filter = self.filter.with_status(status);
        self.status_set = true;
        Ok(self)
    }

    /// Attempts to add a priority filter.
    ///
    /// Returns an error if a priority filter was already set.
    pub fn with_priority(mut self, priority: Priority) -> Result<Self, String> {
        if self.priority_set {
            return Err("Cannot specify multiple priority filters (high/medium/low).".to_string());
        }
        self.filter = self.filter.with_priority(priority);
        self.priority_set = true;
        Ok(self)
    }

    /// Attempts to add a category filter.
    ///
    /// Returns an error if a category filter was already set or if the category is empty.
    pub fn with_category(mut self, category: String) -> Result<Self, String> {
        if self.category_set {
            return Err("Cannot specify multiple category filters.".to_string());
        }
        if category.trim().is_empty() {
            return Err("Category name cannot be empty. Use: list category:name".to_string());
        }
        self.filter = self.filter.with_category(category);
        self.category_set = true;
        Ok(self)
    }

    /// Adds an overdue filter.
    pub fn with_overdue(mut self) -> Self {
        self.filter = self.filter.with_overdue(OverdueFilter::OnlyOverdue);
        self
    }

    /// Parses a filter argument and adds it to the builder.
    ///
    /// Returns an error if the argument is invalid or conflicts with existing filters.
    pub fn parse_argument(self, arg: &str) -> Result<Self, String> {
        let arg_lower = arg.to_lowercase();
        
        // Check for category filter (format: category:name or cat:name)
        if arg_lower.starts_with("category:") || arg_lower.starts_with("cat:") {
            let category = if arg_lower.starts_with("category:") {
                arg[9..].to_string()
            } else {
                arg[4..].to_string()
            };
            return self.with_category(category);
        }
        
        match arg_lower.as_str() {
            "completed" | "done" => self.with_status(TaskStatus::Completed),
            "pending" | "todo" => self.with_status(TaskStatus::Pending),
            "high" | "h" => self.with_priority(Priority::High),
            "medium" | "med" | "m" => self.with_priority(Priority::Medium),
            "low" | "l" => self.with_priority(Priority::Low),
            "overdue" => Ok(self.with_overdue()),
            _ => Err(format!(
                "Unknown filter: '{}'. Valid filters: done, todo, high, medium, low, overdue, category:name",
                arg
            )),
        }
    }

    /// Builds the final filter.
    ///
    /// Returns None if no filters were applied (meaning show all tasks).
    pub fn build(self) -> Option<TaskFilter> {
        if !self.status_set 
            && !self.priority_set 
            && !self.category_set
            && self.filter.overdue == OverdueFilter::All {
            None
        } else {
            Some(self.filter)
        }
    }
}

impl Default for FilterBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_builder_no_filters() {
        let builder = FilterBuilder::new();
        assert!(builder.build().is_none());
    }

    #[test]
    fn test_filter_builder_with_status() {
        let builder = FilterBuilder::new()
            .with_status(TaskStatus::Completed)
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.status, Some(TaskStatus::Completed));
    }

    #[test]
    fn test_filter_builder_duplicate_status() {
        let result = FilterBuilder::new()
            .with_status(TaskStatus::Completed)
            .unwrap()
            .with_status(TaskStatus::Pending);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_builder_with_priority() {
        let builder = FilterBuilder::new()
            .with_priority(Priority::High)
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.priority, Some(Priority::High));
    }

    #[test]
    fn test_filter_builder_duplicate_priority() {
        let result = FilterBuilder::new()
            .with_priority(Priority::High)
            .unwrap()
            .with_priority(Priority::Low);
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_builder_with_category() {
        let builder = FilterBuilder::new()
            .with_category("work".to_string())
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.category, Some("work".to_string()));
    }

    #[test]
    fn test_filter_builder_empty_category() {
        let result = FilterBuilder::new()
            .with_category("".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_builder_parse_argument_completed() {
        let builder = FilterBuilder::new()
            .parse_argument("completed")
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.status, Some(TaskStatus::Completed));
    }

    #[test]
    fn test_filter_builder_parse_argument_high() {
        let builder = FilterBuilder::new()
            .parse_argument("high")
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.priority, Some(Priority::High));
    }

    #[test]
    fn test_filter_builder_parse_argument_category() {
        let builder = FilterBuilder::new()
            .parse_argument("category:work")
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.category, Some("work".to_string()));
    }

    #[test]
    fn test_filter_builder_parse_argument_overdue() {
        let builder = FilterBuilder::new()
            .parse_argument("overdue")
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.overdue, OverdueFilter::OnlyOverdue);
    }

    #[test]
    fn test_filter_builder_parse_argument_invalid() {
        let result = FilterBuilder::new()
            .parse_argument("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_builder_combined_filters() {
        let builder = FilterBuilder::new()
            .parse_argument("completed")
            .unwrap()
            .parse_argument("high")
            .unwrap()
            .parse_argument("category:urgent")
            .unwrap();
        let filter = builder.build().unwrap();
        assert_eq!(filter.status, Some(TaskStatus::Completed));
        assert_eq!(filter.priority, Some(Priority::High));
        assert_eq!(filter.category, Some("urgent".to_string()));
    }
}
