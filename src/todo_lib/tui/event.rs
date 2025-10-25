use std::io;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use super::app::{App, InputMode};
use super::ui;

/// Run the TUI application
pub fn run_tui(mut app: App) -> io::Result<App> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    res?;
    Ok(app)
}

/// Main app loop
fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        // Draw the UI
        terminal.draw(|f| ui::render(f, app))?;

        // Handle input
        if let Event::Key(key) = event::read()? {
            // Only process key press events (not release)
            if key.kind != KeyEventKind::Press {
                continue;
            }

            match app.input_mode {
                InputMode::Normal => handle_normal_mode(app, key.code),
                InputMode::Adding | InputMode::Editing | InputMode::Searching => {
                    handle_input_mode(app, key.code)
                }
                InputMode::Help => {
                    // Any key exits help
                    app.input_mode = InputMode::Normal;
                }
            }
        }

        // Check if we should quit
        if app.should_quit {
            break;
        }
    }

    Ok(())
}

/// Handle keyboard input in normal mode
fn handle_normal_mode(app: &mut App, key: KeyCode) {
    match key {
        // Navigation
        KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
        KeyCode::Down | KeyCode::Char('j') => app.select_next(),

        // Actions
        KeyCode::Enter | KeyCode::Char(' ') => app.toggle_selected_task(),
        KeyCode::Char('a') => app.start_adding(),
        KeyCode::Char('e') => app.start_editing(),
        KeyCode::Char('d') => app.delete_selected_task(),
        KeyCode::Char('/') => app.start_searching(),
        KeyCode::Char('?') => app.show_help(),
        KeyCode::Char('f') => app.cycle_filter(),
        KeyCode::Char('c') => app.clear_search(),
        KeyCode::Char('q') | KeyCode::Esc => app.quit(),

        _ => {}
    }
}

/// Handle keyboard input in input modes (adding, editing, searching)
fn handle_input_mode(app: &mut App, key: KeyCode) {
    match key {
        KeyCode::Enter => app.submit_input(),
        KeyCode::Esc => app.cancel_input(),
        KeyCode::Char(c) => app.push_char(c),
        KeyCode::Backspace => app.pop_char(),
        _ => {}
    }
}
