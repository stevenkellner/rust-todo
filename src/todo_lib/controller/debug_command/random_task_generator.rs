use crate::models::priority::Priority;
use crate::models::recurrence::Recurrence;
use crate::models::task::TaskWithoutId;
use rand::Rng;

/// Generates random tasks for testing purposes
pub struct RandomTaskGenerator {
    task_templates: Vec<&'static str>,
    subtask_templates: Vec<&'static str>,
    priorities: Vec<Priority>,
    categories: Vec<&'static str>,
    project_templates: Vec<&'static str>,
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
            subtask_templates: vec![
                "Research requirements",
                "Create outline",
                "Draft initial version",
                "Review and revise",
                "Get feedback",
                "Make final changes",
                "Test thoroughly",
                "Update documentation",
                "Notify stakeholders",
                "Archive old files",
                "Create backup",
                "Verify results",
                "Clean up code",
                "Add error handling",
                "Write tests",
                "Update changelog",
                "Deploy changes",
                "Monitor for issues",
                "Document decisions",
                "Schedule follow-up",
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
            project_templates: vec![
                "Work",
                "Personal",
                "Home",
                "Shopping",
                "Health",
                "Finance",
                "Learning",
                "Projects",
                "Goals",
                "Ideas",
                "Research",
                "Development",
                "Marketing",
                "Design",
                "Testing",
                "Documentation",
                "Client Work",
                "Side Projects",
                "Hobbies",
                "Travel",
            ],
        }
    }

    /// Generates a random task description
    pub fn generate_description(&self) -> String {
        let mut rng = rand::rng();
        let template_idx = rng.random_range(0..self.task_templates.len());
        self.task_templates[template_idx].to_string()
    }

    /// Generates a random subtask description
    pub fn generate_subtask_description(&self) -> String {
        let mut rng = rand::rng();
        let template_idx = rng.random_range(0..self.subtask_templates.len());
        self.subtask_templates[template_idx].to_string()
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

    /// Generates a random project name
    /// 
    /// # Returns
    /// 
    /// A random project name string
    pub fn generate_project_name(&self) -> String {
        let mut rng = rand::rng();
        let project_idx = rng.random_range(0..self.project_templates.len());
        self.project_templates[project_idx].to_string()
    }

    /// Generates multiple random project names
    /// 
    /// # Arguments
    /// 
    /// * `count` - Number of project names to generate
    /// 
    /// # Returns
    /// 
    /// A vector of unique project name strings
    pub fn generate_project_names(&self, count: usize) -> Vec<String> {
        let mut names = Vec::new();
        let mut used_indices = Vec::new();
        let mut rng = rand::rng();
        
        // Generate unique project names
        for i in 0..count {
            if i < self.project_templates.len() {
                // For the first pass, pick random unique names
                let mut idx;
                loop {
                    idx = rng.random_range(0..self.project_templates.len());
                    if !used_indices.contains(&idx) {
                        used_indices.push(idx);
                        break;
                    }
                }
                names.push(self.project_templates[idx].to_string());
            } else {
                // If we run out of unique names, add suffixes
                let base_idx = rng.random_range(0..self.project_templates.len());
                let base_name = self.project_templates[base_idx];
                names.push(format!("{} {}", base_name, i - self.project_templates.len() + 2));
            }
        }
        
        names
    }

    /// Generates an optional random recurrence pattern
    /// 
    /// # Arguments
    /// 
    /// * `probability` - Probability of generating a recurrence (0.0 to 1.0)
    /// 
    /// # Returns
    /// 
    /// `Some(Recurrence)` with the given probability, `None` otherwise
    pub fn generate_recurrence(&self, probability: f64) -> Option<Recurrence> {
        let mut rng = rand::rng();
        if rng.random_bool(probability) {
            let recurrence_patterns = [Recurrence::Daily, Recurrence::Weekly, Recurrence::Monthly];
            let pattern_idx = rng.random_range(0..recurrence_patterns.len());
            Some(recurrence_patterns[pattern_idx])
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
    /// * `recurrence_probability` - Probability of adding recurrence (default: 0.2)
    ///
    /// # Returns
    ///
    /// A `TaskWithoutId` with all randomly generated properties
    pub fn generate_single_task(
        &self,
        complete_probability: f64,
        due_date_probability: f64,
        category_probability: f64,
        recurrence_probability: f64,
    ) -> TaskWithoutId {
        let mut rng = rand::rng();
        
        // Generate task components
        let description = self.generate_description();
        let priority = self.generate_priority();
        let completed = rng.random_bool(complete_probability);
        let due_date = self.generate_due_date(due_date_probability);
        let category = self.generate_category(category_probability);
        let recurrence = self.generate_recurrence(recurrence_probability);
        
        TaskWithoutId {
            description,
            priority,
            completed,
            due_date,
            category,
            parent_id: None,
            recurrence,
            depends_on: Vec::new(),
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
            .map(|_| self.generate_single_task(0.3, 0.6, 0.7, 0.2))
            .collect()
    }

    /// Generates a random number of subtasks for a given parent
    ///
    /// # Arguments
    ///
    /// * `subtask_probability` - Probability of generating subtasks (0.0 to 1.0)
    ///
    /// # Returns
    ///
    /// Number of subtasks to generate (0-5)
    pub fn generate_subtask_count(&self, subtask_probability: f64) -> usize {
        let mut rng = rand::rng();
        if rng.random_bool(subtask_probability) {
            rng.random_range(1..6) // 1 to 5 subtasks
        } else {
            0
        }
    }

    /// Generates a single subtask for a parent task
    ///
    /// # Arguments
    ///
    /// * `complete_probability` - Probability of marking subtask as completed
    ///
    /// # Returns
    ///
    /// A `TaskWithoutId` configured as a subtask (parent_id will be set by caller)
    pub fn generate_single_subtask(&self, complete_probability: f64) -> TaskWithoutId {
        let mut rng = rand::rng();
        
        let description = self.generate_subtask_description();
        let priority = self.generate_priority();
        let completed = rng.random_bool(complete_probability);
        
        // Subtasks typically don't have due dates or categories - they inherit from parent
        TaskWithoutId {
            description,
            priority,
            completed,
            due_date: None,
            category: None,
            parent_id: None, // Will be set when added to TodoList
            recurrence: None, // Subtasks typically don't recur independently
            depends_on: Vec::new(),
        }
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
    fn test_default_trait() {
        let generator = RandomTaskGenerator::default();
        assert!(!generator.task_templates.is_empty());
    }

    #[test]
    fn test_generate_single_task() {
        let generator = RandomTaskGenerator::new();
        
        let task = generator.generate_single_task(0.5, 0.5, 0.5, 0.5);
        
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

    #[test]
    fn test_generate_subtask_description() {
        let generator = RandomTaskGenerator::new();
        
        let description = generator.generate_subtask_description();
        
        assert!(!description.is_empty());
        assert!(generator.subtask_templates.contains(&description.as_str()));
    }

    #[test]
    fn test_generate_subtask_count_zero_probability() {
        let generator = RandomTaskGenerator::new();
        
        // With 0.0 probability, should always return 0
        let count = generator.generate_subtask_count(0.0);
        assert_eq!(count, 0);
    }

    #[test]
    fn test_generate_subtask_count_full_probability() {
        let generator = RandomTaskGenerator::new();
        
        // With 1.0 probability, should return between 1 and 5
        let count = generator.generate_subtask_count(1.0);
        assert!((1..=5).contains(&count));
    }

    #[test]
    fn test_generate_single_subtask() {
        let generator = RandomTaskGenerator::new();
        
        let subtask = generator.generate_single_subtask(0.5);
        
        assert!(!subtask.description.is_empty());
        assert!(matches!(subtask.priority, Priority::High | Priority::Medium | Priority::Low));
        // Subtasks should not have due dates or categories
        assert!(subtask.due_date.is_none());
        assert!(subtask.category.is_none());
        // parent_id should be None initially (will be set by TodoList.add_subtask)
        assert!(subtask.parent_id.is_none());
    }

    #[test]
    fn test_generate_recurrence_zero_probability() {
        let generator = RandomTaskGenerator::new();
        
        // With 0.0 probability, should always return None
        let recurrence = generator.generate_recurrence(0.0);
        assert!(recurrence.is_none());
    }

    #[test]
    fn test_generate_recurrence_full_probability() {
        let generator = RandomTaskGenerator::new();
        
        // With 1.0 probability, should return a valid recurrence pattern
        let recurrence = generator.generate_recurrence(1.0);
        assert!(recurrence.is_some());
        let pattern = recurrence.unwrap();
        assert!(matches!(pattern, Recurrence::Daily | Recurrence::Weekly | Recurrence::Monthly));
    }

    #[test]
    fn test_generate_project_name() {
        let generator = RandomTaskGenerator::new();
        
        let project_name = generator.generate_project_name();
        
        assert!(!project_name.is_empty());
        assert!(generator.project_templates.contains(&project_name.as_str()));
    }

    #[test]
    fn test_generate_project_names() {
        let generator = RandomTaskGenerator::new();
        
        let project_names = generator.generate_project_names(5);
        
        assert_eq!(project_names.len(), 5);
        for name in &project_names {
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_generate_project_names_more_than_templates() {
        let generator = RandomTaskGenerator::new();
        let template_count = generator.project_templates.len();
        
        // Request more projects than we have templates
        let project_names = generator.generate_project_names(template_count + 5);
        
        assert_eq!(project_names.len(), template_count + 5);
        // All names should be unique or have suffixes
        for name in &project_names {
            assert!(!name.is_empty());
        }
    }

    #[test]
    fn test_generate_project_names_zero() {
        let generator = RandomTaskGenerator::new();
        
        let project_names = generator.generate_project_names(0);
        
        assert_eq!(project_names.len(), 0);
    }

}

