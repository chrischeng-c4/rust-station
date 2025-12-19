# Data Model: rush Shell MVP

**Feature**: rush Shell MVP
**Date**: 2025-11-14
**Purpose**: Define core entities and their relationships

---

## Entity Overview

rush's data model consists of five core entities:

1. **Command** - A user-entered instruction to execute
2. **HistoryEntry** - A record of previously executed command
3. **Job** - A running or suspended process
4. **Config** - User preferences and settings
5. **CompletionResult** - A potential completion match

---

## Entity 1: Command

**Purpose**: Represents a parsed user command ready for execution

### Attributes

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `raw_input` | `String` | Original user input | Non-empty |
| `program` | `String` | Command name (first word) | Non-empty |
| `args` | `Vec<String>` | Command arguments | Can be empty |
| `background` | `bool` | Run in background (ends with `&`) | Default: false |
| `operators` | `Vec<Operator>` | Chaining operators (`&&`, `\|\|`, `;`) | Default: empty |
| `redirects` | `Vec<Redirect>` | Output redirections (`>`, `>>`) | Default: empty |

### Supporting Types

```rust
enum Operator {
    And,        // && - run next if this succeeds
    Or,         // || - run next if this fails
    Sequence,   // ; - run next regardless
    Pipe,       // | - pipe output to next
}

struct Redirect {
    fd: i32,           // File descriptor (1=stdout, 2=stderr)
    mode: RedirectMode,
    target: PathBuf,
}

enum RedirectMode {
    Overwrite,  // >
    Append,     // >>
}
```

### Validation Rules

- `program` MUST not be empty string
- If `background` is true, MUST be last command in chain
- Pipe operator creates command chain (for script execution)

### State Transitions

```
User Input → Parsing → Command → Validation → Execution
```

1. **Created**: Command parsed from user input
2. **Validated**: Checked for syntax errors
3. **Executing**: Passed to executor
4. **Completed**: Exit code received

### Example

```rust
// Input: "ls -la /etc > files.txt && cat files.txt"
// Parsed into two Commands with And operator:

Command {
    raw_input: "ls -la /etc > files.txt",
    program: "ls",
    args: vec!["-la".to_string(), "/etc".to_string()],
    background: false,
    operators: vec![Operator::And],
    redirects: vec![Redirect {
        fd: 1,
        mode: RedirectMode::Overwrite,
        target: PathBuf::from("files.txt"),
    }],
}
```

---

## Entity 2: HistoryEntry

**Purpose**: Record of a previously executed command for history navigation and autosuggestions

### Attributes

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `command` | `String` | The executed command text | Non-empty |
| `timestamp` | `u64` | Unix timestamp of execution | > 0 |
| `exit_code` | `Option<i32>` | Command exit code | None if still running |
| `working_dir` | `PathBuf` | Directory where command ran | Valid path |
| `session_id` | `u64` | Shell session identifier | > 0 |

### Validation Rules

- `command` MUST not be empty or whitespace-only
- `timestamp` MUST be valid Unix timestamp
- `exit_code` is None for background jobs still running
- `working_dir` captured at execution time (for context)

### Relationships

- One Command → creates one HistoryEntry upon execution
- HistoryEntry used by autosuggestion engine to match user input
- HistoryEntry used by history navigation (up/down arrows)

### Storage Format

```
# ~/.config/rush/history (plain text, newline-delimited)
# Format: command (timestamp, exit_code, working_dir in future)

ls -la /etc
cd /tmp
echo "hello world"
git status
cargo build
```

**MVP**: Store only command text, one per line. Future: extend to include metadata.

### Example

```rust
HistoryEntry {
    command: "git commit -m \"Add feature\"".to_string(),
    timestamp: 1731628800,  // 2025-11-14
    exit_code: Some(0),
    working_dir: PathBuf::from("/Users/dev/project"),
    session_id: 42,
}
```

---

## Entity 3: Job

**Purpose**: Represents a running or suspended process managed by the shell

### Attributes

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `id` | `usize` | Shell-internal job ID | Unique, sequential |
| `pid` | `i32` | Process ID from OS | > 0 |
| `pgid` | `i32` | Process group ID | > 0 |
| `command` | `String` | Command text | Non-empty |
| `state` | `JobState` | Current job state | See enum |
| `background` | `bool` | Started with `&` | - |

### Supporting Types

```rust
enum JobState {
    Running,            // Currently executing
    Suspended,          // Stopped (Ctrl+Z)
    Completed(i32),     // Finished with exit code
}
```

### Validation Rules

- `id` MUST be unique within shell session
- `pid` and `pgid` MUST be valid OS process IDs
- Only one job can be in foreground at a time
- Suspended jobs can be resumed with `fg` or `bg`

### State Transitions

```
Spawned → Running ⇄ Suspended → Completed
                  ↓
              Background Running
```

1. **Spawned**: Process started, assigned job ID
2. **Running**: Process executing in foreground
3. **Suspended**: User pressed Ctrl+Z, process stopped
4. **Background Running**: Process executing in background
5. **Completed**: Process exited with status code

### Relationships

- One Command → creates one Job
- Job MUST have valid process group for job control
- Job IDs reused after completion (wrap around)

### Example

```rust
// Job started with: sleep 30 &
Job {
    id: 1,
    pid: 12345,
    pgid: 12345,  // Leader of its own process group
    command: "sleep 30".to_string(),
    state: JobState::Running,
    background: true,
}

// After user presses Ctrl+Z on foreground job:
Job {
    id: 2,
    pid: 12350,
    pgid: 12350,
    command: "npm install".to_string(),
    state: JobState::Suspended,
    background: false,
}
```

---

## Entity 4: Config

**Purpose**: User preferences and shell behavior settings

### Attributes

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `history_size` | `usize` | Max history entries | Default: 10,000 |
| `prompt` | `String` | Prompt string | Default: "$ " |
| `theme` | `Theme` | Color scheme | Default: `Theme::Default` |
| `completion_timeout_ms` | `u64` | Tab completion timeout | Default: 100 |
| `suggestion_delay_ms` | `u64` | Autosuggestion delay | Default: 50 |

### Supporting Types

```rust
struct Theme {
    command_color: Color,
    flag_color: Color,
    path_color: Color,
    string_color: Color,
    error_color: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            command_color: Color::Green,
            flag_color: Color::Blue,
            path_color: Color::Cyan,
            string_color: Color::Yellow,
            error_color: Color::Red,
        }
    }
}
```

### Validation Rules

- `history_size` MUST be > 0 and < 1,000,000 (sanity limit)
- `prompt` can be any string (including empty)
- Timeouts MUST be > 0 and < 10,000ms (sanity limit)

### Storage Format

```toml
# ~/.config/rush/rush.toml (optional, zero-config works without it)

[appearance]
prompt = "rush> "
theme = "default"  # Future: support custom themes

[behavior]
history_size = 10000
completion_timeout_ms = 100
suggestion_delay_ms = 50
```

### Default Behavior

If no config file exists, all defaults apply. Shell works perfectly with zero configuration (Principle II: Zero-Config Philosophy).

### Example

```rust
Config {
    history_size: 10_000,
    prompt: "$ ".to_string(),
    theme: Theme::default(),
    completion_timeout_ms: 100,
    suggestion_delay_ms: 50,
}
```

---

## Entity 5: CompletionResult

**Purpose**: A potential completion match for user input

### Attributes

| Attribute | Type | Description | Constraints |
|-----------|------|-------------|-------------|
| `text` | `String` | Completion text to insert | Non-empty |
| `completion_type` | `CompletionType` | What kind of completion | See enum |
| `description` | `Option<String>` | Helper text (optional) | - |
| `score` | `f32` | Relevance score for ranking | 0.0 to 1.0 |

### Supporting Types

```rust
enum CompletionType {
    Command,   // Executable from PATH
    Path,      // File or directory
    Flag,      // Command-line flag
}
```

### Validation Rules

- `text` MUST not be empty
- `score` MUST be between 0.0 and 1.0
- Higher scores rank higher in completion list

### Ranking Logic

```rust
// Score calculation for different completion types:

// Command completion: exact prefix match = 1.0, fuzzy = 0.5
if input == "ls" && completion == "ls" {
    score = 1.0
} else if completion.starts_with(input) {
    score = 0.8
} else {
    score = 0.5  // fuzzy match
}

// Path completion: prefer files in current directory
if path.parent() == current_dir {
    score *= 1.2
}

// Flag completion: exact match preferred
if flag.starts_with(input) {
    score = 1.0
}
```

### Example

```rust
// User typed: "ls /et" and pressed Tab
// Completion engine returns:

CompletionResult {
    text: "/etc".to_string(),
    completion_type: CompletionType::Path,
    description: Some("System configuration directory".to_string()),
    score: 1.0,  // Exact prefix match
}

// User typed: "git com" and pressed Tab
// Multiple results:

vec![
    CompletionResult {
        text: "commit".to_string(),
        completion_type: CompletionType::Command,
        description: Some("Record changes to repository".to_string()),
        score: 1.0,  // Starts with "com"
    },
    CompletionResult {
        text: "compare".to_string(),
        completion_type: CompletionType::Command,
        description: None,
        score: 1.0,
    },
]
```

---

## Entity Relationships

```
User Input → Command
             ↓
          Execution
             ↓
        HistoryEntry ←─┐
             ↓         │
        (stored)       │ (suggestions)
                       │
User Types → CompletionResult
             ↓
        (Tab completion)

Command → Job (if background or Ctrl+Z)
          ↓
      JobState transitions
```

### Key Relationships:

1. **Command → HistoryEntry**: Each executed command becomes a history entry
2. **HistoryEntry → CompletionResult**: History used for autosuggestions
3. **Command → Job**: Background/suspended commands become jobs
4. **Config**: Loaded once at startup, influences all other entities

---

## Data Flow Example

```rust
// 1. User types: "ls -la /etc"
let input = read_line();  // "ls -la /etc"

// 2. Parse into Command
let cmd = parse_command(input)?;  // Command { program: "ls", args: ["-la", "/etc"], ... }

// 3. Execute
let exit_code = executor.run(&cmd)?;  // 0

// 4. Add to history
let entry = HistoryEntry {
    command: input.clone(),
    timestamp: now(),
    exit_code: Some(exit_code),
    working_dir: current_dir()?,
    session_id: SESSION_ID,
};
history.append(entry)?;

// 5. Later, user types "ls" → autosuggestion finds matching entry
let suggestion = history.suggest("ls")?;  // " -la /etc" (ghost text)
```

---

## Persistence

| Entity | Persisted? | Storage | Format |
|--------|-----------|---------|--------|
| Command | ❌ No | In-memory only | - |
| HistoryEntry | ✅ Yes | `~/.config/rush/history` | Plain text, one per line |
| Job | ❌ No | In-memory only | - |
| Config | ✅ Yes | `~/.config/rush/rush.toml` | TOML format |
| CompletionResult | ❌ No | Generated on-demand | - |

**Rationale**:
- Commands ephemeral (recreated from history if needed)
- Jobs ephemeral (tied to shell session)
- CompletionResults ephemeral (generated from PATH, filesystem, flag database)

---

## Summary

Five core entities with clear responsibilities:

1. **Command**: Parsed user input ready for execution
2. **HistoryEntry**: Historical record for navigation and suggestions
3. **Job**: Active process with state management
4. **Config**: User preferences with sensible defaults
5. **CompletionResult**: On-demand completion matches

All entities designed for:
- ✅ Performance (minimal copying, efficient storage)
- ✅ Simplicity (straightforward relationships)
- ✅ Testability (clear validation rules)
