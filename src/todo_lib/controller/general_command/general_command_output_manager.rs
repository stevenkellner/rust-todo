use crate::ui::formatters::MessageFormatter;
use crate::ui::output::OutputWriter;
use colored::*;
use std::cell::RefCell;
use std::rc::Rc;

/// Output writer for general application commands.
///
/// Handles all output operations for general commands
/// like help, quit, welcome messages, etc.
pub struct GeneralCommandOutputManager<O: OutputWriter> {
    output_writer: Rc<RefCell<O>>,
}

impl<O: OutputWriter> GeneralCommandOutputManager<O> {
    /// Creates a new GeneralCommandOutputManager with a custom writer.
    pub fn new(output_writer: Rc<RefCell<O>>) -> Self {
        GeneralCommandOutputManager { output_writer }
    }

    /// Displays the goodbye message.
    pub fn show_goodbye(&mut self) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(
            &"─────────────────────────────────────────────────────"
                .bright_black()
                .to_string(),
        );
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(
            &"    ✨ Thank you for using To-Do List Manager! ✨    "
                .bright_cyan()
                .bold()
                .to_string(),
        );
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(
            &"           Stay organized and productive! 🚀          "
                .bright_green()
                .to_string(),
        );
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(
            &"═════════════════════════════════════════════════════"
                .bright_cyan()
                .bold()
                .to_string(),
        );
        self.output_writer.borrow_mut().write_line("");
    }

    /// Displays the help menu.
    pub fn show_help(&mut self) {
        self.output_writer.borrow_mut().write_line(&format!(
            "\n{}",
            MessageFormatter::section_title("To-Do List Manager Commands")
        ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "add <description>",
                "Add a new task",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "add-subtask <parent_id> <description>",
                "Add a subtask to a parent task",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "subtask"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "list [status] [priority]",
                "List tasks (filters can be combined)",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Status:",
                "completed/done, pending/todo, overdue",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Priority:",
                "high/h, medium/med/m, low/l",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Category:",
                "category:name or cat:name",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Example:",
                "list pending high category:work",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "remove <id|range|all>",
                "Remove task(s)",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1, 1-5, 1,3,5, all",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "rm, delete"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "complete <id|range|all>",
                "Mark task(s) as completed",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1, 1-5, 1,3,5, all",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "done"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "uncomplete <id|range|all>",
                "Mark task(s) as pending",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1, 1-5, 1,3,5, all",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "undo"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "toggle <id|range|all>",
                "Toggle task(s) completion status",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1, 1-5, 1,3,5, all",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "priority <id|range|all> <level>",
                "Set task(s) priority",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1 high, 1-5 low, all medium",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Levels:",
                "high/h, medium/med/m, low/l",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "pri"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "set-due <id> <date>",
                "Set task due date",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Format:",
                "DD.MM.YYYY or 'none' to clear",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "due"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "edit <id> <description>",
                "Edit task description",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "set-category <id|range|all> <name>",
                "Set task(s) category",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1 work, 1-5 personal, all none",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Format:",
                "<name> or 'none' to clear",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "category, cat"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "set-recurring <id|range|all> <frequency>",
                "Set task(s) recurrence",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1 daily, 1-5 weekly, all none",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Frequency:",
                "daily/d, weekly/w, monthly/m, none",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "recurring, recur"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "add-dependency <task_id> <depends_on_id>",
                "Add a dependency to a task",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "2 1 (task 2 depends on task 1)",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "add-dep, depends-on"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "remove-dependency <task_id> <depends_on_id>",
                "Remove a dependency from a task",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "2 1 (remove dependency of task 2 on task 1)",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "remove-dep, rm-dep"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "show-dependencies <task_id>",
                "Show dependency graph for a task",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::subinfo(
                "Examples:",
                "1 (show all dependencies and dependents for task 1)",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "dependencies, deps, dep-graph, dependency-graph",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "categories",
                "List all categories",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "list-categories"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "search <keyword>",
                "Search tasks by keyword",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "find"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "statistics",
                "Display task statistics",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "stats"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "new-project <name>",
                "Create a new project",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "new-proj, create-project",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "switch-project <name>",
                "Switch to a different project",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "switch, switch-to, use-project",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "list-projects",
                "List all projects",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "projects, show-projects",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "delete-project <name>",
                "Delete a project",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "rm-project, remove-project",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command(
                "rename-project <old_name> <new_name>",
                "Rename a project",
            ));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label(
                "Aliases:",
                "mv-project, move-project",
            ));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command("help", "Show this help message"));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Alias:", "h"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::command("quit", "Exit the program"));
        self.output_writer
            .borrow_mut()
            .write_line(&MessageFormatter::label("Aliases:", "q, exit"));
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&format!("{}\n", MessageFormatter::separator(40)));
    }

    /// Shows debug mode enabled message.
    pub fn show_debug_enabled(&mut self) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&"Debug mode enabled.".green().to_string());
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(
            &"Additional debug commands available:"
                .bright_yellow()
                .to_string(),
        );
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&format!(
            "  {} <count>                - Generate random tasks",
            "debug:gen".bright_cyan()
        ));
        self.output_writer.borrow_mut().write_line(&format!(
            "  {}                        - Clear all tasks",
            "debug:clear".bright_cyan()
        ));
        self.output_writer.borrow_mut().write_line(&format!(
            "  {} <projects> <tasks>  - Generate random projects with tasks",
            "debug:gen-projects".bright_cyan()
        ));
        self.output_writer.borrow_mut().write_line(&format!(
            "  {}                - Clear all projects (keep default)",
            "debug:clear-projects".bright_cyan()
        ));
        self.output_writer.borrow_mut().write_line(&format!(
            "  {}                        - Disable debug mode",
            "debug".bright_cyan()
        ));
        self.output_writer.borrow_mut().write_line("");
    }

    /// Shows debug mode disabled message.
    pub fn show_debug_disabled(&mut self) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer
            .borrow_mut()
            .write_line(&"Debug mode disabled.".yellow().to_string());
        self.output_writer.borrow_mut().write_line("");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::output::FileOutputWriter;

    // Disable colors for all tests
    fn setup() {
        colored::control::set_override(false);
    }

    #[test]
    fn test_show_goodbye() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = GeneralCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        writer.show_goodbye();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Thank you for using To-Do List Manager"));
        assert!(output.contains("Stay organized and productive"));
    }

    #[test]
    fn test_show_help() {
        setup();
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        let mut writer = GeneralCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        writer.show_help();

        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("--- To-Do List Manager Commands ---"));
        assert!(output.contains("add <description>"));
        assert!(output.contains("list [status] [priority]"));
        assert!(output.contains("remove <id|range|all>"));
        assert!(output.contains("complete <id|range|all>"));
        assert!(output.contains("priority <id|range|all> <level>"));
        assert!(output.contains("set-category <id|range|all> <name>"));
        assert!(output.contains("help"));
        assert!(output.contains("quit"));
    }

    #[test]
    fn test_show_debug_enabled() {
        setup();
        let mut output = Vec::new();
        let output_writer = FileOutputWriter::new(&mut output);
        let mut manager = GeneralCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_debug_enabled();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Debug mode enabled"));
        assert!(output_str.contains("debug:gen"));
        assert!(output_str.contains("debug:clear"));
        assert!(output_str.contains("debug:gen-projects"));
        assert!(output_str.contains("debug:clear-projects"));
        assert!(output_str.contains("Additional debug commands"));
    }

    #[test]
    fn test_show_debug_disabled() {
        setup();
        let mut output = Vec::new();
        let output_writer = FileOutputWriter::new(&mut output);
        let mut manager = GeneralCommandOutputManager::new(Rc::new(RefCell::new(output_writer)));

        manager.show_debug_disabled();

        let output_str = String::from_utf8(output).unwrap();
        assert!(output_str.contains("Debug mode disabled"));
    }
}
