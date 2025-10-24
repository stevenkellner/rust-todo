use std::fmt;

/// Represents errors that can occur during command parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseError {
    /// Command requires more arguments than provided
    MissingArguments { command: String, usage: String },
    /// Task ID is not a valid number
    InvalidId(String),
    /// Input format is incorrect (e.g., date format, filter syntax)
    InvalidFormat { field: String, expected: String, actual: String },
    /// Required field is empty (e.g., task description, search keyword)
    EmptyInput(String),
    /// Value is not valid for the field (e.g., priority level, date)
    InvalidValue { field: String, value: String, allowed: String },
    /// Value exceeds allowed range
    OutOfRange { field: String, value: String, range: String },
    /// Date parsing or validation failed
    InvalidDate(String),
    /// No command was entered
    EmptyCommand(String),
}

impl ParseError {
    /// Returns the error message.
    pub fn message(&self) -> String {
        match self {
            ParseError::MissingArguments { command: _, usage } => 
                format!("Usage: {}", usage),
            ParseError::InvalidId(msg) => msg.clone(),
            ParseError::InvalidFormat { field, expected, actual } => 
                format!("Invalid {} format. Expected: {}, got: {}", field, expected, actual),
            ParseError::EmptyInput(field) => 
                format!("{} cannot be empty.", field),
            ParseError::InvalidValue { field, value, allowed } => 
                format!("Invalid {} '{}'. Allowed values: {}", field, value, allowed),
            ParseError::OutOfRange { field, value, range } => 
                format!("{} '{}' is out of range. {}", field, value, range),
            ParseError::InvalidDate(msg) => msg.clone(),
            ParseError::EmptyCommand(msg) => msg.clone(),
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}

impl std::error::Error for ParseError {}

