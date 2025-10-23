# TODO Manager - Selected Features Implementation Plan

**Date:** October 23, 2025  
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

- ✅ Add priority levels: High, Medium, Low
- ✅ Color-code tasks by priority (High=Red, Medium=Yellow, Low=Blue)
- ✅ Sort and filter tasks by priority
- ✅ Display priority in task list

**Implementation Details:**

- ✅ Add `priority: Priority` enum field to Task struct
- ✅ Update task creation to accept optional priority (default: Medium)
- ✅ Add `priority <id> <high|medium|low>` command (alias: `pri`)
- ✅ Modify display to show priority indicator (▼ ■ ▲)
- ✅ Add priority filter to list command
- ✅ Support combined filtering (e.g., `list pending high`)

**Implementation Notes:**

- Priority enum in `models/priority.rs` with Low/Medium/High variants
- Visual indicators: ▼ (Low/Blue), ■ (Medium/Yellow), ▲ (High/Red)
- `TaskFilter` struct supports combined status + priority filtering
- `get_filtered_tasks()` method filters by both criteria simultaneously
- Command aliases: high/h, medium/med/m, low/l

---

### 2. Task Search

**Status:** Pending  
**Complexity:** Easy  
**Description:**

- Search tasks by keyword in description
- Case-insensitive search
- Display matching tasks

**Implementation Details:**

- Add `search <keyword>` command
- Implement search method in TodoList
- Return tasks matching keyword
- Display results with highlighting

---

### 3. Task Statistics Display

**Status:** Pending  
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

---

### 4. Edit Task Description

**Status:** Pending  
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

---

### 6. Due Dates

**Status:** Pending  
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

---

### 7. Task Categories/Tags (User-Created)

**Status:** Pending  
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

---

### 9. Persistence (Save/Load)

**Status:** Pending  
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

**Status:** Pending  
**Complexity:** Medium  
**Description:**

- Select multiple tasks by ID ranges
- Complete/delete multiple tasks at once
- Support syntax: `1-5`, `1,3,5`, `1-3,7,9-11`

**Implementation Details:**

- Create ID parser for ranges and lists
- Update commands to accept multiple IDs:
  - `complete 1-5`
  - `remove 1,3,5`
  - `complete 1-3,7,9-11`
- Add `complete-all` and `remove-all` commands
- Add confirmation for bulk deletes
- Display count of affected tasks

---

### 12. Subtasks

**Status:** Pending  
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

**Status:** Pending  
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

**Status:** Pending  
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

---

### 15. Time Tracking

**Status:** Pending  
**Complexity:** Advanced  
**Description:**

- Log time spent on tasks
- Time estimates vs actual time
- Start/stop timer for tasks
- View time statistics

**Implementation Details:**

- Add fields to Task:
  - `estimated_minutes: Option<u32>`
  - `actual_minutes: u32`
  - `time_entries: Vec<TimeEntry>`
- Add `start-timer <id>` command
- Add `stop-timer <id>` command
- Add `log-time <id> <minutes>` command
- Add `estimate <id> <minutes>` command
- Display time in task list
- Show total time in statistics

---

### 16. Multiple Lists/Projects

**Status:** Pending  
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

### 17. Reminders/Notifications

**Status:** Pending  
**Complexity:** Advanced  
**Description:**

- Desktop notifications for due tasks
- Reminder system
- Custom reminder times

**Implementation Details:**

- Use `notify-rust` crate for desktop notifications
- Add `reminder_time: Option<NaiveDateTime>` to Task
- Add `remind <id> <time>` command
- Background thread to check reminders
- Notify when:
  - Task is due today
  - Task is overdue
  - Custom reminder time reached
- Add `snooze <id> <minutes>` command

---

### 20. Task History

**Status:** Pending  
**Complexity:** Medium  
**Description:**

- View completed tasks history
- Completion timeline
- Archive old tasks

**Implementation Details:**

- Add `completed_at: Option<NaiveDateTime>` to Task
- Add `archived: bool` field to Task
- Add `history` command to view completed tasks
- Add `archive <id>` command
- Add `unarchive <id>` command
- Filter archived tasks from normal list
- Display completion date in history

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
3. Time Tracking (#15)
4. Multiple Projects (#16)

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
