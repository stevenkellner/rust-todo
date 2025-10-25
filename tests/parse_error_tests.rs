use todo_manager::models::ParseError;

#[test]
fn test_missing_arguments_error() {
    let error = ParseError::MissingArguments {
        command: "add".to_string(),
        usage: "add <description>".to_string(),
    };
    assert_eq!(error.message(), "Usage: add <description>");
    assert_eq!(
        format!("{}", error),
        "Usage: add <description>"
    );
}

#[test]
fn test_invalid_id_error() {
    let error = ParseError::InvalidId("Task ID must be a number".to_string());
    assert_eq!(error.message(), "Task ID must be a number");
    assert_eq!(format!("{}", error), "Task ID must be a number");
}

#[test]
fn test_invalid_format_error() {
    let error = ParseError::InvalidFormat {
        field: "date".to_string(),
        expected: "DD.MM.YYYY".to_string(),
        actual: "2023/01/15".to_string(),
    };
    assert_eq!(
        error.message(),
        "Invalid date format. Expected: DD.MM.YYYY, got: 2023/01/15"
    );
    assert_eq!(
        format!("{}", error),
        "Invalid date format. Expected: DD.MM.YYYY, got: 2023/01/15"
    );
}

#[test]
fn test_empty_input_error() {
    let error = ParseError::EmptyInput("Task description".to_string());
    assert_eq!(error.message(), "Task description cannot be empty.");
    assert_eq!(format!("{}", error), "Task description cannot be empty.");
}

#[test]
fn test_invalid_value_error() {
    let error = ParseError::InvalidValue {
        field: "priority".to_string(),
        value: "urgent".to_string(),
        allowed: "high, medium, low".to_string(),
    };
    assert_eq!(
        error.message(),
        "Invalid priority 'urgent'. Allowed values: high, medium, low"
    );
    assert_eq!(
        format!("{}", error),
        "Invalid priority 'urgent'. Allowed values: high, medium, low"
    );
}

#[test]
fn test_out_of_range_error() {
    let error = ParseError::OutOfRange {
        field: "Task ID".to_string(),
        value: "999".to_string(),
        range: "Valid range: 1-100".to_string(),
    };
    assert_eq!(
        error.message(),
        "Task ID '999' is out of range. Valid range: 1-100"
    );
    assert_eq!(
        format!("{}", error),
        "Task ID '999' is out of range. Valid range: 1-100"
    );
}

#[test]
fn test_invalid_date_error() {
    let error = ParseError::InvalidDate("Date must be in the future".to_string());
    assert_eq!(error.message(), "Date must be in the future");
    assert_eq!(format!("{}", error), "Date must be in the future");
}

#[test]
fn test_empty_command_error() {
    let error = ParseError::EmptyCommand("No command entered".to_string());
    assert_eq!(error.message(), "No command entered");
    assert_eq!(format!("{}", error), "No command entered");
}

#[test]
fn test_unknown_command_error() {
    let error = ParseError::UnknownCommand("invalidcmd".to_string());
    assert_eq!(
        error.message(),
        "Unknown command 'invalidcmd'. Type 'help' for available commands."
    );
    assert_eq!(
        format!("{}", error),
        "Unknown command 'invalidcmd'. Type 'help' for available commands."
    );
}

#[test]
fn test_parse_error_clone() {
    let error = ParseError::InvalidId("Test".to_string());
    let cloned = error.clone();
    assert_eq!(error, cloned);
}

#[test]
fn test_parse_error_debug() {
    let error = ParseError::InvalidId("Test".to_string());
    let debug_str = format!("{:?}", error);
    assert!(debug_str.contains("InvalidId"));
    assert!(debug_str.contains("Test"));
}

#[test]
fn test_parse_error_equality() {
    let error1 = ParseError::InvalidId("Test".to_string());
    let error2 = ParseError::InvalidId("Test".to_string());
    let error3 = ParseError::InvalidId("Different".to_string());
    
    assert_eq!(error1, error2);
    assert_ne!(error1, error3);
}

#[test]
fn test_parse_error_as_std_error() {
    let error: Box<dyn std::error::Error> = Box::new(ParseError::InvalidId("Test".to_string()));
    assert_eq!(error.to_string(), "Test");
}

#[test]
fn test_all_error_variants_display() {
    let errors = vec![
        ParseError::MissingArguments {
            command: "cmd".to_string(),
            usage: "usage".to_string(),
        },
        ParseError::InvalidId("id".to_string()),
        ParseError::InvalidFormat {
            field: "field".to_string(),
            expected: "exp".to_string(),
            actual: "act".to_string(),
        },
        ParseError::EmptyInput("input".to_string()),
        ParseError::InvalidValue {
            field: "field".to_string(),
            value: "val".to_string(),
            allowed: "allowed".to_string(),
        },
        ParseError::OutOfRange {
            field: "field".to_string(),
            value: "val".to_string(),
            range: "range".to_string(),
        },
        ParseError::InvalidDate("date".to_string()),
        ParseError::EmptyCommand("cmd".to_string()),
        ParseError::UnknownCommand("unknown".to_string()),
    ];
    
    for error in errors {
        // Each error should have a non-empty message
        assert!(!error.message().is_empty());
        assert!(!format!("{}", error).is_empty());
    }
}
