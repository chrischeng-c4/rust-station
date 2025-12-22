---
title: "Cancellation Support"
description: "Stop running Claude commands with Ctrl+C or Esc"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
tags: [claude-code, cancellation, signal-handling]
weight: 8
---

## 7. Cancellation Support

**Status**: ✅ VERIFIED (Phase 2 - Already Exists)

### CLI Mode: Ctrl+C → SIGINT

**Implementation**: Automatically handled by OS process tree signal propagation.

When user presses Ctrl+C in CLI mode:
1. SIGINT sent to rstn process
2. OS automatically propagates SIGINT to all child processes (including Claude subprocess)
3. Claude subprocess terminates gracefully
4. rstn exits

**No additional code required** - this is standard Unix/Linux process group behavior.

**Verification**:
```bash
rstn prompt "long running task" --max-turns 10
# Press Ctrl+C → Both rstn and claude processes terminate
```

### TUI Mode: Esc → Close View

**File**: [crates/rstn/src/tui/views/session_output.rs:338-342](../../crates/rstn/src/tui/views/session_output.rs#L338-L342)

**Implementation**:
```rust
impl View for SessionOutputView {
    fn handle_key(&mut self, key: KeyEvent) -> ViewAction {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                // Close session output view (subprocess continues in background)
                ViewAction::None // Parent will handle closing
            }
            // ... other keys ...
        }
    }
}
```

**Behavior**:
- User presses Esc → SessionOutputView closes
- Claude subprocess continues running in background
- User can switch to other views while Claude runs
- When Claude completes, completion status saved to session history

**Future Enhancement** (optional):
Add confirmation dialog before closing if session is still running:
```rust
KeyCode::Esc if !self.is_complete() => {
    // Show confirmation dialog
    ViewAction::ShowConfirmDialog {
        title: "Close Session View?",
        message: "Claude is still running. Close view? (session continues in background)",
        on_confirm: Box::new(|| ViewAction::CloseSessionOutput),
    }
}
```

**File**: `crates/rstn/src/tui/app.rs`

```rust
ViewAction::CancelClaudeSession => {
    if let Some(session_id) = self.active_claude_session {
        // Send cancellation signal
        self.cancel_claude_session(&session_id).await?;

        // Update session manager
        self.session_mgr.complete_claude_session(
            &self.current_rstn_session,
            &session_id,
            ClaudeSessionStatus::Error {
                message: "Cancelled by user".to_string()
            },
            0,
            None,
        )?;

        // Close session output view
        self.worktree_view.close_session_output();
    }
}
```

### Graceful Shutdown

Ensure subprocess cleanup:

```rust
impl Drop for ClaudeSessionHandle {
    fn drop(&mut self) {
        if let Some(child) = &mut self.child {
            // Try graceful shutdown first
            let _ = child.kill();
        }
    }
}
```

---

