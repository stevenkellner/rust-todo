# Todo List Manager

A command-line todo list application built in Rust with a clean, event-driven architecture and comprehensive test coverage.

## Features

- Add tasks with descriptive names
- List all tasks, or filter by completion status (all/completed/pending)
- Mark tasks as complete or incomplete
- Toggle task completion status
- Remove tasks by ID
- Intuitive command-line interface with descriptive commands
- Event-driven architecture with clean separation of concerns
- Comprehensive test suite with 70 tests (33 unit + 8 integration + 29 doc tests)
- Full documentation with `cargo doc`

## Project Structure

```text
src/
├── main.rs         # Binary entry point
└── todo_lib/       # Library module (todo_manager)
    ├── lib.rs            # Library interface
    ├── task.rs           # Task data structure and methods
    ├── todo_list.rs      # Todo list management logic (business logic)
    ├── input_reader.rs   # User input reading and command parsing
    ├── output_writer.rs  # Output formatting and display
    ├── ui_event.rs       # Event definitions for UI interactions
    ├── task_filter.rs    # Filter options for listing tasks
    ├── loop_control.rs   # Loop control flow enum
    └── todo_controller.rs # Event-driven controller (reacts to UI events)

tests/
└── integration_tests.rs  # Comprehensive integration tests

Cargo.toml          # Project configuration with todo_manager library
```

## Architecture Overview

The application follows an **event-driven architecture** with clear separation of concerns:

### Core Components

1. **Task** (`task.rs`)
   - Data structure representing a single todo item
   - Methods: `new()`, `toggle_completion()`, `is_completed()`, `get_status_symbol()`
   - Includes comprehensive unit tests

2. **TodoList** (`todo_list.rs`)
   - Business logic for managing a collection of tasks
   - CRUD operations: add, remove, complete, uncomplete, toggle
   - Task querying: get all tasks, completed tasks, pending tasks
   - Auto-incrementing task IDs
   - Extensively tested with edge cases

3. **InputReader** (`input_reader.rs`)
   - Handles reading user input from stdin
   - Parses user commands into `UiEvent` enums
   - Validates command syntax and arguments
   - Clean separation of input concerns

4. **OutputWriter** (`output_writer.rs`)
   - Handles all output operations to stdout
   - Semantic display methods for different scenarios (task added, removed, etc.)
   - Formats task lists with proper styling
   - Separates presentation logic from business logic

5. **UiEvent** (`ui_event.rs`)
   - Event definitions: `AddTask`, `ListTasks`, `RemoveTask`, `CompleteTask`, etc.
   - Clean abstraction between UI input and controller actions

6. **TaskFilter** (`task_filter.rs`)
   - Filter options: `All`, `Completed`, `Pending`
   - Used for listing tasks by status

7. **LoopControl** (`loop_control.rs`)
   - Control flow enum: `Continue`, `Exit`
   - More descriptive than boolean for loop control

8. **TodoController** (`todo_controller.rs`)
   - Event-driven controller that reacts to UI events
   - Coordinates between InputReader, OutputWriter, and TodoList
   - Owns InputReader, OutputWriter, and TodoList
   - Clean handler methods for each event type

9. **Main** (`main.rs`)
   - Simple application entry point
   - Creates and runs the TodoController

### Event Flow

```text
User Input → InputReader.read_input() 
          → InputReader.parse_command() 
          → UiEvent 
          → TodoController.handle_event() 
          → TodoList operations 
          → OutputWriter.show_*() methods 
          → Output to user
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
| `list [filter]` | List tasks (all/completed/pending) | `list`, `list completed`, `list pending` |
| `remove <id>` | Remove a task by ID | `remove 1` |
| `complete <id>` | Mark task as completed | `complete 1` |
| `uncomplete <id>` | Mark task as pending | `uncomplete 1` |
| `toggle <id>` | Toggle task completion status | `toggle 1` |
| `exit` | Exit the program | `exit` |

### Example Session

```text
add Buy groceries
Task added: Buy groceries

add Read a book
Task added: Read a book

list
1. [ ] Buy groceries
2. [ ] Read a book

complete 1
Task marked as complete: Buy groceries

list
1. [x] Buy groceries
2. [ ] Read a book

remove 1
Task removed: Buy groceries

exit
Goodbye!
```

## Testing

The project has a comprehensive test suite with **70 total tests** (33 unit tests + 8 integration tests + 29 doc tests):

### Run All Tests

```bash
cargo test
```

### Unit Tests

Each module includes extensive unit tests:

- `Task`: Creation, completion toggling, status display
- `TodoList`: CRUD operations, edge cases, ID management
- `InputReader`: Command parsing validation
- `OutputWriter`: Output formatting
- `TodoController`: Event handling, component coordination

### Integration Tests

The `tests/integration_tests.rs` file contains:

- End-to-end workflow tests
- Command parsing and execution
- Multi-step task management scenarios
- Error handling verification

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
