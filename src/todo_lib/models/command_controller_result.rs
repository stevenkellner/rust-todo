
/// Result of handling a command.
///
/// This enum allows commands to signal special actions
/// like toggling debug mode.
#[derive(Debug, Clone, PartialEq)]
pub enum CommandControllerResult {
    /// Continue normal operation
    Continue,

    /// Exit the main loop
    ExitMainLoop,
    
    /// Toggle debug mode on/off
    ToggleDebug,
}
