use crate::models::priority::Priority;
use crate::models::task::TaskWithoutId;
use rand::Rng;

/// Generates random tasks for testing purposes
pub struct RandomTaskGenerator {
    task_templates: Vec<&'static str>,
    priorities: Vec<Priority>,
    categories: Vec<&'static str>,
}

impl RandomTaskGenerator {
    /// Creates a new RandomTaskGenerator with default templates
    pub fn new() -> Self {
        Self {
            task_templates: vec![
                "Buy groceries",
                "Write documentation",
                "Review pull requests",
                "Update dependencies",
                "Fix bug in authentication",
                "Implement new feature",
                "Refactor legacy code",
                "Write unit tests",
                "Deploy to production",
                "Meeting with team",
                "Code review session",
                "Update README",
                "Optimize database queries",
                "Design new UI",
                "Research new technology",
                "Client presentation",
                "Performance testing",
                "Security audit",
                "Backup database",
                "Configure CI/CD pipeline",
            ],
            priorities: vec![Priority::High, Priority::Medium, Priority::Low],
            categories: vec![
                "work",
                "personal",
                "urgent",
                "bug",
                "feature",
                "documentation",
                "testing",
                "deployment",
                "maintenance",
                "research",
            ],
        }
    }

    /// Generates a random task description
    pub fn generate_description(&self) -> String {
        let mut rng = rand::rng();
        let template_idx = rng.random_range(0..self.task_templates.len());
        self.task_templates[template_idx].to_string()
    }

    /// Generates a random priority
    pub fn generate_priority(&self) -> Priority {
        let mut rng = rand::rng();
        let priority_idx = rng.random_range(0..self.priorities.len());
        self.priorities[priority_idx]
    }

    /// Generates an optional random due date
    /// 
    /// # Arguments
    /// 
    /// * `probability` - Probability of generating a due date (0.0 to 1.0)
    /// 
    /// # Returns
    /// 
    /// `Some(NaiveDate)` with the given probability, `None` otherwise
    pub fn generate_due_date(&self, probability: f64) -> Option<chrono::NaiveDate> {
        let mut rng = rand::rng();
        if rng.random_bool(probability) {
            let today = chrono::Local::now().date_naive();
            // Generate due dates from -7 days to +30 days
            let days_offset = rng.random_range(-7..31);
            Some(today + chrono::Duration::days(days_offset))
        } else {
            None
        }
    }

    /// Generates an optional random category
    /// 
    /// # Arguments
    /// 
    /// * `probability` - Probability of generating a category (0.0 to 1.0)
    /// 
    /// # Returns
    /// 
    /// `Some(String)` with the given probability, `None` otherwise
    pub fn generate_category(&self, probability: f64) -> Option<String> {
        let mut rng = rand::rng();
        if rng.random_bool(probability) {
            let category_idx = rng.random_range(0..self.categories.len());
            Some(self.categories[category_idx].to_string())
        } else {
            None
        }
    }

    /// Generates a single random task with all properties
    ///
    /// # Arguments
    ///
    /// * `complete_probability` - Probability of marking task as completed (default: 0.3)
    /// * `due_date_probability` - Probability of adding due date (default: 0.6)
    /// * `category_probability` - Probability of adding category (default: 0.7)
    ///
    /// # Returns
    ///
    /// A `TaskWithoutId` with all randomly generated properties
    pub fn generate_single_task(
        &self,
        complete_probability: f64,
        due_date_probability: f64,
        category_probability: f64,
    ) -> TaskWithoutId {
        let mut rng = rand::rng();
        
        // Generate task components
        let description = self.generate_description();
        let priority = self.generate_priority();
        let completed = rng.random_bool(complete_probability);
        let due_date = self.generate_due_date(due_date_probability);
        let category = self.generate_category(category_probability);
        
        TaskWithoutId {
            description,
            priority,
            completed,
            due_date,
            category,
        }
    }

    /// Generates multiple random tasks
    ///
    /// # Arguments
    ///
    /// * `count` - Number of random tasks to generate
    ///
    /// # Returns
    ///
    /// A vector of `TaskWithoutId` objects
    pub fn generate_tasks(&self, count: usize) -> Vec<TaskWithoutId> {
        (0..count)
            .map(|_| self.generate_single_task(0.3, 0.6, 0.7))
            .collect()
    }

    /// Generates random tasks (legacy method for backward compatibility)
    ///
    /// # Arguments
    ///
    /// * `count` - Number of random tasks to generate
    ///
    /// # Returns
    ///
    /// A vector of `TaskWithoutId` objects
    pub fn generate(&self, count: usize) -> Vec<TaskWithoutId> {
        self.generate_tasks(count)
    }
}

impl Default for RandomTaskGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_generator() {
        let generator = RandomTaskGenerator::new();
        assert!(!generator.task_templates.is_empty());
        assert!(!generator.priorities.is_empty());
        assert!(!generator.categories.is_empty());
    }

    #[test]
    fn test_generate_tasks() {
        let generator = RandomTaskGenerator::new();
        
        let tasks = generator.generate(5);
        
        assert_eq!(tasks.len(), 5);
        for task in &tasks {
            assert!(!task.description.is_empty());
        }
    }

    #[test]
    fn test_generate_zero_tasks() {
        let generator = RandomTaskGenerator::new();
        
        let tasks = generator.generate(0);
        
        assert_eq!(tasks.len(), 0);
    }

    #[test]
    fn test_generated_tasks_have_properties() {
        let generator = RandomTaskGenerator::new();
        
        let tasks = generator.generate(10);
        
        // All tasks should have properties set
        assert_eq!(tasks.len(), 10);
        
        // Verify tasks have descriptions and priorities
        for task in &tasks {
            assert!(!task.description.is_empty());
            // Priority should be one of the defined values
            assert!(matches!(task.priority, Priority::High | Priority::Medium | Priority::Low));
        }
        
        // Some tasks should have categories and due dates (due to randomness)
        let has_category = tasks.iter().any(|t| t.category.is_some());
        let has_due_date = tasks.iter().any(|t| t.due_date.is_some());
        let has_completed = tasks.iter().any(|t| t.completed);
        
        assert!(has_category, "At least one task should have a category");
        assert!(has_due_date, "At least one task should have a due date");
        assert!(has_completed, "At least one task should be completed");
    }

    #[test]
    fn test_default_trait() {
        let generator = RandomTaskGenerator::default();
        assert!(!generator.task_templates.is_empty());
    }

    #[test]
    fn test_generate_single_task() {
        let generator = RandomTaskGenerator::new();
        
        let task = generator.generate_single_task(0.5, 0.5, 0.5);
        
        assert!(!task.description.is_empty());
        assert!(matches!(task.priority, Priority::High | Priority::Medium | Priority::Low));
    }

    #[test]
    fn test_generate_description() {
        let generator = RandomTaskGenerator::new();
        
        let description = generator.generate_description();
        
        assert!(!description.is_empty());
        assert!(generator.task_templates.contains(&description.as_str()));
    }

    #[test]
    fn test_generate_priority() {
        let generator = RandomTaskGenerator::new();
        
        let priority = generator.generate_priority();
        
        assert!(matches!(priority, Priority::High | Priority::Medium | Priority::Low));
    }

}
