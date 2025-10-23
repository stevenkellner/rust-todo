# Project Structure

This document explains the organization of the Rust Todo List Manager codebase.

## Directory Structure

```text
src/
├── main.rs                          # Application entry point
└── todo_lib/
    ├── lib.rs                       # Library root with module exports
    │
    ├── models/                      # Domain Models Layer
    │   ├── mod.rs                   # Models module definition
    │   ├── loop_control.rs          # Control flow enum (Continue, Break)
    │   ├── priority.rs              # Priority enum (Low, Medium, High)
    │   ├── task.rs                  # Task struct with priority and status
    │   ├── task_filter.rs           # Filter criteria for querying tasks
    │   ├── task_status.rs           # Task status enum (Pending, Completed)
    │   ├── todo_list.rs             # TodoList collection and business logic
    │   └── ui_event.rs              # Event definitions for user actions
    │
    ├── ui/                          # User Interface Layer
    │   ├── mod.rs                   # UI module definition
    │   ├── input_reader.rs          # Reads and parses user commands
    │   └── output_writer.rs         # Formats and displays output
    │
    └── controller/                  # Controller Layer
        ├── mod.rs                   # Controller module definition
        └── todo_controller.rs       # Orchestrates UI events with models

tests/
├── integration_tests.rs             # Integration tests for workflows
└── filtering_tests.rs               # Tests for combined filtering feature
```

## Architecture Layers

### 1. Models Layer (`models/`)

The domain layer containing all business logic and data structures.

- **`task.rs`** - Core Task type with:
  - Unique ID, description, completion status
  - Priority level (Low/Medium/High)
  - Methods for state management

- **`todo_list.rs`** - Collection managing tasks with:
  - Add, remove, complete, uncomplete operations
  - Filtering by status and/or priority
  - Task queries and retrieval

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
  - Combined filters (e.g., "pending high priority tasks")

- **`ui_event.rs`** - Event definitions representing user actions:
  - AddTask, RemoveTask, CompleteTask
  - ListTasks with filters
  - SetPriority
  - Quit, Help, InvalidInput

- **`loop_control.rs`** - Control flow signals:
  - Continue - keep running the application loop
  - Break - exit the application

### 2. UI Layer (`ui/`)

Handles all user interaction and terminal I/O.

- **`input_reader.rs`** - Parses commands:
  - Reads from stdin (or custom reader for testing)
  - Converts text commands to UiEvent enums
  - Supports command aliases (e.g., "rm" for "remove")

- **`output_writer.rs`** - Formats output:
  - Colored terminal output
  - Task list formatting
  - Error and success messages
  - Help text display

### 3. Controller Layer (`controller/`)

Coordinates between UI and model layers.

- **`todo_controller.rs`** - Main controller:
  - Processes UI events
  - Updates TodoList based on events
  - Delegates output to OutputWriter
  - Main application loop

## Import Paths

When using the library, import from the organized modules:

```rust
// Models (domain types, business logic, events)
use todo_manager::models::{Task, TodoList, Priority, TaskFilter, TaskStatus, UiEvent, LoopControl};

// UI components (input/output)
use todo_manager::ui::{InputReader, OutputWriter};

// Controller (application orchestration)
use todo_manager::controller::TodoController;

// Or use the convenient re-exports from the root
use todo_manager::{Task, TodoList, Priority, TodoController, UiEvent, LoopControl};
```

## Design Principles

1. **Separation of Concerns**: Each layer has a specific responsibility
   - Models: Business logic and data
   - UI: User interaction
   - Controller: Coordination

2. **One Type Per File**: Each struct/enum has its own file for clarity

3. **Dependency Flow**:
   - Controller depends on both UI and Models
   - UI depends on Models (for events)
   - Models are independent

4. **Testability**:
   - Each layer can be tested independently
   - InputReader and OutputWriter accept generic readers/writers
   - Integration tests verify full workflows

## Adding New Features

When adding features, follow this structure:

1. **New data types** → Add to `models/`
2. **New commands** → Update `ui_event.rs`, `input_reader.rs`, `output_writer.rs`
3. **New operations** → Add to `todo_list.rs` or `task.rs`
4. **New coordination** → Update `todo_controller.rs`

## Testing

```bash
# Run all tests
cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'

# Run doc tests
cargo test --doc

# Run with output
cargo test -- --nocapture
```
