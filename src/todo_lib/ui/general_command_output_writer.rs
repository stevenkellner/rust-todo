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

    /// Displays the welcome message.
    pub fn show_welcome(&mut self) {
        self.writer.print_line("");
        self.writer.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.writer.print_line(&"       ████████╗  ██████╗  ██████╗   ██████╗       ".bright_cyan().bold().to_string());
        self.writer.print_line(&"       ╚══██╔══╝ ██╔═══██╗ ██╔══██╗ ██╔═══██╗      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ██║    ╚██████╔╝ ██████╔╝ ╚██████╔╝      ".bright_cyan().bold().to_string());
        self.writer.print_line(&"          ╚═╝     ╚═════╝  ╚═════╝   ╚═════╝       ".bright_cyan().bold().to_string());
        self.writer.print_line(&"                 📝 LIST MANAGER 📝                 ".bright_green().bold().to_string());
        self.writer.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.writer.print_line("");
        self.writer.print_line(&"    Welcome to your personal task management system!".white().to_string());
        self.writer.print_line("");
        self.writer.print_line(&format!("    Type {} to see available commands.", "help".bright_yellow().bold()));
        self.writer.print_line("");
        self.writer.print_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.writer.print_line("");
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

    /// Displays an error for unknown commands.
    pub fn show_unknown_command(&mut self, command: &str) {
        self.writer.show_error(&format!("Unknown command '{}'. Type help for available commands.", command));
    }

    /// Displays a generic error message.
    pub fn show_error(&mut self, message: &str) {
        self.writer.show_error(message);
    }

    /// Displays a generic success message.
    pub fn show_success(&mut self, message: &str) {
        self.writer.show_success(message);
    }

    /// Prints a line of text.
    pub fn print_line(&mut self, text: &str) {
        self.writer.print_line(text);
    }

    /// Displays the command prompt.
    pub fn print_prompt(&mut self) {
        self.writer.print_prompt();
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
    fn test_general_output_writer_unknown_command() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = GeneralCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_unknown_command("invalid");
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Unknown command"));
        assert!(output.contains("invalid"));
    }

    #[test]
    fn test_show_welcome() {
        setup();
        let mut buffer = Vec::new();
        let mut writer = GeneralCommandOutputWriter::with_writer(&mut buffer);
        
        writer.show_welcome();
        
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("LIST MANAGER"));
        assert!(output.contains("Welcome to your personal task management system"));
        assert!(output.contains("Type help to see available commands"));
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
}
