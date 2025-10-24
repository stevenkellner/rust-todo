use crate::ui::output_writer::OutputWriter;
use crate::ui::formatters::MessageFormatter;
use std::io::Write;
use colored::*;

/// Output writer for general application commands.
///
/// Handles all output operations for general commands
/// like help, quit, welcome messages, etc.
pub struct GeneralCommandOutputWriter<W: Write> {
    writer: OutputWriter<W>,
}

impl GeneralCommandOutputWriter<std::io::Stdout> {
    /// Creates a new GeneralCommandOutputWriter with stdout.
    pub fn new() -> Self {
        GeneralCommandOutputWriter {
            writer: OutputWriter::new(),
        }
    }
}

impl<W: Write> GeneralCommandOutputWriter<W> {
    /// Creates a new GeneralCommandOutputWriter with a custom writer.
    pub fn with_writer(writer: W) -> Self {
        GeneralCommandOutputWriter {
            writer: OutputWriter::with_writer(writer),
        }
    }

    /// Displays the goodbye message.
    pub fn show_goodbye(&mut self) {
        self.writer.print_line("");
        self.writer.print_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"    ✨ Thank you for using To-Do List Manager! ✨    ".bright_cyan().bold().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"           Stay organized and productive! 🚀          ".bright_green().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.writer.print_line("");
    }

    /// Displays the help menu.
    pub fn show_help(&mut self) {
        self.writer.print_line(&format!("\n{}", MessageFormatter::section_title("To-Do List Manager Commands")));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("add <description>", "Add a new task"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("list [status] [priority]", "List tasks (filters can be combined)"));
        self.writer.print_line(&MessageFormatter::subinfo("Status:", "completed/done, pending/todo, overdue"));
        self.writer.print_line(&MessageFormatter::subinfo("Priority:", "high/h, medium/med/m, low/l"));
        self.writer.print_line(&MessageFormatter::subinfo("Category:", "category:name or cat:name"));
        self.writer.print_line(&MessageFormatter::subinfo("Example:", "list pending high category:work"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("remove <id>", "Remove a task by ID"));
        self.writer.print_line(&MessageFormatter::label("Aliases:", "rm, delete"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("complete <id>", "Mark task as completed"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "done"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("uncomplete <id>", "Mark task as pending"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "undo"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("toggle <id>", "Toggle task completion status"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("priority <id> <level>", "Set task priority"));
        self.writer.print_line(&MessageFormatter::subinfo("Levels:", "high/h, medium/med/m, low/l"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "pri"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("set-due <id> <date>", "Set task due date"));
        self.writer.print_line(&MessageFormatter::subinfo("Format:", "DD.MM.YYYY or 'none' to clear"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "due"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("edit <id> <description>", "Edit task description"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("set-category <id> <name>", "Set task category"));
        self.writer.print_line(&MessageFormatter::subinfo("Format:", "<name> or 'none' to clear"));
        self.writer.print_line(&MessageFormatter::label("Aliases:", "category, cat"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("categories", "List all categories"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "list-categories"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("search <keyword>", "Search tasks by keyword"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "find"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("statistics", "Display task statistics"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "stats"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("help", "Show this help message"));
        self.writer.print_line(&MessageFormatter::label("Alias:", "h"));
        self.writer.print_line("");
        self.writer.print_line(&MessageFormatter::command("quit", "Exit the program"));
        self.writer.print_line(&MessageFormatter::label("Aliases:", "q, exit"));
        self.writer.print_line("");
        self.writer.print_line(&format!("{}\n", MessageFormatter::separator(40)));
    }

    /// Shows debug mode enabled message.
    pub fn show_debug_enabled(&mut self) {
        self.writer.print_line("");
        self.writer.print_line(&"Debug mode enabled.".green().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"Additional debug commands available:".bright_yellow().to_string());
        self.writer.print_line("");
        self.writer.print_line(&format!("  {} <count>  - Generate random tasks", "debug:gen".bright_cyan()));
        self.writer.print_line(&format!("  {}          - Clear all tasks", "debug:clear".bright_cyan()));
        self.writer.print_line(&format!("  {}          - Disable debug mode", "debug".bright_cyan()));
        self.writer.print_line("");
    }

    /// Shows debug mode disabled message.
    pub fn show_debug_disabled(&mut self) {
        self.writer.print_line("");
        self.writer.print_line(&"Debug mode disabled.".yellow().to_string());
        self.writer.print_line("");
    }
}

impl Default for GeneralCommandOutputWriter<std::io::Stdout> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Disable colors for all tests
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_show_goodbye() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = GeneralCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_goodbye();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Thank you for using To-Do List Manager"));
        assert!(output.contains("Stay organized and productive"));
    }

    #[test]
    fn test_show_help() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = GeneralCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_help();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("--- To-Do List Manager Commands ---"));
        assert!(output.contains("add <description>"));
        assert!(output.contains("list [status] [priority]"));
        assert!(output.contains("remove <id>"));
        assert!(output.contains("help"));
        assert!(output.contains("quit"));
    }

    #[test]
    fn test_show_debug_enabled() {
        setup();
        let mut output = Vec::new();
        let mut manager = GeneralCommandOutputWriter::with_writer(&mut output);
        
        manager.show_debug_enabled();
        
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Debug mode enabled"));
        assert!(output_str.contains("debug:gen"));
        assert!(output_str.contains("debug:clear"));
        assert!(output_str.contains("Additional debug commands"));
    }

    #[test]
    fn test_show_debug_disabled() {
        setup();
        let mut output = Vec::new();
        let mut manager = GeneralCommandOutputWriter::with_writer(&mut output);
        
        manager.show_debug_disabled();
        
        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Debug mode disabled"));
    }
}
