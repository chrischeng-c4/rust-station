---
title: "Permission Modes"
description: "Control how Claude Code executes tools: plan, auto, or ask"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "060"
tags: [claude-code, permissions, cli, tui]
weight: 2
---

# Permission Modes

**Status**: ✅ IMPLEMENTED (Phase 1)

## Overview

Claude Code supports three permission modes that control how it handles tool execution:

```bash
--permission-mode plan  # Plan before executing (like TUI Shift+Tab)
--permission-mode auto  # Execute without asking
--permission-mode ask   # Ask before each tool (default)
```

**Reference**: [CLI Reference](../cli-reference.md)

## Implementation Details

**File**: `crates/rstn/src/runners/cargo.rs:40-80`

**Code**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionMode {
    Plan,
    Auto,
    Ask,
}

impl PermissionMode {
    pub fn as_cli_arg(&self) -> &'static str {
        match self {
            PermissionMode::Plan => "plan",
            PermissionMode::Auto => "auto",
            PermissionMode::Ask => "ask",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ClaudeCliOptions {
    pub max_turns: Option<u32>,
    pub skip_permissions: bool,
    pub continue_session: bool,
    pub session_id: Option<String>,
    pub allowed_tools: Vec<String>,
    pub system_prompt_file: Option<std::path::PathBuf>,
    pub add_dirs: Vec<std::path::PathBuf>,
    pub permission_mode: Option<PermissionMode>, // ✅ IMPLEMENTED
    pub context_files: Vec<std::path::PathBuf>,
}
```

**Command Building** (`cargo.rs:420-440`):
```rust
// Add permission mode if specified
if let Some(mode) = options.permission_mode {
    command.arg("--permission-mode").arg(mode.as_cli_arg());
}
```

**Usage in rstn TUI**:
```rust
// When user presses 'p' (Prompt Claude)
let options = ClaudeCliOptions {
    permission_mode: Some(PermissionMode::Plan), // Plan first
    max_turns: Some(10),
    allowed_tools: vec!["Read".to_string(), "Glob".to_string()],
    context_files: vec![],
    // ...
};
```

**Usage in rstn CLI** (future enhancement - not yet exposed in CLI args):
```bash
# Future: rstn prompt "Add dark mode" --permission-mode plan
# Future: rstn prompt "Fix bug" --permission-mode auto --allowed-tools Read,Edit
```

## Benefits

1. **Transparency**: User sees what Claude plans to do before execution
2. **Safety**: Review tool calls before they run
3. **Compatibility**: Matches TUI Shift+Tab behavior in CLI mode
4. **Flexibility**: Can switch modes per command

## Related

- [CLI Reference](../cli-reference.md) - Complete CLI flags
- [Headless Mode](../headless-mode.md) - Automated workflows
- [Overview](overview.md) - All advanced features
