use std::io::{self, Write};
use crate::models::task::Task;
use crate::models::priority::Priority;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use colored::*;
use super::formatters::{TaskFormatter, MessageFormatter};

/// Handles output operations for the command-line interface.
///
/// `OutputWriter` is responsible for displaying messages to the user.
/// It provides semantic methods for different types of output.
///
/// # Examples
///
/// ```
/// use todo_manager::ui::output_writer::OutputWriter;
///
/// let mut output = OutputWriter::new();
/// output.print_line("Hello, World!");
/// ```
pub struct OutputWriter<W: Write = io::Stdout> {
    writer: W,
}

impl OutputWriter<io::Stdout> {
    /// Creates a new output writer that writes to stdout.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let output = OutputWriter::new();
    /// ```
    pub fn new() -> Self {
        OutputWriter {
            writer: io::stdout(),
        }
    }
}

impl<W: Write> OutputWriter<W> {
    /// Creates a new output writer with a custom writer (for testing).
    pub fn with_writer(writer: W) -> Self {
        OutputWriter { writer }
    }

    /// Displays the welcome message.
    pub fn show_welcome(&mut self) {
        self.print_line("");
        self.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.print_line(&"       ████████╗  ██████╗  ██████╗   ██████╗       ".bright_cyan().bold().to_string());
        self.print_line(&"       ╚══██╔══╝ ██╔═══██╗ ██╔══██╗ ██╔═══██╗      ".bright_cyan().bold().to_string());
        self.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.print_line(&"          ██║    ██║   ██║ ██║  ██║ ██║   ██║      ".bright_cyan().bold().to_string());
        self.print_line(&"          ██║    ╚██████╔╝ ██████╔╝ ╚██████╔╝      ".bright_cyan().bold().to_string());
        self.print_line(&"          ╚═╝     ╚═════╝  ╚═════╝   ╚═════╝       ".bright_cyan().bold().to_string());
        self.print_line(&"                 📝 LIST MANAGER 📝                 ".bright_green().bold().to_string());
        self.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.print_line("");
        self.print_line(&"    Welcome to your personal task management system!".white().to_string());
        self.print_line("");
        self.print_line(&format!("    Type {} to see available commands.", "help".bright_yellow().bold()));
        self.print_line("");
        self.print_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.print_line("");
    }

    /// Displays the help message.
    pub fn show_help(&mut self) {
        self.print_line(&format!("\n{}", MessageFormatter::section_title("To-Do List Manager Commands")));
        self.print_line("");
        self.print_line(&MessageFormatter::command("add <description>", "Add a new task"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("list [status] [priority]", "List tasks (filters can be combined)"));
        self.print_line(&MessageFormatter::subinfo("Status:", "completed/done, pending/todo"));
        self.print_line(&MessageFormatter::subinfo("Priority:", "high/h, medium/med/m, low/l"));
        self.print_line(&MessageFormatter::subinfo("Example:", "list pending high"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("remove <id>", "Remove a task by ID"));
        self.print_line(&MessageFormatter::label("Aliases:", "rm, delete"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("complete <id>", "Mark task as completed"));
        self.print_line(&MessageFormatter::label("Alias:", "done"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("uncomplete <id>", "Mark task as pending"));
        self.print_line(&MessageFormatter::label("Alias:", "undo"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("toggle <id>", "Toggle task completion status"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("priority <id> <level>", "Set task priority"));
        self.print_line(&MessageFormatter::subinfo("Levels:", "high/h, medium/med/m, low/l"));
        self.print_line(&MessageFormatter::label("Alias:", "pri"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("edit <id> <description>", "Edit task description"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("search <keyword>", "Search tasks by keyword"));
        self.print_line(&MessageFormatter::label("Alias:", "find"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("statistics", "Display task statistics"));
        self.print_line(&MessageFormatter::label("Alias:", "stats"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("help", "Show this help message"));
        self.print_line(&MessageFormatter::label("Alias:", "h"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("quit", "Exit the program"));
        self.print_line(&MessageFormatter::label("Aliases:", "q, exit"));
        self.print_line("");
        self.print_line(&format!("{}\n", MessageFormatter::separator(40)));
    }

    /// Displays debug mode help information.
    pub fn show_debug_help(&mut self) {
        self.print_line(&MessageFormatter::section_title("Debug Commands"));
        self.print_line("");
        self.print_line(&MessageFormatter::command("debug:gen <count>", "Generate N random tasks"));
        self.print_line(&MessageFormatter::command("debug:clear", "Clear all tasks"));
        self.print_line(&MessageFormatter::command("debug", "Toggle debug mode"));
        self.print_line("");
    }

    /// Displays a list of all tasks.
    pub fn show_all_tasks(&mut self, tasks: &[Task]) {
        if tasks.is_empty() {
            self.print_line(&MessageFormatter::warning("No tasks found. Use 'add <description>' to create a task."));
            return;
        }

        self.show_task_list("All Tasks", tasks.iter().collect());
    }

    /// Displays a list of completed tasks.
    pub fn show_completed_tasks(&mut self, tasks: &[&Task]) {
        if tasks.is_empty() {
            self.print_line(&MessageFormatter::warning("No completed tasks found."));
            return;
        }

        self.show_task_list("Completed Tasks", tasks.to_vec());
    }

    /// Displays a list of pending tasks.
    pub fn show_pending_tasks(&mut self, tasks: &[&Task]) {
        if tasks.is_empty() {
            self.print_line(&MessageFormatter::warning("No pending tasks found."));
            return;
        }

        self.show_task_list("Pending Tasks", tasks.to_vec());
    }

    /// Displays tasks filtered by priority level.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A slice of task references to display
    /// * `priority` - The priority level being filtered
    pub fn show_tasks_by_priority(&mut self, tasks: &[&Task], priority: Priority) {
        if tasks.is_empty() {
            let message = format!("No {} priority tasks found.", priority.as_str());
            self.print_line(&MessageFormatter::warning(&message));
            return;
        }

        let title = format!("{} Priority Tasks", priority.as_str());
        self.show_task_list(&title, tasks.to_vec());
    }

    /// Displays tasks filtered by both status and priority.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A slice of task references to display
    /// * `filter` - The filter criteria used
    pub fn show_filtered_tasks(&mut self, tasks: &[&Task], filter: &TaskFilter) {
        if tasks.is_empty() {
            let status_str = match filter.status {
                Some(TaskStatus::Completed) => "completed ",
                Some(TaskStatus::Pending) => "pending ",
                None => "",
            };
            let priority_str = match filter.priority {
                Some(priority) => format!("{} priority ", priority.as_str()),
                None => String::new(),
            };
            let message = format!("No {}{}tasks found.", status_str, priority_str);
            self.print_line(&MessageFormatter::warning(&message));
            return;
        }

        // Build title based on filter
        let mut title_parts = Vec::new();
        if let Some(priority) = filter.priority {
            title_parts.push(format!("{} Priority", priority.as_str()));
        }
        if let Some(status) = filter.status {
            match status {
                TaskStatus::Completed => title_parts.push("Completed".to_string()),
                TaskStatus::Pending => title_parts.push("Pending".to_string()),
            }
        }
        
        let title = if title_parts.is_empty() {
            "Tasks".to_string()
        } else {
            format!("{} Tasks", title_parts.join(" "))
        };
        
        self.show_task_list(&title, tasks.to_vec());
    }

    /// Displays search results for tasks matching a keyword.
    ///
    /// # Arguments
    ///
    /// * `tasks` - A slice of task references that match the search
    /// * `keyword` - The search keyword used
    pub fn show_search_results(&mut self, tasks: &[&Task], keyword: &str) {
        if tasks.is_empty() {
            let message = format!("No tasks found matching '{}'.", keyword);
            self.print_line(&MessageFormatter::warning(&message));
            return;
        }

        let title = format!("Search Results for '{}'", keyword);
        self.show_task_list(&title, tasks.to_vec());
    }

    /// Displays task statistics.
    ///
    /// Shows comprehensive statistics including:
    /// - Total task count
    /// - Completed and pending counts
    /// - Completion percentage
    /// - Task breakdown by priority
    ///
    /// # Arguments
    ///
    /// * `stats` - The TaskStatistics struct containing all statistics
    pub fn show_statistics(&mut self, stats: &crate::models::TaskStatistics) {
        use colored::Colorize;
        
        self.print_line("");
        self.print_line(&MessageFormatter::section_title("Task Statistics"));
        self.print_line("");
        
        // Overall statistics
        self.print_line(&format!("  {}: {}", 
            "Total Tasks".bright_white().bold(), 
            stats.total));
        self.print_line(&format!("  {}: {}", 
            "Completed".bright_white().bold(), 
            stats.completed.to_string().green()));
        self.print_line(&format!("  {}: {}", 
            "Pending".bright_white().bold(), 
            stats.pending.to_string().yellow()));
        self.print_line(&format!("  {}: {:.1}%", 
            "Completion".bright_white().bold(), 
            stats.completion_percentage));
        
        // Priority breakdown
        if stats.total > 0 {
            self.print_line("");
            self.print_line(&format!("  {}", "By Priority:".bright_cyan()));
            self.print_line(&format!("    {} {}", 
                "▲ High:  ".red(), 
                stats.high_priority));
            self.print_line(&format!("    {} {}", 
                "■ Medium:".yellow(), 
                stats.medium_priority));
            self.print_line(&format!("    {} {}", 
                "▼ Low:   ".blue(), 
                stats.low_priority));
        }
        
        self.print_line("");
    }

    /// Helper method to display a list of tasks with a given title.
    ///
    /// # Arguments
    ///
    /// * `title` - The title to display above the task list
    /// * `tasks` - A vector of task references to display
    fn show_task_list(&mut self, title: &str, tasks: Vec<&Task>) {
        let separator_length = title.len() + 8;
        let max_id_width = TaskFormatter::calculate_max_id_width(&tasks);
        
        self.print_line(&format!("\n{}", MessageFormatter::section_title(title)));
        
        for task in &tasks {
            let formatted_task = TaskFormatter::format_task(task, max_id_width);
            self.print_line(&formatted_task);
        }
        
        self.print_line(&format!("{}\n", MessageFormatter::separator(separator_length)));
    }

    /// Displays a message when a task is successfully added.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the newly added task
    /// * `description` - The description of the task
    pub fn show_task_added(&mut self, task_id: usize, description: &str) {
        let message = format!("Task added with ID {}: '{}'", task_id.to_string().bright_blue(), description);
        self.print_line(&MessageFormatter::success(&message));
    }

    /// Displays a message when a task is successfully removed.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the removed task
    pub fn show_task_removed(&mut self, description: &str) {
        let message = format!("Task removed: '{}'", description);
        self.print_line(&MessageFormatter::success(&message));
    }

    /// Displays an error message when a task is not found.
    ///
    /// # Arguments
    ///
    /// * `task_id` - The ID of the task that was not found
    pub fn show_task_not_found(&mut self, task_id: usize) {
        let message = format!("Task with ID {} not found.", task_id.to_string().bright_blue());
        self.print_line(&MessageFormatter::error(&message));
    }

    /// Displays a message when a task is marked as completed.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the completed task
    pub fn show_task_completed(&mut self, description: &str) {
        let message = format!("Task '{}' marked as completed.", description);
        self.print_line(&MessageFormatter::success(&message));
    }

    /// Displays a message when a task is marked as pending.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the task
    pub fn show_task_uncompleted(&mut self, description: &str) {
        let message = format!("Task '{}' marked as pending.", description);
        self.print_line(&MessageFormatter::warning(&message));
    }

    /// Displays a message when a task status is toggled.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the task
    /// * `is_completed` - Whether the task is now completed
    pub fn show_task_toggled(&mut self, description: &str, is_completed: bool) {
        let message = format!("Task '{}' marked as {}.", description, 
            if is_completed { "completed".bright_green() } else { "pending".bright_yellow() });
        if is_completed {
            self.print_line(&MessageFormatter::success(&message));
        } else {
            self.print_line(&MessageFormatter::warning(&message));
        }
    }

    /// Displays a message when a task description is edited.
    ///
    /// # Arguments
    ///
    /// * `old_description` - The previous description of the task
    /// * `new_description` - The new description of the task
    pub fn show_task_edited(&mut self, old_description: &str, new_description: &str) {
        let message = format!("Task '{}' updated to '{}'.", old_description, new_description);
        self.print_line(&MessageFormatter::success(&message));
    }

    /// Displays a message when a task priority is set.
    ///
    /// # Arguments
    ///
    /// * `description` - The description of the task
    /// * `priority` - The new priority level
    pub fn show_priority_set(&mut self, description: &str, priority: Priority) {
        let colored_priority = TaskFormatter::format_priority_with_name(priority);
        let message = format!("Priority set to {} for task: '{}'", colored_priority, description);
        self.print_line(&MessageFormatter::success(&message));
    }

    /// Displays a goodbye message.
    pub fn show_goodbye(&mut self) {
        self.print_line("");
        self.print_line(&"─────────────────────────────────────────────────────".bright_black().to_string());
        self.print_line("");
        self.print_line(&"    ✨ Thank you for using To-Do List Manager! ✨    ".bright_cyan().bold().to_string());
        self.print_line("");
        self.print_line(&"           Stay organized and productive! 🚀          ".bright_green().to_string());
        self.print_line("");
        self.print_line(&"═════════════════════════════════════════════════════".bright_cyan().bold().to_string());
        self.print_line("");
    }

    /// Displays an error message for unknown commands.
    ///
    /// # Arguments
    ///
    /// * `command` - The unknown command entered
    pub fn show_unknown_command(&mut self, command: &str) {
        let message = format!("Unknown command '{}'. Type {} for available commands.", 
            command.bright_yellow(), "help".bright_yellow());
        self.print_line(&MessageFormatter::error(&message));
    }

    /// Displays an error message for invalid input.
    ///
    /// # Arguments
    ///
    /// * `message` - The error message to display
    pub fn show_error(&mut self, message: &str) {
        self.print_line(&MessageFormatter::error(message));
    }

    /// Displays a success message to the user.
    ///
    /// # Arguments
    ///
    /// * `message` - The success message to display
    pub fn show_success(&mut self, message: &str) {
        self.print_line(&MessageFormatter::success(message));
    }

    /// Prints a line of text to the output.
    ///
    /// # Arguments
    ///
    /// * `text` - The text to print
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let mut output = OutputWriter::new();
    /// output.print_line("Task added successfully!");
    /// ```
    pub fn print_line(&mut self, text: &str) {
        writeln!(self.writer, "{}", text).unwrap();
    }

    /// Displays the command prompt without a newline.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::output_writer::OutputWriter;
    ///
    /// let mut output = OutputWriter::new();
    /// output.print_prompt();
    /// ```
    pub fn print_prompt(&mut self) {
        write!(self.writer, "{}", "> ".bright_green().bold()).unwrap();
        self.writer.flush().unwrap();
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
    fn test_new_output_writer() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("Test message");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Test message\n");
    }

    #[test]
    fn test_show_task_added() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_added(1, "Buy groceries");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task added with ID 1: 'Buy groceries'\n");
    }

    #[test]
    fn test_show_task_added_large_id() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_added(42, "Complete project");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task added with ID 42: 'Complete project'\n");
    }

    #[test]
    fn test_show_task_removed() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_removed("Buy groceries");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task removed: 'Buy groceries'\n");
    }

    #[test]
    fn test_show_task_removed_special_chars() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_removed("Task with special chars: !@#$%");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task removed: 'Task with special chars: !@#$%'\n");
    }

    #[test]
    fn test_show_task_not_found() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_not_found(1);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Task with ID 1 not found.\n");
    }

    #[test]
    fn test_show_task_not_found_large_id() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_not_found(999);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Task with ID 999 not found.\n");
    }

    #[test]
    fn test_show_task_completed() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_completed("Finish homework");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task 'Finish homework' marked as completed.\n");
    }

    #[test]
    fn test_show_task_uncompleted() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_uncompleted("Review code");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "↻ Task 'Review code' marked as pending.\n");
    }

    #[test]
    fn test_show_task_toggled_completed() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_toggled("Write tests", true);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✓ Task 'Write tests' marked as completed.\n");
    }

    #[test]
    fn test_show_task_toggled_pending() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_toggled("Fix bug", false);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "↻ Task 'Fix bug' marked as pending.\n");
    }

    #[test]
    fn test_show_goodbye() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_goodbye();
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Thank you for using To-Do List Manager"));
        assert!(result.contains("Stay organized and productive"));
    }

    #[test]
    fn test_show_welcome() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_welcome();
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("LIST MANAGER"));
        assert!(result.contains("Welcome to your personal task management system"));
        assert!(result.contains("Type help to see available commands"));
    }

    #[test]
    fn test_show_help() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_help();
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("--- To-Do List Manager Commands ---"));
        assert!(result.contains("add <description>"));
        assert!(result.contains("list [status] [priority]"));
        assert!(result.contains("remove <id>"));
        assert!(result.contains("complete <id>"));
        assert!(result.contains("uncomplete <id>"));
        assert!(result.contains("toggle <id>"));
        assert!(result.contains("priority <id> <level>"));
        assert!(result.contains("help"));
        assert!(result.contains("quit"));
        assert!(result.contains("Alias:") || result.contains("Aliases:"));
        assert!(result.contains("rm, delete"));
        assert!(result.contains("undo"));
    }

    #[test]
    fn test_show_unknown_command() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_unknown_command("foo");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Unknown command 'foo'. Type help for available commands.\n");
    }

    #[test]
    fn test_show_unknown_command_different() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_unknown_command("bar123");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Unknown command 'bar123'. Type help for available commands.\n");
    }

    #[test]
    fn test_show_error() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_error("Invalid input");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Invalid input\n");
    }

    #[test]
    fn test_show_error_different_message() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_error("Task ID must be a number");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "✗ Task ID must be a number\n");
    }

    #[test]
    fn test_show_all_tasks_empty() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks: Vec<Task> = vec![];
        output.show_all_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "↻ No tasks found. Use 'add <description>' to create a task.\n");
    }

    #[test]
    fn test_show_all_tasks_with_tasks() {
        setup();
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks = vec![
            Task { id: 1, description: "Task 1".to_string(), completed: false, priority: Priority::Medium },
            Task { id: 2, description: "Task 2".to_string(), completed: true, priority: Priority::Medium },
        ];
        output.show_all_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("All Tasks"));
        assert!(result.contains("1. [ ] ■ Task 1"));
        assert!(result.contains("2. [✓] ■ Task 2"));
    }

    #[test]
    fn test_show_completed_tasks_empty() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks: Vec<&Task> = vec![];
        output.show_completed_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "↻ No completed tasks found.\n");
    }

    #[test]
    fn test_show_completed_tasks_with_tasks() {
        setup();
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let task1 = Task { id: 1, description: "Completed task".to_string(), completed: true, priority: Priority::Medium };
        let task2 = Task { id: 2, description: "Another completed".to_string(), completed: true, priority: Priority::Medium };
        let tasks = vec![&task1, &task2];
        output.show_completed_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Completed Tasks"));
        assert!(result.contains("1. [✓] ■ Completed task"));
        assert!(result.contains("2. [✓] ■ Another completed"));
    }

    #[test]
    fn test_show_pending_tasks_empty() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks: Vec<&Task> = vec![];
        output.show_pending_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "↻ No pending tasks found.\n");
    }

    #[test]
    fn test_show_pending_tasks_with_tasks() {
        setup();
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let task1 = Task { id: 1, description: "Pending task".to_string(), completed: false, priority: Priority::Medium };
        let task2 = Task { id: 2, description: "Another pending".to_string(), completed: false, priority: Priority::Medium };
        let tasks = vec![&task1, &task2];
        output.show_pending_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Pending Tasks"));
        assert!(result.contains("1. [ ] ■ Pending task"));
        assert!(result.contains("2. [ ] ■ Another pending"));
    }

    #[test]
    fn test_show_tasks_with_special_characters() {
        setup();
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks = vec![
            Task { id: 1, description: "Task with émojis 🎉".to_string(), completed: false, priority: Priority::Medium },
            Task { id: 2, description: "Special chars: <>&\"'".to_string(), completed: true, priority: Priority::Medium },
        ];
        output.show_all_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("Task with émojis 🎉"));
        assert!(result.contains("Special chars: <>&\"'"));
    }

    #[test]
    fn test_show_tasks_with_long_description() {
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let long_desc = "A".repeat(200);
        let tasks = vec![
            Task { id: 1, description: long_desc.clone(), completed: false, priority: Priority::Medium },
        ];
        output.show_all_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains(&long_desc));
    }

    #[test]
    fn test_print_line_empty_string() {
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "\n");
    }

    #[test]
    fn test_print_line_multiline_string() {
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_line("Line 1\nLine 2\nLine 3");
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "Line 1\nLine 2\nLine 3\n");
    }

    #[test]
    fn test_print_prompt() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.print_prompt();
        let result = String::from_utf8(buffer).unwrap();
        assert_eq!(result, "> ");
    }

    #[test]
    fn test_task_list_separator_length() {
        setup();
        use crate::models::task::Task;
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        let tasks = vec![
            Task { id: 1, description: "Test".to_string(), completed: false, priority: Priority::Medium },
        ];
        output.show_all_tasks(&tasks);
        let result = String::from_utf8(buffer).unwrap();
        // Separator should be "--- All Tasks ---" length + 8 dashes
        let separator = "-".repeat("All Tasks".len() + 8);
        assert!(result.contains(&separator));
    }

    #[test]
    fn test_multiple_operations() {
        setup();
        let mut buffer = Vec::new();
        let mut output = OutputWriter::with_writer(&mut buffer);
        output.show_task_added(1, "First task");
        output.show_task_added(2, "Second task");
        output.show_task_completed("First task");
        let result = String::from_utf8(buffer).unwrap();
        assert!(result.contains("✓ Task added with ID 1: 'First task'"));
        assert!(result.contains("✓ Task added with ID 2: 'Second task'"));
        assert!(result.contains("✓ Task 'First task' marked as completed"));
    }
}
