use crate::models::todo_list::TodoList;
use crate::models::parse_error::ParseError;
use crate::models::command_controller_result::CommandControllerResult;

/// Trait for command controllers that can handle user input.
///
/// This trait allows different command controllers to be stored and used
/// dynamically, enabling the addition and removal of controllers at runtime.
pub trait CommandController {
    /// Attempts to parse and handle a command from raw input.
    ///
    /// # Arguments
    ///
    /// * `input` - The raw input string to parse
    /// * `todo_list` - The todo list to operate on
    ///
    /// # Returns
    ///
    /// * `Some(Ok(()))` - Command was successfully parsed and executed
    /// * `Some(Err(ParseError))` - Command was recognized but had an error
    /// * `None` - Not this controller's command, should try other controllers
    fn try_handle(
        &mut self,
        input: &str,
        todo_list: &mut TodoList,
    ) -> Option<Result<CommandControllerResult, ParseError>>;
}
