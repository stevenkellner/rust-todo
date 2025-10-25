use crate::controller::task_command::TaskSelection;
use crate::models::command_controller_result::CommandControllerResult;

/// Handler for dispatching task operations based on TaskSelection (Single, Multiple, All)
/// 
/// This eliminates repetitive match statements in the controller by providing
/// a generic dispatch mechanism.
pub struct TaskSelectionHandler;

impl TaskSelectionHandler {
    /// Executes a task operation based on the selection type
    /// 
    /// # Arguments
    /// 
    /// * `selection` - The task selection (Single, Multiple, or All)
    /// * `single_fn` - Function to execute for a single task
    /// * `multiple_fn` - Function to execute for multiple tasks
    /// * `all_fn` - Function to execute for all tasks
    pub fn execute<F1, F2, F3>(
        selection: &TaskSelection,
        mut single_fn: F1,
        mut multiple_fn: F2,
        mut all_fn: F3,
    ) -> CommandControllerResult
    where
        F1: FnMut(usize) -> CommandControllerResult,
        F2: FnMut(&[usize]) -> CommandControllerResult,
        F3: FnMut() -> CommandControllerResult,
    {
        match selection {
            TaskSelection::Single(id) => single_fn(*id),
            TaskSelection::Multiple(ids) => multiple_fn(ids),
            TaskSelection::All => all_fn(),
        }
    }

    /// Executes a task operation with an additional parameter based on the selection type
    /// 
    /// This is useful for operations like set_priority or set_category that require
    /// an additional parameter.
    pub fn execute_with_param<P, F1, F2, F3>(
        selection: &TaskSelection,
        param: P,
        mut single_fn: F1,
        mut multiple_fn: F2,
        mut all_fn: F3,
    ) -> CommandControllerResult
    where
        P: Clone,
        F1: FnMut(usize, P) -> CommandControllerResult,
        F2: FnMut(&[usize], P) -> CommandControllerResult,
        F3: FnMut(P) -> CommandControllerResult,
    {
        match selection {
            TaskSelection::Single(id) => single_fn(*id, param),
            TaskSelection::Multiple(ids) => multiple_fn(ids, param),
            TaskSelection::All => all_fn(param),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::command_controller_result::CommandControllerResultAction;

    #[test]
    fn test_execute_single() {
        let selection = TaskSelection::Single(1);
        let result = TaskSelectionHandler::execute(
            &selection,
            |id| {
                assert_eq!(id, 1);
                CommandControllerResult::empty()
            },
            |_ids| panic!("Should not call multiple_fn"),
            || panic!("Should not call all_fn"),
        );
        
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_execute_multiple() {
        let selection = TaskSelection::Multiple(vec![1, 2, 3]);
        let result = TaskSelectionHandler::execute(
            &selection,
            |_id| panic!("Should not call single_fn"),
            |ids| {
                assert_eq!(ids, &[1, 2, 3]);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            },
            || panic!("Should not call all_fn"),
        );
        
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_execute_all() {
        let selection = TaskSelection::All;
        let result = TaskSelectionHandler::execute(
            &selection,
            |_id| panic!("Should not call single_fn"),
            |_ids| panic!("Should not call multiple_fn"),
            CommandControllerResult::empty,
        );
        
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_execute_with_param_single() {
        let selection = TaskSelection::Single(5);
        let result = TaskSelectionHandler::execute_with_param(
            &selection,
            "test_value",
            |id, param| {
                assert_eq!(id, 5);
                assert_eq!(param, "test_value");
                CommandControllerResult::empty()
            },
            |_ids, _param| panic!("Should not call multiple_fn"),
            |_param| panic!("Should not call all_fn"),
        );
        
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_execute_with_param_multiple() {
        let selection = TaskSelection::Multiple(vec![10, 20]);
        let result = TaskSelectionHandler::execute_with_param(
            &selection,
            42,
            |_id, _param| panic!("Should not call single_fn"),
            |ids, param| {
                assert_eq!(ids, &[10, 20]);
                assert_eq!(param, 42);
                CommandControllerResult::empty()
            },
            |_param| panic!("Should not call all_fn"),
        );
        
        assert!(!result.has_action(&CommandControllerResultAction::SaveTodoList));
    }

    #[test]
    fn test_execute_with_param_all() {
        let selection = TaskSelection::All;
        let result = TaskSelectionHandler::execute_with_param(
            &selection,
            true,
            |_id, _param| panic!("Should not call single_fn"),
            |_ids, _param| panic!("Should not call multiple_fn"),
            |param| {
                assert!(param);
                CommandControllerResult::with_action(CommandControllerResultAction::SaveTodoList)
            },
        );
        
        assert!(result.has_action(&CommandControllerResultAction::SaveTodoList));
    }
}
