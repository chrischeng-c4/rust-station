---
title: "MCP Interaction in CLI Mode"
description: "Mini TUI mode for interactive MCP prompts in CLI mode"
category: reference
status: implemented
last_updated: 2025-12-21
version: 0.2.0
phase: "065"
tags: [claude-code, mcp, cli, mini-tui]
weight: 3
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

