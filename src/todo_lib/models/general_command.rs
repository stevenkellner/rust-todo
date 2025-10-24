/// Represents general application commands.
///
/// These commands are for general application control and information.
#[derive(Debug, Clone, PartialEq)]
pub enum GeneralCommand {
    /// Show help information
    ShowHelp,
    
    /// Quit the application
    Quit,
    
    /// Unknown command entered
    Unknown(String),
    
    /// Invalid input (e.g., missing arguments)
    InvalidInput(String),
}
