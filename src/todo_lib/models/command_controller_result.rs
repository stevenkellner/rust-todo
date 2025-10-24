
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
    
    /// Enable debug mode
    EnableDebugMode,

    /// Disable debug mode
    DisableDebugMode,
}
