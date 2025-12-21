---
title: "Edit Approval with Diff Preview"
description: "Preview file edits with unified diff before approval"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "066"
tags: [claude-code, edit, diff, approval]
weight: 7
---

## 6. Edit Approval via stream-json

**Status**: ✅ IMPLEMENTED (Phase 3)

### Problem Statement

**User Question**: "我不確定，應該是hook還是mcp，還是stream output?"

How does rstn show diff previews when Claude wants to edit files?

### Answer: stream-json Output (Intercept tool_use)

Claude Code stream-json output includes `tool_use` messages before execution:

```jsonl
{"type":"assistant","message":{"role":"assistant","content":[
  {"type":"tool_use","id":"toolu_01ABC","name":"Edit","input":{
    "file_path": "/path/to/file.rs",
    "old_string": "pub fn old_code() {\n    // ...\n}",
    "new_string": "pub fn new_code() {\n    // improved\n}"
  }}
]}}
```

### Implementation

**Files Modified**:
- [crates/rstn/Cargo.toml:54](../../crates/rstn/Cargo.toml#L54) - Added `similar` dependency
- [crates/rstn/src/tui/views/session_output.rs:121-187](../../crates/rstn/src/tui/views/session_output.rs#L121-L187) - Edit tool interception
- [crates/rstn/src/tui/views/session_output.rs:347-396](../../crates/rstn/src/tui/views/session_output.rs#L347-L396) - Diff generation helpers
- [crates/rstn/src/tui/views/session_output.rs:455-468](../../crates/rstn/src/tui/views/session_output.rs#L455-L468) - Syntax highlighting

#### 1. Dependency Added

**File**: [crates/rstn/Cargo.toml:54](../../crates/rstn/Cargo.toml#L54)

```toml
similar = "2.4"  # Text diffing for edit previews
```

#### 2. Tool Interception (Lines 121-187)

**File**: [crates/rstn/src/tui/views/session_output.rs:133-165](../../crates/rstn/src/tui/views/session_output.rs#L133-L165)

```rust
"tool_use" => {
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_use" {
                let tool_name = content.name.as_deref().unwrap_or("Unknown");
                let tool_id = content.id.as_deref()
                    .and_then(|id| id.get(..8))
                    .unwrap_or("????????");

                // Special handling for Edit tool - show diff preview
                if tool_name == "Edit" {
                    if let Some(input) = &content.input {
                        let file_path = input.get("file_path")
                            .and_then(|v| v.as_str())
                            .unwrap_or("unknown");
                        let old_string = input.get("old_string")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");
                        let new_string = input.get("new_string")
                            .and_then(|v| v.as_str())
                            .unwrap_or("");

                        self.output_lines.push(format!(
                            "🔧 Tool: {} [{}]",
                            tool_name, tool_id
                        ));

                        // Add diff preview
                        let preview_lines =
                            Self::format_edit_preview(file_path, old_string, new_string);
                        self.output_lines.extend(preview_lines);
                    }
                } else {
                    // Generic tool handling for non-Edit tools
                    // ...
                }
            }
        }
    }
}
```

#### 3. Diff Generation Helpers (Lines 347-396)

**File**: [crates/rstn/src/tui/views/session_output.rs:347-396](../../crates/rstn/src/tui/views/session_output.rs#L347-L396)

```rust
/// Generate a unified diff preview for Edit tool
fn generate_diff_preview(old_text: &str, new_text: &str) -> Vec<String> {
    use similar::{ChangeTag, TextDiff};

    let diff = TextDiff::from_lines(old_text, new_text);
    let mut result = Vec::new();

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            ChangeTag::Delete => "-",
            ChangeTag::Insert => "+",
            ChangeTag::Equal => " ",
        };
        result.push(format!("{} {}", sign, change.value().trim_end()));
    }

    result
}

/// Format Edit tool parameters for display
fn format_edit_preview(file_path: &str, old_string: &str, new_string: &str) -> Vec<String> {
    let mut lines = Vec::new();

    lines.push("─".repeat(60));
    lines.push(format!("📝 Edit Preview: {}", file_path));
    lines.push("─".repeat(60));
    lines.push(String::new());

    let diff_lines = Self::generate_diff_preview(old_string, new_string);

    if diff_lines.len() > 20 {
        // Show first 10 lines
        for line in &diff_lines[..10] {
            lines.push(line.clone());
        }
        lines.push(format!("... ({} more lines) ...", diff_lines.len() - 20));
        // Show last 10 lines
        for line in &diff_lines[diff_lines.len() - 10..] {
            lines.push(line.clone());
        }
    } else {
        lines.extend(diff_lines);
    }

    lines.push(String::new());
    lines.push("─".repeat(60));

    lines
}
```

#### 4. Syntax Highlighting (Lines 455-468)

**File**: [crates/rstn/src/tui/views/session_output.rs:455-468](../../crates/rstn/src/tui/views/session_output.rs#L455-L468)

```rust
let style = if line.starts_with("─") {
    Style::default().fg(Color::DarkGray)
} else if line.starts_with("+ ") {
    // Diff added line
    Style::default().fg(Color::Green)
} else if line.starts_with("- ") {
    // Diff deleted line
    Style::default().fg(Color::Red)
} else if line.starts_with("  ") && !line.starts_with("  ✓") {
    // Diff context line (but not tool result)
    Style::default().fg(Color::DarkGray)
} else if line.starts_with("📝 Edit Preview:") {
    // Edit preview header
    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
}
// ... other styling rules
```

#### 5. Tests (Lines 770-941)

**File**: [crates/rstn/src/tui/views/session_output.rs:770-941](../../crates/rstn/src/tui/views/session_output.rs#L770-L941)

**Test Coverage**:
- ✅ `test_generate_diff_preview` - Basic diff generation with +/- markers
- ✅ `test_format_edit_preview` - Full preview formatting with header
- ✅ `test_format_edit_preview_long_diff` - Truncation for long diffs (>20 lines)
- ✅ `test_edit_tool_interception` - Edit tool message triggers diff display
- ✅ `test_non_edit_tool_no_diff` - Non-Edit tools don't trigger diff preview

**Example Test**:
```rust
#[test]
fn test_edit_tool_interception() {
    use serde_json::json;

    let mut view = SessionOutputView::new(5);
    view.start_session("Test prompt", 5);

    // Simulate Edit tool_use message
    let edit_msg = ClaudeStreamMessage {
        msg_type: "tool_use".to_string(),
        message: Some(ClaudeMessage {
            role: "assistant".to_string(),
            content: vec![ClaudeContent {
                content_type: "tool_use".to_string(),
                name: Some("Edit".to_string()),
                input: Some(json!({
                    "file_path": "/tmp/test.rs",
                    "old_string": "let x = 1;",
                    "new_string": "let x = 2;",
                })),
                // ...
            }],
        }),
        // ...
    };

    view.add_message(&edit_msg);

    // Verify output contains diff preview
    let output = view.output_lines.join("\n");
    assert!(output.contains("📝 Edit Preview: /tmp/test.rs"));
    assert!(output.contains("- let x = 1;"));
    assert!(output.contains("+ let x = 2;"));
}
```

**Test Results**:
```bash
cargo test -p rstn session_output --lib
# Result: 13 passed (296 total, 1 pre-existing failure)
```

### Visual Output Example

When Claude uses the Edit tool, the TUI displays:

```
🔧 Tool: Edit [toolu_01A]
────────────────────────────────────────────────────────────
📝 Edit Preview: src/main.rs
────────────────────────────────────────────────────────────

  fn main() {
-     println!("Hello");
+     println!("Hello, World!");
  }

────────────────────────────────────────────────────────────
```

**Color Scheme**:
- Green: `+ ` (additions)
- Red: `- ` (deletions)
- Dark Gray: `  ` (context lines)
- Cyan Bold: `📝 Edit Preview:` (header)

### MCP Status Extension (Optional Future)

For interactive approval, extend MCP with `needs_edit_approval`:

```rust
// MCP handler
"rstn_report_status" if status == "needs_edit_approval" => {
    let (tx, rx) = oneshot::channel();
    state.push_tui_event(Event::EditApprovalRequest {
        file_path: args["file_path"].as_str().unwrap(),
        diff: args["diff"].as_str().unwrap(),
        response_tx: tx,
    });

    // Block until user approves/rejects
    match rx.await {
        Ok(true) => ToolResult::text("approved"),
        Ok(false) => ToolResult::text("rejected"),
        Err(_) => ToolResult::error("Approval request cancelled"),
    }
}
```

---

