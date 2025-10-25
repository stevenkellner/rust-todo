# TODO Manager - Selected Features Implementation Plan

**Date:** October 25, 2025  
**Project:** Rust TODO List Manager

---

## Selected Features to Implement

This document tracks the 19 features selected for implementation in the TODO manager application.

---

## ✅ Feature List

### 1. Task Priorities

**Status:** ✅ Complete  
**Complexity:** Easy  
**Description:**

- Add priority levels: High, Medium, Low
- Color-code tasks by priority (High=Red, Medium=Yellow, Low=Blue)
- Sort and filter tasks by priority
- Display priority in task list

**Implementation Details:**

- Add `priority: Priority` enum field to Task struct
- Update task creation to accept optional priority (default: Medium)
- Add `priority <id> <high|medium|low>` command (alias: `pri`)
- Modify display to show priority indicator (▼ ■ ▲)
- Add priority filter to list command
- Support combined filtering (e.g., `list pending high`)

**Implementation Notes:**

- Priority enum in `models/priority.rs` with Low/Medium/High variants
- Visual indicators: ▼ (Low/Blue), ■ (Medium/Yellow), ▲ (High/Red)
- `TaskFilter` struct supports combined status + priority filtering
- `get_filtered_tasks()` method filters by both criteria simultaneously
- Command aliases: high/h, medium/med/m, low/l

---

### 2. Task Search

**Status:** ✅ Complete  
**Complexity:** Easy  
**Description:**

- Search tasks by keyword in description
- Case-insensitive search
- Display matching tasks

**Implementation Details:**

- Add `search <keyword>` command with `find` alias
- Implement search method in TodoList (case-insensitive, partial match)
- Return tasks matching keyword
- Display results with title "Search Results for '&lt;keyword&gt;'"
- Handle empty results gracefully

**Implementation Notes:**

- Added SearchTasks(String) event variant to UiEvent
- Implemented parse_search_command() in InputReader
- Implemented search_tasks() method in TodoList (case-insensitive filtering)
- Implemented show_search_results() in OutputWriter
- Implemented handle_search_tasks() in TodoController
- Added comprehensive unit tests (11 new tests)
- Updated help text to include search command

---

### 3. Task Statistics Display

**Status:** ✅ Complete  
**Complexity:** Easy  
**Description:**

- Show total task count
- Show completed task count
- Show pending task count
- Display completion percentage
- Show statistics summary

**Implementation Details:**

- Add `stats` or `statistics` command
- Calculate and display:
  - Total tasks
  - Completed tasks
  - Pending tasks
  - Completion percentage
  - Tasks by priority breakdown
  - Tasks by category breakdown

**Implementation Notes:**

- Added `ShowStatistics` event variant to UiEvent enum
- Created `TaskStatistics` struct in `models/todo_list.rs` with fields:
  - `total`, `completed`, `pending`
  - `completion_percentage` (f64, 0.0-100.0)
  - `high_priority`, `medium_priority`, `low_priority` counts
- Implemented `get_statistics()` method in TodoList
- Added `show_statistics()` in OutputWriter with colored, formatted display:
  - Overall statistics section (total, completed, pending, completion %)
  - Priority breakdown section (high/medium/low counts with colored symbols)
- Added `handle_show_statistics()` in TodoController
- Commands: `statistics` (primary), `stats` (alias)
- Added to help output with proper formatting
- All tests passing (201 total tests)

---

### 4. Edit Task Description

**Status:** ✅ Complete  
**Complexity:** Easy  
**Description:**

- Modify existing task descriptions
- Command: `edit <id> <new description>`

**Implementation Details:**

- Add `edit_task(id, new_description)` method to TodoList
- Add EditTask event to UiEvent enum
- Parse edit command in InputReader
- Update task description
- Display confirmation message

**Implementation Notes:**

- Added `EditTask(usize, String)` event variant to UiEvent enum
- Implemented `edit_task()` method in TodoList that:
  - Finds task by ID
  - Updates description
  - Returns `Option<&Task>` (Some if found, None if not)
- Added `parse_edit_command()` in InputReader:
  - Validates ID format
  - Validates description is not empty
  - Joins multi-word descriptions
- Added `show_task_edited()` in OutputWriter:
  - Shows old and new descriptions
  - Uses success message formatting
- Added `handle_edit_task()` in TodoController:
  - Stores old description before editing
  - Displays appropriate success/error message
- Command: `edit <id> <description>`
- Added to help output with proper formatting
- All tests passing (202 total tests)
- Doc test included in edit_task() method

---

### 6. Due Dates

**Status:** ✅ Complete  
**Complexity:** Medium  
**Description:**

- Add due dates to tasks
- Parse date formats (YYYY-MM-DD, natural language)
- Show overdue tasks in red
- Sort by due date
- Filter by date range

**Implementation Details:**

- Add `due_date: Option<NaiveDate>` field to Task
- Use `chrono` crate for date handling
- Add `set-due <id> <date>` command
- Add `overdue` filter to list command
- Display days until due / days overdue
- Color code: overdue (red), due today (yellow), due soon (orange)

**Implementation Notes:**

- Added `chrono = "0.4"` dependency to Cargo.toml
- Added `due_date: Option<NaiveDate>` field to Task struct
- Implemented due date helper methods in Task:
  - `set_due_date(due_date)` - Sets or clears the due date
  - `get_due_date()` - Returns the optional due date
  - `is_overdue(today)` - Checks if task is overdue
- Added `SetDueDate(usize, Option<NaiveDate>)` event variant to UiEvent
- Implemented `set_due_date()` method in TodoList
- Added `parse_set_due_command()` in InputReader:
  - Parses YYYY-MM-DD format dates
  - Accepts "none" or "clear" to remove due date
  - Validates date format and task ID
- Added `show_due_date_set()` in OutputWriter for confirmation messages
- Updated `format_task()` in TaskFormatter to display due dates:
  - Overdue tasks shown in bright red
  - Due today shown in bright yellow
  - Due within 3 days shown in yellow
  - Future dates shown in cyan
  - Format: "(due: YYYY-MM-DD)"
- Added `handle_set_due_date()` in TodoController
- Extended TaskFilter with `overdue: Option<bool>` field
- Added `overdue()` constructor and `with_overdue()` method to TaskFilter
- Updated `get_filtered_tasks()` in TodoList to filter by overdue status
- Added "overdue" filter keyword to list command parser
- Commands: `set-due <id> <YYYY-MM-DD|none>` (alias: `due`)
- Filters: `list overdue` to show only overdue tasks
- Updated help text with due date commands and filters
- All tests passing (206 total: 150 unit + 6 filtering + 8 integration + 42 doc tests)

---

### 7. Task Categories/Tags (User-Created)

**Status:** ✅ Complete  
**Complexity:** Medium  
**Description:**

- User-defined categories/tags
- Assign categories to tasks
- Filter tasks by category
- Color-code by category
- List all categories

**Implementation Details:**

- Add `category: Option<String>` field to Task
- Add `set-category <id> <category>` command
- Add `remove-category <id>` command
- Add `categories` command to list all categories
- Add category filter: `list category:<name>`
- Assign colors to categories dynamically
- Display category in task list

**Implementation Notes:**

- Added `category: Option<String>` field to Task struct
- Implemented category helper methods in Task:
  - `set_category(category)` - Sets or clears the category (None to clear)
  - `get_category()` - Returns the optional category reference
- Added `SetCategory(usize, Option<String>)` event variant to UiEvent
- Added `ListCategories` event variant to UiEvent
- Implemented `set_task_category()` method in TodoList
- Implemented `get_all_categories()` method in TodoList:
  - Returns sorted, deduplicated Vec of all category names
  - Efficiently collects unique categories from all tasks
- Added `parse_set_category_command()` in InputReader:
  - Parses category names (single or multi-word)
  - Accepts "none" or "clear" to remove category
  - Validates task ID format
- Extended list command parser to support category filtering:
  - Syntax: `category:name` or `cat:name`
  - Example: `list pending category:work`
  - Detects conflicts with other category filters
- Added category field to TaskFilter struct
- Updated `get_filtered_tasks()` in TodoList to filter by category
- Added `format_category()` in TaskFormatter:
  - Displays category badges in bright magenta color
  - Format: `[category]`
- Added `show_category_set()` in OutputWriter for confirmation messages
- Added `show_categories()` in OutputWriter to list all categories
- Added `handle_set_category()` in TodoController
- Added `handle_list_categories()` in TodoController
- Commands:
  - `set-category <id> <name>` (aliases: `category`, `cat`)
  - `categories` (alias: `list-categories`)
- Filters: `list category:work` or `list cat:work`
- All category badges displayed in bright magenta color
- Updated help text with category commands and filtering syntax
- All tests passing (211 total: 150 unit + 6 filtering + 8 integration + 47 doc tests)

---

### 9. Persistence (Save/Load)

**Status:** ✅ Complete  
**Complexity:** Medium  
**Description:**

- Save tasks to JSON file
- Load tasks on startup
- Auto-save on changes
- Backup functionality

**Implementation Details:**

- Use `serde` and `serde_json` crates
- Add `#[derive(Serialize, Deserialize)]` to Task
- Default file: `~/.todo_manager/tasks.json`
- Auto-save after every change
- Load tasks in TodoController::new()
- Add `save` and `load` commands for manual control
- Add `backup` command to create timestamped backup

---

### 11. Bulk Operations

**Status:** ✅ Completed  
**Complexity:** Medium  
**Description:**

- Select multiple tasks by ID ranges
- Complete/delete multiple tasks at once
- Support syntax: `1-5`, `1,3,5`, `1-3,7,9-11`

**Implementation Details:**

- ✅ Create ID parser for ranges and lists (`parse_ids` function in `id_parser.rs`)
- ✅ Update commands to accept multiple IDs:
  - `complete 1-5` - Complete tasks with IDs 1 through 5
  - `remove 1,3,5` - Remove tasks with IDs 1, 3, and 5
  - `complete 1-3,7,9-11` - Complete tasks in ranges 1-3 and 9-11, plus task 7
- ✅ Add `complete all` and `remove all` commands
- ✅ Display count of affected tasks in output
- ⚠️ Bulk delete confirmation not yet implemented (could be added later)

**Files Modified:**

- `src/todo_lib/models/id_parser.rs` - NEW file with parse_ids() function
- `src/todo_lib/models/mod.rs` - Exported parse_ids function
- `src/todo_lib/models/todo_list.rs` - Added complete_tasks(), remove_tasks(), complete_all_tasks(), remove_all_tasks()
- `src/todo_lib/controller/task_command/task.rs` - Added CompleteMultiple, RemoveMultiple, CompleteAll, RemoveAll variants
- `src/todo_lib/controller/task_command/task_command_input_parser.rs` - Updated parsers to handle ID ranges
- `src/todo_lib/controller/task_command/task_command_controller.rs` - Added bulk operation handlers
- `src/todo_lib/controller/task_command/task_command_output_manager.rs` - Added output methods for bulk operations

---

### 12. Subtasks

**Status:** ✅ Completed
**Complexity:** Advanced  
**Description:**

- Create subtasks under main tasks
- Hierarchical task structure
- Track subtask completion
- Parent task shows subtask progress

**Implementation Details:**

- Add `parent_id: Option<usize>` field to Task
- Add `subtasks: Vec<usize>` or use parent_id for lookup
- Add `add-subtask <parent_id> <description>` command
- Add `list-subtasks <id>` command
- Display subtasks indented under parent
- Parent task auto-completes when all subtasks complete
- Show progress: "Main Task (2/5 subtasks complete)"

---

### 13. Recurring Tasks

**Status:** ✅ Completed  
**Complexity:** Advanced  
**Description:**

- Daily, weekly, monthly recurring tasks
- Auto-recreate when completed
- Skip/snooze recurring tasks

**Implementation Details:**

- Add `recurrence: Option<Recurrence>` field to Task
- Recurrence enum: Daily, Weekly, Monthly, Custom
- Add `set-recurring <id> <daily|weekly|monthly>` command
- When recurring task completed, create new instance
- Add `next_due_date` calculation
- Add `skip-recurrence <id>` command
- Display recurrence icon: "🔄"

---

### 14. Task Dependencies

**Status:** ✅ Complete  
**Complexity:** Advanced  
**Description:**

- Define task prerequisites
- Can't complete task until dependencies complete
- Visualize dependency chain

**Implementation Details:**

- Add `depends_on: Vec<usize>` field to Task
- Add `add-dependency <task_id> <depends_on_id>` command
- Add `remove-dependency <task_id> <depends_on_id>` command
- Validate completion: check all dependencies complete
- Display blocked tasks differently
- Add `dependencies <id>` command to show chain
- Detect circular dependencies
- Show dependencies with the list command
- RandomTaskGenerator also generate dependencies

**Implementation Notes:**

- Added `depends_on: Vec<usize>` field to both `Task` and `TaskWithoutId` structs
- Implemented dependency management methods in Task:
  - `add_dependency(dependency_id)` - Adds a dependency (prevents duplicates)
  - `remove_dependency(dependency_id)` - Removes a dependency
  - `get_dependencies()` - Returns the dependencies vector
  - `has_dependency(id)` - Checks if task has specific dependency
  - `has_dependencies()` - Checks if task has any dependencies
- Implemented TodoList methods for dependency management:
  - `add_task_dependency(task_id, depends_on_id)` - Adds dependency with validation
  - `remove_task_dependency(task_id, depends_on_id)` - Removes a dependency
  - `are_dependencies_completed(task_id)` - Checks if all dependencies are completed
  - `would_create_circular_dependency()` - Detects circular dependencies
  - `has_transitive_dependency()` - Performs depth-first search for transitive dependencies
- Added command variants:
  - `TaskCommand::AddDependency(task_id, depends_on_id)`
  - `TaskCommand::RemoveDependency(task_id, depends_on_id)`
- Implemented command parsers with aliases:
  - `add-dependency`, `add-dep`, `depends-on`
  - `remove-dependency`, `remove-dep`, `rm-dep`
- Updated `complete_task()` to validate dependencies before allowing completion
- Added dependency indicators in task display:
  - Shows "🔒 depends on: [ids]" in yellow for tasks with dependencies
  - Format: "🔒 depends on: 1" or "🔒 depends on: 1, 2, 3" for multiple
- Updated `RandomTaskGenerator`:
  - 30% probability for each task (after first) to depend on a random earlier task
  - Displays dependency count in generation message
- Updated help text with dependency commands and examples
- Validation features:
  - Prevents self-dependencies (task depending on itself)
  - Detects and prevents circular dependencies
  - Prevents completion of tasks with incomplete dependencies
  - Shows appropriate error messages for each validation failure
- All 297 tests passing (194 unit + 6 filtering + 11 integration + 86 doc tests)

---

### 16. Multiple Lists/Projects

**Status:** ✅ Complete  
**Complexity:** Advanced  
**Description:**

- Separate todo lists for different projects
- Switch between lists
- List management commands

**Implementation Details:**

- Add `Project` struct with name and TodoList
- Add `projects: HashMap<String, Project>` to controller
- Add `current_project: String` to track active project
- Commands:
  - `new-project <name>`
  - `switch-project <name>`
  - `list-projects`
  - `delete-project <name>`
  - `rename-project <old> <new>`
- Save/load multiple projects
- Display current project in prompt

---

### 21. Progress Analytics

**Status:** Pending  
**Complexity:** Medium  
**Description:**

- ASCII progress bars
- Weekly/monthly completion graphs
- Productivity statistics

**Implementation Details:**

- Add `analytics` command
- Display:
  - Completion rate over time
  - Tasks completed per day/week/month
  - Average completion time
  - Most productive days
  - Category breakdown
  - Priority completion rates
- ASCII bar charts for visualization
- Trend analysis

---

### 24. Task Sorting

**Status:** Pending  
**Complexity:** Easy  
**Description:**

- Sort by ID, date created, priority, due date
- Customizable sort order (ascending/descending)

**Implementation Details:**

- Add `sort` parameter to list command
- Sort options:
  - `list --sort id`
  - `list --sort priority`
  - `list --sort due`
  - `list --sort created`
  - `list --sort category`
- Add `--reverse` flag for descending order
- Default sort by ID
- Save preferred sort in config

---

### 26. TUI (Text User Interface)

**Status:** Pending  
**Complexity:** Advanced  
**Description:**

- Full-screen terminal UI
- Mouse support
- Arrow key navigation
- Interactive task selection

**Implementation Details:**

- Use `ratatui` (formerly tui-rs) crate
- Add `--tui` flag to launch TUI mode
- Features:
  - Task list with arrow key navigation
  - Multi-pane layout (tasks, details, stats)
  - Mouse click to select tasks
  - Keyboard shortcuts
  - Modal dialogs for input
  - Real-time updates
  - Color themes
- Key bindings:
  - `j/k` or `↑/↓`: Navigate
  - `Enter`: Toggle complete
  - `d`: Delete
  - `e`: Edit
  - `a`: Add
  - `q`: Quit
  - `/`: Search
  - `?`: Help

---

### 27. CLI Arguments

**Status:** Pending  
**Complexity:** Medium  
**Description:**

- Non-interactive mode
- Quick commands from shell
- Scriptable operations

**Implementation Details:**

- Use `clap` crate for argument parsing
- Commands:
  - `todo add "Task description"`
  - `todo list`
  - `todo complete <id>`
  - `todo remove <id>`
  - `todo search <keyword>`
  - `todo stats`
- If no args provided, enter interactive mode
- Support all interactive commands
- Output suitable for scripting
- Add `--json` flag for JSON output
- Add `--quiet` flag for minimal output

---

## Implementation Order

### Phase 1: Core Enhancements (Weeks 1-2)

1. Task Priorities (#1)
2. Task Search (#2)
3. Task Statistics (#3)
4. Edit Command (#4)
5. Persistence (#9)

### Phase 2: Date & Organization (Weeks 3-4)

1. Due Dates (#6)
2. Categories/Tags (#7)
3. Task Sorting (#24)

### Phase 3: Advanced Task Features (Weeks 5-6)

1. Bulk Operations (#11)
2. Task History (#20)
3. Subtasks (#12)

### Phase 4: Complex Features (Weeks 7-9)

1. Recurring Tasks (#13)
2. Task Dependencies (#14)
3. Multiple Projects (#16)

### Phase 5: UI & Integration (Weeks 10-12)

1. CLI Arguments (#27)
2. Progress Analytics (#21)
3. Reminders/Notifications (#17)
4. TUI Interface (#26)

---

## Dependencies to Add to Cargo.toml

```toml
[dependencies]
colored = "2.1"           # Already added
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
clap = { version = "4.4", features = ["derive"] }
ratatui = "0.24"
crossterm = "0.27"
notify-rust = "4.10"
dirs = "5.0"              # For home directory
regex = "1.10"            # For advanced parsing
```

---

## Testing Strategy

Each feature should include:

- Unit tests for core functionality
- Integration tests for command flow
- Documentation examples
- User acceptance testing

---

## Notes

- Maintain backward compatibility where possible
- Add migration for existing data when structure changes
- Keep interactive mode responsive
- Ensure all features work in both CLI and interactive modes
- Document all new commands in help text
- Use semantic versioning for releases

---

## Future Considerations (Not in this phase)

- Web interface
- Mobile app sync
- Cloud backup
- Collaboration features
- Plugin system
- Custom themes
- Export to various formats (Markdown, CSV, iCal)
- Import from other apps
- AI-powered task suggestions
- Voice commands
- Integration with calendars
- GitHub/GitLab issue sync

---

**Last Updated:** October 23, 2025
