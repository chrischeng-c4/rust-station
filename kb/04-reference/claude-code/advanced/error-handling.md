---
title: "MCP Error Handling"
description: "Structured error responses with actionable suggestions"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "067"
tags: [claude-code, mcp, errors, suggestions]
weight: 10
---

## 9. MCP Error Handling

**Status**: ✅ IMPLEMENTED (Phase 3)

### Implementation Details

**File**: [crates/rstn/src/tui/mcp_server.rs:273-282](../../crates/rstn/src/tui/mcp_server.rs#L273-L282)

**Code**:
```rust
impl ToolResult {
    /// Create an error result with a suggestion
    /// Format: "Error: {message} | Suggestion: {suggestion}"
    fn error_with_suggestion(message: &str, suggestion: &str) -> Self {
        Self {
            content: vec![ContentBlock::Text {
                text: format!("{} | Suggestion: {}", message, suggestion),
            }],
            is_error: Some(true),
        }
    }
}

**Enhanced Tool Handler** ([mcp_server.rs:567-644](../../crates/rstn/src/tui/mcp_server.rs#L567-L644)):
```rust
async fn handle_read_spec(...) -> ToolResult {
    // ... validate artifact ...

    match std::fs::read_to_string(&file_path) {
        Ok(content) => ToolResult::text(&content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            // Provide specific suggestion based on artifact type
            let suggestion = match artifact {
                "spec" => "Run /speckit.specify to generate spec first",
                "plan" => "Run /speckit.plan to generate plan first",
                "tasks" => "Run /speckit.tasks to generate tasks first",
                "checklist" => "Run /speckit.specify to create a feature with checklist",
                "analysis" => "Run /speckit.clarify to generate analysis first",
                _ => "Check if the file exists in the spec directory",
            };

            ToolResult::error_with_suggestion(
                &format!("{} not found", filename),
                suggestion,
            )
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            ToolResult::error_with_suggestion(
                &format!("Permission denied reading {}", filename),
                "Check file permissions or run with appropriate access rights",
            )
        }
        Err(e) => ToolResult::error(&format!("Could not read {}: {}", artifact, e)),
    }
}
```

### Enhanced Error Display in TUI

**File**: [crates/rstn/src/tui/views/session_output.rs:170-194](../../crates/rstn/src/tui/views/session_output.rs#L170-L194)

**Code**:
```rust
if message.is_error.unwrap_or(false) {
    let error_msg = message.result.as_deref().unwrap_or("Unknown error");

    // Extract suggestion if present
    let suggestion = Self::extract_suggestion(error_msg);
    let clean_error = Self::strip_suggestion(error_msg);

    self.completion_status = Some(CompletionStatus::Error {
        message: clean_error.clone(),
        duration_secs: duration,
    });
    self.output_lines.push("─".repeat(60));
    self.output_lines
        .push(format!("❌ Session failed: {}", clean_error));
    self.output_lines
        .push(format!("   Duration: {}s", duration));

    // Display suggestion if available
    if let Some(suggestion_text) = suggestion {
        self.output_lines.push(String::new());
        self.output_lines
            .push(format!("💡 Suggestion: {}", suggestion_text));
    }

    self.output_lines.push("─".repeat(60));
}

/// Extract suggestion from error message
/// Format: "{error message} | Suggestion: {suggestion}"
fn extract_suggestion(error_msg: &str) -> Option<String> {
    error_msg
        .split(" | Suggestion: ")
        .nth(1)
        .map(|s| s.to_string())
}

/// Get error message without suggestion part
fn strip_suggestion(error_msg: &str) -> String {
    error_msg
        .split(" | Suggestion: ")
        .next()
        .unwrap_or(error_msg)
        .to_string()
}
```

**Tests** ([session_output.rs:578-668](../../crates/rstn/src/tui/views/session_output.rs#L578-L668)):
```rust
#[test]
fn test_error_display_with_suggestion() {
    let mut view = SessionOutputView::new(5);
    view.start_session("Test prompt", 5);

    let error_msg = ClaudeStreamMessage {
        result: Some(
            "spec.md not found | Suggestion: Run /speckit.specify to generate spec first"
        ),
        is_error: Some(true),
        // ...
    };
    view.add_message(&error_msg);

    let output = view.output_lines.join("\n");
    assert!(output.contains("❌ Session failed: spec.md not found"));
    assert!(output.contains("💡 Suggestion: Run /speckit.specify"));
    assert!(!output.contains("| Suggestion:"));  // Separator not shown
}
```

### Common Error Suggestions

| Error | Suggestion |
|-------|------------|
| spec.md not found | Run `/speckit.specify` to generate spec first |
| plan.md not found | Run `/speckit.plan` to generate plan first |
| tasks.md not found | Run `/speckit.tasks` to generate tasks first |
| Feature not found | Select a feature from the worktree list first |
| Permission denied | Check file permissions or run with sudo |
| Session expired | Start a new session with `/speckit.specify` |

---

