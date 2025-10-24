/// Represents the different types of command controllers available in the system.
///
/// This enum is used to identify and manage specialized command controllers
/// that handle different categories of commands in the todo list application.
///
/// # Variants
///
/// * `Task` - Controller for task-related commands (add, remove, complete, list, etc.)
/// * `Debug` - Controller for debug commands (generate, clear, toggle debug mode)
/// * `General` - Controller for general commands (help, quit)
///
/// # Examples
///
/// ```
/// use todo_manager::models::command_controller_type::CommandControllerType;
///
/// let controller_type = CommandControllerType::Task;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CommandControllerType {
    /// Handles task management commands (add, remove, complete, list, etc.)
    Task,
    /// Handles debug commands (generate random tasks, clear all, toggle debug mode)
    Debug,
    /// Handles general application commands (help, quit)
    General,
}
