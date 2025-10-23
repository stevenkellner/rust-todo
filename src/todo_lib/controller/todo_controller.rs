use crate::models::todo_list::TodoList;
use crate::ui::input_reader::InputReader;
use crate::ui::output_writer::OutputWriter;
use crate::models::ui_event::UiEvent;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use crate::models::loop_control::LoopControl;
use crate::models::priority::Priority;

/// Controls the todo list application by reacting to UI events.
///
/// `TodoController` acts as the controller layer, handling events from the UI
/// and managing the todo list state. It processes events and delegates
/// display responsibilities back to the UI.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::controller::todo_controller::TodoController;
///
/// let mut controller = TodoController::new();
/// controller.run();
/// ```
pub struct TodoController {
    todo_list: TodoList,
    input: InputReader,
    output: OutputWriter,
}

impl TodoController {
    /// Creates a new controller with an empty todo list and new UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::controller::todo_controller::TodoController;
    ///
    /// let controller = TodoController::new();
    /// ```
    pub fn new() -> Self {
        TodoController {
            todo_list: TodoList::new(),
            input: InputReader::new(),
            output: OutputWriter::new(),
        }
    }

    /// Starts the interactive command loop.
    ///
    /// This method displays a welcome message and enters an event loop
    /// that processes UI events until the user quits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::controller::todo_controller::TodoController;
    ///
    /// let mut controller = TodoController::new();
    /// controller.run();
    /// ```
    pub fn run(&mut self) {
        self.output.show_welcome();

        loop {
            self.output.print_prompt();
            let event = self.input.read_event();
            
            if self.handle_event(&event) == LoopControl::Exit {
                break;
            }
        }
    }

    /// Handles a UI event and returns the loop control decision.
    ///
    /// # Arguments
    ///
    /// * `event` - The UI event to handle
    ///
    /// # Returns
    ///
    /// `LoopControl::Continue` to continue the event loop, `LoopControl::Exit` to quit
    fn handle_event(&mut self, event: &UiEvent) -> LoopControl {
        match event {
            UiEvent::AddTask(description) => self.handle_add_task(description),
            UiEvent::ListTasks(filter) => self.handle_list_tasks(filter),
            UiEvent::RemoveTask(id) => self.handle_remove_task(*id),
            UiEvent::CompleteTask(id) => self.handle_complete_task(*id),
            UiEvent::UncompleteTask(id) => self.handle_uncomplete_task(*id),
            UiEvent::ToggleTask(id) => self.handle_toggle_task(*id),
            UiEvent::SetPriority(id, priority) => self.handle_set_priority(*id, *priority),
            UiEvent::SearchTasks(keyword) => self.handle_search_tasks(keyword),
            UiEvent::ShowHelp => self.handle_show_help(),
            UiEvent::Quit => return self.handle_quit(),
            UiEvent::UnknownCommand(command) => self.handle_unknown_command(command),
            UiEvent::InvalidInput(message) => self.handle_invalid_input(message),
        }
        
        LoopControl::Continue
    }

    /// Handles the AddTask event.
    fn handle_add_task(&mut self, description: &str) {
        let task_id = self.todo_list.add_task(description.to_string());
        self.output.show_task_added(task_id, description);
    }

    /// Handles the ListTasks event with optional filter.
    fn handle_list_tasks(&mut self, filter: &Option<TaskFilter>) {
        let tasks = match filter {
            Some(filter) => self.todo_list.get_filtered_tasks(filter),
            None => self.todo_list.get_tasks().iter().collect(),
        };
        
        // Determine the appropriate display method based on filter
        match filter {
            Some(f) if f.status == Some(TaskStatus::Completed) && f.priority.is_none() => {
                self.output.show_completed_tasks(&tasks);
            }
            Some(f) if f.status == Some(TaskStatus::Pending) && f.priority.is_none() => {
                self.output.show_pending_tasks(&tasks);
            }
            Some(f) if f.status.is_none() && f.priority.is_some() => {
                self.output.show_tasks_by_priority(&tasks, f.priority.unwrap());
            }
            Some(f) => {
                // Combined filters
                self.output.show_filtered_tasks(&tasks, f);
            }
            None => {
                let all_tasks = self.todo_list.get_tasks();
                self.output.show_all_tasks(all_tasks);
            }
        }
    }

    /// Handles the RemoveTask event.
    fn handle_remove_task(&mut self, id: usize) {
        if let Some(removed_task) = self.todo_list.remove_task(id) {
            self.output.show_task_removed(&removed_task.description);
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Handles the CompleteTask event.
    fn handle_complete_task(&mut self, id: usize) {
        match self.todo_list.complete_task(id) {
            Some(task) => {
                if task.is_completed() {
                    self.output.show_task_completed(&task.description);
                }
            }
            None => {
                self.output.show_task_not_found(id);
            }
        }
    }

    /// Handles the UncompleteTask event.
    fn handle_uncomplete_task(&mut self, id: usize) {
        match self.todo_list.uncomplete_task(id) {
            Some(task) => {
                if !task.is_completed() {
                    self.output.show_task_uncompleted(&task.description);
                }
            }
            None => {
                self.output.show_task_not_found(id);
            }
        }
    }

    /// Handles the ToggleTask event.
    fn handle_toggle_task(&mut self, id: usize) {
        if let Some(task) = self.todo_list.toggle_task(id) {
            self.output.show_task_toggled(&task.description, task.is_completed());
        } else {
            self.output.show_task_not_found(id);
        }
    }

    /// Handles the SetPriority event.
    fn handle_set_priority(&mut self, id: usize, priority: Priority) {
        match self.todo_list.set_task_priority(id, priority) {
            Some(task) => {
                self.output.show_priority_set(&task.description, priority);
            }
            None => {
                self.output.show_task_not_found(id);
            }
        }
    }

    /// Handles the SearchTasks event.
    fn handle_search_tasks(&mut self, keyword: &str) {
        let results = self.todo_list.search_tasks(keyword);
        self.output.show_search_results(&results, keyword);
    }

    /// Handles the ShowHelp event.
    fn handle_show_help(&mut self) {
        self.output.show_help();
    }

    /// Handles the Quit event.
    fn handle_quit(&mut self) -> LoopControl {
        self.output.show_goodbye();
        LoopControl::Exit
    }

    /// Handles the UnknownCommand event.
    fn handle_unknown_command(&mut self, command: &str) {
        self.output.show_unknown_command(command);
    }

    /// Handles the InvalidInput event.
    fn handle_invalid_input(&mut self, message: &str) {
        self.output.show_error(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controller() {
        let controller = TodoController::new();
        assert!(controller.todo_list.is_empty());
    }

    #[test]
    fn test_handle_add_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Test task");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
        assert_eq!(controller.todo_list.get_tasks()[0].description, "Test task");
    }

    #[test]
    fn test_handle_add_multiple_tasks() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task 1");
        controller.handle_add_task("Task 2");
        controller.handle_add_task("Task 3");
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
    }

    #[test]
    fn test_handle_remove_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task to remove");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_remove_task(task_id);
        
        assert!(controller.todo_list.is_empty());
    }

    #[test]
    fn test_handle_remove_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Test task");
        controller.handle_remove_task(999);
        
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task to complete");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_complete_task(task_id);
        
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Test task");
        controller.handle_complete_task(999);
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_uncomplete_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task to uncomplete");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        controller.handle_complete_task(task_id);
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_uncomplete_task(task_id);
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_uncomplete_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Test task");
        controller.handle_uncomplete_task(999);
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task to toggle");
        let task_id = controller.todo_list.get_tasks()[0].id;
        
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_toggle_task(task_id);
        assert!(controller.todo_list.get_tasks()[0].is_completed());
        
        controller.handle_toggle_task(task_id);
        assert!(!controller.todo_list.get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_nonexistent_task() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Test task");
        let initial_status = controller.todo_list.get_tasks()[0].is_completed();
        
        controller.handle_toggle_task(999);
        
        assert_eq!(controller.todo_list.get_tasks()[0].is_completed(), initial_status);
    }

    #[test]
    fn test_handle_list_tasks_all() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task 1");
        controller.handle_add_task("Task 2");
        controller.handle_complete_task(1);
        
        controller.handle_list_tasks(&None);
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
    }

    #[test]
    fn test_handle_list_tasks_completed() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task 1");
        controller.handle_add_task("Task 2");
        controller.handle_complete_task(1);
        
        controller.handle_list_tasks(&Some(TaskFilter::completed()));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_list_tasks_pending() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task 1");
        controller.handle_add_task("Task 2");
        controller.handle_complete_task(1);
        
        controller.handle_list_tasks(&Some(TaskFilter::pending()));
        
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_event_quit() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::Quit);
        
        assert_eq!(control, LoopControl::Exit);
    }

    #[test]
    fn test_handle_event_non_quit() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::ShowHelp);
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_event_add_task() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::AddTask("New task".to_string()));
        
        assert_eq!(control, LoopControl::Continue);
        assert_eq!(controller.todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_event_invalid_input() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::InvalidInput("Error".to_string()));
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_event_unknown_command() {
        let mut controller = TodoController::new();
        
        let control = controller.handle_event(&UiEvent::UnknownCommand("invalid".to_string()));
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_complex_workflow() {
        let mut controller = TodoController::new();
        
        // Add multiple tasks
        controller.handle_event(&UiEvent::AddTask("Task 1".to_string()));
        controller.handle_event(&UiEvent::AddTask("Task 2".to_string()));
        controller.handle_event(&UiEvent::AddTask("Task 3".to_string()));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 3);
        
        // Complete some tasks
        controller.handle_event(&UiEvent::CompleteTask(1));
        controller.handle_event(&UiEvent::CompleteTask(2));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 2);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
        
        // Remove a task
        controller.handle_event(&UiEvent::RemoveTask(3));
        
        assert_eq!(controller.todo_list.get_tasks().len(), 2);
        
        // Toggle a task
        controller.handle_event(&UiEvent::ToggleTask(1));
        
        assert_eq!(controller.todo_list.get_completed_tasks().len(), 1);
        assert_eq!(controller.todo_list.get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_search_tasks() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Buy groceries");
        controller.handle_add_task("Read a book");
        controller.handle_add_task("Buy concert tickets");
        
        controller.handle_search_tasks("buy");
        
        // No panic means search executed successfully
        // The actual display is tested in OutputWriter tests
    }

    #[test]
    fn test_handle_search_tasks_no_results() {
        let mut controller = TodoController::new();
        
        controller.handle_add_task("Task one");
        controller.handle_add_task("Task two");
        
        controller.handle_search_tasks("nonexistent");
        
        // Should handle gracefully with no results
    }

    #[test]
    fn test_handle_search_tasks_empty_list() {
        let mut controller = TodoController::new();
        
        controller.handle_search_tasks("anything");
        
        // Should handle gracefully with empty list
    }
}

