use crate::models::command_controller_result::CommandControllerResult;
use crate::models::todo_list::TodoList;
use crate::models::parse_error::ParseError;

pub trait CommandController {
    fn try_execute(&mut self, input: &str, todo_list: &mut TodoList) -> Option<Result<CommandControllerResult, ParseError>>;
}
