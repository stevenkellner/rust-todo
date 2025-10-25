use crate::controller::CommandControllerRegistry;
use crate::controller::project_command::ProjectManager;
use crate::models::command_controller_result::{CommandControllerResult, CommandControllerResultAction};
use crate::models::loop_control::LoopControl;
use crate::persistence::TodoListStorage;
use crate::ui::{InputStream, OutputManager};
use crate::{FileInputStream, FileOutputWriter, OutputWriter};
use std::rc::Rc;
use std::cell::RefCell;

/// Controls the todo list application by coordinating specialized controllers.
///
/// and delegating command processing to specialized controllers that can be added or removed dynamically.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::TodoManager;
/// use todo_manager::FileInputStream;
/// use todo_manager::FileOutputWriter;
/// use std::rc::Rc;
/// use std::cell::RefCell;
///
/// let input_stream = FileInputStream::new(std::io::stdin());
/// let output_writer = FileOutputWriter::new(std::io::stdout());
/// let mut manager = TodoManager::new(
///     Rc::new(RefCell::new(input_stream)),
///     Rc::new(RefCell::new(output_writer)),
///     "tasks.json"
/// );
/// manager.run();
/// ```
pub struct TodoManager<I: InputStream, O: OutputWriter> {
    project_manager: Rc<RefCell<ProjectManager>>,
    input_stream: Rc<RefCell<I>>,
    output_manager: OutputManager<O>,
    command_controller_registry: CommandControllerRegistry<O>,
    storage: TodoListStorage,
}

impl<I: InputStream, O: OutputWriter> TodoManager<I, O> {

    /// Creates a new manager with an empty todo list and new UI components.
    ///
    /// # Arguments
    ///
    /// * `input_stream` - The input stream for reading user commands
    /// * `output_writer` - The output writer for displaying results
    /// * `storage_path` - The file path where tasks will be saved/loaded
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::TodoManager;
    /// use todo_manager::FileInputStream;
    /// use todo_manager::FileOutputWriter;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let input_stream = FileInputStream::new(std::io::stdin());
    /// let output_writer = FileOutputWriter::new(std::io::stdout());
    /// let manager = TodoManager::new(
    ///     Rc::new(RefCell::new(input_stream)),
    ///     Rc::new(RefCell::new(output_writer)),
    ///     "tasks.json"
    /// );
    /// ```
    pub fn new<P: AsRef<std::path::Path>>(input_stream: Rc<RefCell<I>>, output_writer: Rc<RefCell<O>>, storage_path: P) -> Self {
        let storage = TodoListStorage::new(storage_path);

        // During tests we prefer a fresh in-memory ProjectManager to avoid interfering with
        // local developer/state files. When not testing, try to load persisted projects.
        let project_manager_inner = if cfg!(test) {
            ProjectManager::new()
        } else {
            storage.load_projects().unwrap_or_else(|_| ProjectManager::new())
        };

        let project_manager = Rc::new(RefCell::new(project_manager_inner));

        Self {
            project_manager: Rc::clone(&project_manager),
            input_stream,
            output_manager: OutputManager::new(Rc::clone(&output_writer)),
            command_controller_registry: CommandControllerRegistry::new(
                Rc::clone(&project_manager),
                Rc::clone(&output_writer)
            ),
            storage,
        }
    }
}

impl TodoManager<FileInputStream<std::io::Stdin>, FileOutputWriter<std::io::Stdout>> {
    /// Creates a new manager with an empty todo list and stdio-based UI components.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::TodoManager;
    ///
    /// let manager = TodoManager::with_stdio("tasks.json");
    /// ```
    pub fn with_stdio<P: AsRef<std::path::Path>>(storage_path: P) -> Self {
        let input_stream = Rc::new(RefCell::new(FileInputStream::new(std::io::stdin())));
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(std::io::stdout())));
        Self::new(input_stream, output_writer, storage_path)
    }

    pub fn with_stdio_default() -> Self {
        let input_stream = Rc::new(RefCell::new(FileInputStream::new(std::io::stdin())));
        let output_writer = Rc::new(RefCell::new(FileOutputWriter::new(std::io::stdout())));
        Self::new(input_stream, output_writer, std::env::temp_dir().join("tasks.json"))
    }
}

impl<I: InputStream, O: OutputWriter> TodoManager<I, O> {

    /// Starts the interactive command loop.
    ///
    /// This method displays a welcome message and enters an event loop
    /// that processes user input until the user quits.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::TodoManager;
    /// use todo_manager::FileInputStream;
    /// use todo_manager::FileOutputWriter;
    /// use std::rc::Rc;
    /// use std::cell::RefCell;
    ///
    /// let input_stream = FileInputStream::new(std::io::stdin());
    /// let output_writer = FileOutputWriter::new(std::io::stdout());
    /// let mut manager = TodoManager::new(
    ///     Rc::new(RefCell::new(input_stream)),
    ///     Rc::new(RefCell::new(output_writer)),
    ///    "tasks.json"
    /// );
    /// manager.run();
    /// ```
    pub fn run(&mut self) {
        self.output_manager.show_welcome();

        loop {
            self.output_manager.print_prompt();
            let input = self.input_stream.borrow_mut().get_next_input();
            
            if self.handle_input(&input) == LoopControl::Exit {
                break;
            }
        }
    }

    /// Handles user input by trying each specialized controller in turn.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string from the user
    ///
    /// # Returns
    ///
    /// `LoopControl::Continue` to continue the event loop, `LoopControl::Exit` to quit
    fn handle_input(&mut self, input: &str) -> LoopControl {
        let trimmed = input.trim();
        
        // Empty input
        if trimmed.is_empty() {
            return LoopControl::Continue;
        }

        if let Some(result) = self.command_controller_registry.try_execute(trimmed) {
            match result {
                Ok(result) => return self.handle_command_controller_result(result),
                Err(err) => {
                    self.output_manager.show_error(&err.message());
                    return LoopControl::Continue;
                }
            }
        }

        // Unknown command
        self.output_manager.handle_unknown_command(trimmed);
        LoopControl::Continue
    }

    fn handle_command_controller_result(&mut self, result: CommandControllerResult) -> LoopControl {
        for action in result.actions() {
            match action {
                CommandControllerResultAction::ExitMainLoop => return LoopControl::Exit,
                CommandControllerResultAction::EnableDebugMode => {
                    self.command_controller_registry.enable_debug();
                }
                CommandControllerResultAction::DisableDebugMode => {
                    self.command_controller_registry.disable_debug();
                }
                CommandControllerResultAction::SaveTodoList => {
                    if let Err(e) = self.save_tasks_to_disk() {
                        self.output_manager.show_error(&format!("failed to save tasks: {}", e));
                    }
                }
            }
        }
        LoopControl::Continue
    }

    fn save_tasks_to_disk(&self) -> Result<(), String> {
        self.storage.save_projects(&self.project_manager.borrow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::input::FileInputStream;
    use crate::ui::output::FileOutputWriter;
    use std::env;
    use std::path::PathBuf;

    fn get_test_storage_path(test_name: &str) -> PathBuf {
        env::temp_dir().join(format!("rust_todo_test_{}.json", test_name))
    }

    #[test]
    fn test_new_controller() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("new_controller");
        let manager = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        assert!(manager.project_manager.borrow().get_current_todo_list().is_empty());
    }

    #[test]
    fn test_handle_add_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("add_task");
        let mut manager = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        manager.handle_input("add Test task");
        
        assert_eq!(manager.project_manager.borrow().get_current_todo_list().get_tasks().len(), 1);
        assert_eq!(manager.project_manager.borrow().get_current_todo_list().get_tasks()[0].description, "Test task");
    }

    #[test]
    fn test_handle_add_multiple_tasks() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("add_multiple_tasks");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 3);
    }

    #[test]
    fn test_handle_remove_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("remove_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task to remove");
        let task_id = controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].id;
        
        controller.handle_input(&format!("remove {}", task_id));
        
        assert!(controller.project_manager.borrow().get_current_todo_list().is_empty());
    }

    #[test]
    fn test_handle_remove_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("remove_nonexistent_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Test task");
        controller.handle_input("remove 999");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("complete_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task to complete");
        let task_id = controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        
        assert!(controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_complete_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("complete_nonexistent_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Test task");
        controller.handle_input("complete 999");
        
        assert!(!controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_uncomplete_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("uncomplete_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task to uncomplete");
        let task_id = controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].id;
        
        controller.handle_input(&format!("complete {}", task_id));
        assert!(controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("uncomplete {}", task_id));
        assert!(!controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_uncomplete_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("uncomplete_nonexistent_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Test task");
        controller.handle_input("uncomplete 999");
        
        assert!(!controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("toggle_task");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task to toggle");
        let task_id = controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].id;
        
        assert!(!controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
        
        controller.handle_input(&format!("toggle {}", task_id));
        assert!(!controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed());
    }

    #[test]
    fn test_handle_toggle_nonexistent_task() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("toggle_nonexistent_task");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Test task");
        let initial_status = controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed();
        
        controller.handle_input("toggle 999");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks()[0].is_completed(), initial_status);
    }

    #[test]
    fn test_handle_list_tasks_all() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("list_tasks_all");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 2);
    }

    #[test]
    fn test_handle_list_tasks_completed() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("list_tasks_completed");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list completed");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_completed_tasks().len(), 1);
    }

    #[test]
    fn test_handle_list_tasks_pending() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("list_tasks_pending");
         let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("complete 1");
        
        controller.handle_input("list pending");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_quit_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("quit_command");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        let control = controller.handle_input("quit");
        
        assert_eq!(control, LoopControl::Exit);
    }

    #[test]
    fn test_handle_help_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("help_command");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        let control = controller.handle_input("help");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_add_command_returns_continue() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("add_command_returns_continue");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        let control = controller.handle_input("add New task");
        
        assert_eq!(control, LoopControl::Continue);
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 1);
    }

    #[test]
    fn test_handle_empty_input() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("empty_input");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        let control = controller.handle_input("");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_handle_unknown_command() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("unknown_command");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        let control = controller.handle_input("invalidcommand");
        
        assert_eq!(control, LoopControl::Continue);
    }

    #[test]
    fn test_complex_workflow() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("complex_workflow");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        // Add multiple tasks
        controller.handle_input("add Task 1");
        controller.handle_input("add Task 2");
        controller.handle_input("add Task 3");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 3);
        
        // Complete some tasks
        controller.handle_input("complete 1");
        controller.handle_input("complete 2");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_completed_tasks().len(), 2);
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_pending_tasks().len(), 1);
        
        // Remove a task
        controller.handle_input("remove 3");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_tasks().len(), 2);
        
        // Toggle a task
        controller.handle_input("toggle 1");
        
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_completed_tasks().len(), 1);
        assert_eq!(controller.project_manager.borrow().get_current_todo_list().get_pending_tasks().len(), 1);
    }

    #[test]
    fn test_handle_search_tasks() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("search_tasks");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Buy groceries");
        controller.handle_input("add Read a book");
        controller.handle_input("add Buy concert tickets");
        
        controller.handle_input("search buy");
        
        // No panic means search executed successfully
        // The actual display is tested in OutputWriter tests
    }

    #[test]
    fn test_handle_search_tasks_no_results() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("search_tasks_no_results");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("add Task one");
        controller.handle_input("add Task two");
        
        controller.handle_input("search nonexistent");
        
        // Should handle gracefully with no results
    }

    #[test]
    fn test_handle_search_tasks_empty_list() {
        let input_stream = FileInputStream::new(std::io::stdin());
        let output_writer = FileOutputWriter::new(std::io::stdout());
        let storage_path = get_test_storage_path("search_tasks_empty_list");
        let mut controller = TodoManager::new(Rc::new(RefCell::new(input_stream)), Rc::new(RefCell::new(output_writer)), storage_path);
        
        controller.handle_input("search anything");
        
        // Should handle gracefully with empty list
    }
}
