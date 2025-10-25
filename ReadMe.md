# Todo List Manager

A command-line todo list application built in Rust with a clean, layered architecture and comprehensive test coverage.

## Features

- ✅ Add tasks with descriptive names
- 🔍 List all tasks, or filter by completion status, priority, and category
- ✔️ Mark tasks as complete or incomplete
- 🔄 Toggle task completion status
- 🎯 Set task priorities (Low, Medium, High) with colored indicators
- 🔖 Assign categories/tags to tasks for better organization
- 📅 Set due dates for tasks with visual indicators for overdue items
- 📝 Edit task descriptions
- 📊 View task statistics (completion rate, priority breakdown, category breakdown)
- 🔍 Search tasks by keyword
- 🗑️ Remove tasks by ID
- 🎨 Intuitive command-line interface with colored output
- 📋 Combined filtering (e.g., "list pending high category:work")
- 🐛 Debug mode for testing and development
- 🏗️ Event-driven architecture with clean separation of concerns
- ✅ Comprehensive test suite with 211 tests (150 unit + 8 integration + 6 filtering + 47 doc tests)
- 📚 Full documentation with `cargo doc`

## Quick Start

```bash
# Build and run
cargo run

# Run tests
cargo test

# Build optimized release version
cargo build --release

# Generate documentation
cargo doc --open
```

## Project Structure

For a detailed explanation of the project organization, see **[STRUCTURE.md](STRUCTURE.md)**.

### Quick Overview

```text
src/todo_lib/
├── models/            # Domain layer (business logic, data structures, events)
├── ui/                # User interface layer (input/output)
│   └── formatters/    # Formatting modules (task and message formatters)
└── controller/        # Controller layer (orchestration)
```

The application follows a **layered architecture** with three distinct layers:

- **Models**: All business logic, domain types, and events
- **UI**: Terminal I/O, command parsing, and output formatting  
- **Controller**: Orchestrates interactions between UI and models

> For detailed architecture documentation, see **[STRUCTURE.md](STRUCTURE.md)**

## Installation and Running

### Prerequisites

- Rust 1.70+ installed on your system

### Building and Running

```bash
# Build the project
cargo build

# Run the application
cargo run

# Run tests
cargo test

# Run tests with verbose output
cargo test -- --nocapture

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test integration_tests
```

## Usage

When you run the application, you'll see a command prompt where you can enter various commands:

### Available Commands

| Command | Description | Example |
|---------|-------------|---------|
| `add <description>` | Add a new task | `add Buy groceries` |
| `list [status] [priority]` | List tasks with optional filters | `list`, `list pending high` |
| `remove <id>` | Remove a task by ID | `remove 1` |
| `complete <id>` | Mark task as completed | `complete 1` |
| `uncomplete <id>` | Mark task as pending | `uncomplete 1` |
| `toggle <id>` | Toggle task completion status | `toggle 1` |
| `priority <id> <level>` | Set task priority (high/medium/low) | `priority 1 high` |
| `set-category <id> <name>` | Assign category to task | `set-category 1 work` |
| `categories` | List all categories | `categories` |
| `set-due <id> <date>` | Set task due date (YYYY-MM-DD) | `set-due 1 2024-12-31` |
| `edit <id> <description>` | Edit task description | `edit 1 New description` |
| `search <keyword>` | Search tasks by keyword | `search groceries` |
| `statistics` | Show task statistics | `statistics` |
| `help` | Show help message | `help` |
| `quit` | Exit the program | `quit` |

### Debug Mode Commands

Debug mode provides additional commands for testing and development:

| Command | Description | Example |
|---------|-------------|---------|
| `debug` | Toggle debug mode on/off | `debug` |
| `debug:gen <count>` | Generate N random tasks | `debug:gen 10` |
| `debug:clear` | Clear all tasks | `debug:clear` |

**Note:** Debug commands are only available after enabling debug mode with the `debug` command.

### Example Session

```text
> add Buy groceries
✓ Task added with ID 1: 'Buy groceries'

> add Read a book
✓ Task added with ID 2: 'Read a book'

> add Write report
✓ Task added with ID 3: 'Write report'

> priority 1 high
✓ Priority set to ▲ High for task: 'Buy groceries'

> category 2 personal
✓ Category for task 'Read a book' set to: personal

> set-category 3 work
✓ Category for task 'Write report' set to: work

> list

--- All Tasks ---
─────────────────────────────────────────────────────

1. [ ] ▲ Buy groceries
2. [ ] ■ Read a book [personal]
3. [ ] ■ Write report [work]

─────────────────────────────────────────────────────

> list category:work

--- All Tasks ---
─────────────────────────────────────────────────────

3. [ ] ■ Write report [work]

─────────────────────────────────────────────────────

> complete 1
✓ Task 'Buy groceries' marked as completed.

> search book

--- Search Results for 'book' ---
─────────────────────────────────────────────────────

2. [ ] ■ Read a book [personal]

─────────────────────────────────────────────────────

> quit

─────────────────────────────────────────────────────

    ✨ Thank you for using To-Do List Manager! ✨    

           Stay organized and productive! 🚀          

═════════════════════════════════════════════════════
```

## Testing

The project has a comprehensive test suite with **211 total tests** (150 unit tests + 8 integration tests + 6 filtering tests + 47 doc tests):

### Run All Tests

```bash
cargo test
```

### Unit Tests

Each module includes extensive unit tests:

- `Task`: Creation, completion toggling, status display, priority management
- `TodoList`: CRUD operations, edge cases, ID management, filtering, search
- `TaskFilter`: Filter building, combined filters
- `InputReader`: Command parsing validation, aliases
- `OutputWriter`: Output formatting, message display
- `TaskFormatter`: Task display formatting, ID alignment
- `MessageFormatter`: Message and UI element formatting
- `TodoController`: Event handling, component coordination
- `DebugController`: Debug mode, random task generation, bulk operations

### Integration Tests

The `tests/integration_tests.rs` file contains:

- End-to-end workflow tests
- Command parsing and execution
- Multi-step task management scenarios
- Error handling verification
- Large-scale operations (100+ tasks)

### Filtering Tests

The `tests/filtering_tests.rs` file contains:

- Status filtering (completed/pending)
- Priority filtering (high/medium/low)
- Combined filtering scenarios
- Dynamic filter updates
- Empty filter results

### Documentation Tests

Documentation examples are also tested:

```bash
cargo test --doc
```

## Documentation

Generate and view full API documentation:

```bash
cargo doc --open
```

## Debug Mode

The application includes a **debug mode** for testing and development purposes. Debug mode is disabled by default and can be toggled on/off during runtime.

### Enabling Debug Mode

```bash
> debug
✓ Debug mode enabled
Debug commands available:
  - debug:gen <count>  : Generate N random tasks
  - debug:clear        : Clear all tasks
  - debug               : Toggle debug mode
```

### Debug Commands

- **`debug:gen <count>`** - Generates N random tasks with:
  - Random descriptions from a predefined template list
  - Random priorities (High, Medium, Low)
  - ~30% randomly marked as completed
  - Useful for testing filters, sorting, and UI with realistic data

- **`debug:clear`** - Removes all tasks from the list
  - Quick way to reset the application state
  - Shows count of cleared tasks

- **`debug`** - Toggles debug mode on/off
  - When disabled, debug commands are not available
  - Prevents accidental use of debug features in normal operation

### Example Debug Session

```bash
> debug
✓ Debug mode enabled

> debug:gen 5
✓ Generated 5 random tasks

> list
--- All Tasks ---
1. [✓] ▲ Write documentation
2. [ ] ■ Fix bug in authentication
3. [ ] ▼ Update README
4. [✓] ▲ Deploy to production
5. [ ] ■ Code review session
-----------------

> debug:clear
✓ Cleared 5 tasks

> debug
✓ Debug mode disabled
```

### Implementation Details

Debug functionality is isolated in `DebugController` (`controller/debug_controller.rs`), keeping it separate from production code. This makes it easy to:

- Test the application with realistic data
- Verify UI behavior with many tasks
- Quickly reset state during development
- Remove debug features in production builds (future enhancement)

## Features to Implement

The following features are planned for future releases:

### Phase 1: Enhanced Task Structure

- [ ] Add `due_date` field to tasks with date parsing
- [ ] Add `category` field for task organization
- [ ] Add `created_at` timestamp
- [ ] Enhance display to show new fields
- [ ] Add setters/getters for new fields in TodoList

### Phase 2: Enhanced Commands

- [ ] **Edit command** - Modify task description
- [ ] **Stats command** - Show statistics (total tasks, completion rate, etc.)
- [ ] **Sorting** - Sort tasks by priority, date, ID, or creation time

### Phase 3: Persistence

- [ ] JSON save/load functionality using `serde`
- [ ] Auto-save on changes
- [ ] Configurable save location

### Phase 4: Advanced Features

- [ ] **Subtasks** - Parent-child task relationships
- [ ] **Recurring tasks** - Daily/weekly/monthly repeating tasks
- [ ] **Time tracking** - Track time spent on tasks
- [ ] **Dependencies** - Tasks that depend on other tasks
- [ ] **Multi-project** - Separate task lists/projects
- [ ] **Notifications** - Reminders for due dates
- [ ] **History** - Track task changes and completion history

### Phase 5: Interface Enhancements

- [ ] CLI arguments with `clap` crate
  - Run commands directly: `todo add "Task description"`
  - Batch operations without interactive mode
- [ ] Terminal UI with `ratatui` crate
  - Interactive mode with keyboard navigation
  - Visual task panels and views
  - Mouse support

### Contributing

Contributions are welcome! If you'd like to implement any of these features or suggest new ones, please:

1. Fork the repository
2. Create a feature branch
3. Add tests for your changes
4. Submit a pull request

## Design Decisions

### Event-Driven Architecture

- Clean separation between UI input and controller logic
- `UiEvent` enum provides type-safe event handling
- Easy to extend with new commands/events

### Separation of Input and Output

- `InputReader` handles reading and parsing user commands
- `OutputWriter` handles formatting and displaying messages
- Complete independence between input and output concerns
- Each can be tested and modified independently

### Semantic Output Methods

- Instead of generic `print_line()`, OutputWriter has specific methods like `show_task_added()`, `show_task_removed()`
- Better separation of concerns (Controller doesn't know formatting details)
- Easier to maintain and modify output formatting

### DRY Principle

- Task listing methods share a common `show_task_list()` helper
- Eliminates duplication across `show_all_tasks()`, `show_completed_tasks()`, `show_pending_tasks()`

### LoopControl Enum

- More descriptive than boolean for main loop control
- `LoopControl::Continue` and `LoopControl::Exit` are self-documenting
- Easier to extend with additional control flow states if needed

### Controller Owns Components

- `TodoController` owns `InputReader`, `OutputWriter`, and `TodoList` as fields
- Eliminates repetitive parameter passing
- Clearer ownership model and lifecycle management

### Test-Driven Development

- 70 tests covering core functionality, edge cases, and integration scenarios
- Tests serve as living documentation
- High confidence in refactoring and changes

## Project License

This project is for educational purposes.
