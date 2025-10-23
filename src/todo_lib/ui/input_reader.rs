use std::io::{self, BufRead, BufReader, Read};
use crate::models::ui_event::UiEvent;
use crate::models::task_filter::TaskFilter;
use crate::models::task_status::TaskStatus;
use crate::models::overdue_filter::OverdueFilter;
use crate::models::priority::Priority;

/// Handles input operations for the command-line interface.
///
/// `InputReader` is responsible for reading user input and parsing commands
/// into events that can be processed by the application.
///
/// # Examples
///
/// ```no_run
/// use todo_manager::ui::input_reader::InputReader;
///
/// let mut input = InputReader::new();
/// let event = input.read_event();
/// ```
pub struct InputReader<R: Read = io::Stdin> {
    reader: BufReader<R>,
}

impl InputReader<io::Stdin> {
    /// Creates a new input reader that reads from stdin.
    ///
    /// # Examples
    ///
    /// ```
    /// use todo_manager::ui::input_reader::InputReader;
    ///
    /// let mut input = InputReader::new();
    /// ```
    pub fn new() -> Self {
        InputReader {
            reader: BufReader::new(io::stdin()),
        }
    }
}

impl<R: Read> InputReader<R> {
    /// Creates a new input reader with a custom reader for testing.
    ///
    /// # Arguments
    ///
    /// * `reader` - Any type implementing Read
    pub fn with_reader(reader: R) -> Self {
        InputReader {
            reader: BufReader::new(reader),
        }
    }

    /// Reads user input and parses it into a UI event.
    ///
    /// # Returns
    ///
    /// A `UiEvent` representing the user's command.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::input_reader::InputReader;
    ///
    /// let mut input = InputReader::new();
    /// let event = input.read_event();
    /// ```
    pub fn read_event(&mut self) -> UiEvent {
        let input = self.read_input();
        self.parse_command(&input)
    }

    /// Parses a command string into a UI event.
    fn parse_command(&self, input: &str) -> UiEvent {
        let parts: Vec<&str> = input.split_whitespace().collect();
        
        if parts.is_empty() {
            return UiEvent::InvalidInput("Please enter a command. Type 'help' for available commands.".to_string());
        }

        let command = parts[0].to_lowercase();
        let args: Vec<&str> = parts[1..].to_vec();

        match command.as_str() {
            "add" => self.parse_add_command(&args),
            "list" => self.parse_list_command(&args),
            "remove" | "delete" | "rm" => self.parse_remove_command(&args),
            "complete" | "done" => self.parse_complete_command(&args),
            "uncomplete" | "undo" => self.parse_uncomplete_command(&args),
            "toggle" => self.parse_toggle_command(&args),
            "priority" | "pri" => self.parse_priority_command(&args),
            "set-due" | "due" => self.parse_set_due_command(&args),
            "edit" => self.parse_edit_command(&args),
            "search" | "find" => self.parse_search_command(&args),
            "statistics" | "stats" => UiEvent::ShowStatistics,
            "debug" => self.parse_debug_command(&args),
            "debug:gen" => self.parse_debug_generate_command(&args),
            "debug:clear" => UiEvent::DebugClearAll,
            "help" | "h" => UiEvent::ShowHelp,
            "quit" | "exit" | "q" => UiEvent::Quit,
            _ => UiEvent::UnknownCommand(command),
        }
    }

    /// Parses the 'add' command and validates the task description.
    fn parse_add_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::InvalidInput("Usage: add <task description>".to_string())
        } else {
            let description = args.join(" ");
            if description.trim().is_empty() {
                UiEvent::InvalidInput("Task description cannot be empty.".to_string())
            } else {
                UiEvent::AddTask(description)
            }
        }
    }

    /// Parses the 'list' command and determines the filter.
    fn parse_list_command(&self, args: &[&str]) -> UiEvent {
        let mut filter = TaskFilter::all();
        let mut status_set = false;
        let mut priority_set = false;
        
        // Parse all arguments to allow combinations like "list completed high overdue"
        for arg in args.iter().map(|s| s.to_lowercase()) {
            match arg.as_str() {
                "completed" | "done" => {
                    if status_set {
                        return UiEvent::InvalidInput(
                            "Cannot specify multiple status filters (done/todo).".to_string()
                        );
                    }
                    filter = filter.with_status(TaskStatus::Completed);
                    status_set = true;
                }
                "pending" | "todo" => {
                    if status_set {
                        return UiEvent::InvalidInput(
                            "Cannot specify multiple status filters (done/todo).".to_string()
                        );
                    }
                    filter = filter.with_status(TaskStatus::Pending);
                    status_set = true;
                }
                "high" | "h" => {
                    if priority_set {
                        return UiEvent::InvalidInput(
                            "Cannot specify multiple priority filters (high/medium/low).".to_string()
                        );
                    }
                    filter = filter.with_priority(Priority::High);
                    priority_set = true;
                }
                "medium" | "med" | "m" => {
                    if priority_set {
                        return UiEvent::InvalidInput(
                            "Cannot specify multiple priority filters (high/medium/low).".to_string()
                        );
                    }
                    filter = filter.with_priority(Priority::Medium);
                    priority_set = true;
                }
                "low" | "l" => {
                    if priority_set {
                        return UiEvent::InvalidInput(
                            "Cannot specify multiple priority filters (high/medium/low).".to_string()
                        );
                    }
                    filter = filter.with_priority(Priority::Low);
                    priority_set = true;
                }
                "overdue" => {
                    filter = filter.with_overdue(OverdueFilter::OnlyOverdue);
                }
                _ => {
                    return UiEvent::InvalidInput(
                        format!("Unknown filter: '{}'. Valid filters: done, todo, high, medium, low, overdue", arg)
                    );
                }
            }
        }
        
        // If no filter was specified, return None to show all tasks
        if filter.status.is_none() && filter.priority.is_none() && filter.overdue == OverdueFilter::All {
            UiEvent::ListTasks(None)
        } else {
            UiEvent::ListTasks(Some(filter))
        }
    }
    /// Parses the 'remove' command and validates the task ID.
    fn parse_remove_command(&self, args: &[&str]) -> UiEvent {
        self.parse_id_command(args, "remove", UiEvent::RemoveTask)
    }

    /// Parses the 'complete' command and validates the task ID.
    fn parse_complete_command(&self, args: &[&str]) -> UiEvent {
        self.parse_id_command(args, "complete", UiEvent::CompleteTask)
    }

    /// Parses the 'uncomplete' command and validates the task ID.
    fn parse_uncomplete_command(&self, args: &[&str]) -> UiEvent {
        self.parse_id_command(args, "uncomplete", UiEvent::UncompleteTask)
    }

    /// Parses the 'toggle' command and validates the task ID.
    fn parse_toggle_command(&self, args: &[&str]) -> UiEvent {
        self.parse_id_command(args, "toggle", UiEvent::ToggleTask)
    }

    /// Parses the 'priority' command to set task priority.
    fn parse_priority_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            return UiEvent::InvalidInput("Usage: priority <task_id> <high|medium|low>".to_string());
        }
        
        match args[0].parse::<usize>() {
            Ok(id) => {
                match Priority::from_str(args[1]) {
                    Some(priority) => UiEvent::SetPriority(id, priority),
                    None => UiEvent::InvalidInput(
                        "Invalid priority level. Use: high, medium, or low".to_string()
                    ),
                }
            }
            Err(_) => UiEvent::InvalidInput("Please enter a valid task ID (number).".to_string()),
        }
    }

    /// Parses the 'set-due' command to set or clear a task's due date.
    fn parse_set_due_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            return UiEvent::InvalidInput("Usage: set-due <task_id> <DD.MM.YYYY|none>".to_string());
        }
        
        match args[0].parse::<usize>() {
            Ok(id) => {
                // Check if user wants to clear the due date
                if args[1].to_lowercase() == "none" || args[1].to_lowercase() == "clear" {
                    return UiEvent::SetDueDate(id, None);
                }
                
                // Try to parse the date in DD.MM.YYYY format
                match chrono::NaiveDate::parse_from_str(args[1], "%d.%m.%Y") {
                    Ok(date) => UiEvent::SetDueDate(id, Some(date)),
                    Err(_) => UiEvent::InvalidInput(
                        "Invalid date format. Use DD.MM.YYYY (e.g., 31.12.2025) or 'none' to clear.".to_string()
                    ),
                }
            }
            Err(_) => UiEvent::InvalidInput("Please enter a valid task ID (number).".to_string()),
        }
    }

    /// Parses the 'search' command and validates the keyword.
    fn parse_search_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::InvalidInput("Usage: search <keyword>".to_string())
        } else {
            let keyword = args.join(" ");
            if keyword.trim().is_empty() {
                UiEvent::InvalidInput("Search keyword cannot be empty.".to_string())
            } else {
                UiEvent::SearchTasks(keyword)
            }
        }
    }

    /// Parses the 'edit' command to edit a task description.
    fn parse_edit_command(&self, args: &[&str]) -> UiEvent {
        if args.len() < 2 {
            return UiEvent::InvalidInput("Usage: edit <id> <new description>".to_string());
        }
        
        match args[0].parse::<usize>() {
            Ok(id) => {
                let new_description = args[1..].join(" ");
                if new_description.trim().is_empty() {
                    UiEvent::InvalidInput("Task description cannot be empty.".to_string())
                } else {
                    UiEvent::EditTask(id, new_description)
                }
            }
            Err(_) => UiEvent::InvalidInput(format!("Invalid task ID: '{}'", args[0])),
        }
    }

    /// Parses the 'debug' command to toggle debug mode.
    fn parse_debug_command(&self, args: &[&str]) -> UiEvent {
        if !args.is_empty() {
            UiEvent::InvalidInput("Usage: debug (no arguments needed)".to_string())
        } else {
            UiEvent::DebugToggle
        }
    }

    /// Parses the 'debug:gen' command to generate random tasks.
    fn parse_debug_generate_command(&self, args: &[&str]) -> UiEvent {
        if args.is_empty() {
            UiEvent::InvalidInput("Usage: debug:gen <count>".to_string())
        } else {
            match args[0].parse::<usize>() {
                Ok(count) if count > 0 && count <= 100 => UiEvent::DebugGenerateTasks(count),
                Ok(_) => UiEvent::InvalidInput("Count must be between 1 and 100".to_string()),
                Err(_) => UiEvent::InvalidInput("Please enter a valid number.".to_string()),
            }
        }
    }

    /// Helper method to parse commands that require a task ID.
    fn parse_id_command<F>(&self, args: &[&str], command_name: &str, constructor: F) -> UiEvent
    where
        F: FnOnce(usize) -> UiEvent,
    {
        if args.is_empty() {
            UiEvent::InvalidInput(format!("Usage: {} <task_id>", command_name))
        } else {
            match args[0].parse::<usize>() {
                Ok(id) => constructor(id),
                Err(_) => UiEvent::InvalidInput("Please enter a valid task ID (number).".to_string()),
            }
        }
    }

    /// Reads a line of input from the user.
    ///
    /// # Returns
    ///
    /// A trimmed string containing the user's input.
    ///
    /// # Panics
    ///
    /// Panics if reading from the reader fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use todo_manager::ui::input_reader::InputReader;
    ///
    /// let mut input = InputReader::new();
    /// let user_input = input.read_input();
    /// println!("You entered: {}", user_input);
    /// ```
    pub fn read_input(&mut self) -> String {
        let mut input = String::new();
        self.reader
            .read_line(&mut input)
            .expect("Failed to read input");
        input.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_input_reader() {
        let _input = InputReader::new();
        // Just test that it can be created
    }

    #[test]
    fn test_parse_add_command_valid() {
        let input = InputReader::new();
        let event = input.parse_command("add Buy groceries");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Buy groceries"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_parse_add_command_multiple_words() {
        let input = InputReader::new();
        let event = input.parse_command("add This is a longer task description");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "This is a longer task description"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_parse_add_command_empty() {
        let input = InputReader::new();
        let event = input.parse_command("add");
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("Usage")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_add_command_whitespace_only() {
        let input = InputReader::new();
        // When input is "add    ", split_whitespace removes trailing spaces,
        // so it becomes just "add" with no arguments
        let event = input.parse_command("add    ");
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("Usage")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_list_command_all() {
        let input = InputReader::new();
        let event = input.parse_command("list");
        match event {
            UiEvent::ListTasks(None) => {},
            _ => panic!("Expected ListTasks with no filter"),
        }
    }

    #[test]
    fn test_parse_list_command_completed() {
        let input = InputReader::new();
        let event = input.parse_command("list completed");
        match event {
            UiEvent::ListTasks(Some(filter)) if filter.status == Some(TaskStatus::Completed) && filter.priority.is_none() => {},
            _ => panic!("Expected ListTasks with Completed filter"),
        }
    }

    #[test]
    fn test_parse_list_command_done_alias() {
        let input = InputReader::new();
        let event = input.parse_command("list done");
        match event {
            UiEvent::ListTasks(Some(filter)) if filter.status == Some(TaskStatus::Completed) && filter.priority.is_none() => {},
            _ => panic!("Expected ListTasks with Completed filter"),
        }
    }

    #[test]
    fn test_parse_list_command_pending() {
        let input = InputReader::new();
        let event = input.parse_command("list pending");
        match event {
            UiEvent::ListTasks(Some(filter)) if filter.status == Some(TaskStatus::Pending) && filter.priority.is_none() => {},
            _ => panic!("Expected ListTasks with Pending filter"),
        }
    }

    #[test]
    fn test_parse_list_command_todo_alias() {
        let input = InputReader::new();
        let event = input.parse_command("list todo");
        match event {
            UiEvent::ListTasks(Some(filter)) if filter.status == Some(TaskStatus::Pending) && filter.priority.is_none() => {},
            _ => panic!("Expected ListTasks with Pending filter"),
        }
    }

    #[test]
    fn test_parse_remove_command_valid() {
        let input = InputReader::new();
        let event = input.parse_command("remove 42");
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 42),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_parse_remove_command_delete_alias() {
        let input = InputReader::new();
        let event = input.parse_command("delete 5");
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 5),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_parse_remove_command_rm_alias() {
        let input = InputReader::new();
        let event = input.parse_command("rm 10");
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 10),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_parse_remove_command_no_id() {
        let input = InputReader::new();
        let event = input.parse_command("remove");
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("Usage")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_remove_command_invalid_id() {
        let input = InputReader::new();
        let event = input.parse_command("remove abc");
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("valid task ID")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_complete_command_valid() {
        let input = InputReader::new();
        let event = input.parse_command("complete 3");
        match event {
            UiEvent::CompleteTask(id) => assert_eq!(id, 3),
            _ => panic!("Expected CompleteTask event"),
        }
    }

    #[test]
    fn test_parse_complete_command_done_alias() {
        let input = InputReader::new();
        let event = input.parse_command("done 7");
        match event {
            UiEvent::CompleteTask(id) => assert_eq!(id, 7),
            _ => panic!("Expected CompleteTask event"),
        }
    }

    #[test]
    fn test_parse_complete_command_no_id() {
        let input = InputReader::new();
        let event = input.parse_command("complete");
        match event {
            UiEvent::InvalidInput(_) => {},
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_uncomplete_command_valid() {
        let input = InputReader::new();
        let event = input.parse_command("uncomplete 8");
        match event {
            UiEvent::UncompleteTask(id) => assert_eq!(id, 8),
            _ => panic!("Expected UncompleteTask event"),
        }
    }

    #[test]
    fn test_parse_uncomplete_command_undo_alias() {
        let input = InputReader::new();
        let event = input.parse_command("undo 12");
        match event {
            UiEvent::UncompleteTask(id) => assert_eq!(id, 12),
            _ => panic!("Expected UncompleteTask event"),
        }
    }

    #[test]
    fn test_parse_toggle_command_valid() {
        let input = InputReader::new();
        let event = input.parse_command("toggle 15");
        match event {
            UiEvent::ToggleTask(id) => assert_eq!(id, 15),
            _ => panic!("Expected ToggleTask event"),
        }
    }

    #[test]
    fn test_parse_toggle_command_no_id() {
        let input = InputReader::new();
        let event = input.parse_command("toggle");
        match event {
            UiEvent::InvalidInput(_) => {},
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_help_command() {
        let input = InputReader::new();
        let event = input.parse_command("help");
        match event {
            UiEvent::ShowHelp => {},
            _ => panic!("Expected ShowHelp event"),
        }
    }

    #[test]
    fn test_parse_help_command_h_alias() {
        let input = InputReader::new();
        let event = input.parse_command("h");
        match event {
            UiEvent::ShowHelp => {},
            _ => panic!("Expected ShowHelp event"),
        }
    }

    #[test]
    fn test_parse_quit_command() {
        let input = InputReader::new();
        let event = input.parse_command("quit");
        match event {
            UiEvent::Quit => {},
            _ => panic!("Expected Quit event"),
        }
    }

    #[test]
    fn test_parse_quit_command_exit_alias() {
        let input = InputReader::new();
        let event = input.parse_command("exit");
        match event {
            UiEvent::Quit => {},
            _ => panic!("Expected Quit event"),
        }
    }

    #[test]
    fn test_parse_quit_command_q_alias() {
        let input = InputReader::new();
        let event = input.parse_command("q");
        match event {
            UiEvent::Quit => {},
            _ => panic!("Expected Quit event"),
        }
    }

    #[test]
    fn test_parse_unknown_command() {
        let input = InputReader::new();
        let event = input.parse_command("foobar");
        match event {
            UiEvent::UnknownCommand(cmd) => assert_eq!(cmd, "foobar"),
            _ => panic!("Expected UnknownCommand event"),
        }
    }

    #[test]
    fn test_parse_empty_input() {
        let input = InputReader::new();
        let event = input.parse_command("");
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("enter a command")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_whitespace_only_input() {
        let input = InputReader::new();
        let event = input.parse_command("   ");
        match event {
            UiEvent::InvalidInput(_) => {},
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_command_case_insensitive() {
        let input = InputReader::new();
        
        let event1 = input.parse_command("LIST");
        match event1 {
            UiEvent::ListTasks(_) => {},
            _ => panic!("Expected ListTasks for uppercase"),
        }
        
        let event2 = input.parse_command("LiSt");
        match event2 {
            UiEvent::ListTasks(_) => {},
            _ => panic!("Expected ListTasks for mixed case"),
        }
    }

    #[test]
    fn test_parse_command_with_extra_spaces() {
        let input = InputReader::new();
        let event = input.parse_command("  add    Task with   spaces  ");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Task with spaces"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_parse_id_command_with_zero() {
        let input = InputReader::new();
        let event = input.parse_command("remove 0");
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 0),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_parse_id_command_with_large_number() {
        let input = InputReader::new();
        let event = input.parse_command("complete 999999");
        match event {
            UiEvent::CompleteTask(id) => assert_eq!(id, 999999),
            _ => panic!("Expected CompleteTask event"),
        }
    }

    #[test]
    fn test_parse_id_command_with_negative_number() {
        let input = InputReader::new();
        let event = input.parse_command("toggle -5");
        match event {
            UiEvent::InvalidInput(_) => {},
            _ => panic!("Expected InvalidInput for negative number"),
        }
    }

    #[test]
    fn test_parse_add_with_special_characters() {
        let input = InputReader::new();
        let event = input.parse_command("add Task with !@#$%^&*()");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Task with !@#$%^&*()"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_parse_add_with_unicode() {
        let input = InputReader::new();
        let event = input.parse_command("add Task with émojis 🎉 and ñ");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Task with émojis 🎉 and ñ"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_parse_command_with_tabs() {
        let input = InputReader::new();
        let event = input.parse_command("add\tTask\twith\ttabs");
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Task with tabs"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_all_remove_aliases() {
        let input = InputReader::new();
        
        let aliases = vec!["remove", "delete", "rm"];
        for alias in aliases {
            let event = input.parse_command(&format!("{} 1", alias));
            match event {
                UiEvent::RemoveTask(1) => {},
                _ => panic!("Expected RemoveTask for alias: {}", alias),
            }
        }
    }

    #[test]
    fn test_all_complete_aliases() {
        let input = InputReader::new();
        
        let aliases = vec!["complete", "done"];
        for alias in aliases {
            let event = input.parse_command(&format!("{} 1", alias));
            match event {
                UiEvent::CompleteTask(1) => {},
                _ => panic!("Expected CompleteTask for alias: {}", alias),
            }
        }
    }

    #[test]
    fn test_all_uncomplete_aliases() {
        let input = InputReader::new();
        
        let aliases = vec!["uncomplete", "undo"];
        for alias in aliases {
            let event = input.parse_command(&format!("{} 1", alias));
            match event {
                UiEvent::UncompleteTask(1) => {},
                _ => panic!("Expected UncompleteTask for alias: {}", alias),
            }
        }
    }

    #[test]
    fn test_all_help_aliases() {
        let input = InputReader::new();
        
        let aliases = vec!["help", "h"];
        for alias in aliases {
            let event = input.parse_command(alias);
            match event {
                UiEvent::ShowHelp => {},
                _ => panic!("Expected ShowHelp for alias: {}", alias),
            }
        }
    }

    #[test]
    fn test_all_quit_aliases() {
        let input = InputReader::new();
        
        let aliases = vec!["quit", "exit", "q"];
        for alias in aliases {
            let event = input.parse_command(alias);
            match event {
                UiEvent::Quit => {},
                _ => panic!("Expected Quit for alias: {}", alias),
            }
        }
    }

    // Tests for read_input and read_event with custom readers
    #[test]
    fn test_read_input_with_custom_reader() {
        let input_data = b"add Buy milk\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let result = reader.read_input();
        assert_eq!(result, "add Buy milk");
    }

    #[test]
    fn test_read_event_with_custom_reader() {
        let input_data = b"add Buy milk\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "Buy milk"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_read_event_list_command() {
        let input_data = b"list completed\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::ListTasks(Some(filter)) if filter.status == Some(TaskStatus::Completed) => {},
            _ => panic!("Expected ListTasks(Completed) event"),
        }
    }

    #[test]
    fn test_read_event_remove_command() {
        let input_data = b"remove 5\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 5),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_read_event_complete_command() {
        let input_data = b"complete 3\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::CompleteTask(id) => assert_eq!(id, 3),
            _ => panic!("Expected CompleteTask event"),
        }
    }

    #[test]
    fn test_read_event_toggle_command() {
        let input_data = b"toggle 7\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::ToggleTask(id) => assert_eq!(id, 7),
            _ => panic!("Expected ToggleTask event"),
        }
    }

    #[test]
    fn test_read_event_help_command() {
        let input_data = b"help\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::ShowHelp => {},
            _ => panic!("Expected ShowHelp event"),
        }
    }

    #[test]
    fn test_read_event_quit_command() {
        let input_data = b"quit\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::Quit => {},
            _ => panic!("Expected Quit event"),
        }
    }

    #[test]
    fn test_read_event_unknown_command() {
        let input_data = b"unknown\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::UnknownCommand(cmd) => assert_eq!(cmd, "unknown"),
            _ => panic!("Expected UnknownCommand event"),
        }
    }

    #[test]
    fn test_read_event_invalid_input_empty() {
        let input_data = b"\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::InvalidInput(_) => {},
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_read_input_trims_whitespace() {
        let input_data = b"  add task  \n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let result = reader.read_input();
        assert_eq!(result, "add task");
    }

    #[test]
    fn test_read_input_handles_tabs() {
        let input_data = b"\tadd\ttask\t\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let result = reader.read_input();
        assert_eq!(result, "add\ttask");
    }

    #[test]
    fn test_read_event_multiword_task() {
        let input_data = b"add This is a complex task with many words\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::AddTask(desc) => assert_eq!(desc, "This is a complex task with many words"),
            _ => panic!("Expected AddTask event"),
        }
    }

    #[test]
    fn test_read_event_case_insensitive() {
        let input_data = b"LIST\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::ListTasks(_) => {},
            _ => panic!("Expected ListTasks event"),
        }
    }

    #[test]
    fn test_read_event_with_alias() {
        let input_data = b"rm 10\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::RemoveTask(id) => assert_eq!(id, 10),
            _ => panic!("Expected RemoveTask event"),
        }
    }

    #[test]
    fn test_parse_search_command_valid() {
        let input_data = b"search meeting\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::SearchTasks(keyword) => assert_eq!(keyword, "meeting"),
            _ => panic!("Expected SearchTasks event"),
        }
    }

    #[test]
    fn test_parse_search_command_multiple_words() {
        let input_data = b"search buy groceries\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::SearchTasks(keyword) => assert_eq!(keyword, "buy groceries"),
            _ => panic!("Expected SearchTasks event"),
        }
    }

    #[test]
    fn test_parse_search_command_find_alias() {
        let input_data = b"find important\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::SearchTasks(keyword) => assert_eq!(keyword, "important"),
            _ => panic!("Expected SearchTasks event"),
        }
    }

    #[test]
    fn test_parse_search_command_empty() {
        let input_data = b"search\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("Usage: search")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_search_command_whitespace_only() {
        let input_data = b"search   \n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::InvalidInput(msg) => assert!(msg.contains("Usage: search")),
            _ => panic!("Expected InvalidInput event"),
        }
    }

    #[test]
    fn test_parse_search_command_case_insensitive() {
        let input_data = b"SEARCH test\n";
        let mut reader = InputReader::with_reader(&input_data[..]);
        let event = reader.read_event();
        match event {
            UiEvent::SearchTasks(keyword) => assert_eq!(keyword, "test"),
            _ => panic!("Expected SearchTasks event"),
        }
    }
}

