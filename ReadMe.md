# Todo List Manager

[![CI](https://github.com/stevenkellner/rust-todo/actions/workflows/ci.yml/badge.svg)](https://github.com/stevenkellner/rust-todo/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/stevenkellner/rust-todo/branch/main/graph/badge.svg)](https://codecov.io/gh/stevenkellner/rust-todo)

A command-line todo list application built in Rust with a clean, layered architecture and comprehensive test coverage.

## Features

- ✅ Add tasks with descriptive names
- 🔍 List all tasks, or filter by completion status, priority, and category
- ✔️ Mark tasks as complete or incomplete
- 🔄 Toggle task completion status
- 📦 **Bulk operations** - Complete/remove multiple tasks using ID ranges (e.g., `1-5`), lists (e.g., `1,3,5`), or `all`
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

The application follows a **layered architecture** with three distinct layers:

### Directory Structure

```text
src/
├── main.rs                          # Application entry point
└── todo_lib/
    ├── lib.rs                       # Library root with module exports
    │
    ├── models/                      # Domain Models Layer
    │   ├── mod.rs                   # Models module definition
    │   ├── command_controller_result.rs # Result type for command execution
    │   ├── filter_builder.rs        # Builder pattern for task filters
    │   ├── id_parser.rs             # Parse ID ranges and lists
    │   ├── loop_control.rs          # Control flow enum (Continue, Exit)
    │   ├── overdue_filter.rs        # Filter for overdue tasks
    │   ├── parse_error.rs           # Error types for parsing
    │   ├── priority.rs              # Priority enum (Low, Medium, High)
    │   ├── task.rs                  # Task struct with priority and status
    │   ├── task_filter.rs           # Filter criteria for querying tasks
    │   ├── task_status.rs           # Task status enum (Pending, Completed)
    │   └── todo_list.rs             # TodoList collection and business logic
    │
    ├── ui/                          # User Interface Layer
    │   ├── mod.rs                   # UI module definition
    │   ├── input/                   # Input handling
    │   │   ├── file_input_stream.rs # Read from stdin or files
    │   │   └── interactive_task_properties_prompt.rs # Interactive prompts
    │   ├── output/                  # Output handling
    │   │   ├── file_output_writer.rs # Write to stdout or files
    │   │   └── output_manager.rs    # Manage output operations
    │   └── formatters/              # Formatting modules
    │       ├── mod.rs               # Formatters module definition
    │       ├── task_formatter.rs    # Task display formatting
    │       └── message_formatter.rs # Message and UI element formatting
    │
    ├── controller/                  # Controller Layer
    │   ├── mod.rs                   # Controller module definition
    │   ├── command_controller.rs    # Trait for command controllers
    │   ├── command_controller_registry.rs # Registry to route commands
    │   ├── todo_manager.rs          # Top-level application manager
    │   ├── debug_command/           # Debug-mode command controllers
    │   │   ├── debug_command_controller.rs
    │   │   ├── debug_command_output_manager.rs
    │   │   └── random_task_generator.rs
    │   ├── general_command/         # General command controllers
    │   │   ├── general_command.rs
    │   │   ├── general_command_controller.rs
    │   │   └── general_command_output_manager.rs
    │   └── task_command/            # Task-specific command controllers
    │       ├── task.rs
    │       ├── task_selection.rs
    │       ├── task_command_controller.rs
    │       ├── task_command_input_parser.rs
    │       └── task_command_output_manager.rs
    │
    └── persistence/                 # Data Persistence Layer
        └── todo_list_storage.rs     # Save/load TodoList to JSON

tests/
├── integration_tests.rs             # Integration tests for workflows
└── filtering_tests.rs               # Tests for combined filtering feature
```

### Architecture Layers

#### 1. Models Layer (`models/`)

The domain layer containing all business logic and data structures.

- **`task.rs`** - Core Task type with:
  - Unique ID, description, completion status
  - Priority level (Low/Medium/High)
  - Category/tag support
  - Due date with overdue detection
  - Methods for state management

- **`todo_list.rs`** - Collection managing tasks with:
  - Add, remove, complete, uncomplete operations
  - Bulk operations (complete/remove multiple tasks)
  - Filtering by status, priority, and/or category
  - Search functionality
  - Task queries and retrieval
  - Statistics generation

- **`priority.rs`** - Priority levels:
  - Low (▼), Medium (■), High (▲)
  - Color-coded visual indicators
  - String parsing and formatting

- **`task_status.rs`** - Completion states:
  - Pending - task not yet done
  - Completed - task finished

- **`task_filter.rs`** - Query builder for tasks:
  - Filter by status (completed/pending)
  - Filter by priority (high/medium/low)
  - Filter by category
  - Combined filters (e.g., "pending high priority work tasks")
  - Overdue task filtering

- **`id_parser.rs`** - Parse task IDs:
  - Single IDs: `1`
  - Ranges: `1-5`
  - Lists: `1,3,5`
  - Combined: `1-3,7,9-11`

- **`loop_control.rs`** - Control flow signals:
  - Continue - keep running the application loop
  - Exit - exit the application

#### 2. UI Layer (`ui/`)

Handles all user interaction and terminal I/O.

- **`input/file_input_stream.rs`** - Reads input:
  - Reads from stdin (or custom reader for testing)
  - Generic over any `BufRead` implementation

- **`output/file_output_writer.rs`** - Writes output:
  - Writes to stdout (or custom writer for testing)
  - Generic over any `Write` implementation

- **`output/output_manager.rs`** - Manages output:
  - Welcome messages
  - Error handling
  - Unknown command messages

- **`formatters/task_formatter.rs`** - Task formatting:
  - Task display with status symbols, priorities, descriptions
  - Category and due date display
  - Dynamic ID width calculation for alignment
  - Reusable formatting functions

- **`formatters/message_formatter.rs`** - Message formatting:
  - Success, error, and warning messages with icons
  - Section titles and separators
  - Help text formatting (commands, labels, sub-info)

#### 3. Controller Layer (`controller/`)

Coordinates between UI and model layers.

- **`application_controller.rs`** - Top-level application manager (`ApplicationController`):
  - Coordinates specialized command controllers
  - Processes commands by delegating to the registry
  - Manages the main application loop and I/O

- **`command_controller_registry.rs`** - Command routing:
  - Routes commands to appropriate controller
  - Manages controller lifecycle
  - Supports debug mode toggling

- **`task_command/`** - Task command controllers:
  - Add, remove, complete, uncomplete, toggle tasks
  - Set priority, category, due date
  - Edit task descriptions
  - List, search, and view statistics
  - Bulk operations support

- **`general_command/`** - General command controllers:
  - Help command
  - Quit command
  - Debug mode toggle

- **`debug_command/`** - Debug command controllers:
  - Generate random tasks
  - Clear all tasks
  - Isolated from production code

#### 4. Persistence Layer (`persistence/`)

Handles data storage and retrieval.

- **`todo_list_storage.rs`** - JSON persistence:
  - Save TodoList to JSON file
  - Load TodoList from JSON file
  - Auto-create directories if needed

### Design Principles

1. **Separation of Concerns**: Each layer has a specific responsibility
   - Models: Business logic and data
   - UI: User interaction
   - Controller: Coordination
   - Persistence: Data storage

2. **One Type Per File**: Each struct/enum has its own file for clarity

3. **Dependency Flow**:
   - Controller depends on UI, Models, and Persistence
   - UI depends on Models (for formatting)
   - Persistence depends on Models
   - Models are independent

4. **Testability**:
   - Each layer can be tested independently
   - Generic input/output for testing
   - Integration tests verify full workflows

### Import Paths

When using the library, import from the organized modules:

```rust
// Models (domain types, business logic)
use todo_manager::models::{Task, TodoList, Priority, TaskFilter, TaskStatus};

// UI components (input/output)
use todo_manager::ui::input::FileInputStream;
use todo_manager::ui::output::FileOutputWriter;

// Controller (application orchestration)
use todo_manager::controller::ApplicationController;

// Persistence
use todo_manager::persistence::TodoListStorage;
```

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
| `remove <id\|range\|all>` | Remove task(s) by ID, range, or all | `remove 1`, `remove 1-5`, `remove 1,3,5`, `remove all` |
| `complete <id\|range\|all>` | Mark task(s) as completed | `complete 1`, `complete 1-5`, `complete 1,3,5`, `complete all` |
| `uncomplete <id\|range\|all>` | Mark task(s) as pending | `uncomplete 1`, `uncomplete 1-5`, `uncomplete all` |
| `toggle <id\|range\|all>` | Toggle task(s) completion status | `toggle 1`, `toggle 1-5`, `toggle all` |
| `priority <id\|range> <level>` | Set task priority (high/medium/low) | `priority 1 high`, `priority 1-5 medium` |
| `set-category <id\|range> <name>` | Assign category to task(s) | `set-category 1 work`, `set-category 1-3 personal` |
| `categories` | List all categories | `categories` |
| `set-due <id> <date>` | Set task due date (YYYY-MM-DD) | `set-due 1 2024-12-31` |
| `edit <id> <description>` | Edit task description | `edit 1 New description` |
| `search <keyword>` | Search tasks by keyword | `search groceries` |
| `statistics` | Show task statistics | `statistics` |
| `help` | Show help message | `help` |
| `quit` | Exit the program | `quit` |

**Note:** The `complete`, `remove`, `uncomplete`, `toggle`, `priority`, and `set-category` commands support bulk operations:

- Single ID: `complete 1` or `priority 1 high`
- ID range: `complete 1-5` (completes tasks 1, 2, 3, 4, 5)
- ID list: `remove 1,3,5` (removes tasks 1, 3, and 5)
- Combined: `complete 1-3,7,9-11` (completes tasks 1,2,3,7,9,10,11)
- All tasks (where applicable): `complete all`, `remove all`, `uncomplete all`, `toggle all`

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

To enable debug mode, type:

```bash
> debug
```

You'll see:

```text
✓ Debug mode enabled

Additional debug commands available:

  debug:gen <count>  - Generate random tasks
  debug:clear        - Clear all tasks
  debug              - Disable debug mode
```

### Debug Commands

#### 1. Generate Random Tasks

Generate multiple random tasks with random states and priorities:

```bash
> debug:gen 10
```

This command:

- Creates 10 tasks with random descriptions from a predefined list
- Assigns random priorities (Low, Medium, High)
- Randomly completes ~30% of the tasks
- Maximum limit: 100 tasks per command

**Example output:**

```text
✓ Generated 10 random tasks
```

The generated tasks will have:

- Random task descriptions like "Buy groceries #4523", "Fix bug in authentication module #7812"
- Random priorities (Low ▼, Medium ■, High ▲)
- Random completion status (~30% completed)

#### 2. Clear All Tasks

Remove all tasks from the list:

```bash
> debug:clear
```

**Example output:**

```text
✓ Cleared 15 tasks
```

#### 3. Toggle Debug Mode

Disable debug mode to return to normal operation:

```bash
> debug
```

**Example output:**

```text
✓ Debug mode disabled
```

### Security

Debug commands are protected and will only work when debug mode is enabled. If you try to use debug commands without enabling debug mode first:

```bash
> debug:gen 5
✗ Debug mode is not enabled. Type 'debug' to enable it.
```

### Use Cases

Debug mode is useful for:

1. **Quick Testing** - Instantly populate the list with test data
2. **UI Testing** - Test how the interface handles many tasks
3. **Filter Testing** - Test filtering with diverse task states and priorities
4. **Performance Testing** - Generate large numbers of tasks to test performance
5. **Demo Purposes** - Quickly create a populated list for demonstrations

### Implementation Details

Debug functionality is isolated in the `debug_command/` module, keeping it separate from production code. This makes it easy to:

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
