use std::str::FromStr;

use crate::controller::task_command::{TaskCommand, TaskSelection};
use crate::models::filter_builder::FilterBuilder;
use crate::models::parse_error::ParseError;
use crate::models::parse_ids;
use crate::models::priority::Priority;
use chrono::NaiveDate;

/// Parser for task-related commands.
pub struct TaskCommandInputParser;

impl TaskCommandInputParser {
    /// Creates a new task command parser.
    pub fn new() -> Self {
        TaskCommandInputParser
    }

    /// Tries to parse a task command from the given command string and arguments.
    ///
    /// # Arguments
    ///
    /// * `command` - The command string (already lowercased)
    /// * `args` - The command arguments
    ///
    /// # Returns
    ///
    /// * `Some(Ok(TaskCommand))` - Successfully parsed task command
    /// * `Some(Err(ParseError))` - Recognized as task command but has errors
    /// * `None` - Not a task command
    pub fn try_parse(
        &self,
        command: &str,
        args: &[&str],
    ) -> Option<Result<TaskCommand, ParseError>> {
        match command {
            "add" => Some(self.parse_add_command(args)),
            "add-subtask" | "subtask" => Some(self.parse_add_subtask_command(args)),
            "list" => Some(self.parse_list_command(args)),
            "remove" | "delete" | "rm" => Some(self.parse_remove_command(args)),
            "complete" | "done" => Some(self.parse_complete_command(args)),
            "uncomplete" | "undo" => Some(self.parse_uncomplete_command(args)),
            "toggle" => Some(self.parse_toggle_command(args)),
            "priority" | "pri" => Some(self.parse_priority_command(args)),
            "set-due" | "due" => Some(self.parse_set_due_command(args)),
            "set-category" | "category" | "cat" => Some(self.parse_set_category_command(args)),
            "set-recurring" | "recurring" | "recur" => Some(self.parse_set_recurring_command(args)),
            "add-dependency" | "add-dep" | "depends-on" => {
                Some(self.parse_add_dependency_command(args))
            }
            "remove-dependency" | "remove-dep" | "rm-dep" => {
                Some(self.parse_remove_dependency_command(args))
            }
            "show-dependencies" | "dependencies" | "deps" | "dep-graph" | "dependency-graph" => {
                Some(self.parse_show_dependency_graph_command(args))
            }
            "categories" | "list-categories" => Some(Ok(TaskCommand::ListCategories)),
            "edit" => Some(self.parse_edit_command(args)),
            "search" | "find" => Some(self.parse_search_command(args)),
            "statistics" | "stats" => Some(Ok(TaskCommand::ShowStatistics)),
            _ => None,
        }
    }

    /// Parses the 'add' command and validates the task description.
    fn parse_add_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments {
                command: "add".to_string(),
                usage: "add <task description>".to_string(),
            })
        } else {
            let description = args.join(" ");
            if description.is_empty() {
                Err(ParseError::EmptyInput("Task description".to_string()))
            } else {
                Ok(TaskCommand::Add(description))
            }
        }
    }

    /// Parses the 'add-subtask' command and validates parent ID and description.
    fn parse_add_subtask_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            return Err(ParseError::MissingArguments {
                command: "add-subtask".to_string(),
                usage: "add-subtask <parent_id> <subtask description>".to_string(),
            });
        }

        if args.len() < 2 {
            return Err(ParseError::MissingArguments {
                command: "add-subtask".to_string(),
                usage: "add-subtask <parent_id> <subtask description>".to_string(),
            });
        }

        let parent_id = args[0]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "parent_id".to_string(),
                expected: "positive integer".to_string(),
                actual: args[0].to_string(),
            })?;

        let description = args[1..].join(" ");
        if description.is_empty() {
            return Err(ParseError::EmptyInput("Subtask description".to_string()));
        }

        Ok(TaskCommand::AddSubtask(parent_id, description))
    }

    /// Parses the 'list' command with optional filter arguments.
    fn parse_list_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let mut filter_builder = FilterBuilder::new();

        for arg in args {
            let lower = arg.to_lowercase();
            filter_builder = match filter_builder.parse_argument(&lower) {
                Ok(builder) => builder,
                Err(err) => return Err(ParseError::InvalidFormat {
                    field: "filter".to_string(),
                    expected: "status (completed/pending/overdue), priority (high/medium/low), or category:name".to_string(),
                    actual: err
                }),
            };
        }

        Ok(TaskCommand::List(filter_builder.build()))
    }

    /// Parses the 'remove' command with task ID validation.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_remove_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let selection = self.parse_task_selection(args, "remove")?;
        Ok(TaskCommand::Remove(selection))
    }

    /// Parses the 'complete' command.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_complete_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let selection = self.parse_task_selection(args, "complete")?;
        Ok(TaskCommand::Complete(selection))
    }

    /// Parses the 'uncomplete' command.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_uncomplete_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let selection = self.parse_task_selection(args, "uncomplete")?;
        Ok(TaskCommand::Uncomplete(selection))
    }

    /// Parses the 'toggle' command.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_toggle_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        let selection = self.parse_task_selection(args, "toggle")?;
        Ok(TaskCommand::Toggle(selection))
    }

    /// Helper method to parse task selection (single, multiple, or all)
    fn parse_task_selection(
        &self,
        args: &[&str],
        command_name: &str,
    ) -> Result<TaskSelection, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments {
                command: command_name.to_string(),
                usage: format!("{} <task id|range|all>", command_name),
            })
        } else if args[0].eq_ignore_ascii_case("all") {
            Ok(TaskSelection::All)
        } else if args[0].contains('-') || args[0].contains(',') {
            // ID range or list format
            match parse_ids(args[0]) {
                Ok(ids) => Ok(TaskSelection::Multiple(ids)),
                Err(err) => Err(ParseError::InvalidId(err)),
            }
        } else if let Ok(id) = args[0].parse::<usize>() {
            Ok(TaskSelection::Single(id))
        } else {
            Err(ParseError::InvalidId("Invalid task ID. Please provide a number, range (e.g., 1-5), list (e.g., 1,3,5), or 'all'.".to_string()))
        }
    }

    /// Parses the 'priority' command with priority level validation.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_priority_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments {
                command: "priority".to_string(),
                usage:
                    "priority <task id|range|all> <priority level (high/h, medium/med/m, low/l)>"
                        .to_string(),
            })
        } else {
            let priority_str = args[1].to_lowercase();
            let priority = match Priority::from_str(&priority_str) {
                Some(p) => p,
                None => {
                    return Err(ParseError::InvalidValue {
                        field: "priority level".to_string(),
                        value: priority_str,
                        allowed: "high/h, medium/med/m, or low/l".to_string(),
                    })
                }
            };

            let selection = self.parse_task_selection(&args[0..1], "priority")?;
            Ok(TaskCommand::SetPriority(selection, priority))
        }
    }

    /// Parses the 'set-due' command with date validation.
    fn parse_set_due_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments {
                command: "set-due".to_string(),
                usage: "set-due <task id> <date (DD.MM.YYYY) or 'none' to clear>".to_string(),
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let date_str = args[1];

            if date_str.to_lowercase() == "none" {
                return Ok(TaskCommand::SetDueDate(id, None));
            }

            let parts: Vec<&str> = date_str.split('.').collect();
            if parts.len() != 3 {
                return Err(ParseError::InvalidFormat {
                    field: "date".to_string(),
                    expected: "DD.MM.YYYY (e.g., 31.12.2024)".to_string(),
                    actual: date_str.to_string(),
                });
            }

            if let (Ok(day), Ok(month), Ok(year)) = (
                parts[0].parse::<u32>(),
                parts[1].parse::<u32>(),
                parts[2].parse::<i32>(),
            ) {
                match NaiveDate::from_ymd_opt(year, month, day) {
                    Some(date) => Ok(TaskCommand::SetDueDate(id, Some(date))),
                    None => Err(ParseError::InvalidDate(
                        "Invalid date. Please check the date is valid.".to_string(),
                    )),
                }
            } else {
                Err(ParseError::InvalidFormat {
                    field: "date values".to_string(),
                    expected: "DD.MM.YYYY".to_string(),
                    actual: date_str.to_string(),
                })
            }
        } else {
            Err(ParseError::InvalidId(
                "Invalid task ID. Please provide a number.".to_string(),
            ))
        }
    }

    /// Parses the 'set-category' command.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_set_category_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments {
                command: "set-category".to_string(),
                usage: "set-category <task id|range|all> <category name or 'none' to clear>"
                    .to_string(),
            })
        } else {
            let category_str = args[1..].join(" ");
            let category = if category_str.to_lowercase() == "none" {
                None
            } else if category_str.is_empty() {
                return Err(ParseError::EmptyInput("Category name".to_string()));
            } else {
                Some(category_str)
            };

            let selection = self.parse_task_selection(&args[0..1], "set-category")?;
            Ok(TaskCommand::SetCategory(selection, category))
        }
    }

    /// Parses the 'set-recurring' command.
    /// Supports single ID, ID ranges (1-5), lists (1,3,5), combined (1-3,7), or "all"
    fn parse_set_recurring_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        use crate::models::recurrence::Recurrence;

        if args.len() < 2 {
            Err(ParseError::MissingArguments {
                command: "set-recurring".to_string(),
                usage: "set-recurring <task id|range|all> <daily|weekly|monthly|none>".to_string(),
            })
        } else {
            let recurrence_str = args[1].to_lowercase();
            let recurrence = if recurrence_str == "none" {
                None
            } else {
                match Recurrence::from_str(&recurrence_str) {
                    Ok(r) => Some(r),
                    Err(()) => {
                        return Err(ParseError::InvalidFormat {
                            field: "recurrence".to_string(),
                            expected: "daily, weekly, monthly, or none".to_string(),
                            actual: args[1].to_string(),
                        })
                    }
                }
            };

            let selection = self.parse_task_selection(&args[0..1], "set-recurring")?;
            Ok(TaskCommand::SetRecurring(selection, recurrence))
        }
    }

    /// Parses the 'add-dependency' command.
    fn parse_add_dependency_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            return Err(ParseError::MissingArguments {
                command: "add-dependency".to_string(),
                usage: "add-dependency <task_id> <depends_on_id>".to_string(),
            });
        }

        let task_id = args[0]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "task ID".to_string(),
                expected: "positive integer".to_string(),
                actual: args[0].to_string(),
            })?;

        let depends_on_id = args[1]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "dependency ID".to_string(),
                expected: "positive integer".to_string(),
                actual: args[1].to_string(),
            })?;

        Ok(TaskCommand::AddDependency(task_id, depends_on_id))
    }

    /// Parses the 'remove-dependency' command.
    fn parse_remove_dependency_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            return Err(ParseError::MissingArguments {
                command: "remove-dependency".to_string(),
                usage: "remove-dependency <task_id> <depends_on_id>".to_string(),
            });
        }

        let task_id = args[0]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "task ID".to_string(),
                expected: "positive integer".to_string(),
                actual: args[0].to_string(),
            })?;

        let depends_on_id = args[1]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "dependency ID".to_string(),
                expected: "positive integer".to_string(),
                actual: args[1].to_string(),
            })?;

        Ok(TaskCommand::RemoveDependency(task_id, depends_on_id))
    }

    /// Parses the 'show-dependencies' command.
    fn parse_show_dependency_graph_command(
        &self,
        args: &[&str],
    ) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            return Err(ParseError::MissingArguments {
                command: "show-dependencies".to_string(),
                usage: "show-dependencies <task_id>".to_string(),
            });
        }

        let task_id = args[0]
            .parse::<usize>()
            .map_err(|_| ParseError::InvalidFormat {
                field: "task ID".to_string(),
                expected: "positive integer".to_string(),
                actual: args[0].to_string(),
            })?;

        Ok(TaskCommand::ShowDependencyGraph(task_id))
    }

    /// Parses the 'search' command.
    fn parse_search_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.is_empty() {
            Err(ParseError::MissingArguments {
                command: "search".to_string(),
                usage: "search <keyword>".to_string(),
            })
        } else {
            let keyword = args.join(" ");
            if keyword.is_empty() {
                Err(ParseError::EmptyInput("Search keyword".to_string()))
            } else {
                Ok(TaskCommand::Search(keyword))
            }
        }
    }

    /// Parses the 'edit' command.
    fn parse_edit_command(&self, args: &[&str]) -> Result<TaskCommand, ParseError> {
        if args.len() < 2 {
            Err(ParseError::MissingArguments {
                command: "edit".to_string(),
                usage: "edit <task id> <new description>".to_string(),
            })
        } else if let Ok(id) = args[0].parse::<usize>() {
            let description = args[1..].join(" ");
            if description.is_empty() {
                Err(ParseError::EmptyInput("Task description".to_string()))
            } else {
                Ok(TaskCommand::Edit(id, description))
            }
        } else {
            Err(ParseError::InvalidId(
                "Invalid task ID. Please provide a number.".to_string(),
            ))
        }
    }
}

impl Default for TaskCommandInputParser {
    fn default() -> Self {
        Self::new()
    }
}
