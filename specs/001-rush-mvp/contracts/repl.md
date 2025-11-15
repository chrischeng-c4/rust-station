# REPL Module Contract

**Module**: `rush::repl`
**Purpose**: Interactive Read-Eval-Print Loop with line editing, syntax highlighting, and autosuggestions

---

## Public API

### Main Entry Point

```rust
pub struct Repl {
    editor: Reedline,
    history: Arc<Mutex<HistoryManager>>,
    executor: CommandExecutor,
    config: Config,
}

impl Repl {
    /// Create new REPL with default configuration
    pub fn new() -> Result<Self>;

    /// Create REPL with custom configuration
    pub fn with_config(config: Config) -> Result<Self>;

    /// Run the REPL loop until exit
    /// Returns exit code for the shell process
    pub fn run(&mut self) -> Result<i32>;
}
```

---

## Responsibilities

1. **Line Editing**: Handle user input with cursor movement, backspace, delete
2. **Syntax Highlighting**: Real-time command colorization as user types
3. **Autosuggestions**: Display ghost text based on history matches
4. **History Navigation**: Up/down arrows to navigate command history
5. **Tab Completion**: Invoke completion engine on Tab key
6. **Command Execution**: Pass validated commands to executor
7. **Signal Handling**: Handle Ctrl+C (interrupt), Ctrl+D (EOF), Ctrl+Z (suspend)

---

## Dependencies

- `reedline` - Core line editing functionality
- `HistoryManager` - Command history storage and retrieval
- `CommandExecutor` - Execute validated commands
- `RushHighlighter` - Syntax highlighting implementation
- `RushCompleter` - Tab completion implementation
- `RushHinter` - Autosuggestion implementation

---

## Behavior Contracts

### Startup

- MUST initialize in <100ms on modern hardware
- MUST load history from disk without blocking
- MUST display prompt immediately
- MUST work with zero configuration (all defaults)

### Input Handling

- MUST respond to keystrokes within 16ms (60 FPS requirement)
- MUST update syntax highlighting in real-time
- MUST update autosuggestions within 50ms of user pausing
- MUST handle Unicode input correctly (emojis, CJK characters)

### Command Execution

- MUST validate command before execution
- MUST add executed command to history
- MUST persist history to disk after each command
- MUST display command output in real-time
- MUST preserve exit code for next prompt (optional feature)

### Signal Handling

- **Ctrl+C**: Cancel current input, show new prompt (do NOT exit shell)
- **Ctrl+D**: Exit shell if input is empty, otherwise delete character
- **Ctrl+Z**: Suspend foreground job (if running), return to prompt

### Error Handling

- Invalid commands MUST display helpful error message
- History file errors MUST NOT crash shell (warn and continue)
- Config file errors MUST NOT crash shell (use defaults and warn)

---

## Performance Requirements

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Startup | <100ms | Time from process start to first prompt |
| Keystroke response | <16ms | Input event to screen update |
| Syntax highlighting | <16ms | Per keystroke |
| Autosuggestion | <50ms | From last keystroke to suggestion display |
| History append | <5ms | Synchronous write to disk |

---

## Testing Contract

### Unit Tests

- Line editing with arrow keys, backspace, delete
- Multiline input handling
- Signal handling (Ctrl+C, Ctrl+D, Ctrl+Z)

### Integration Tests

- Full REPL session (spawn process, send input, verify output)
- History persistence across sessions
- Syntax highlighting for various command patterns
- Autosuggestions from history

### Benchmark Tests

- Startup time measurement
- Input responsiveness (keystroke latency)

---

## Example Usage

```rust
use rush::repl::Repl;

fn main() -> Result<()> {
    let mut repl = Repl::new()?;
    let exit_code = repl.run()?;
    std::process::exit(exit_code);
}
```

---

## Error Cases

- **History file unreadable**: Log warning, start with empty history
- **Config file malformed**: Log warning, use defaults
- **Terminal not TTY**: Return error (rush requires interactive terminal)
- **Executor failure**: Display error, continue REPL (don't crash)
