use crate::controller::command_controller::CommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_result::CommandControllerResultAction;
use crate::models::todo_list::TodoList;
use crate::controller::task_command::{TaskCommand, TaskSelection};
use crate::models::task::TaskWithoutId;
use crate::models::priority::Priority;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use crate::models::ParseError;
use crate::controller::task_command::TaskCommandInputParser;
use crate::controller::task_command::TaskCommandOutputManager;
use crate::OutputWriter;
use chrono::NaiveDate;
use std::rc::Rc;
use std::cell::RefCell;

/// Handles task-related commands and operations.
///
/// `TaskCommandController` encapsulates all operations related to individual tasks
/// such as completing, editing, setting priority, due date, and category.
pub struct TaskCommandController<O: OutputWriter> {
    todo_list: Rc<RefCell<TodoList>>,
    input_parser: TaskCommandInputParser,
    output_manager: TaskCommandOutputManager<O>,
}

impl<O: OutputWriter> TaskCommandController<O> {
    /// Creates a new task command handler with a custom output writer.
    pub fn new(todo_list: Rc<RefCell<TodoList>>, output_writer: Rc<RefCell<O>>) -> Self {
        Self {
            todo_list,
            input_parser: TaskCommandInputParser::new(),
            output_manager: TaskCommandOutputManager::new(output_writer),
        }
    }

    /// Handles a task command
    fn handle_command(&mut self, command: &TaskCommand) -> CommandControllerResult {
        match command {
            TaskCommand::Add(description) => self.add_task(description),
            TaskCommand::AddSubtask(parent_id, description) => self.add_subtask(*parent_id, description),
            TaskCommand::List(filter) => self.list_tasks(filter),
            TaskCommand::Remove(selection) => self.handle_remove(selection),
            TaskCommand::Complete(selection) => self.handle_complete(selection),
            TaskCommand::Uncomplete(selection) => self.handle_uncomplete(selection),
            TaskCommand::Toggle(selection) => self.handle_toggle(selection),
            TaskCommand::SetPriority(selection, priority) => self.handle_set_priority(selection, *priority),
            TaskCommand::SetDueDate(id, due_date) => self.set_due_date(*id, *due_date),
            TaskCommand::SetCategory(selection, category) => self.handle_set_category(selection, category.clone()),
            TaskCommand::SetRecurring(selection, recurrence) => self.handle_set_recurring(selection, *recurrence),
            TaskCommand::ListCategories => self.list_categories(),
            TaskCommand::Edit(id, new_description) => self.edit_task(*id, new_description),
            TaskCommand::Search(keyword) => self.search_tasks(keyword),
            TaskCommand::ShowStatistics => self.show_statistics(),
        }
    }

    fn add_task(&mut self, description: &str) -> CommandControllerResult {
        let new_task = TaskWithoutId::new(description.to_string());
        let task_id = self.todo_list.borrow_mut().add_task(new_task);
        self.output_manager.show_task_added(task_id, description);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    fn add_subtask(&mut self, parent_id: usize, description: &str) -> CommandControllerResult {
        match self.todo_list.borrow_mut().add_subtask(parent_id, description.to_string()) {
            Some(subtask_id) => {
                self.output_manager.show_subtask_added(subtask_id, parent_id, description);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            }
            None => {
                self.output_manager.show_task_not_found(parent_id);
                CommandControllerResult::default()
            }
        }
    }


    /// Lists tasks with optional filtering
    fn list_tasks(&mut self, filter: &Option<TaskFilter>) -> CommandControllerResult {
        match filter {
            None => self.output_manager.show_all_tasks_hierarchical(&self.todo_list.borrow()),
            Some(task_filter) => {
                let todo_list_ref = self.todo_list.borrow();
                let filtered_tasks = todo_list_ref.get_filtered_tasks(task_filter);
                
                if task_filter.status == Some(TaskStatus::Completed) && task_filter.priority.is_none() {
                    self.output_manager.show_completed_tasks(&filtered_tasks);
                } else if task_filter.status == Some(TaskStatus::Pending) && task_filter.priority.is_none() {
                    self.output_manager.show_pending_tasks(&filtered_tasks);
                } else if let Some(priority) = task_filter.priority {
                    self.output_manager.show_tasks_by_priority(&filtered_tasks, priority);
                } else {
                    self.output_manager.show_filtered_tasks(&filtered_tasks, task_filter);
                }
            }
        }
        CommandControllerResult::empty()
    }

    /// Removes a task by ID.
    fn remove_task(&mut self, id: usize) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().remove_task(id) {
            self.output_manager.show_task_removed(&task.description);
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Marks a task as completed.
    fn complete_task(&mut self, id: usize) -> CommandControllerResult {
        // Check if the task is recurring before completing it
        let is_recurring = self.todo_list.borrow()
            .get_tasks()
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.is_recurring())
            .unwrap_or(false);
        
        // Store task details before completing (for recurring task recreation)
        let recurring_task_data = if is_recurring {
            self.todo_list.borrow()
                .get_tasks()
                .iter()
                .find(|t| t.id == id)
                .map(|task| {
                    (
                        task.description.clone(),
                        task.priority,
                        task.category.clone(),
                        task.parent_id,
                        task.recurrence,
                        task.calculate_next_due_date(),
                    )
                })
        } else {
            None
        };
        
        // Store subtask details before completing (for recurring task subtask recreation)
        let subtasks_data: Vec<_> = if is_recurring {
            self.todo_list.borrow()
                .get_subtasks(id)
                .iter()
                .map(|subtask| {
                    (
                        subtask.description.clone(),
                        subtask.priority,
                    )
                })
                .collect()
        } else {
            Vec::new()
        };
        
        // Complete the task
        if let Some(task) = self.todo_list.borrow_mut().complete_task(id) {
            if task.is_completed() {
                self.output_manager.show_task_completed(&task.description);
            }
        } else {
            self.output_manager.show_task_not_found(id);
            return CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList);
        }
        
        // If task was recurring, create a new instance with the next due date
        if let Some((description, priority, category, parent_id, recurrence, next_due_date)) = recurring_task_data {
            let mut new_task = crate::models::task::TaskWithoutId::new(description.clone());
            new_task.priority = priority;
            new_task.category = category;
            new_task.parent_id = parent_id;
            new_task.recurrence = recurrence;
            new_task.due_date = next_due_date;
            
            let new_id = self.todo_list.borrow_mut().add_task(new_task);
            self.output_manager.show_recurring_task_created(new_id, &description);
            
            // Recreate subtasks as pending
            for (subtask_description, subtask_priority) in subtasks_data {
                let mut new_subtask = crate::models::task::TaskWithoutId::new(subtask_description);
                new_subtask.priority = subtask_priority;
                new_subtask.completed = false; // Ensure subtask is pending
                
                if let Some(subtask_id) = self.todo_list.borrow_mut().add_subtask(new_id, new_subtask.description) {
                    // Set the priority of the newly created subtask
                    self.todo_list.borrow_mut().set_task_priority(subtask_id, subtask_priority);
                }
            }
        }
        
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Marks a task as not completed.
    fn uncomplete_task(&mut self, id: usize) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().uncomplete_task(id) {
            if !task.is_completed() {
                self.output_manager.show_task_uncompleted(&task.description);
            }
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Completes multiple tasks by their IDs.
    fn complete_multiple_tasks(&mut self, ids: &[usize]) -> CommandControllerResult {
        // Collect recurring task data before completing
        let recurring_tasks_data: Vec<_> = self.todo_list.borrow()
            .get_tasks()
            .iter()
            .filter(|t| ids.contains(&t.id) && t.is_recurring())
            .map(|task| {
                let subtasks: Vec<_> = self.todo_list.borrow()
                    .get_subtasks(task.id)
                    .iter()
                    .map(|subtask| {
                        (
                            subtask.description.clone(),
                            subtask.priority,
                        )
                    })
                    .collect();
                
                (
                    task.description.clone(),
                    task.priority,
                    task.category.clone(),
                    task.parent_id,
                    task.recurrence,
                    task.calculate_next_due_date(),
                    subtasks,
                )
            })
            .collect();
        
        let (completed_count, not_found) = self.todo_list.borrow_mut().complete_tasks(ids);
        
        self.output_manager.show_multiple_tasks_completed(completed_count, &not_found);
        
        // Create new instances for recurring tasks
        for (description, priority, category, parent_id, recurrence, next_due_date, subtasks_data) in recurring_tasks_data {
            let mut new_task = crate::models::task::TaskWithoutId::new(description.clone());
            new_task.priority = priority;
            new_task.category = category;
            new_task.parent_id = parent_id;
            new_task.recurrence = recurrence;
            new_task.due_date = next_due_date;
            
            let new_id = self.todo_list.borrow_mut().add_task(new_task);
            self.output_manager.show_recurring_task_created(new_id, &description);
            
            // Recreate subtasks as pending
            for (subtask_description, subtask_priority) in subtasks_data {
                let mut new_subtask = crate::models::task::TaskWithoutId::new(subtask_description);
                new_subtask.priority = subtask_priority;
                new_subtask.completed = false; // Ensure subtask is pending
                
                if let Some(subtask_id) = self.todo_list.borrow_mut().add_subtask(new_id, new_subtask.description) {
                    // Set the priority of the newly created subtask
                    self.todo_list.borrow_mut().set_task_priority(subtask_id, subtask_priority);
                }
            }
        }
        
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Completes all tasks.
    fn complete_all_tasks(&mut self) -> CommandControllerResult {
        // Collect recurring task data before completing
        let recurring_tasks_data: Vec<_> = self.todo_list.borrow()
            .get_tasks()
            .iter()
            .filter(|t| !t.is_completed() && t.is_recurring())
            .map(|task| {
                let subtasks: Vec<_> = self.todo_list.borrow()
                    .get_subtasks(task.id)
                    .iter()
                    .map(|subtask| {
                        (
                            subtask.description.clone(),
                            subtask.priority,
                        )
                    })
                    .collect();
                
                (
                    task.description.clone(),
                    task.priority,
                    task.category.clone(),
                    task.parent_id,
                    task.recurrence,
                    task.calculate_next_due_date(),
                    subtasks,
                )
            })
            .collect();
        
        let count = self.todo_list.borrow_mut().complete_all_tasks();
        
        self.output_manager.show_all_tasks_completed(count);
        
        // Create new instances for recurring tasks
        for (description, priority, category, parent_id, recurrence, next_due_date, subtasks_data) in recurring_tasks_data {
            let mut new_task = crate::models::task::TaskWithoutId::new(description.clone());
            new_task.priority = priority;
            new_task.category = category;
            new_task.parent_id = parent_id;
            new_task.recurrence = recurrence;
            new_task.due_date = next_due_date;
            
            let new_id = self.todo_list.borrow_mut().add_task(new_task);
            self.output_manager.show_recurring_task_created(new_id, &description);
            
            // Recreate subtasks as pending
            for (subtask_description, subtask_priority) in subtasks_data {
                let mut new_subtask = crate::models::task::TaskWithoutId::new(subtask_description);
                new_subtask.priority = subtask_priority;
                new_subtask.completed = false; // Ensure subtask is pending
                
                if let Some(subtask_id) = self.todo_list.borrow_mut().add_subtask(new_id, new_subtask.description) {
                    // Set the priority of the newly created subtask
                    self.todo_list.borrow_mut().set_task_priority(subtask_id, subtask_priority);
                }
            }
        }
        
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Removes multiple tasks by their IDs.
    fn remove_multiple_tasks(&mut self, ids: &[usize]) -> CommandControllerResult {
        let (removed_count, not_found) = self.todo_list.borrow_mut().remove_tasks(ids);
        
        self.output_manager.show_multiple_tasks_removed(removed_count, &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Removes all tasks.
    fn remove_all_tasks(&mut self) -> CommandControllerResult {
        let count = self.todo_list.borrow_mut().remove_all_tasks();
        
        self.output_manager.show_all_tasks_removed(count);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Marks multiple tasks as pending (incomplete) by their IDs.
    fn uncomplete_multiple_tasks(&mut self, ids: &[usize]) -> CommandControllerResult {
        let (uncompleted_count, not_found) = self.todo_list.borrow_mut().uncomplete_tasks(ids);
        
        self.output_manager.show_multiple_tasks_uncompleted(uncompleted_count, &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Marks all tasks as pending (incomplete).
    fn uncomplete_all_tasks(&mut self) -> CommandControllerResult {
        let count = self.todo_list.borrow_mut().uncomplete_all_tasks();
        
        self.output_manager.show_all_tasks_uncompleted(count);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Toggles multiple tasks' completion status by their IDs.
    fn toggle_multiple_tasks(&mut self, ids: &[usize]) -> CommandControllerResult {
        let (toggled_count, not_found) = self.todo_list.borrow_mut().toggle_tasks(ids);
        
        self.output_manager.show_multiple_tasks_toggled(toggled_count, &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Toggles all tasks' completion status.
    fn toggle_all_tasks(&mut self) -> CommandControllerResult {
        let count = self.todo_list.borrow_mut().toggle_all_tasks();
        
        self.output_manager.show_all_tasks_toggled(count);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the priority of multiple tasks by their IDs.
    fn set_priority_multiple(&mut self, ids: &[usize], priority: Priority) -> CommandControllerResult {
        let (updated_count, not_found) = self.todo_list.borrow_mut().set_priority_multiple(ids, priority);
        
        self.output_manager.show_multiple_priorities_set(updated_count, priority, &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the category of multiple tasks by their IDs.
    fn set_category_multiple(&mut self, ids: &[usize], category: Option<String>) -> CommandControllerResult {
        let (updated_count, not_found) = self.todo_list.borrow_mut().set_category_multiple(ids, category.clone());
        
        self.output_manager.show_multiple_categories_set(updated_count, category.as_deref(), &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Toggles a task's completion status.
    fn toggle_task(&mut self, id: usize) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().toggle_task(id) {
            self.output_manager.show_task_toggled(&task.description, task.is_completed());
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the priority of a task.
    fn set_priority(&mut self, id: usize, priority: Priority) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().set_task_priority(id, priority) {
            self.output_manager.show_priority_set(&task.description, priority);
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the due date of a task.
    fn set_due_date(&mut self, id: usize, due_date: Option<NaiveDate>) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().set_due_date(id, due_date) {
            self.output_manager.show_due_date_set(&task.description, due_date);
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the category of a task.
    fn set_category(&mut self, id: usize, category: Option<String>) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().set_task_category(id, category.clone()) {
            self.output_manager.show_category_set(&task.description, category);
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the recurrence pattern of a task.
    fn set_recurring(&mut self, id: usize, recurrence: Option<crate::models::recurrence::Recurrence>) -> CommandControllerResult {
        if let Some(task) = self.todo_list.borrow_mut().set_task_recurrence(id, recurrence) {
            self.output_manager.show_recurrence_set(&task.description, recurrence);
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Sets the recurrence pattern of multiple tasks by their IDs.
    fn set_recurring_multiple(&mut self, ids: &[usize], recurrence: Option<crate::models::recurrence::Recurrence>) -> CommandControllerResult {
        let (updated_count, not_found) = self.todo_list.borrow_mut().set_recurrence_multiple(ids, recurrence);
        
        self.output_manager.show_multiple_recurrences_set(updated_count, recurrence, &not_found);
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    /// Handles remove command based on TaskSelection.
    fn handle_remove(&mut self, selection: &TaskSelection) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.remove_task(*id),
            TaskSelection::Multiple(ids) => self.remove_multiple_tasks(ids),
            TaskSelection::All => self.remove_all_tasks(),
        }
    }

    /// Handles complete command based on TaskSelection.
    fn handle_complete(&mut self, selection: &TaskSelection) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.complete_task(*id),
            TaskSelection::Multiple(ids) => self.complete_multiple_tasks(ids),
            TaskSelection::All => self.complete_all_tasks(),
        }
    }

    /// Handles uncomplete command based on TaskSelection.
    fn handle_uncomplete(&mut self, selection: &TaskSelection) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.uncomplete_task(*id),
            TaskSelection::Multiple(ids) => self.uncomplete_multiple_tasks(ids),
            TaskSelection::All => self.uncomplete_all_tasks(),
        }
    }

    /// Handles toggle command based on TaskSelection.
    fn handle_toggle(&mut self, selection: &TaskSelection) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.toggle_task(*id),
            TaskSelection::Multiple(ids) => self.toggle_multiple_tasks(ids),
            TaskSelection::All => self.toggle_all_tasks(),
        }
    }

    /// Handles set priority command based on TaskSelection.
    fn handle_set_priority(&mut self, selection: &TaskSelection, priority: Priority) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.set_priority(*id, priority),
            TaskSelection::Multiple(ids) => self.set_priority_multiple(ids, priority),
            TaskSelection::All => {
                let all_ids: Vec<usize> = self.todo_list.borrow().get_tasks().iter().map(|t| t.id).collect();
                self.set_priority_multiple(&all_ids, priority)
            }
        }
    }

    /// Handles set category command based on TaskSelection.
    fn handle_set_category(&mut self, selection: &TaskSelection, category: Option<String>) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.set_category(*id, category),
            TaskSelection::Multiple(ids) => self.set_category_multiple(ids, category),
            TaskSelection::All => {
                let all_ids: Vec<usize> = self.todo_list.borrow().get_tasks().iter().map(|t| t.id).collect();
                self.set_category_multiple(&all_ids, category)
            }
        }
    }

    fn handle_set_recurring(&mut self, selection: &TaskSelection, recurrence: Option<crate::models::recurrence::Recurrence>) -> CommandControllerResult {
        match selection {
            TaskSelection::Single(id) => self.set_recurring(*id, recurrence),
            TaskSelection::Multiple(ids) => self.set_recurring_multiple(ids, recurrence),
            TaskSelection::All => {
                let all_ids: Vec<usize> = self.todo_list.borrow().get_tasks().iter().map(|t| t.id).collect();
                self.set_recurring_multiple(&all_ids, recurrence)
            }
        }
    }

    fn list_categories(&mut self) -> CommandControllerResult {
        self.output_manager.show_categories(&self.todo_list.borrow());
        CommandControllerResult::empty()
    }

    /// Edits a task's description.
    fn edit_task(&mut self, id: usize, new_description: &str) -> CommandControllerResult {
        if new_description.trim().is_empty() {
            self.output_manager.show_error("Task description cannot be empty.");
            return CommandControllerResult::empty();
        }

        // Get the old description before editing
        let old_description = self.todo_list.borrow().get_tasks()
            .iter()
            .find(|task| task.id == id)
            .map(|task| task.description.clone());

        if let Some(_task) = self.todo_list.borrow_mut().edit_task(id, new_description.to_string()) {
            if let Some(old_desc) = old_description {
                self.output_manager.show_task_edited(&old_desc, new_description);
            }
        } else {
            self.output_manager.show_task_not_found(id);
        }
        CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
    }

    fn search_tasks(&mut self, keyword: &str) -> CommandControllerResult {
        self.output_manager.show_search_results(&self.todo_list.borrow(), keyword);
        CommandControllerResult::empty()
    }

    fn show_statistics(&mut self) -> CommandControllerResult {
        self.output_manager.show_statistics(&self.todo_list.borrow());
        CommandControllerResult::empty()
    }
}

impl<O: OutputWriter> CommandController for TaskCommandController<O> {
    fn try_execute(&mut self, input: &str) -> Option<Result<CommandControllerResult, ParseError>> {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return None;
        }

        let command = parts[0].to_lowercase();
        let args = &parts[1..];

        match self.input_parser.try_parse(&command, args) {
            Some(Ok(cmd)) => {
                let result = self.handle_command(&cmd);
                Some(Ok(result))
            }
            Some(Err(err)) => Some(Err(err)),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_task_existing() {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        let id = todo_list.borrow_mut().add_task(TaskWithoutId::new("Test task".to_string()));
        let buffer = Vec::new();
        let output_writer = crate::ui::output::FileOutputWriter::new(buffer);
        let mut handler = TaskCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));
        
        handler.remove_task(id);

        assert!(!todo_list.borrow().get_tasks().iter().any(|t| t.id == id));
    }

    #[test]
    fn test_complete_task() {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        let id = todo_list.borrow_mut().add_task(TaskWithoutId::new("Test task".to_string()));
        let buffer = Vec::new();
        let output_writer = crate::ui::output::FileOutputWriter::new(buffer);
        let mut handler = TaskCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));
        
        handler.complete_task(id);
        
        assert!(todo_list.borrow().get_tasks().iter().find(|t| t.id == id).unwrap().is_completed());
    }

    #[test]
    fn test_set_priority() {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        let id = todo_list.borrow_mut().add_task(TaskWithoutId::new("Test task".to_string()));
        let buffer = Vec::new();
        let output_writer = crate::ui::output::FileOutputWriter::new(buffer);
        let mut handler = TaskCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));
        
        handler.set_priority(id, Priority::High);
        
        assert_eq!(todo_list.borrow().get_tasks().iter().find(|t| t.id == id).unwrap().priority, Priority::High);
    }

    #[test]
    fn test_edit_task() {
        let todo_list = Rc::new(RefCell::new(TodoList::new()));
        let id = todo_list.borrow_mut().add_task(TaskWithoutId::new("Old description".to_string()));
        let buffer = Vec::new();
        let output_writer = crate::ui::output::FileOutputWriter::new(buffer);
        let mut handler = TaskCommandController::new(Rc::clone(&todo_list), Rc::new(RefCell::new(output_writer)));
        
        handler.edit_task(id, "New description");
        
        assert_eq!(todo_list.borrow().get_tasks().iter().find(|t| t.id == id).unwrap().description, "New description");
    }
}
