---
title: "Additional UI Improvements"
description: "Tool progress indicators, session history, and enhanced display"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "068"
tags: [claude-code, ui, progress, history]
weight: 11
---

## 10. Additional Improvements

### 10.1 Tool Execution Progress

**Status**: ✅ IMPLEMENTED (Phase 3)

**Files Modified**:
- [crates/rstn/src/tui/views/session_output.rs:15-20](../../crates/rstn/src/tui/views/session_output.rs#L15-L20) - ActiveTool tracking struct
- [crates/rstn/src/tui/views/session_output.rs:147-156](../../crates/rstn/src/tui/views/session_output.rs#L147-L156) - Tool tracking on tool_use
- [crates/rstn/src/tui/views/session_output.rs:213-246](../../crates/rstn/src/tui/views/session_output.rs#L213-L246) - Elapsed time on tool_result
- [crates/rstn/src/tui/views/session_output.rs:526-533](../../crates/rstn/src/tui/views/session_output.rs#L526-L533) - Syntax highlighting for running tools

#### Implementation

**Tool Tracking Structure**:
```rust
/// Tool execution tracking
#[derive(Debug, Clone)]
struct ActiveTool {
    name: String,
    start_time: Instant,
}

pub struct SessionOutputView {
    // ... other fields
    /// Track active tool executions (tool_id → tool info)
    active_tools: HashMap<String, ActiveTool>,
}
```

**Tool Start Tracking** (Lines 147-156):
```rust
// Track tool execution start
if let Some(id) = &content.id {
    self.active_tools.insert(
        id.clone(),
        ActiveTool {
            name: tool_name.to_string(),
            start_time: Instant::now(),
        },
    );
}

// Display with running indicator
self.output_lines.push(format!(
    "🔧 Tool: {} [{}] ⏳ Running... {}",
    tool_name, tool_id, params
));
```

**Tool Completion with Timing** (Lines 213-246):
```rust
"tool_result" => {
    // Check if we have execution time tracking
    let mut elapsed_ms: Option<u128> = None;

    // Find and remove from active tools
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_result" {
                if let Some(tool_use_id) = &content.tool_use_id {
                    if let Some(active_tool) = self.active_tools.remove(tool_use_id) {
                        elapsed_ms = Some(active_tool.start_time.elapsed().as_millis());
                    }
                }
            }
        }
    }

    // Display result with elapsed time if available
    if let Some(ms) = elapsed_ms {
        self.output_lines.push(format!("  ✓ Result: {} ({}ms)", truncated, ms));
    } else {
        self.output_lines.push(format!("  ✓ Result: {}", truncated));
    }
}
```

**Syntax Highlighting** (Lines 526-533):
```rust
} else if line.starts_with("🔧 Tool:") && line.contains("⏳ Running...") {
    // Tool execution in progress
    Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD)
} else if line.starts_with("🔧 Tool:") {
    // Tool execution (completed)
    Style::default().fg(Color::Yellow)
```

#### Tests (Lines 994-1224)

**Test Coverage**:
- ✅ `test_tool_execution_progress_indicator` - Verifies "⏳ Running..." is shown
- ✅ `test_tool_execution_completion_with_timing` - Verifies elapsed time in result
- ✅ `test_multiple_active_tools` - Verifies multiple concurrent tool tracking

**Test Results**:
```bash
cargo test -p rstn session_output --lib
# Result: 16 passed (299 total, 1 pre-existing failure)
```

#### Visual Output

**Before** (tool execution):
```
🔧 Tool: Read [toolu_01A] ⏳ Running... {"file_path":"/tmp/test.rs"}
```

**After** (tool completion):
```
🔧 Tool: Read [toolu_01A] ⏳ Running... {"file_path":"/tmp/test.rs"}
  ✓ Result: File read successfully (42ms)
```

**Color Scheme**:
- Yellow Bold: `⏳ Running...` (active tools)
- Yellow: Tool name (completed)
- Blue: Result text
- Timing: Shown in milliseconds for precise measurement

### 10.2 Session History UI

**New View**: [crates/rstn/src/tui/views/session_history.rs](../../crates/rstn/src/tui/views/session_history.rs)

```rust
pub struct SessionHistoryView {
    sessions: Vec<RstnSession>,
    selected_index: usize,
}

impl View for SessionHistoryView {
    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let items: Vec<ListItem> = self.sessions.iter().map(|session| {
            let cost_str = format!("${:.4}", session.total_cost_usd);
            let claude_count = session.claude_sessions.len();

            ListItem::new(format!(
                "{} | {} | {} Claude sessions | {}",
                session.id,
                session.workflow,
                claude_count,
                cost_str
            ))
        }).collect();

        let list = List::new(items)
            .block(Block::default().borders(Borders::ALL).title("Session History"))
            .highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_widget(list, area);
    }
}
```

**Keybinding**: Press `H` in worktree view to show session history

### 10.3 Diff Preview Widget

**New Widget**: `crates/rstn/src/tui/widgets/diff_preview.rs`

```rust
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use similar::TextDiff;

pub struct DiffPreview {
    old_content: String,
    new_content: String,
}

impl DiffPreview {
    pub fn new(old: String, new: String) -> Self {
        Self {
            old_content: old,
            new_content: new,
        }
    }

    pub fn render(&self) -> Paragraph {
        let diff = TextDiff::from_lines(&self.old_content, &self.new_content);

        let mut lines = vec![];
        for change in diff.iter_all_changes() {
            let (prefix, color) = match change.tag() {
                ChangeTag::Delete => ("-", Color::Red),
                ChangeTag::Insert => ("+", Color::Green),
                ChangeTag::Equal => (" ", Color::Gray),
            };

            lines.push(Line::from(vec![
                Span::styled(prefix, Style::default().fg(color)),
                Span::raw(" "),
                Span::styled(change.to_string(), Style::default().fg(color)),
            ]));
        }

        Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Diff Preview"))
            .wrap(Wrap { trim: false })
    }
}
```

**Usage**:
```rust
// In session output view
if edit_pending {
    let diff_widget = DiffPreview::new(old_content, new_content);
    frame.render_widget(diff_widget.render(), diff_area);
}
```

---

## Implementation Priority

**Phase 1** (High Priority):
1. ✅ Permission Modes (`--permission-mode plan`)
2. ✅ Extended ClaudeStreamMessage Structure
3. ✅ Dual-Layer Session Management

**Phase 2** (Medium Priority):
4. ✅ Multi-File Context (`--context` flag)
5. ✅ MCP Interaction in CLI Mode (Mini TUI)
6. ✅ Cancellation Support

**Phase 3** (Nice to Have):
7. ✅ Edit Approval with Diff Preview
8. ✅ Real-time Cost Tracking
9. ✅ MCP Error Handling
10. ✅ Additional Improvements (Session History, Tool Progress)

---

## Testing Strategy

### Unit Tests

```rust
// File: crates/rstn/tests/claude_stream_test.rs

#[test]
fn test_extended_tool_use_parsing() {
    let json = r#"{"type":"assistant","message":{"role":"assistant","content":[
        {"type":"tool_use","id":"toolu_01ABC","name":"Read","input":{"file_path":"/test.rs"}}
    ]}}"#;

    let msg: ClaudeStreamMessage = serde_json::from_str(json).unwrap();
    let content = &msg.message.unwrap().content[0];

    assert_eq!(content.content_type, "tool_use");
    assert_eq!(content.id, Some("toolu_01ABC".to_string()));
    assert_eq!(content.name, Some("Read".to_string()));
}

#[test]
fn test_session_manager_hierarchy() {
    let mut mgr = SessionManager::default();

    let rstn_id = mgr.start_rstn_session(WorkflowType::Specify);
    mgr.start_claude_session(&rstn_id, "uuid1".to_string(), "Generate".to_string(), 10).unwrap();
    mgr.start_claude_session(&rstn_id, "uuid2".to_string(), "Clarify".to_string(), 5).unwrap();

    let history = mgr.get_session_history(&rstn_id).unwrap();
    assert_eq!(history.len(), 2);
}
```

### Integration Tests

```bash
# Test CLI with --permission-mode
rstn prompt "test" --permission-mode plan --max-turns 1

# Test multi-file context
rstn prompt "Add feature" --context src/main.rs,src/lib.rs

# Test session management
rstn session history
rstn session show rstn-sess-20251221-001
```

---

## References

- [Claude CLI Reference](claude-code-cli-reference.md) - Complete CLI flags
- [Communication Channels](claude-code-communication.md) - stream-json, hooks, MCP
- [Prompt Command Architecture](../../kb/04-development/prompt-command-architecture.md) - Workflow diagrams
- [MCP Tools Reference](mcp-tools.md) - Tool schemas
- [Claude Headless Mode](claude-code-headless.md) - Headless patterns

---

## Changelog

- 2025-12-21: Initial version covering all 10 advanced features
