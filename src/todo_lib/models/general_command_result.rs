use crate::models::loop_control::LoopControl;

/// Result of handling a general command.
///
/// This enum allows general commands to signal special actions
/// like toggling debug mode.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneralCommandResult {
    /// Continue normal operation
    Continue(LoopControl),
    
    /// Toggle debug mode on/off
    ToggleDebug,
}
