# Plan: MCP Status Tool

**Feature**: 061-mcp-status-tool
**Created**: 2024-12-17
**Depends On**: 060-mcp-server-infrastructure

## Architecture Overview

```
Claude Code                    rstn MCP Server              rstn TUI
    │                               │                          │
    │ tools/call rstn_report_status │                          │
    │ ─────────────────────────────>│                          │
    │                               │ Event::McpStatus         │
    │                               │ ────────────────────────>│
    │                               │                          │ Update state
    │         {"content":[...]}     │                          │
    │ <─────────────────────────────│                          │
```

## Implementation Approach

### Phase 1: Define Tool Schema
- Create ReportStatusArgs struct
- Define JSON Schema for tool

### Phase 2: Implement Tool Handler
- Create handler function
- Send Event::McpStatus via channel
- Return success response

### Phase 3: Register Tool
- Add rstn_report_status to McpServer
- Verify it appears in tools/list

### Phase 4: Handle Events
- Add Event::McpStatus to event.rs
- Handle in app.rs main loop
- Trigger state transitions (InputDialog, etc.)

## Key Components

### Tool Schema

```rust
#[derive(Deserialize)]
struct ReportStatusArgs {
    status: String,      // "needs_input" | "completed" | "error"
    prompt: Option<String>,
    message: Option<String>,
}
```

### Event Type

```rust
pub enum Event {
    // ... existing ...
    McpStatus {
        status: String,
        prompt: Option<String>,
        message: Option<String>,
    },
}
```

### State Transitions

| Status | Action |
|--------|--------|
| needs_input | Show InputDialog with prompt |
| completed | Mark phase complete |
| error | Show error message |

## Files to Modify

| File | Changes |
|------|---------|
| `mcp_server.rs` | Add tool handler |
| `event.rs` | Add McpStatus event |
| `app.rs` | Handle McpStatus in main loop |

## Estimated Complexity

~200-300 lines
