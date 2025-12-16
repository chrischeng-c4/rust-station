# Plan: MCP Migration & Cleanup

**Feature**: 064-mcp-migration-cleanup
**Created**: 2024-12-17
**Depends On**: 063-mcp-task-completion

## Architecture Overview

```
Before (Dual Path):
claude_stream.rs
    ├── parse_status() → RscliStatus → state change
    └── get_text() → display

After (Single Path):
claude_stream.rs
    └── get_text() → display

mcp_server.rs
    └── tool calls → Event → state change
```

## Implementation Approach

### Phase 1: Remove Status Parsing
- Delete `RscliStatus` struct
- Delete `parse_status()` method
- Delete status helper methods
- Delete status block constants

### Phase 2: Simplify Display
- Simplify `get_display_text()` (no more stripping)
- Keep `get_text()` unchanged

### Phase 3: Update Event Handling
- Remove text-based fallback in `handle_claude_completed()`
- MCP events are now the sole control channel

### Phase 4: Update System Prompt
- Update `RSCLI_SYSTEM_PROMPT` in cargo.rs
- Replace status block instructions with MCP tool usage

## Key Components

### Code to Remove

```rust
// From claude_stream.rs
const STATUS_BLOCK_START: &str = "```rscli-status";
const STATUS_BLOCK_END: &str = "```";

pub struct RscliStatus { ... }

impl ClaudeStreamMessage {
    pub fn parse_status(&self) -> Option<RscliStatus> { ... }
    pub fn needs_input(&self) -> bool { ... }
    pub fn is_completed(&self) -> bool { ... }
    pub fn has_error(&self) -> bool { ... }
    pub fn get_input_prompt(&self) -> Option<String> { ... }
    pub fn get_error_message(&self) -> Option<String> { ... }
}
```

### Code to Keep

```rust
pub struct ClaudeStreamMessage {
    pub msg_type: String,
    pub message: Option<ClaudeMessage>,
    pub session_id: Option<String>,
    pub result: Option<String>,
    pub total_cost_usd: Option<f64>,
    pub is_error: Option<bool>,
}

impl ClaudeStreamMessage {
    pub fn get_text(&self) -> Option<String> { ... }
}
```

### Simplified get_display_text()

```rust
// Before (complex stripping logic)
pub fn get_display_text(&self) -> Option<String> {
    let text = self.get_text()?;
    // Strip status blocks...
    Some(stripped)
}

// After (simple passthrough)
pub fn get_display_text(&self) -> Option<String> {
    self.get_text()
}
```

## Files to Modify

| File | Changes |
|------|---------|
| `claude_stream.rs` | Remove ~100 lines |
| `cargo.rs` | Update system prompt |
| `app.rs` | Remove text fallback |

## Estimated Complexity

~200 lines removed, ~50 lines modified
