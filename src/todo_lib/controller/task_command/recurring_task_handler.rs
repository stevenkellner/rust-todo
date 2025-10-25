use crate::controller::project_command::ProjectManager;
use crate::models::priority::Priority;
use crate::models::recurrence::Recurrence;
use crate::models::task::{Task, TaskWithoutId};
use chrono::NaiveDate;
use std::cell::RefCell;
use std::rc::Rc;

/// Data structure to hold information about a recurring task
#[derive(Debug, Clone)]
pub struct RecurringTaskData {
    pub description: String,
    pub priority: Priority,
    pub category: Option<String>,
    pub parent_id: Option<usize>,
    pub recurrence: Option<Recurrence>,
    pub next_due_date: Option<NaiveDate>,
    pub subtasks: Vec<SubtaskData>,
}

#[derive(Debug, Clone)]
pub struct SubtaskData {
    pub description: String,
    pub priority: Priority,
}

/// Handles recurring task logic including data collection, recreation, and subtask management
pub struct RecurringTaskHandler {
    project_manager: Rc<RefCell<ProjectManager>>,
}

impl RecurringTaskHandler {
    pub fn new(project_manager: Rc<RefCell<ProjectManager>>) -> Self {
        Self { project_manager }
    }

    /// Collects data from a single recurring task by ID
    pub fn collect_recurring_task_data(&self, id: usize) -> Option<RecurringTaskData> {
        let project_manager = self.project_manager.borrow();
        let todo_list = project_manager.get_current_todo_list();
        let task = todo_list.get_tasks().iter().find(|t| t.id == id)?;

        if !task.is_recurring() {
            return None;
        }

        Some(self.extract_task_data(task))
    }

    /// Collects data from multiple recurring tasks by IDs
    pub fn collect_multiple_recurring_tasks(&self, ids: &[usize]) -> Vec<RecurringTaskData> {
        let project_manager = self.project_manager.borrow();
        let todo_list = project_manager.get_current_todo_list();

        // First pass: collect task info (without subtasks) to avoid nested borrows
        let recurring_task_ids_and_info: Vec<_> = todo_list
            .get_tasks()
            .iter()
            .filter(|t| ids.contains(&t.id) && t.is_recurring())
            .map(|task| {
                (
                    task.id,
                    task.description.clone(),
                    task.priority,
                    task.category.clone(),
                    task.parent_id,
                    task.recurrence,
                    task.calculate_next_due_date(),
                )
            })
            .collect();

        let _ = todo_list; // Explicitly release borrow
        let _ = project_manager;

        // Second pass: collect subtasks for each recurring task
        recurring_task_ids_and_info
            .into_iter()
            .map(
                |(
                    task_id,
                    description,
                    priority,
                    category,
                    parent_id,
                    recurrence,
                    next_due_date,
                )| {
                    let subtasks = self.collect_subtasks(task_id);
                    RecurringTaskData {
                        description,
                        priority,
                        category,
                        parent_id,
                        recurrence,
                        next_due_date,
                        subtasks,
                    }
                },
            )
            .collect()
    }

    /// Collects data from all pending recurring tasks
    pub fn collect_all_pending_recurring_tasks(&self) -> Vec<RecurringTaskData> {
        let project_manager = self.project_manager.borrow();
        let todo_list = project_manager.get_current_todo_list();

        // First pass: collect task info (without subtasks)
        let recurring_task_ids_and_info: Vec<_> = todo_list
            .get_tasks()
            .iter()
            .filter(|t| !t.is_completed() && t.is_recurring())
            .map(|task| {
                (
                    task.id,
                    task.description.clone(),
                    task.priority,
                    task.category.clone(),
                    task.parent_id,
                    task.recurrence,
                    task.calculate_next_due_date(),
                )
            })
            .collect();

        let _ = todo_list; // Explicitly release borrow
        let _ = project_manager;

        // Second pass: collect subtasks
        recurring_task_ids_and_info
            .into_iter()
            .map(
                |(
                    task_id,
                    description,
                    priority,
                    category,
                    parent_id,
                    recurrence,
                    next_due_date,
                )| {
                    let subtasks = self.collect_subtasks(task_id);
                    RecurringTaskData {
                        description,
                        priority,
                        category,
                        parent_id,
                        recurrence,
                        next_due_date,
                        subtasks,
                    }
                },
            )
            .collect()
    }

    /// Recreates a recurring task with its subtasks
    pub fn recreate_recurring_task(&self, data: &RecurringTaskData) -> usize {
        // Create new parent task
        let mut new_task = TaskWithoutId::new(data.description.clone());
        new_task.priority = data.priority;
        new_task.category = data.category.clone();
        new_task.parent_id = data.parent_id;
        new_task.recurrence = data.recurrence;
        new_task.due_date = data.next_due_date;

        let new_id = self
            .project_manager
            .borrow_mut()
            .get_current_todo_list_mut()
            .add_task(new_task);

        // Recreate subtasks
        self.recreate_subtasks(new_id, &data.subtasks);

        new_id
    }

    /// Recreates multiple recurring tasks
    pub fn recreate_multiple_recurring_tasks(
        &self,
        tasks_data: &[RecurringTaskData],
    ) -> Vec<usize> {
        tasks_data
            .iter()
            .map(|data| self.recreate_recurring_task(data))
            .collect()
    }

    // Private helper methods

    fn extract_task_data(&self, task: &Task) -> RecurringTaskData {
        let subtasks = self.collect_subtasks(task.id);

        RecurringTaskData {
            description: task.description.clone(),
            priority: task.priority,
            category: task.category.clone(),
            parent_id: task.parent_id,
            recurrence: task.recurrence,
            next_due_date: task.calculate_next_due_date(),
            subtasks,
        }
    }

    fn collect_subtasks(&self, parent_id: usize) -> Vec<SubtaskData> {
        self.project_manager
            .borrow()
            .get_current_todo_list()
            .get_subtasks(parent_id)
            .iter()
            .map(|subtask| SubtaskData {
                description: subtask.description.clone(),
                priority: subtask.priority,
            })
            .collect()
    }

    fn recreate_subtasks(&self, parent_id: usize, subtasks: &[SubtaskData]) {
        for subtask_data in subtasks {
            let mut new_subtask = TaskWithoutId::new(subtask_data.description.clone());
            new_subtask.priority = subtask_data.priority;
            new_subtask.completed = false; // Ensure subtask is pending

            // Add subtask and get its ID, releasing the borrow before setting priority
            let subtask_id = self
                .project_manager
                .borrow_mut()
                .get_current_todo_list_mut()
                .add_subtask(parent_id, new_subtask.description);
            if let Some(subtask_id) = subtask_id {
                // Set the priority of the newly created subtask
                self.project_manager
                    .borrow_mut()
                    .get_current_todo_list_mut()
                    .set_task_priority(subtask_id, subtask_data.priority);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::recurrence::Recurrence;

    #[test]
    fn test_collect_recurring_task_data() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let handler = RecurringTaskHandler::new(Rc::clone(&project_manager));

        // Add a recurring task
        let mut task = TaskWithoutId::new("Daily task".to_string());
        task.recurrence = Some(Recurrence::Daily);
        let task_id = project_manager
            .borrow_mut()
            .get_current_todo_list_mut()
            .add_task(task);

        // Collect data
        let data = handler.collect_recurring_task_data(task_id);
        assert!(data.is_some());
        assert_eq!(data.unwrap().description, "Daily task");
    }

    #[test]
    fn test_collect_non_recurring_returns_none() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let handler = RecurringTaskHandler::new(Rc::clone(&project_manager));

        // Add a non-recurring task
        let task = TaskWithoutId::new("Normal task".to_string());
        let task_id = project_manager
            .borrow_mut()
            .get_current_todo_list_mut()
            .add_task(task);

        // Collect data should return None
        let data = handler.collect_recurring_task_data(task_id);
        assert!(data.is_none());
    }

    #[test]
    fn test_recreate_recurring_task() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let handler = RecurringTaskHandler::new(Rc::clone(&project_manager));

        let data = RecurringTaskData {
            description: "Weekly task".to_string(),
            priority: Priority::High,
            category: Some("work".to_string()),
            parent_id: None,
            recurrence: Some(Recurrence::Weekly),
            next_due_date: None,
            subtasks: vec![],
        };

        let new_id = handler.recreate_recurring_task(&data);

        let binding = project_manager.borrow();
        let tasks = binding.get_current_todo_list().get_tasks();
        let task = &tasks[0];
        assert_eq!(task.id, new_id);
        assert_eq!(task.description, "Weekly task");
        assert_eq!(task.priority, Priority::High);
    }

    #[test]
    fn test_recreate_with_subtasks() {
        let project_manager = Rc::new(RefCell::new(ProjectManager::new()));
        let handler = RecurringTaskHandler::new(Rc::clone(&project_manager));

        let data = RecurringTaskData {
            description: "Parent task".to_string(),
            priority: Priority::Medium,
            category: None,
            parent_id: None,
            recurrence: Some(Recurrence::Daily),
            next_due_date: None,
            subtasks: vec![
                SubtaskData {
                    description: "Subtask 1".to_string(),
                    priority: Priority::High,
                },
                SubtaskData {
                    description: "Subtask 2".to_string(),
                    priority: Priority::Low,
                },
            ],
        };

        let new_id = handler.recreate_recurring_task(&data);

        let binding = project_manager.borrow();
        let subtasks = binding.get_current_todo_list().get_subtasks(new_id);
        assert_eq!(subtasks.len(), 2);
        assert_eq!(subtasks[0].description, "Subtask 1");
        assert_eq!(subtasks[1].description, "Subtask 2");
    }
}
