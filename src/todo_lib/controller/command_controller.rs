use crate::models::command_controller_result::CommandControllerResult;
use crate::models::parse_error::ParseError;

pub trait CommandController {
    fn try_execute(&mut self, input: &str) -> Option<Result<CommandControllerResult, ParseError>>;
}
