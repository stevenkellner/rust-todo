use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::models::Task;
use super::app::{App, InputMode};

/// Render the main UI
pub fn render(f: &mut Frame, app: &App) {
    // Create the main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),     // Title
            Constraint::Min(10),       // Main content
            Constraint::Length(3),     // Status bar
        ])
        .split(f.area());

    // Render title
    render_title(f, chunks[0], app);

    // Render main content based on mode
    match app.input_mode {
        InputMode::Help => render_help(f, chunks[1]),
        _ => render_main_content(f, chunks[1], app),
    }

    // Render status bar
    render_status_bar(f, chunks[2], app);

    // Render modals on top if in input mode
    if matches!(
        app.input_mode,
        InputMode::Adding | InputMode::Editing | InputMode::Searching
    ) {
        render_input_modal(f, f.area(), app);
    }
}

/// Render the title bar
fn render_title(f: &mut Frame, area: Rect, app: &App) {
    let title_text = format!(
        "📝 TODO List Manager - {} mode",
        match app.input_mode {
            InputMode::Normal => "Navigation",
            InputMode::Adding => "Adding Task",
            InputMode::Editing => "Editing Task",
            InputMode::Searching => "Searching",
            InputMode::Help => "Help",
        }
    );

    let title = Paragraph::new(title_text)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));

    f.render_widget(title, area);
}

/// Render the main content area
fn render_main_content(f: &mut Frame, area: Rect, app: &App) {
    // Split into task list and details
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area);

    render_task_list(f, chunks[0], app);
    render_task_details(f, chunks[1], app);
}

/// Render the task list
fn render_task_list(f: &mut Frame, area: Rect, app: &App) {
    let tasks = app.get_displayed_tasks();

    let items: Vec<ListItem> = tasks
        .iter()
        .enumerate()
        .map(|(i, task)| {
            let is_selected = i == app.selected;
            let content = format_task(task, is_selected);
            let style = if is_selected {
                Style::default().bg(Color::DarkGray).add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            ListItem::new(content).style(style)
        })
        .collect();

    let title = if app.search_query.is_empty() {
        format!(" Tasks ({}) - Filter: {} ", tasks.len(), app.filter)
    } else {
        format!(
            " Tasks ({}) - Filter: {} | Search: '{}' ",
            tasks.len(),
            app.filter,
            app.search_query
        )
    };

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::White)),
        )
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        );

    f.render_widget(list, area);
}

/// Format a task for display
fn format_task(task: &Task, _is_selected: bool) -> Text<'_> {
    let mut spans = vec![];

    // Status checkbox
    let status_char = if task.is_completed() { "✓" } else { " " };
    spans.push(Span::styled(
        format!("[{}] ", status_char),
        Style::default().fg(if task.is_completed() {
            Color::Green
        } else {
            Color::White
        }),
    ));

    // Priority indicator
    let (priority_symbol, priority_color) = match task.get_priority() {
        crate::models::Priority::High => ("▲", Color::Red),
        crate::models::Priority::Medium => ("■", Color::Yellow),
        crate::models::Priority::Low => ("▼", Color::Blue),
    };
    spans.push(Span::styled(
        format!("{} ", priority_symbol),
        Style::default().fg(priority_color),
    ));

    // Task ID and description
    let description_style = if task.is_completed() {
        Style::default()
            .fg(Color::Gray)
            .add_modifier(Modifier::CROSSED_OUT)
    } else {
        Style::default().fg(Color::White)
    };
    spans.push(Span::styled(
        format!("{}. {}", task.id, task.description),
        description_style,
    ));

    // Due date
    if let Some(due_date) = task.get_due_date() {
        let today = chrono::Local::now().date_naive();
        let is_overdue = task.is_overdue(today);
        let due_color = if is_overdue {
            Color::Red
        } else if due_date == today {
            Color::Yellow
        } else {
            Color::Cyan
        };
        spans.push(Span::styled(
            format!(" (due: {})", due_date.format("%Y-%m-%d")),
            Style::default().fg(due_color),
        ));
    }

    // Category
    if let Some(category) = task.get_category() {
        spans.push(Span::styled(
            format!(" [{}]", category),
            Style::default().fg(Color::Magenta),
        ));
    }

    // Recurring
    if let Some(recurrence) = task.get_recurrence() {
        spans.push(Span::styled(
            format!(" 🔄 {:?}", recurrence),
            Style::default().fg(Color::Cyan),
        ));
    }

    // Dependencies
    if task.has_dependencies() {
        let deps: Vec<String> = task.get_dependencies().iter().map(|id| id.to_string()).collect();
        spans.push(Span::styled(
            format!(" 🔒 depends on: {}", deps.join(", ")),
            Style::default().fg(Color::Yellow),
        ));
    }

    Text::from(Line::from(spans))
}

/// Render task details panel
fn render_task_details(f: &mut Frame, area: Rect, app: &App) {
    let content = if let Some(task) = app.get_selected_task() {
        format_task_details(task, &app.todo_list)
    } else {
        vec![Line::from("No task selected")]
    };

    let stats = app.todo_list.get_statistics();
    let stats_text = format!(
        "\n\n═══ Statistics ═══\n\
        Total: {}\n\
        Completed: {}\n\
        Pending: {}\n\
        Progress: {:.1}%\n\
        \n\
        High Priority: {}\n\
        Medium Priority: {}\n\
        Low Priority: {}",
        stats.total,
        stats.completed,
        stats.pending,
        stats.completion_percentage,
        stats.high_priority,
        stats.medium_priority,
        stats.low_priority
    );

    let mut all_content = content;
    all_content.push(Line::from(""));
    all_content.extend(stats_text.lines().map(|line| Line::from(line.to_string())));

    let details = Paragraph::new(all_content)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Details & Statistics ")
                .border_style(Style::default().fg(Color::White)),
        )
        .wrap(Wrap { trim: true });

    f.render_widget(details, area);
}

/// Format detailed task information
fn format_task_details(task: &Task, todo_list: &crate::models::TodoList) -> Vec<Line<'static>> {
    let mut lines = vec![];

    lines.push(Line::from(Span::styled(
        "═══ Task Details ═══",
        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )));

    lines.push(Line::from(""));
    lines.push(Line::from(format!("ID: {}", task.id)));
    lines.push(Line::from(format!("Description: {}", task.description)));

    let status = if task.is_completed() {
        "Completed ✓"
    } else {
        "Pending"
    };
    lines.push(Line::from(format!("Status: {}", status)));

    lines.push(Line::from(format!("Priority: {:?}", task.get_priority())));

    if let Some(due_date) = task.get_due_date() {
        lines.push(Line::from(format!("Due Date: {}", due_date)));
        let today = chrono::Local::now().date_naive();
        if task.is_overdue(today) {
            lines.push(Line::from(Span::styled(
                "⚠ OVERDUE",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )));
        }
    }

    if let Some(category) = task.get_category() {
        lines.push(Line::from(format!("Category: {}", category)));
    }

    if let Some(recurrence) = task.get_recurrence() {
        lines.push(Line::from(format!("Recurrence: {:?}", recurrence)));
    }

    if task.has_dependencies() {
        let deps: Vec<String> = task.get_dependencies().iter().map(|id| id.to_string()).collect();
        lines.push(Line::from(format!("Dependencies: {}", deps.join(", "))));
    }

    if let Some(parent_id) = task.get_parent_id() {
        lines.push(Line::from(format!("Parent Task: {}", parent_id)));
    }

    let subtasks = todo_list.get_subtasks(task.id);
    if !subtasks.is_empty() {
        let subtasks_str: Vec<String> = subtasks.iter().map(|t| t.id.to_string()).collect();
        lines.push(Line::from(format!("Subtasks: {}", subtasks_str.join(", "))));
    }

    lines
}

/// Render the status bar
fn render_status_bar(f: &mut Frame, area: Rect, app: &App) {
    let status_text = if let Some(ref msg) = app.status_message {
        msg.clone()
    } else {
        "Press '?' for help, 'q' to quit".to_string()
    };

    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::White)));

    f.render_widget(status, area);
}

/// Render input modal
fn render_input_modal(f: &mut Frame, area: Rect, app: &App) {
    let popup_area = centered_rect(60, 20, area);

    let title = match app.input_mode {
        InputMode::Adding => " Add New Task ",
        InputMode::Editing => " Edit Task ",
        InputMode::Searching => " Search Tasks ",
        _ => " Input ",
    };

    let input_text = app.input.clone();
    let input = Paragraph::new(input_text)
        .style(Style::default().fg(Color::Yellow))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(title)
                .border_style(Style::default().fg(Color::Cyan)),
        );

    f.render_widget(Clear, popup_area);
    f.render_widget(input, popup_area);
}

/// Render help screen
fn render_help(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "═══ Keyboard Shortcuts ═══",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from("Navigation:"),
        Line::from("  ↑/k          - Move up"),
        Line::from("  ↓/j          - Move down"),
        Line::from(""),
        Line::from("Actions:"),
        Line::from("  Enter/Space  - Toggle task completion"),
        Line::from("  a            - Add new task"),
        Line::from("  e            - Edit selected task"),
        Line::from("  d            - Delete selected task"),
        Line::from("  /            - Search tasks"),
        Line::from("  f            - Cycle filter (all/pending/completed)"),
        Line::from("  c            - Clear search"),
        Line::from(""),
        Line::from("Other:"),
        Line::from("  ?            - Show this help"),
        Line::from("  q/Esc        - Quit / Cancel"),
        Line::from(""),
        Line::from(Span::styled(
            "Press any key to return",
            Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC),
        )),
    ];

    let help = Paragraph::new(help_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .alignment(Alignment::Left);

    f.render_widget(help, area);
}

/// Helper function to create a centered rectangle
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
