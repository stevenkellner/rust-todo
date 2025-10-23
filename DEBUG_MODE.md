# Debug Mode

The application includes a hidden debug mode for testing and development purposes.

## Enabling Debug Mode

To enable debug mode, type:

```bash
> debug
```

You'll see:

```text
✓ 🐛 Debug mode ENABLED
Debug commands available:
  - debug:gen <count>  : Generate N random tasks
  - debug:clear        : Clear all tasks
  - debug               : Toggle debug mode
```

## Debug Commands

### 1. Generate Random Tasks

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

### 2. Clear All Tasks

Remove all tasks from the list:

```bash
> debug:clear
```

**Example output:**

```text
✓ Cleared 15 tasks
```

### 3. Toggle Debug Mode

Disable debug mode to return to normal operation:

```bash
> debug
```

**Example output:**

```text
✓ Debug mode disabled
```

## Security

Debug commands are protected and will only work when debug mode is enabled. If you try to use debug commands without enabling debug mode first:

```bash
> debug:gen 5
✗ Debug mode is not enabled. Type 'debug' to enable it.
```

## Use Cases

Debug mode is useful for:

1. **Quick Testing** - Instantly populate the list with test data
2. **UI Testing** - Test how the interface handles many tasks
3. **Filter Testing** - Test filtering with diverse task states and priorities
4. **Performance Testing** - Generate large numbers of tasks to test performance
5. **Demo Purposes** - Quickly create a populated list for demonstrations

## Example Session

```bash
> debug
✓ 🐛 Debug mode ENABLED

> debug:gen 20
✓ Generated 20 random tasks

> list
--- All Tasks ---
1. [ ] ▲ Fix bug in authentication module #5234
2. [✓] ■ Buy groceries #8912
3. [ ] ▼ Write unit tests #3456
... (17 more tasks)

> list pending high
--- High Priority Pending Tasks ---
1. [ ] ▲ Fix bug in authentication module #5234
5. [ ] ▲ Deploy to production #7823

> debug:clear
✓ Cleared 20 tasks

> list
No tasks found. Use 'add <description>' to create a task.

> debug
✓ Debug mode disabled
```

## Notes

- Debug mode is **not** shown in the regular help menu (it's hidden by design)
- The debug:gen command has a maximum limit of 100 tasks to prevent accidental system overload
- Task IDs continue incrementing normally even after debug:clear
- All debug commands follow the pattern `debug:` or standalone `debug`
