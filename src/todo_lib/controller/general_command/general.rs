/// Represents general application commands.
///
/// These commands are for general application control and information.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneralCommand {
    /// Show help information
    ShowHelp,

    /// Quit the application
    Quit,

    /// Toggle debug mode on/off
    ToggleDebug,
}
