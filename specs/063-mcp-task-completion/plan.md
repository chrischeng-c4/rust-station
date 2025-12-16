# Plan: MCP Task Completion

**Feature**: 063-mcp-task-completion
**Created**: 2024-12-17
**Depends On**: 062-mcp-resource-tools

## Architecture Overview

```
Claude Code                    rstn MCP Server              rstn TUI
    │                               │                          │
    │ tools/call rstn_complete_task │                          │
    │ {"task_id": "T001"}           │                          │
    │ ─────────────────────────────>│                          │
    │                               │ Validate completion      │
    │                               │ Update tasks.md          │
    │                               │ Event::McpTaskCompleted  │
    │                               │ ────────────────────────>│
    │                               │                          │ Refresh TUI
    │         {"next_task":...}     │                          │
    │ <─────────────────────────────│                          │
```

## Implementation Approach

### Phase 1: Tool Handler
- Parse task_id from arguments
- Find task in TaskListState
- Mark complete and save to file

### Phase 2: Validation (Optional)
- Run configurable checks before marking complete
- Return error if validation fails

### Phase 3: TUI Update
- Send McpTaskCompleted event
- Refresh task list in worktree view
- Update progress indicator

## Key Components

### Tool Handler

```rust
async fn handle_complete_task(
    args: CompleteTaskArgs,
    state: Arc<Mutex<AppState>>,
    event_sender: mpsc::Sender<Event>,
) -> Result<ToolResult> {
    let task_id = &args.task_id;

    // Mark complete
    state.lock().complete_task_by_id(task_id)?;

    // Notify TUI
    event_sender.send(Event::McpTaskCompleted {
        task_id: task_id.clone(),
        next_task: state.lock().get_next_incomplete_task(),
    }).await?;

    Ok(ToolResult::json(response))
}
```

### Event Type

```rust
pub enum Event {
    // ... existing ...
    McpTaskCompleted {
        task_id: String,
        next_task: Option<String>,
    },
}
```

## Files to Modify

| File | Changes |
|------|---------|
| `mcp_server.rs` | Add tool handler |
| `event.rs` | Add McpTaskCompleted event |
| `app.rs` | Handle event, refresh view |
| `worktree.rs` | Add complete_task_by_id() |

## Estimated Complexity

~200-250 lines
