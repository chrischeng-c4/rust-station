# Mini TUI Mode Architecture Design

**Feature**: MCP Interaction in CLI Mode
**Status**: Design Phase
**Date**: 2025-12-21

---

## Problem Statement

When running rstn in CLI mode (`rstn prompt "message"`), the subprocess can invoke MCP tools that request user input via `rstn_report_status({status: "needs_input"})`.

**Current Behavior**:
- CLI mode spawns Claude CLI subprocess
- Claude CLI connects to rstn's MCP server (HTTP endpoint)
- MCP server receives `needs_input` tool call
- MCP server sends `Event::McpStatus` to event loop
- **Problem**: CLI mode has no event loop, so event is never processed
- **Result**: MCP server blocks indefinitely waiting for response

**User Impact**: CLI mode cannot handle interactive workflows that need user input mid-execution.

---

## Solution: Mini TUI Mode

Allow CLI mode to **temporarily enter a minimal TUI** when MCP needs input, then return to CLI streaming.

### High-Level Flow

```
CLI Mode (stdout streaming)
    │
    ├─► Spawn Claude CLI subprocess
    │
    ├─► Claude calls MCP tool: rstn_report_status({needs_input})
    │
    ├─► MCP server sends Event::McpStatus
    │
    ├─► CLI detects event via polling
    │
    ├─► Switch to Mini TUI Mode
    │   │
    │   ├─► Show minimal input dialog
    │   │
    │   ├─► User enters response
    │   │
    │   └─► Send response to MCP server
    │
    └─► Switch back to CLI Mode
        │
        └─► Continue stdout streaming
```

---

## Architecture Components

### 1. InputMode Enum

**File**: `crates/rstn/src/commands/prompt.rs`

```rust
/// CLI execution mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputMode {
    /// Pure CLI mode (stdout streaming)
    CLI,
    /// Temporary TUI mode for input dialog
    MiniTUI,
}
```

**Purpose**: Track whether CLI is currently streaming or showing input dialog.

---

### 2. MiniTUIDialog Component

**File**: `crates/rstn/src/tui/mini_dialog.rs` (NEW)

```rust
use crossterm::event::{self, Event as CrosstermEvent, KeyCode, KeyEvent};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame, Terminal,
};
use std::io::Stdout;

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
    /// Create a new mini dialog
    pub fn new(prompt: String) -> Self {
        Self {
            prompt,
            input: String::new(),
            cursor: 0,
        }
    }

    /// Run the dialog and return user input (or None if cancelled)
    pub fn run(mut self) -> Result<Option<String>, std::io::Error> {
        // Setup terminal
        crossterm::terminal::enable_raw_mode()?;
        let mut stdout = std::io::stdout();
        crossterm::execute!(
            stdout,
            crossterm::terminal::EnterAlternateScreen,
            crossterm::event::EnableMouseCapture
        )?;

        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Event loop
        let result = loop {
            terminal.draw(|f| self.render(f))?;

            if let CrosstermEvent::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Enter => {
                        break Ok(Some(self.input.clone()));
                    }
                    KeyCode::Esc => {
                        break Ok(None);
                    }
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
                    KeyCode::Left => {
                        if self.cursor > 0 {
                            self.cursor -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor < self.input.len() {
                            self.cursor += 1;
                        }
                    }
                    _ => {}
                }
            }
        };

        // Restore terminal
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::event::DisableMouseCapture
        )?;

        result
    }

    /// Render the dialog
    fn render(&self, frame: &mut Frame) {
        let area = frame.area();

        // Center the dialog (50% width, 5 lines height)
        let dialog_area = Self::centered_rect(50, 5, area);

        // Layout: Prompt (1 line) + Input (3 lines)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
            ])
            .split(dialog_area);

        // Render prompt
        let prompt_widget = Paragraph::new(self.prompt.as_str())
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        frame.render_widget(prompt_widget, chunks[0]);

        // Render input field
        let input_widget = Paragraph::new(self.input.as_str())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title("Input (Enter to submit, Esc to cancel)")
                    .border_style(Style::default().fg(Color::Yellow))
            )
            .style(Style::default().fg(Color::White));
        frame.render_widget(input_widget, chunks[1]);

        // Set cursor position
        frame.set_cursor_position((
            chunks[1].x + 1 + self.cursor as u16,
            chunks[1].y + 1,
        ));
    }

    /// Helper to create centered rectangle
    fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
        let vertical_margin = (area.height.saturating_sub(height)) / 2;
        let horizontal_margin = (area.width * (100 - percent_x) / 100) / 2;

        Rect {
            x: area.x + horizontal_margin,
            y: area.y + vertical_margin,
            width: area.width.saturating_sub(horizontal_margin * 2),
            height,
        }
    }
}
```

**Purpose**: Minimal TUI component that shows a centered input dialog, collects user input, and returns it.

**Features**:
- ✅ Simple centered layout (50% width, 5 lines height)
- ✅ Prompt display
- ✅ Text input with cursor navigation
- ✅ Enter to submit, Esc to cancel
- ✅ Automatic terminal setup/cleanup
- ✅ No dependencies on full TUI app state

---

### 3. CLI Event Polling Integration

**File**: `crates/rstn/src/commands/prompt.rs`

**Modification to `run_claude_with_cli_streaming`**:

```rust
/// CLI-specific streaming handler with MCP event polling
async fn run_claude_with_cli_streaming(
    message: &str,
    options: &ClaudeCliOptions,
    mcp_state: Option<Arc<Mutex<crate::tui::mcp_server::McpState>>>,
) -> Result<ClaudeResult> {
    // ... existing code ...

    let mut result = ClaudeResult { /* ... */ };
    let mut mode = InputMode::CLI;

    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    loop {
        // Poll for MCP events if MCP server is active
        if let Some(ref state) = mcp_state {
            if let Ok(mut state_guard) = state.try_lock() {
                // Drain TUI events (MCP sends events here)
                for event in state_guard.drain_tui_events() {
                    if let Event::McpStatus { status, prompt, .. } = event {
                        if status == "needs_input" {
                            // Switch to Mini TUI mode
                            mode = InputMode::MiniTUI;

                            let prompt_text = prompt.unwrap_or_else(|| "Enter input:".to_string());

                            // Show mini dialog
                            let dialog = crate::tui::mini_dialog::MiniTUIDialog::new(prompt_text);
                            let user_input = dialog.run()?;

                            // Send response back to MCP server
                            if let Some(input) = user_input {
                                state_guard.send_input_response(input);
                            } else {
                                // User cancelled - send empty response
                                state_guard.send_input_response("".to_string());
                            }

                            // Switch back to CLI mode
                            mode = InputMode::CLI;
                        }
                    }
                }
            }
        }

        // Continue streaming JSONL output
        match lines.next_line().await {
            Ok(Some(line)) => {
                if let Ok(msg) = serde_json::from_str::<ClaudeStreamMessage>(&line) {
                    // ... existing streaming logic ...
                }
            }
            Ok(None) => break, // EOF
            Err(_) => break,
        }
    }

    // ... wait for process completion ...

    Ok(result)
}
```

**Key Changes**:
1. Accept optional `mcp_state: Option<Arc<Mutex<McpState>>>` parameter
2. Poll `state_guard.drain_tui_events()` each iteration
3. Detect `needs_input` event → show `MiniTUIDialog`
4. Send user response via `state_guard.send_input_response()`
5. Continue streaming after dialog closes

---

### 4. MCP State Access from CLI

**File**: `crates/rstn/src/commands/prompt.rs`

**Modification to `run` function**:

```rust
pub async fn run(
    message: &str,
    max_turns: u32,
    skip_permissions: bool,
    continue_session: bool,
    session_id: Option<String>,
    allowed_tools: Vec<String>,
    context: Vec<std::path::PathBuf>,
    verbose: bool,
) -> Result<ClaudeResult> {
    // ... existing code ...

    // Get MCP state if available (from global or passed in)
    let mcp_state = crate::tui::mcp_server::get_global_mcp_state();

    // Run Claude command with MCP event polling
    let result = run_claude_with_cli_streaming(message, &options, mcp_state).await?;

    // ... existing code ...
}
```

**Note**: This requires adding a global `McpState` accessor or passing it through from `main.rs`.

---

### 5. Global MCP State Singleton

**File**: `crates/rstn/src/tui/mcp_server.rs`

**Add global state accessor**:

```rust
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

/// Global MCP state singleton (shared between TUI and CLI modes)
static GLOBAL_MCP_STATE: Lazy<Arc<Mutex<McpState>>> = Lazy::new(|| {
    Arc::new(Mutex::new(McpState::default()))
});

/// Get the global MCP state (for CLI mode integration)
pub fn get_global_mcp_state() -> Option<Arc<Mutex<McpState>>> {
    Some(GLOBAL_MCP_STATE.clone())
}

/// Initialize the global MCP state (called from main.rs)
pub fn init_global_mcp_state(state: Arc<Mutex<McpState>>) {
    // Replace the global state with the provided one
    // (This allows TUI and CLI to share the same state)
}
```

**Usage in `main.rs`**:
```rust
// Start MCP server and share state with global accessor
let mcp_state = Arc::new(Mutex::new(McpState::default()));
init_global_mcp_state(mcp_state.clone());
let mcp_handle = mcp_server::start_server(config, tx, mcp_state.clone()).await?;
```

---

## Implementation Sequence

### Step 1: Create MiniTUIDialog Component
- Create `crates/rstn/src/tui/mini_dialog.rs`
- Implement simple centered input dialog
- Add tests for terminal setup/cleanup

### Step 2: Add InputMode Enum
- Add enum to `commands/prompt.rs`
- Track mode during streaming

### Step 3: Integrate Event Polling
- Modify `run_claude_with_cli_streaming` to poll MCP events
- Switch to MiniTUI on `needs_input`
- Send response back to MCP

### Step 4: Global MCP State
- Add global state accessor to `mcp_server.rs`
- Initialize in `main.rs`
- Pass to CLI command

### Step 5: Testing
- Unit tests: MiniTUIDialog rendering
- Integration tests: CLI mode with MCP input requests
- Manual test: `rstn prompt` with interactive workflow

### Step 6: Documentation
- Update KB: `claude-code-advanced-features.md` Section 2
- Add CLI mode MCP integration examples

---

## Testing Strategy

### Unit Tests

**File**: `crates/rstn/src/tui/mini_dialog.rs`

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
        let centered = MiniTUIDialog::centered_rect(50, 5, area);

        // 50% width = 50, centered at x=25
        assert_eq!(centered.width, 50);
        assert_eq!(centered.x, 25);

        // 5 lines height, centered at y=(50-5)/2=22
        assert_eq!(centered.height, 5);
        assert_eq!(centered.y, 22);
    }
}
```

### Integration Tests

**File**: `crates/rstn/tests/cli_mcp_interaction.rs`

```rust
#[tokio::test]
async fn test_cli_handles_mcp_needs_input() {
    // Setup: Start MCP server with mock state
    let mcp_state = Arc::new(Mutex::new(McpState::default()));

    // Simulate MCP sending needs_input event
    {
        let mut state = mcp_state.lock().unwrap();
        state.push_tui_event(Event::McpStatus {
            status: "needs_input".to_string(),
            prompt: Some("Enter feature name:".to_string()),
            message: None,
        });
    }

    // Run CLI command (should detect event and show dialog)
    // Note: This requires mocking the dialog to avoid blocking on user input

    // Verify response was sent back to MCP
    let state = mcp_state.lock().unwrap();
    assert!(state.has_pending_response());
}
```

---

## Edge Cases

### 1. No MCP Server Running
- **Scenario**: CLI mode runs without MCP server active
- **Behavior**: `mcp_state` is `None`, skip event polling
- **Result**: CLI mode works as before (pure stdout streaming)

### 2. User Cancels Dialog (Esc)
- **Scenario**: User presses Esc in MiniTUI dialog
- **Behavior**: Dialog returns `None`
- **Action**: Send empty string to MCP server
- **Result**: MCP unblocks, Claude continues with empty input

### 3. Multiple Input Requests
- **Scenario**: Claude asks for input multiple times
- **Behavior**: CLI shows MiniTUI dialog for each request
- **Result**: User can provide input iteratively

### 4. Terminal Restore Failure
- **Scenario**: Dialog crashes before terminal cleanup
- **Behavior**: Use `defer` pattern or `Drop` impl to ensure cleanup
- **Result**: Terminal state always restored

---

## Files to Create

1. `crates/rstn/src/tui/mini_dialog.rs` (150 lines) - NEW
2. `crates/rstn/tests/cli_mcp_interaction.rs` (100 lines) - NEW

## Files to Modify

1. `crates/rstn/src/commands/prompt.rs` (50 lines changed)
   - Add `InputMode` enum
   - Modify `run_claude_with_cli_streaming` for event polling
   - Add `mcp_state` parameter

2. `crates/rstn/src/tui/mcp_server.rs` (30 lines added)
   - Add global state accessor
   - Add `init_global_mcp_state` function

3. `crates/rstn/src/tui/mod.rs` (2 lines)
   - Export `mini_dialog` module

4. `crates/rstn/src/main.rs` (5 lines)
   - Initialize global MCP state

## Dependencies

No new dependencies required. Uses existing:
- `ratatui` (TUI framework)
- `crossterm` (terminal control)
- `tokio` (async runtime)
- `std::sync::mpsc` (event channel)

---

## Benefits

1. **Unified Workflow**: CLI and TUI both support MCP interactive workflows
2. **No Breaking Changes**: CLI mode without MCP works as before
3. **Minimal Complexity**: MiniTUI dialog is <150 lines, no complex state
4. **User Experience**: Seamless transition from CLI to dialog and back
5. **Testable**: Event polling and dialog are independently testable

---

## Alternatives Considered

### Alternative 1: `--permission-prompt-tool`
**Approach**: Use Claude Code's `--permission-prompt-tool` flag to redirect prompts to external command

**Pros**:
- No need for TUI component
- Claude handles the prompting

**Cons**:
- Requires external script
- Less control over UX
- More complex integration

**Decision**: Rejected - MiniTUI provides better UX

### Alternative 2: Hybrid Mode with Full TUI
**Approach**: CLI mode runs full TUI app in background

**Pros**:
- Reuses existing TUI components

**Cons**:
- Heavy-weight (full TUI state for simple input)
- Complex state synchronization

**Decision**: Rejected - Too complex for simple input

---

## Open Questions

1. **Cursor Visibility**: Should we show cursor in MiniTUI dialog?
   - **Answer**: Yes, use `frame.set_cursor_position()`

2. **Dialog Timeout**: Should dialog auto-close after N seconds?
   - **Answer**: No, wait for user input (Esc to cancel)

3. **Multi-line Input**: Should dialog support multi-line input?
   - **Answer**: Start with single-line, add multi-line if needed

4. **Styling**: Should dialog match TUI theme?
   - **Answer**: Use simple cyan/yellow colors for clarity

---

## Next Steps

1. ✅ Complete architecture design (this document)
2. ⏭️ Implement MiniTUIDialog component
3. ⏭️ Integrate event polling into CLI streaming
4. ⏭️ Add global MCP state accessor
5. ⏭️ Write tests
6. ⏭️ Update KB documentation
7. ⏭️ Manual testing with interactive workflow

---

## References

- Current CLI implementation: `crates/rstn/src/commands/prompt.rs`
- Event system: `crates/rstn/src/tui/event.rs`
- MCP server: `crates/rstn/src/tui/mcp_server.rs`
- KB documentation: `kb/03-api-reference/claude-code-advanced-features.md`
