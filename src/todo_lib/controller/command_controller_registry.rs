use crate::controller::command_controller::CommandController;
use crate::controller::task_command::TaskCommandController;
use crate::controller::general_command::GeneralCommandController;
use crate::controller::debug_command::DebugCommandController;
use crate::models::command_controller_result::CommandControllerResult;
use crate::models::command_controller_type::CommandControllerType;
use crate::models::todo_list::TodoList;
use crate::models::ParseError;
use std::collections::HashMap;

struct Entry {
    controller: Box<dyn CommandController>,
    active: bool,
}

/// Manages the collection of command controllers and their active states.
///
/// This struct encapsulates the logic for managing different command controllers
/// using a HashMap, allowing for dynamic addition and removal of controllers.
pub struct CommandControllerRegistry {
    /// Map of controller types to their active state (true = active, false = inactive)
    controllers: HashMap<CommandControllerType, Entry>,
}

impl CommandControllerRegistry {
    /// Creates a new CommandControllerRegistry with task and general controllers active,
    /// and debug controller inactive by default.
    pub fn new() -> Self {
        let mut controllers = HashMap::new();
        controllers.insert(CommandControllerType::Task, Entry {
            controller: Box::new(TaskCommandController::new()),
            active: true,
        });
        controllers.insert(CommandControllerType::General, Entry {
            controller: Box::new(GeneralCommandController::new()),
            active: true,
        });
        controllers.insert(CommandControllerType::Debug, Entry {
            controller: Box::new(DebugCommandController::new()),
            active: false,
        });
        
        CommandControllerRegistry {
            controllers,
        }
    }

    /// Enables a controller of the specified type.
    pub fn enable(&mut self, controller_type: CommandControllerType) {
        if let Some(entry) = self.controllers.get_mut(&controller_type) {
            entry.active = true;
        }
    }

    /// Disables a controller of the specified type.
    pub fn disable(&mut self, controller_type: CommandControllerType) {
        if let Some(entry) = self.controllers.get_mut(&controller_type) {
            entry.active = false;
        }
    }

    /// Returns whether a controller of the specified type is active.
    pub fn is_active(&self, controller_type: CommandControllerType) -> bool {
        self.controllers.get(&controller_type).map(|entry| entry.active).unwrap_or(false)
    }

    /// Tries to execute the input with all active controllers.
    ///
    /// This method iterates through the active controllers in a specific order
    /// (Task, General, Debug) and attempts to execute the input with each one.
    /// Returns the first successful match.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string from the user
    /// * `todo_list` - Mutable reference to the todo list
    ///
    /// # Returns
    ///
    /// `Some(Result)` if a controller handled the command, `None` if no controller recognized it
    pub fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>> {
        for entry in self.controllers.values_mut() {
            if entry.active {
                if let Some(result) = entry.controller.try_execute(input, todo_list) {
                    return Some(result);
                }
            }
        }
        None
    }
}

impl Default for CommandControllerRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_controllers() {
        let registry = CommandControllerRegistry::new();
        assert!(registry.is_active(CommandControllerType::Task));
        assert!(registry.is_active(CommandControllerType::General));
        assert!(!registry.is_active(CommandControllerType::Debug));
    }

    #[test]
    fn test_default_trait() {
        let registry = CommandControllerRegistry::default();
        assert!(registry.is_active(CommandControllerType::Task));
        assert!(registry.is_active(CommandControllerType::General));
        assert!(!registry.is_active(CommandControllerType::Debug));
    }

    #[test]
    fn test_enable_controller() {
        let mut registry = CommandControllerRegistry::new();
        assert!(!registry.is_active(CommandControllerType::Debug));
        
        registry.enable(CommandControllerType::Debug);
        assert!(registry.is_active(CommandControllerType::Debug));
    }

    #[test]
    fn test_disable_controller() {
        let mut registry = CommandControllerRegistry::new();
        assert!(registry.is_active(CommandControllerType::Task));
        
        registry.disable(CommandControllerType::Task);
        assert!(!registry.is_active(CommandControllerType::Task));
    }

    #[test]
    fn test_try_execute_with_task_command() {
        let mut registry = CommandControllerRegistry::new();
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("add Test task", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 1);
    }

    #[test]
    fn test_try_execute_with_general_command() {
        let mut registry = CommandControllerRegistry::new();
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("quit", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
    }

    #[test]
    fn test_try_execute_with_debug_command_inactive() {
        let mut registry = CommandControllerRegistry::new();
        let mut todo_list = TodoList::new();
        
        // Debug controller is not active, so debug commands should not be recognized
        let result = registry.try_execute("debug:gen 5", &mut todo_list);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_execute_with_debug_command_active() {
        let mut registry = CommandControllerRegistry::new();
        registry.enable(CommandControllerType::Debug);
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("debug:gen 3", &mut todo_list);
        assert!(result.is_some());
        assert!(result.unwrap().is_ok());
        assert_eq!(todo_list.get_tasks().len(), 3);
    }

    #[test]
    fn test_try_execute_unknown_command() {
        let mut registry = CommandControllerRegistry::new();
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("unknown command", &mut todo_list);
        assert!(result.is_none());
    }

    #[test]
    fn test_try_execute_with_disabled_task_controller() {
        let mut registry = CommandControllerRegistry::new();
        registry.disable(CommandControllerType::Task);
        let mut todo_list = TodoList::new();
        
        let result = registry.try_execute("add Test task", &mut todo_list);
        assert!(result.is_none());
    }
}
    