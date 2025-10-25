use crate::models::TodoList;
use crate::models::Task;
use crate::models::task::TaskWithoutId;

/// Input mode for the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode
    Normal,
    /// Adding a new task
    Adding,
    /// Editing a task
    Editing,
    /// Searching tasks
    Searching,
    /// Viewing help
    Help,
}

/// The main TUI application state
pub struct App {
    /// The todo list
    pub todo_list: TodoList,
    /// Currently selected task index in the filtered list
    pub selected: usize,
    /// Current input mode
    pub input_mode: InputMode,
    /// Current input buffer (for adding/editing/searching)
    pub input: String,
    /// Search query
    pub search_query: String,
    /// Whether the app should quit
    pub should_quit: bool,
    /// Current filter (pending, completed, all)
    pub filter: String,
    /// Scroll offset for task list
    pub scroll_offset: usize,
    /// Task being edited (ID)
    pub editing_task_id: Option<usize>,
    /// Status message to display
    pub status_message: Option<String>,
}

impl App {
    /// Create a new App
    pub fn new(todo_list: TodoList) -> Self {
        Self {
            todo_list,
            selected: 0,
            input_mode: InputMode::Normal,
            input: String::new(),
            search_query: String::new(),
            should_quit: false,
            filter: String::from("all"),
            scroll_offset: 0,
            editing_task_id: None,
            status_message: None,
        }
    }

    /// Get the currently displayed tasks (filtered)
    pub fn get_displayed_tasks(&self) -> Vec<&Task> {
        let all_tasks = self.todo_list.get_tasks();
        
        // Apply filter
        let mut tasks: Vec<&Task> = match self.filter.as_str() {
            "pending" => all_tasks.iter().filter(|t| !t.is_completed()).collect(),
            "completed" => all_tasks.iter().filter(|t| t.is_completed()).collect(),
            _ => all_tasks.iter().collect(),
        };
        
        // Apply search filter
        if !self.search_query.is_empty() {
            let query = self.search_query.to_lowercase();
            tasks.retain(|t| t.description.to_lowercase().contains(&query));
        }
        
        tasks
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
            if self.selected < self.scroll_offset {
                self.scroll_offset = self.selected;
            }
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        let tasks = self.get_displayed_tasks();
        if self.selected < tasks.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    /// Get the currently selected task
    pub fn get_selected_task(&self) -> Option<&Task> {
        let tasks = self.get_displayed_tasks();
        tasks.get(self.selected).copied()
    }

    /// Toggle the completion status of the selected task
    pub fn toggle_selected_task(&mut self) {
        if let Some(task) = self.get_selected_task() {
            let task_id = task.id;
            let new_status = !task.is_completed();
            self.todo_list.toggle_task(task_id);
            
            self.status_message = Some(format!(
                "Task {} marked as {}",
                task_id,
                if new_status { "completed" } else { "pending" }
            ));
        }
    }

    /// Delete the selected task
    pub fn delete_selected_task(&mut self) {
        if let Some(task) = self.get_selected_task() {
            let task_id = task.id;
            self.todo_list.remove_task(task_id);
            
            // Adjust selection if needed
            let tasks_len = self.get_displayed_tasks().len();
            if self.selected >= tasks_len && tasks_len > 0 {
                self.selected = tasks_len - 1;
            }
            
            self.status_message = Some(format!("Task {} deleted", task_id));
        }
    }

    /// Start adding a new task
    pub fn start_adding(&mut self) {
        self.input_mode = InputMode::Adding;
        self.input.clear();
    }

    /// Start editing the selected task
    pub fn start_editing(&mut self) {
        if let Some(task) = self.get_selected_task() {
            let task_id = task.id;
            let description = task.description.clone();
            self.input_mode = InputMode::Editing;
            self.editing_task_id = Some(task_id);
            self.input = description;
        }
    }

    /// Start searching
    pub fn start_searching(&mut self) {
        self.input_mode = InputMode::Searching;
        self.input.clear();
    }

    /// Show help
    pub fn show_help(&mut self) {
        self.input_mode = InputMode::Help;
    }

    /// Submit the current input
    pub fn submit_input(&mut self) {
        match self.input_mode {
            InputMode::Adding => {
                if !self.input.is_empty() {
                    let new_task = TaskWithoutId::new(self.input.clone());
                    let task_id = self.todo_list.add_task(new_task);
                    self.status_message = Some(format!("Task {} added", task_id));
                    self.input.clear();
                }
                self.input_mode = InputMode::Normal;
            }
            InputMode::Editing => {
                if let Some(task_id) = self.editing_task_id {
                    if !self.input.is_empty() {
                        self.todo_list.edit_task(task_id, self.input.clone());
                        self.status_message = Some(format!("Task {} updated", task_id));
                    }
                    self.input.clear();
                    self.editing_task_id = None;
                }
                self.input_mode = InputMode::Normal;
            }
            InputMode::Searching => {
                self.search_query = self.input.clone();
                self.input.clear();
                self.selected = 0;
                self.scroll_offset = 0;
                self.input_mode = InputMode::Normal;
            }
            _ => {}
        }
    }

    /// Cancel the current input
    pub fn cancel_input(&mut self) {
        self.input.clear();
        self.editing_task_id = None;
        self.input_mode = InputMode::Normal;
    }

    /// Add a character to the input
    pub fn push_char(&mut self, c: char) {
        self.input.push(c);
    }

    /// Remove the last character from the input
    pub fn pop_char(&mut self) {
        self.input.pop();
    }

    /// Cycle through filters
    pub fn cycle_filter(&mut self) {
        self.filter = match self.filter.as_str() {
            "all" => "pending".to_string(),
            "pending" => "completed".to_string(),
            _ => "all".to_string(),
        };
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Clear the search query
    pub fn clear_search(&mut self) {
        self.search_query.clear();
        self.selected = 0;
        self.scroll_offset = 0;
    }

    /// Quit the application
    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
