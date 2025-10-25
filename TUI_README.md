# TUI Mode - Text User Interface

## Overview

The TODO List Manager now includes a full-screen terminal user interface (TUI) built with `ratatui`. The TUI provides an interactive, keyboard-driven experience for managing tasks.

## Launching TUI Mode

To launch the TUI, use the `--tui` flag:

```bash
cargo run -- --tui
```

## Features

### Multi-Pane Layout
- **Task List (Left, 60%)**: Displays all tasks with visual indicators
  - Priority indicators: ▲ (High/Red), ■ (Medium/Yellow), ▼ (Low/Blue)
  - Completion checkboxes: [✓] completed, [ ] pending
  - Due dates color-coded: Red (overdue), Yellow (today), Cyan (future)
  - Categories shown in magenta: [category]
  - Recurring tasks: 🔄 icon
  - Dependencies: 🔒 icon with task IDs

- **Details Panel (Right, 40%)**: Shows detailed information
  - Selected task details (ID, description, status, priority, due date, etc.)
  - Statistics (total, completed, pending, completion percentage)
  - Priority breakdown

### Keyboard Shortcuts

#### Navigation
- `↑` or `k` - Move selection up
- `↓` or `j` - Move selection down

#### Actions
- `Enter` or `Space` - Toggle task completion status
- `a` - Add a new task
- `e` - Edit selected task
- `d` - Delete selected task

#### Search & Filter
- `/` - Open search dialog
- `c` - Clear search
- `f` - Cycle filter (all → pending → completed → all)

#### Other
- `?` - Show help screen
- `q` or `Esc` - Quit TUI (or cancel current input)

### Input Modes

The TUI has several input modes:

1. **Normal Mode**: Navigate and perform actions
2. **Adding Mode**: Enter description for new task
3. **Editing Mode**: Modify existing task description
4. **Searching Mode**: Filter tasks by keyword
5. **Help Mode**: View all keyboard shortcuts

### Status Messages

The status bar at the bottom shows:
- Confirmation messages for actions (add, edit, delete, toggle)
- Help hint when idle
- Current mode information in the title bar

## Data Persistence

The TUI automatically:
- Loads tasks from `tasks.json` on startup
- Saves all changes when you quit

## Technical Details

- **Framework**: ratatui 0.28 + crossterm 0.28
- **Architecture**: Event-driven with clean separation of concerns
  - `app.rs` - Application state management
  - `ui.rs` - Rendering logic
  - `event.rs` - Keyboard event handling
- **Terminal Management**: Uses alternate screen (preserves history)

## Example Workflow

1. Launch TUI: `cargo run -- --tui`
2. Press `a` to add a task
3. Type description and press `Enter`
4. Navigate with arrow keys or `j/k`
5. Press `Enter` to toggle completion
6. Press `/` to search
7. Press `f` to filter by status
8. Press `?` to see all commands
9. Press `q` to quit and save

## Switching Between CLI and TUI

- **CLI Mode** (default): `cargo run`
  - Traditional command-line interface
  - Type commands like `add`, `list`, `complete`, etc.

- **TUI Mode**: `cargo run -- --tui`
  - Full-screen interactive interface
  - Keyboard shortcuts for all actions

Both modes share the same data file, so you can switch between them freely.
