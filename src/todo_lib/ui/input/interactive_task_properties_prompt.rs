use std::cell::RefCell;
use std::rc::Rc;

use crate::ui::input::InputStream;
#[cfg(test)]
use crate::ui::output::FileOutputWriter;
use crate::ui::output::OutputWriter;
use crate::models::priority::Priority;
use colored::Colorize;
use chrono::NaiveDate;

/// Handles interactive prompts for user input.
///
/// `InteractiveTaskPropertiesPrompt` encapsulates the logic for prompting users
/// to enter additional task properties like priority, due date, and category.
pub struct InteractiveTaskPropertiesPrompt<I: InputStream, O: OutputWriter> {
    input_stream: Rc<RefCell<I>>,
    output_writer: Rc<RefCell<O>>,
}

impl<I: InputStream, O: OutputWriter> InteractiveTaskPropertiesPrompt<I, O> {
    /// Creates a new interactive prompt handler.
    ///
    /// # Arguments
    ///
    /// * `input` - The input reader for getting user input
    /// * `output` - The output writer for displaying prompts
    pub fn new(input_stream: Rc<RefCell<I>>, output_writer: Rc<RefCell<O>>) -> Self {
        InteractiveTaskPropertiesPrompt { input_stream, output_writer }
    }

    /// Prompts for all task properties (priority, due date, category).
    ///
    /// Returns a tuple of (Option<Priority>, Option<NaiveDate>, Option<String>)
    pub fn prompt_task_properties(&mut self) -> (Option<Priority>, Option<NaiveDate>, Option<String>) {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&"Set additional properties (press Enter to skip):".bright_cyan().bold().to_string());

        let priority = self.prompt_priority();
        let due_date = self.prompt_due_date();
        let category = self.prompt_category();
        
        self.output_writer.borrow_mut().write_line("");
        
        (priority, due_date, category)
    }

    /// Prompts the user for a task priority.
    ///
    /// Returns Some(Priority) if valid input was provided, None if skipped or invalid.
    pub fn prompt_priority(&mut self) -> Option<Priority> {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&format!("{} {}", 
            "Priority".bright_yellow().bold(), 
            "[high/medium/low]:".bright_black()
        ));
        
        let input = self.input_stream.borrow_mut().get_next_input();
        if input.is_empty() {
            return None;
        }

        match Priority::from_str(&input) {
            Some(priority) => {
                self.output_writer.borrow_mut().show_success(&format!("Priority set to {}", priority.as_str()));
                Some(priority)
            }
            None => {
                self.output_writer.borrow_mut().show_error("Invalid priority. Skipping.");
                None
            }
        }
    }

    /// Prompts the user for a task due date.
    ///
    /// Returns Some(NaiveDate) if valid date was provided, None if skipped or invalid.
    pub fn prompt_due_date(&mut self) -> Option<NaiveDate> {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&format!("{} {}", 
            "Due date".bright_yellow().bold(), 
            "[DD.MM.YYYY]:".bright_black()
        ));
        
        let input = self.input_stream.borrow_mut().get_next_input();
        if input.is_empty() {
            return None;
        }

        match NaiveDate::parse_from_str(&input, "%d.%m.%Y") {
            Ok(date) => {
                self.output_writer.borrow_mut().show_success(&format!("Due date set to {}", date.format("%d.%m.%Y")));
                Some(date)
            }
            Err(_) => {
                self.output_writer.borrow_mut().show_error("Invalid date format. Use DD.MM.YYYY");
                None
            }
        }
    }

    /// Prompts the user for a task category.
    ///
    /// Returns Some(String) if input was provided, None if skipped.
    pub fn prompt_category(&mut self) -> Option<String> {
        self.output_writer.borrow_mut().write_line("");
        self.output_writer.borrow_mut().write_line(&format!("{}:", "Category".bright_yellow().bold()));
        
        let input = self.input_stream.borrow_mut().get_next_input();
        if input.is_empty() {
            None
        } else {
            self.output_writer.borrow_mut().show_success(&format!("Category set to '{}'", input));
            Some(input)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ui::input::FileInputStream;

    #[test]
    fn test_prompt_priority_valid_high() {
        let input_data = b"high\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_priority();
        
        assert_eq!(result, Some(Priority::High));
    }

    #[test]
    fn test_prompt_priority_valid_medium() {
        let input_data = b"medium\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_priority();
        
        assert_eq!(result, Some(Priority::Medium));
    }

    #[test]
    fn test_prompt_priority_valid_low() {
        let input_data = b"low\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_priority();
        
        assert_eq!(result, Some(Priority::Low));
    }

    #[test]
    fn test_prompt_priority_empty() {
        let input_data = b"\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_priority();
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_prompt_priority_invalid() {
        let input_data = b"invalid\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_priority();
        
        assert_eq!(result, None);
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Invalid priority"));
    }

    #[test]
    fn test_prompt_due_date_valid() {
        let input_data = b"25.12.2025\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_due_date();
        
        assert!(result.is_some());
        let date = result.unwrap();
        assert_eq!(date.format("%d.%m.%Y").to_string(), "25.12.2025");
    }

    #[test]
    fn test_prompt_due_date_empty() {
        let input_data = b"\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_due_date();
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_prompt_due_date_invalid_format() {
        let input_data = b"2025-12-25\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_due_date();
        
        assert_eq!(result, None);
        let output = String::from_utf8(buffer).unwrap();
        assert!(output.contains("Invalid date format"));
    }

    #[test]
    fn test_prompt_category_valid() {
        let input_data = b"Work\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_category();
        
        assert_eq!(result, Some("Work".to_string()));
    }

    #[test]
    fn test_prompt_category_empty() {
        let input_data = b"\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let result = prompt.prompt_category();
        
        assert_eq!(result, None);
    }

    #[test]
    fn test_prompt_task_properties_all_filled() {
        let input_data = b"high\n25.12.2025\nWork\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let (priority, due_date, category) = prompt.prompt_task_properties();
        
        assert_eq!(priority, Some(Priority::High));
        assert!(due_date.is_some());
        assert_eq!(category, Some("Work".to_string()));
    }

    #[test]
    fn test_prompt_task_properties_all_skipped() {
        let input_data = b"\n\n\n";
        let input_reader = FileInputStream::new(&input_data[..]);
        let mut buffer = Vec::new();
        let output_writer = FileOutputWriter::new(&mut buffer);
        
        let mut prompt = InteractiveTaskPropertiesPrompt::new(Rc::new(RefCell::new(input_reader)), Rc::new(RefCell::new(output_writer)));
        let (priority, due_date, category) = prompt.prompt_task_properties();
        
        assert_eq!(priority, None);
        assert_eq!(due_date, None);
        assert_eq!(category, None);
    }
}
