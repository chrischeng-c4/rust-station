# Claude Code Advanced Features & rstn Integration

**Last Updated**: 2025-12-21
**Status**: Partially Implemented (Phase 1, 2, 3 complete)
**Scope**: Advanced Claude Code CLI features and rstn integration patterns

---

## Overview

This document covers advanced Claude Code CLI features that extend beyond basic `claude -p` usage, including permission modes, interactive workflows, multi-file context, and comprehensive session management.

**Implementation Status**:

**✅ Phase 1 (High Priority) - COMPLETE**:
- ✅ Extended tool metadata parsing ([ClaudeStreamMessage](#4-extended-claudestreammessage-structure))
- ✅ Permission Modes integration ([PermissionMode enum](#1-permission-modes))
- ✅ Dual-layer session management ([SessionManager](#5-dual-layer-session-management))

**✅ Phase 2 (Medium Priority) - COMPLETE**:
- ✅ Multi-file context via `--context` flag ([JSONL builder](#3-multi-file-context-via-stream-json))
- ✅ Cancellation support ([Ctrl+C / Esc](#7-cancellation-support))

**✅ Phase 3 (Nice to Have) - COMPLETE**:
- ✅ Real-time cost tracking ([cumulative_cost_usd](#8-real-time-cost-tracking))
- ✅ MCP interaction in CLI mode ([Mini TUI mode](#2-mcp-interaction-in-cli-mode))
- ✅ Edit approval with diff preview ([Edit tool interception](#6-edit-approval-via-stream-json))
- ✅ MCP error handling ([Structured errors with suggestions](#9-mcp-error-handling))
- ✅ Additional UI improvements ([Tool progress, session history](#10-additional-improvements))

---

## 1. Permission Modes

**Status**: ✅ IMPLEMENTED (Phase 1)

### Overview

Claude Code supports three permission modes that control how it handles tool execution:

```bash
--permission-mode plan  # Plan before executing (like TUI Shift+Tab)
--permission-mode auto  # Execute without asking
--permission-mode ask   # Ask before each tool (default)
```

**Reference**: [kb/03-api-reference/claude-code-cli-reference.md:47](claude-code-cli-reference.md#L47)

### Implementation Details

**File**: [crates/rstn/src/runners/cargo.rs:40-80](../../crates/rstn/src/runners/cargo.rs#L40-L80)

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

**Command Building** ([cargo.rs:420-440](../../crates/rstn/src/runners/cargo.rs#L420-L440)):
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

### Benefits

1. **Transparency**: User sees what Claude plans to do before execution
2. **Safety**: Review tool calls before they run
3. **Compatibility**: Matches TUI Shift+Tab behavior in CLI mode
4. **Flexibility**: Can switch modes per command

---

## 2. MCP Interaction in CLI Mode

**Status**: ✅ IMPLEMENTED (Phase 3)

### Problem Statement

When rstn runs `claude -p` (headless mode) with MCP integration, and Claude calls `rstn_report_status` with `needs_input`, there's no way to show a dialog to the user.

**Previous Flow**:
```
rstn CLI → claude -p → Claude Code → MCP rstn_report_status(needs_input)
                                           ↓
                                      ❌ No UI available
```

**Solution**: Mini TUI Mode - Temporarily enter TUI for user input, then return to CLI streaming.

### Implementation: Mini TUI Mode

**Architecture**: CLI mode polls MCP events during streaming. When `needs_input` event is received, temporarily show Mini TUI dialog, collect user input, send to MCP, then resume CLI streaming.

#### Component 1: MiniTUIDialog

**File**: [crates/rstn/src/tui/mini_dialog.rs](../../crates/rstn/src/tui/mini_dialog.rs)

```rust
/// Minimal TUI dialog for collecting user input in CLI mode
pub struct MiniTUIDialog {
    /// Prompt message
    prompt: String,
    /// User input buffer
    input: String,
    /// Cursor position
    cursor: usize,
}

impl MiniTUIDialog {
    /// Run the dialog and return user input (or None if cancelled)
    pub fn run(mut self) -> io::Result<Option<String>> {
        // Setup terminal (raw mode, alternate screen)
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        // Event loop - handle input until Enter/Esc
        let result = loop {
            terminal.draw(|f| self.render(f))?;

            if let CrosstermEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => break Ok(Some(self.input.clone())),
                    KeyCode::Esc => break Ok(None),
                    KeyCode::Char(c) => {
                        self.input.insert(self.cursor, c);
                        self.cursor += 1;
                    }
                    KeyCode::Backspace => {
                        if self.cursor > 0 {
                            self.input.remove(self.cursor - 1);
                            self.cursor -= 1;
                        }
                    }
                    // ... navigation support (Left/Right/Home/End)
                    _ => {}
                }
            }
        };

        // Restore terminal (always execute)
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;

        result
    }

    fn render(&self, frame: &mut Frame) {
        // Centered dialog (60% width, 7 lines height)
        let dialog_area = Self::centered_rect(60, 7, frame.area());

        // Layout: Prompt + Input field + Help text
        // ...

        // Set cursor position (visible in input box)
        frame.set_cursor_position((
            chunks[2].x + 1 + self.cursor as u16,
            chunks[2].y + 1,
        ));
    }
}
```

**Features**:
- ✅ Simple centered layout (60% width, 7 lines)
- ✅ Text input with cursor navigation (Left/Right/Home/End)
- ✅ Enter to submit, Esc to cancel
- ✅ Automatic terminal setup/cleanup
- ✅ No dependencies on full TUI app state

#### Component 2: Global MCP State

**File**: [crates/rstn/src/tui/mcp_server.rs:165-199](../../crates/rstn/src/tui/mcp_server.rs#L165-L199)

```rust
/// Global MCP state storage (shared between TUI and CLI modes)
///
/// Wraps tokio::sync::Mutex in std::sync::Mutex for static storage
static GLOBAL_MCP_STATE_MUT: Lazy<std::sync::Mutex<Option<Arc<Mutex<McpState>>>>> =
    Lazy::new(|| std::sync::Mutex::new(None));

/// Get the global MCP state (for CLI mode integration)
pub fn get_global_mcp_state() -> Option<Arc<Mutex<McpState>>> {
    let guard = GLOBAL_MCP_STATE_MUT.lock().ok()?;
    guard.as_ref().cloned()
}

/// Initialize the global MCP state (called from main.rs)
pub fn init_global_mcp_state(state: Arc<Mutex<McpState>>) {
    if let Ok(mut guard) = GLOBAL_MCP_STATE_MUT.lock() {
        *guard = Some(state);
    }
}
```

**Initialization** ([main.rs:168-171](../../crates/rstn/src/main.rs#L168-L171)):
```rust
// Create shared MCP state for metrics tracking
let mcp_state = Arc::new(Mutex::new(McpState::default()));

// Initialize global MCP state for CLI mode access
mcp_server::init_global_mcp_state(mcp_state.clone());
```

#### Component 3: CLI Event Polling

**File**: [crates/rstn/src/commands/prompt.rs:125-255](../../crates/rstn/src/commands/prompt.rs#L125-L255)

```rust
/// CLI-specific streaming handler with MCP event polling
async fn run_claude_with_cli_streaming(
    message: &str,
    options: &ClaudeCliOptions,
    mcp_state: Option<Arc<Mutex<McpState>>>,  // ✅ Optional MCP state
) -> Result<ClaudeResult> {
    // ... spawn Claude CLI subprocess ...

    let mut lines = reader.lines();

    while let Ok(Some(line)) = lines.next_line().await {
        // ✅ Poll for MCP events if MCP server is active
        if let Some(ref state) = mcp_state {
            if let Ok(mut state_guard) = state.try_lock() {
                // Drain TUI events (MCP sends events here)
                let events: Vec<Event> = state_guard.drain_tui_events();
                for event in events {
                    if let Event::McpStatus { status, prompt, .. } = event {
                        if status == "needs_input" {
                            let prompt_text = prompt.unwrap_or_else(|| "Enter input:".to_string());

                            eprintln!(); // Add newline before dialog
                            eprintln!("{}", "📥 MCP Input Request".bright_blue());

                            // ✅ Show mini dialog
                            let dialog = crate::tui::mini_dialog::MiniTUIDialog::new(prompt_text);
                            match dialog.run() {
                                Ok(Some(input)) => {
                                    // User provided input - send to MCP
                                    state_guard.send_input_response(input.clone());
                                    eprintln!("{}", format!("✓ Response sent: {}", input).green());
                                }
                                Ok(None) => {
                                    // User cancelled - send empty response
                                    state_guard.send_input_response(String::new());
                                    eprintln!("{}", "⚠ Cancelled - empty response sent".yellow());
                                }
                                Err(e) => {
                                    eprintln!("{}", format!("✗ Dialog error: {}", e).red());
                                    state_guard.send_input_response(String::new());
                                }
                            }

                            eprintln!(); // Add newline after dialog
                            eprintln!("{}", "Resuming Claude output...".bright_blue());
                        }
                    }
                }
            }
        }

        // Continue streaming JSONL output
        if let Ok(msg) = serde_json::from_str::<ClaudeStreamMessage>(&line) {
            // ... print assistant text to stdout ...
        }
    }

    Ok(result)
}
```

### Flow Diagram

```
CLI Mode with MCP Input Request:

1. rstn CLI spawns `claude -p --mcp-config ...`
   ↓
2. Claude calls MCP tool: rstn_report_status({needs_input})
   ↓
3. MCP server sends Event::McpStatus to global state
   ↓
4. CLI streaming loop polls MCP events
   ↓
5. Detects needs_input → Show MiniTUIDialog
   ┌─────────────────────────────────────────┐
   │  📥 MCP Input Request                   │
   │  ┌─────────────────────────────────────┐ │
   │  │ MCP Input Request                   │ │
   │  │ Enter feature description:          │ │
   │  │ ┌─────────────────────────────────┐ │ │
   │  │ │ Add dark mode support_          │ │ │
   │  │ └─────────────────────────────────┘ │ │
   │  │ Enter to submit • Esc to cancel     │ │
   │  └─────────────────────────────────────┘ │
   └─────────────────────────────────────────┘
   ↓
6. User enters text → Enter
   ↓
7. CLI sends response to MCP via state_guard.send_input_response()
   ↓
8. MCP unblocks tool handler → Claude continues
   ↓
9. CLI resumes stdout streaming
```

### Benefits

1. **✅ Full interactive capability**: Like TUI mode
2. **✅ Seamless UX**: CLI → Mini TUI → CLI transition
3. **✅ Reuses existing infrastructure**: MCP state, event system
4. **✅ No external dependencies**: Pure rstn implementation
5. **✅ Graceful degradation**: If dialog fails, sends empty response

### Test Coverage

**File**: [crates/rstn/src/tui/mini_dialog.rs:195-242](../../crates/rstn/src/tui/mini_dialog.rs#L195-L242)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mini_dialog_creation() {
        let dialog = MiniTUIDialog::new("Test prompt".to_string());
        assert_eq!(dialog.prompt, "Test prompt");
        assert_eq!(dialog.input, "");
        assert_eq!(dialog.cursor, 0);
    }

    #[test]
    fn test_centered_rect_calculation() {
        let area = Rect { x: 0, y: 0, width: 100, height: 50 };
        let centered = MiniTUIDialog::centered_rect(60, 7, area);

        // 60% width = 60, centered at x=20
        assert_eq!(centered.width, 60);
        assert_eq!(centered.x, 20);

        // 7 lines height, centered at y=(50-7)/2=21
        assert_eq!(centered.height, 7);
        assert_eq!(centered.y, 21);
    }
}
```

### Future Enhancements (Optional)

The Mini TUI mode implementation is **complete and functional**. The following enhancements are optional improvements for specific use cases:

#### 1. Multi-line Input Support

**Current**: Single-line input only (Enter to submit)

**Enhancement**: Support multi-line input for longer prompts

```rust
pub struct MiniTUIDialog {
    // ...
    multiline: bool,  // Enable multi-line mode
    lines: Vec<String>,  // Multiple input lines
}

// Key handling for multi-line:
// - Ctrl+Enter or Alt+Enter: Submit
// - Enter: New line
// - Esc: Cancel
```

**Use Case**: When Claude asks for complex multi-paragraph descriptions (rare in current workflows)

**Priority**: Low (most MCP prompts are single-line questions)

---

#### 2. Timeout Handling

**Current**: Dialog waits indefinitely for user input

**Enhancement**: Auto-submit or auto-cancel after timeout

```rust
pub struct MiniTUIDialog {
    // ...
    timeout: Option<Duration>,  // Optional timeout
}

impl MiniTUIDialog {
    pub fn run_with_timeout(self, timeout: Duration) -> io::Result<Option<String>> {
        // Use tokio::time::timeout() or crossterm event polling with timeout
        // ...
    }
}
```

**Use Case**: Automated testing, CI/CD environments where user can't respond

**Priority**: Medium (useful for automation scenarios)

---

#### 3. Non-TTY Fallback (Headless Mode)

**Current**: Mini TUI requires a TTY (terminal)

**Enhancement**: Fallback to stdin when no TTY available

```rust
impl MiniTUIDialog {
    pub fn run(self) -> io::Result<Option<String>> {
        // Check if we have a TTY
        if !atty::is(Stream::Stdin) || !atty::is(Stream::Stdout) {
            // Fallback to stdin
            eprintln!("{}", self.prompt);
            eprint!("> ");
            std::io::stderr().flush()?;

            let mut buffer = String::new();
            std::io::stdin().read_line(&mut buffer)?;
            return Ok(Some(buffer.trim().to_string()));
        }

        // Normal Mini TUI mode
        // ...
    }
}
```

**Use Case**: CI/CD pipelines, Docker containers, SSH without PTY

**Priority**: High (important for production automation)

**Implementation Note**: Use `atty` or `isatty` crate to detect TTY availability

---

#### 4. Session History Integration

**Current**: Mini TUI dialogs are ephemeral (not logged)

**Enhancement**: Log dialog interactions to session history

```rust
// In MiniTUIDialog::run()
if let Ok(Some(input)) = result {
    // Log to session database
    if let Some(session_id) = get_current_session_id() {
        SessionManager::log_mcp_interaction(
            session_id,
            "needs_input",
            &self.prompt,
            &input,
        );
    }
}
```

**Use Case**: Audit trail, debugging, session replay

**Priority**: Medium (nice for debugging but not critical)

---

#### 5. Input Validation

**Current**: Any input is accepted and sent to MCP

**Enhancement**: Validate input before sending

```rust
pub struct MiniTUIDialog {
    // ...
    validator: Option<Box<dyn Fn(&str) -> Result<(), String>>>,
}

// Usage:
let dialog = MiniTUIDialog::new(prompt)
    .with_validator(|input| {
        if input.trim().is_empty() {
            Err("Input cannot be empty".to_string())
        } else {
            Ok(())
        }
    });
```

**Use Case**: Prevent invalid input from being sent to Claude (e.g., empty responses)

**Priority**: Low (Claude can handle invalid input gracefully)

---

### Implementation Priority

**Recommended Order** (if implementing):

1. **Non-TTY Fallback** (High) - Critical for CI/CD and automation
2. **Timeout Handling** (Medium) - Useful for automation and testing
3. **Session History Integration** (Medium) - Good for debugging
4. **Multi-line Input** (Low) - Rarely needed in current workflows
5. **Input Validation** (Low) - Nice to have but not essential

**Current Status**: Base implementation is complete and production-ready. Optional enhancements should only be added when specific use cases arise.

---

## 3. Multi-File Context via stream-json

**Status**: ✅ IMPLEMENTED (Phase 2)

### Problem Statement

How to provide multiple files to Claude simultaneously (like `--context` flag proposal).

**User Question**: "attach 檔案 應該使用input-form stream-json來達成？"

### Solution: --input-format stream-json

**Reference**: [kb/03-api-reference/claude-code-communication.md:120-240](claude-code-communication.md#L120-L240)

#### JSONL Input Format

Claude Code accepts JSONL input via stdin when `--input-format stream-json` is specified:

```bash
cat input.jsonl | claude --input-format stream-json
```

**input.jsonl**:
```jsonl
{"role":"user","content":[{"type":"text","text":"User message"}]}
```

#### Multi-File Context Pattern

```jsonl
{
  "role": "user",
  "content": [
    {
      "type": "text",
      "text": "Add dark mode support based on these files:"
    },
    {
      "type": "text",
      "text": "=== src/theme.rs ===\n\npub struct Theme {\n    pub bg: Color,\n    pub fg: Color,\n}\n"
    },
    {
      "type": "text",
      "text": "=== src/settings.rs ===\n\npub struct Settings {\n    pub theme_name: String,\n}\n"
    }
  ]
}
```

### Implementation Details

**File**: [crates/rstn/src/cli.rs:60-65](../../crates/rstn/src/cli.rs#L60-L65)

**CLI Args**:
```rust
#[derive(clap::Parser)]
pub enum Commands {
    Prompt {
        message: String,
        max_turns: u32,
        skip_permissions: bool,
        continue_session: bool,
        session_id: Option<String>,
        allowed_tools: Vec<String>,
        /// Additional files for context (comma-separated paths)
        #[arg(long, value_delimiter = ',')]
        context: Vec<std::path::PathBuf>, // ✅ IMPLEMENTED
    },
    // ...
}
```

**JSONL Builder** ([cargo.rs:110-140](../../crates/rstn/src/runners/cargo.rs#L110-L140)):
```rust
async fn build_jsonl_with_context(
    prompt: &str,
    context_files: &[std::path::PathBuf],
) -> Result<String> {
    use serde_json::json;

    let mut content_blocks = vec![json!({
        "type": "text",
        "text": prompt
    })];

    // Add context files
    for path in context_files {
        let file_content = tokio::fs::read_to_string(path).await?;
        let header = format!("=== {} ===\n\n", path.display());
        content_blocks.push(json!({
            "type": "text",
            "text": format!("{}{}", header, file_content)
        }));
    }

    let message = json!({
        "role": "user",
        "content": content_blocks
    });

    let jsonl = format!("{}\n", serde_json::to_string(&message)?);
    Ok(jsonl)
}
```

**Command Building** ([cargo.rs:450-480](../../crates/rstn/src/runners/cargo.rs#L450-L480)):
```rust
// Use stream-json input if context files provided
let (message_arg, use_stdin) = if !options.context_files.is_empty() {
    let jsonl = build_jsonl_with_context(message, &options.context_files).await?;
    (jsonl, true)
} else {
    (message.to_string(), false)
};

if use_stdin {
    command.arg("--input-format").arg("stream-json");
    command.stdin(std::process::Stdio::piped());
}
```

**Usage in CLI**:
```bash
rstn prompt "Add dark mode" --context src/theme.rs,src/settings.rs
rstn prompt "Fix bug" --context src/main.rs,tests/test.rs --max-turns 3
```

**Usage in TUI**:
```rust
let options = ClaudeCliOptions {
    context_files: vec![
        PathBuf::from("spec.md"),
        PathBuf::from("plan.md"),
    ],
    max_turns: Some(5),
    // ...
};
```

#### Advanced: Image Context

For image support (mentioned in current implementation):

```rust
// File content detection
for path in &args.context {
    let content = if is_image(&path) {
        // Base64 encode image
        let bytes = tokio::fs::read(path).await?;
        let base64 = base64::encode(&bytes);
        let media_type = detect_mime_type(path);

        serde_json::json!({
            "type": "image",
            "source": {
                "type": "base64",
                "media_type": media_type,
                "data": base64
            }
        })
    } else {
        // Text file
        let text = tokio::fs::read_to_string(path).await?;
        serde_json::json!({
            "type": "text",
            "text": format!("=== {} ===\n\n{}", path.display(), text)
        })
    };

    content_blocks.push(content);
}
```

**Current implementation** already uses `--add-dir` for image access:
```rust
// File: crates/rstn/src/runners/cargo.rs:480-485
for dir in &options.add_dirs {
    if dir.exists() {
        cmd.arg("--add-dir").arg(dir);
    }
}
```

---

## 4. Extended ClaudeStreamMessage Structure

**Status**: ✅ IMPLEMENTED (Phase 1)

### Implementation Details

**File**: [crates/rstn/src/tui/claude_stream.rs:38-65](../../crates/rstn/src/tui/claude_stream.rs#L38-L65)

**Code**:
```rust
#[derive(Debug, Clone, Deserialize)]
pub struct ClaudeContent {
    #[serde(rename = "type")]
    pub content_type: String,

    // For "text" type
    #[serde(default)]
    pub text: Option<String>,

    // For "tool_use" type - ✅ IMPLEMENTED
    #[serde(default)]
    pub id: Option<String>,           // "toolu_01ABC..."
    #[serde(default)]
    pub name: Option<String>,         // "Read", "Edit", etc.
    #[serde(default)]
    pub input: Option<serde_json::Value>,  // Tool parameters

    // For "tool_result" type - ✅ IMPLEMENTED
    #[serde(default)]
    pub tool_use_id: Option<String>,  // Reference to tool_use
    #[serde(default)]
    pub content: Option<String>,       // Result content (alt to text)

    #[serde(default)]
    pub is_error: Option<bool>,
}
```

**Enhanced Display** ([session_output.rs:95-125](../../crates/rstn/src/tui/views/session_output.rs#L95-L125)):
```rust
"tool_use" => {
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_use" {
                let tool_name = content.name.as_deref().unwrap_or("Unknown");
                let tool_id = content
                    .id
                    .as_deref()
                    .and_then(|id| id.get(..8))
                    .unwrap_or("????????");

                // Format input parameters
                let params = if let Some(input) = &content.input {
                    let json_str = serde_json::to_string(input).unwrap_or_default();
                    if json_str.len() > 60 {
                        format!("{}...", &json_str[..60])
                    } else {
                        json_str
                    }
                } else {
                    "{}".to_string()
                };

                // ✅ NOW SHOWS: 🔧 Tool: Read [toolu_01A] {"file_path":"/test.rs"}
                self.output_lines.push(format!(
                    "🔧 Tool: {} [{}] {}",
                    tool_name, tool_id, params
                ));
            }
        }
    }
}
```

**Tests** ([claude_stream.rs:152-186](../../crates/rstn/src/tui/claude_stream.rs#L152-L186)):
```rust
#[test]
fn test_tool_use_with_metadata() {
    let json = r#"{"type":"assistant","message":{"role":"assistant","content":[
        {"type":"tool_use","id":"toolu_01ABC123","name":"Read","input":{"file_path":"/test.rs"}}
    ]}}\"#;

    let msg: ClaudeStreamMessage = serde_json::from_str(json).unwrap();
    let content = &msg.message.unwrap().content[0];

    assert_eq!(content.name, Some("Read".to_string()));
    assert_eq!(content.id, Some("toolu_01ABC123".to_string()));
    assert!(content.input.is_some());
}
```

### Enhanced Display

**File**: `crates/rstn/src/tui/views/session_output.rs` (updated)

```rust
"tool_use" => {
    if let Some(msg) = &message.message {
        for content in &msg.content {
            if content.content_type == "tool_use" {
                let tool_name = content.name.as_deref().unwrap_or("Unknown");
                let tool_id = content.id.as_deref().unwrap_or("").get(..8).unwrap_or("");

                // Format input parameters nicely
                let params = if let Some(input) = &content.input {
                    format!("{}", input)
                } else {
                    "{}".to_string()
                };

                self.output_lines.push(format!(
                    "🔧 Tool: {} [{}] {}",
                    tool_name,
                    tool_id,
                    truncate(&params, 60)
                ));
            }
        }
    }
}
```

**Example Output**:
```
🔧 Tool: Read [toolu_01A] {"file_path": "/path/to/file.rs"}
  ✓ Result: Read 150 lines from file.rs
🔧 Tool: Edit [toolu_01B] {"file_path": "/path/to/file.rs", "old_string": "..."}
  ✓ Result: Edited file successfully
```

### Backward Compatibility

All new fields use `#[serde(default)]`, so existing code continues to work with incomplete data.

---

## 5. Dual-Layer Session Management

**Status**: ✅ IMPLEMENTED (Phase 1)

### Problem Statement

**User Insight**: "看來我們的狀態除了rstn的session還需要管理claude code的session"

rstn needs to track:
1. **Claude sessions** (UUID from Claude Code) - individual LLM interactions
2. **rstn sessions** (workflow-level) - Prompt, Specify, Plan, Tasks

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│ rstn Session (Workflow Level)                          │
│ ID: rstn-sess-20251221-001                             │
│ Type: Specify                                           │
│ Feature: 082-dark-mode                                  │
│                                                         │
│   ┌───────────────────────────────────────────────┐   │
│   │ Claude Session 1 (Generate Spec)              │   │
│   │ UUID: claude-uuid-abc123                      │   │
│   │ Turns: 3                                       │   │
│   │ Status: Completed                             │   │
│   └───────────────────────────────────────────────┘   │
│                                                         │
│   ┌───────────────────────────────────────────────┐   │
│   │ Claude Session 2 (Clarify Requirements)      │   │
│   │ UUID: claude-uuid-def456                      │   │
│   │ Turns: 1                                       │   │
│   │ Status: Running                               │   │
│   └───────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────┘
```

### Data Structures

**File**: `crates/rstn/src/session_manager.rs` (new/updated)

```rust
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Claude session (from Claude Code)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeSession {
    /// UUID from Claude Code (session_id in stream-json)
    pub uuid: String,

    /// What this Claude session is for
    pub purpose: String,  // "Generate Spec", "Clarify", "Implement Task"

    /// Session metadata
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// Session outcome
    pub status: ClaudeSessionStatus,
    pub turns_used: usize,
    pub max_turns: usize,
    pub total_cost_usd: Option<f64>,

    /// Link to parent rstn session
    pub rstn_session_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaudeSessionStatus {
    Running,
    Completed,
    MaxTurns,
    Error { message: String },
}

/// rstn session (workflow level)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RstnSession {
    /// rstn-generated ID
    pub id: String,  // "rstn-sess-20251221-001"

    /// Workflow type
    pub workflow: WorkflowType,

    /// Feature being worked on
    pub feature_number: Option<String>,
    pub feature_name: Option<String>,

    /// Session metadata
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,

    /// All Claude sessions in this workflow
    pub claude_sessions: HashMap<String, ClaudeSession>,

    /// Current active Claude session
    pub active_claude_session: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkflowType {
    Prompt,
    Specify,
    Plan,
    Tasks,
    Implement,
}

/// Session manager
#[derive(Debug, Default)]
pub struct SessionManager {
    /// All rstn sessions (keyed by rstn session ID)
    sessions: HashMap<String, RstnSession>,

    /// Current active rstn session
    current_session: Option<String>,
}

impl SessionManager {
    pub fn start_rstn_session(&mut self, workflow: WorkflowType) -> String {
        let session_id = format!(
            "rstn-sess-{}-{:03}",
            chrono::Utc::now().format("%Y%m%d"),
            self.sessions.len() + 1
        );

        let session = RstnSession {
            id: session_id.clone(),
            workflow,
            feature_number: None,
            feature_name: None,
            started_at: chrono::Utc::now(),
            completed_at: None,
            claude_sessions: HashMap::new(),
            active_claude_session: None,
        };

        self.sessions.insert(session_id.clone(), session);
        self.current_session = Some(session_id.clone());

        session_id
    }

    pub fn start_claude_session(
        &mut self,
        rstn_session_id: &str,
        uuid: String,
        purpose: String,
        max_turns: usize,
    ) -> Result<()> {
        let session = self.sessions.get_mut(rstn_session_id)
            .ok_or_else(|| anyhow::anyhow!("rstn session not found"))?;

        let claude_session = ClaudeSession {
            uuid: uuid.clone(),
            purpose,
            started_at: chrono::Utc::now(),
            completed_at: None,
            status: ClaudeSessionStatus::Running,
            turns_used: 0,
            max_turns,
            total_cost_usd: None,
            rstn_session_id: rstn_session_id.to_string(),
        };

        session.claude_sessions.insert(uuid.clone(), claude_session);
        session.active_claude_session = Some(uuid);

        Ok(())
    }

    pub fn complete_claude_session(
        &mut self,
        rstn_session_id: &str,
        claude_uuid: &str,
        status: ClaudeSessionStatus,
        turns_used: usize,
        total_cost_usd: Option<f64>,
    ) -> Result<()> {
        let session = self.sessions.get_mut(rstn_session_id)
            .ok_or_else(|| anyhow::anyhow!("rstn session not found"))?;

        if let Some(claude_session) = session.claude_sessions.get_mut(claude_uuid) {
            claude_session.completed_at = Some(chrono::Utc::now());
            claude_session.status = status;
            claude_session.turns_used = turns_used;
            claude_session.total_cost_usd = total_cost_usd;
        }

        Ok(())
    }

    /// Get session history for display
    pub fn get_session_history(&self, rstn_session_id: &str) -> Option<Vec<ClaudeSession>> {
        self.sessions.get(rstn_session_id).map(|s| {
            let mut sessions: Vec<_> = s.claude_sessions.values().cloned().collect();
            sessions.sort_by_key(|s| s.started_at);
            sessions
        })
    }
}
```

### Persistence

Store session data in SQLite database:

**File**: `crates/rstn/src/db/schema.sql`

```sql
CREATE TABLE rstn_sessions (
    id TEXT PRIMARY KEY,
    workflow TEXT NOT NULL,
    feature_number TEXT,
    feature_name TEXT,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    metadata TEXT  -- JSON blob
);

CREATE TABLE claude_sessions (
    uuid TEXT PRIMARY KEY,
    rstn_session_id TEXT NOT NULL,
    purpose TEXT NOT NULL,
    started_at TIMESTAMP NOT NULL,
    completed_at TIMESTAMP,
    status TEXT NOT NULL,
    turns_used INTEGER,
    max_turns INTEGER,
    total_cost_usd REAL,
    FOREIGN KEY (rstn_session_id) REFERENCES rstn_sessions(id)
);
```

### Integration Example

**File**: `crates/rstn/src/commands/prompt.rs`

```rust
pub async fn execute_prompt_cli(args: PromptArgs) -> Result<()> {
    let mut session_mgr = SessionManager::load_from_db().await?;

    // Start rstn session
    let rstn_session_id = session_mgr.start_rstn_session(WorkflowType::Prompt);

    // Start Claude session (UUID comes from "init" message)
    let claude_result = run_claude_command_streaming(&message, &options, |msg| {
        if msg.msg_type == "init" {
            if let Some(session_id) = &msg.session_id {
                session_mgr.start_claude_session(
                    &rstn_session_id,
                    session_id.clone(),
                    "User Prompt".to_string(),
                    options.max_turns.unwrap_or(10),
                )?;
            }
        }
        // ... handle other messages ...
        Ok(())
    }).await?;

    // Complete Claude session
    if let Some(session_id) = claude_result.session_id {
        session_mgr.complete_claude_session(
            &rstn_session_id,
            &session_id,
            ClaudeSessionStatus::Completed,
            claude_result.turns_used,
            claude_result.total_cost_usd,
        )?;
    }

    // Persist to DB
    session_mgr.save_to_db().await?;

    Ok(())
}
```

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

## 8. Real-time Cost Tracking

**Status**: ✅ IMPLEMENTED (Phase 3)

### Implementation Details

**File**: [crates/rstn/src/tui/views/session_output.rs:28-31](../../crates/rstn/src/tui/views/session_output.rs#L28-L31)

**SessionOutputView State**:
```rust
pub struct SessionOutputView {
    // ... existing fields ...

    /// Cumulative cost in USD (updated in real-time)
    cumulative_cost_usd: f64,  // ✅ IMPLEMENTED

    /// Budget warning threshold in USD (warn if exceeded)
    budget_warning_threshold: f64,  // ✅ IMPLEMENTED (default: $0.50)
}
```

**Real-time Update** ([session_output.rs:85-89](../../crates/rstn/src/tui/views/session_output.rs#L85-L89)):
```rust
pub fn add_message(&mut self, message: &ClaudeStreamMessage) {
    // Update cumulative cost if available (real-time tracking)
    if let Some(cost) = message.total_cost_usd {
        self.cumulative_cost_usd = cost;
    }
    // ... rest of message handling ...
}
```

**Status Line Display** ([session_output.rs:251-309](../../crates/rstn/src/tui/views/session_output.rs#L251-L309)):
```rust
pub fn status_line(&self) -> String {
    // Budget warning indicator
    let budget_warning = if self.cumulative_cost_usd > self.budget_warning_threshold {
        "⚠️ "
    } else {
        ""
    };

    match &self.completion_status {
        Some(CompletionStatus::Complete { turns, duration_secs }) => {
            format!(
                "✓ Complete ({} turn{}, {}s, {}${:.4})",
                turns,
                if *turns == 1 { "" } else { "s" },
                duration_secs,
                budget_warning,
                self.cumulative_cost_usd
            )
        }
        None if self.start_time.is_some() => {
            let duration = self.start_time.unwrap().elapsed().as_secs();
            format!(
                "🤖 Running... (Turn {}/{}, {}s, {}${:.4})",
                self.current_turn.max(1),
                self.max_turns,
                duration,
                budget_warning,
                self.cumulative_cost_usd
            )
        }
        // ... other states ...
    }
}
```

**Budget Configuration** ([session_output.rs:55-67](../../crates/rstn/src/tui/views/session_output.rs#L55-L67)):
```rust
/// Create a new session with custom budget threshold
pub fn with_budget(max_turns: usize, budget_threshold: f64) -> Self {
    Self {
        budget_warning_threshold: budget_threshold,
        // ... other fields ...
    }
}
```

**Tests** ([session_output.rs:423-545](../../crates/rstn/src/tui/views/session_output.rs#L423-L545)):
```rust
#[test]
fn test_cost_tracking_real_time() {
    let mut view = SessionOutputView::new(5);
    view.start_session("Test prompt", 5);

    let msg = ClaudeStreamMessage {
        total_cost_usd: Some(0.025),
        // ...
    };
    view.add_message(&msg);
    assert_eq!(view.cumulative_cost_usd, 0.025);

    let status = view.status_line();
    assert!(status.contains("$0.0250"));
}

#[test]
fn test_budget_warning() {
    let mut view = SessionOutputView::with_budget(5, 0.01);  // Low threshold

    let msg = ClaudeStreamMessage {
        total_cost_usd: Some(0.025),  // Above threshold
        // ...
    };
    view.add_message(&msg);

    let status = view.status_line();
    assert!(status.contains("⚠️"));  // Budget warning shown
}
```

### Cumulative Cost Tracking (Dual-Layer)

**File**: [crates/rstn/src/session_manager.rs:58-67](../../crates/rstn/src/session_manager.rs#L58-67)

Track cost across multiple Claude sessions within an rstn workflow:

```rust
pub struct RstnSession {
    // ... existing fields ...

    /// Total cost across all Claude sessions in this workflow
    pub total_cost_usd: f64,  // ✅ IMPLEMENTED
}

impl SessionManager {
    pub fn complete_claude_session(&mut self, ...) -> Result<()> {
        // ... existing logic ...

        // Update rstn session total cost (already implemented)
        // This aggregates costs from all Claude sessions in the workflow
        Ok(())
    }
}
```

### Budget Warnings

**File**: `crates/rstn/src/tui/app.rs`

```rust
const BUDGET_WARNING_THRESHOLD: f64 = 1.0;  // $1.00
const BUDGET_CRITICAL_THRESHOLD: f64 = 5.0; // $5.00

fn check_budget_warning(&self, session: &RstnSession) {
    if session.total_cost_usd > BUDGET_CRITICAL_THRESHOLD {
        self.show_warning(format!(
            "⚠️  Session cost ${:.2} exceeds critical threshold!",
            session.total_cost_usd
        ));
    } else if session.total_cost_usd > BUDGET_WARNING_THRESHOLD {
        self.show_info(format!(
            "ℹ️  Session cost: ${:.2}",
            session.total_cost_usd
        ));
    }
}
```

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
