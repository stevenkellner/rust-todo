use colored::*;

/// Handles formatting of messages and UI elements
pub struct MessageFormatter;

impl MessageFormatter {
    /// Formats a success message
    pub fn success(message: &str) -> String {
        format!("{} {}", "✓".bright_green().bold(), message.green())
    }
    
    /// Formats an error message
    pub fn error(message: &str) -> String {
        format!("{} {}", "✗".bright_red().bold(), message.red())
    }
    
    /// Formats a warning/pending message
    pub fn warning(message: &str) -> String {
        format!("{} {}", "↻".bright_yellow().bold(), message.yellow())
    }
    
    /// Formats a section title
    pub fn section_title(title: &str) -> String {
        format!("--- {} ---", title).bright_cyan().bold().to_string()
    }
    
    /// Creates a separator line
    pub fn separator(length: usize) -> String {
        "-".repeat(length).bright_cyan().to_string()
    }
    
    /// Formats a command in help text with its description
    pub fn command(cmd: &str, description: &str) -> String {
        format!("{:<25} - {}", cmd.bright_green(), description)
    }
    
    /// Formats sub-info in help text (e.g., "↳ Status: completed, pending")
    pub fn subinfo(label: &str, info: &str) -> String {
        format!("    {} {}", format!("↳ {}",label).bright_black().italic(), info.bright_yellow())
    }
    
    /// Formats a label with value in help text (e.g., "↳ Alias: done")
    pub fn label(label: &str, value: &str) -> String {
        format!("    {} {}", format!("↳ {}",label).bright_black().italic(), value.bright_yellow())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    fn setup() {
        colored::control::set_override(false);
    }
    
    #[test]
    fn test_success_message() {
        setup();
        let result = MessageFormatter::success("Task completed");
        assert!(result.contains("✓"));
        assert!(result.contains("Task completed"));
    }
    
    #[test]
    fn test_error_message() {
        setup();
        let result = MessageFormatter::error("Task not found");
        assert!(result.contains("✗"));
        assert!(result.contains("Task not found"));
    }
}
