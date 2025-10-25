use std::collections::HashSet;

/// Result of handling a command.
///
/// This enum allows commands to signal special actions
/// like toggling debug mode.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CommandControllerResultAction {
    /// Exit the main loop
    ExitMainLoop,

    /// Enable debug mode
    EnableDebugMode,

    /// Disable debug mode
    DisableDebugMode,

    /// Save the current todo list to disk
    SaveTodoList,
}

pub struct CommandControllerResult {
    /// Actions to be taken after command execution
    pub actions: HashSet<CommandControllerResultAction>,
}

impl CommandControllerResult {
    /// Creates a new CommandControllerResult with no actions.
    pub fn empty() -> Self {
        Self {
            actions: HashSet::new(),
        }
    }

    /// Creates a new CommandControllerResult with a single action.
    pub fn with_action(action: CommandControllerResultAction) -> Self {
        let mut actions = HashSet::new();
        actions.insert(action);
        Self { actions }
    }

    /// Creates a new CommandControllerResult with the given actions.
    pub fn with_actions(actions: impl IntoIterator<Item = CommandControllerResultAction>) -> Self {
        Self {
            actions: actions.into_iter().collect(),
        }
    }

    /// Adds an action to the result.
    pub fn add_action(&mut self, action: CommandControllerResultAction) {
        self.actions.insert(action);
    }

    /// Adds multiple actions to the result.
    pub fn add_actions(
        &mut self,
        actions: impl IntoIterator<Item = CommandControllerResultAction>,
    ) {
        self.actions.extend(actions);
    }

    /// Checks if a specific action is present in the result.
    pub fn has_action(&self, action: &CommandControllerResultAction) -> bool {
        self.actions.contains(action)
    }

    pub fn actions(&self) -> impl Iterator<Item = &CommandControllerResultAction> {
        self.actions.iter()
    }
}

impl Default for CommandControllerResult {
    fn default() -> Self {
        Self::empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_empty_result() {
        let result = CommandControllerResult::empty();
        assert!(result.actions.is_empty());
    }

    #[test]
    fn test_with_actions_creates_result_with_actions() {
        let actions = vec![
            CommandControllerResultAction::ExitMainLoop,
            CommandControllerResultAction::EnableDebugMode,
        ];
        let result = CommandControllerResult::with_actions(actions);
        assert_eq!(result.actions.len(), 2);
        assert!(result.has_action(&CommandControllerResultAction::ExitMainLoop));
        assert!(result.has_action(&CommandControllerResultAction::EnableDebugMode));
    }

    #[test]
    fn test_add_action() {
        let mut result = CommandControllerResult::empty();
        result.add_action(CommandControllerResultAction::SaveTodoList);
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_add_actions() {
        let mut result = CommandControllerResult::empty();
        let actions = vec![
            CommandControllerResultAction::DisableDebugMode,
            CommandControllerResultAction::SaveTodoList,
        ];
        result.add_actions(actions);
        assert_eq!(result.actions.len(), 2);
        assert!(result.has_action(&CommandControllerResultAction::DisableDebugMode));
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_has_action_returns_false_for_missing_action() {
        let result = CommandControllerResult::empty();
        assert!(!result.has_action(&CommandControllerResultAction::ExitMainLoop));
    }

    #[test]
    fn test_default_creates_empty_result() {
        let result = CommandControllerResult::default();
        assert!(result.actions.is_empty());
    }

    #[test]
    fn test_duplicate_actions_ignored() {
        let mut result = CommandControllerResult::empty();
        result.add_action(CommandControllerResultAction::ExitMainLoop);
        result.add_action(CommandControllerResultAction::ExitMainLoop);
        assert_eq!(result.actions.len(), 1);
    }

    #[test]
    fn test_command_controller_result_action_clone() {
        let action = CommandControllerResultAction::EnableDebugMode;
        let cloned = action.clone();
        assert_eq!(action, cloned);
    }

    #[test]
    fn test_with_actions_from_hashset() {
        let mut actions = HashSet::new();
        actions.insert(CommandControllerResultAction::ExitMainLoop);
        actions.insert(CommandControllerResultAction::SaveTodoList);
        let result = CommandControllerResult::with_actions(actions);
        assert_eq!(result.actions.len(), 2);
    }
}
