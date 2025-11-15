# Completion Module Contract

**Module**: `rush::completion`
**Purpose**: Tab completion for commands, paths, and flags

---

## Public API

```rust
pub struct CompletionEngine {
    command_completer: CommandCompleter,
    path_completer: PathCompleter,
    flag_completer: FlagCompleter,
}

impl CompletionEngine {
    /// Create new completion engine
    pub fn new() -> Self;

    /// Get completions for user input at cursor position
    pub fn complete(&self, input: &str, cursor: usize) -> Vec<CompletionResult>;
}

pub struct CompletionResult {
    pub text: String,
    pub completion_type: CompletionType,
    pub description: Option<String>,
    pub score: f32,  // 0.0 to 1.0, higher = better match
}

pub enum CompletionType {
    Command,   // Executable from PATH
    Path,      // File or directory
    Flag,      // Command-line flag
}
```

---

## Sub-Completers

### Command Completer

```rust
pub struct CommandCompleter {
    // Caches executables from PATH
}

impl CommandCompleter {
    pub fn new() -> Self;

    /// Get completions for command name
    /// Returns executables from PATH matching prefix
    pub fn complete(&self, prefix: &str) -> Vec<CompletionResult>;

    /// Refresh PATH cache (when PATH changes)
    pub fn refresh(&mut self);
}
```

### Path Completer

```rust
pub struct PathCompleter {
    // Completes filesystem paths
}

impl PathCompleter {
    pub fn new() -> Self;

    /// Get path completions for partial path
    /// Handles both absolute (/etc/pass...) and relative (./src/ma...)
    pub fn complete(&self, partial_path: &str) -> Vec<CompletionResult>;
}
```

### Flag Completer

```rust
pub struct FlagCompleter {
    // Completes common flags for known commands
}

impl FlagCompleter {
    pub fn new() -> Self;

    /// Get flag completions for command
    /// Returns common flags (-h, --help, -v, --version, etc.)
    pub fn complete(&self, command: &str, partial_flag: &str) -> Vec<CompletionResult>;
}
```

---

## Responsibilities

1. **Command Completion**: Find executables in PATH matching prefix
2. **Path Completion**: Find files/directories in filesystem matching partial path
3. **Flag Completion**: Suggest common flags for known commands
4. **Ranking**: Score results by relevance (exact match > prefix match > fuzzy match)
5. **Caching**: Cache PATH contents for performance

---

## Behavior Contracts

### Command Completion

- MUST search all directories in PATH
- MUST return only executable files
- MUST cache PATH contents (refresh on PATH change)
- MUST handle case-sensitive matching (commands are case-sensitive)
- MUST return results sorted by score (highest first)

### Path Completion

- MUST handle absolute paths (/etc/...)
- MUST handle relative paths (./src/...)
- MUST handle home directory expansion (~/)
- MUST show directories with trailing slash (/)
- MUST respect file permissions (only show readable files)
- MUST handle large directories efficiently (1,000+ files)

### Flag Completion

- MUST support common flags (-h, --help, -v, --version, -a, -l, etc.)
- MUST return results for recognized commands (ls, git, cargo, etc.)
- MUST gracefully handle unknown commands (return empty list)
- MUST support both short (-h) and long (--help) flags

---

## Performance Requirements

| Operation | Target | Measurement |
|-----------|--------|-------------|
| Command completion | <50ms | Search entire PATH |
| Path completion | <100ms | For directory with <1,000 files |
| Flag completion | <10ms | Lookup in static database |
| Total completion latency | <100ms | Combined overhead (spec requirement SC-007) |

---

## Completion Logic

### Determining Completion Type

```rust
fn determine_completion_type(input: &str, cursor: usize) -> CompletionType {
    let words = input[..cursor].split_whitespace().collect::<Vec<_>>();

    match words.len() {
        0 => CompletionType::Command,  // Empty input → command
        1 => {
            // First word → command OR path (if contains /)
            if words[0].contains('/') {
                CompletionType::Path
            } else {
                CompletionType::Command
            }
        },
        _ => {
            // After first word → path OR flag
            let last_word = words.last().unwrap();
            if last_word.starts_with('-') {
                CompletionType::Flag
            } else {
                CompletionType::Path
            }
        }
    }
}
```

### Scoring Algorithm

```rust
fn score_match(prefix: &str, candidate: &str) -> f32 {
    if candidate == prefix {
        1.0  // Exact match
    } else if candidate.starts_with(prefix) {
        0.8  // Prefix match
    } else if candidate.contains(prefix) {
        0.5  // Substring match
    } else {
        0.0  // No match
    }
}
```

---

## Testing Contract

### Unit Tests

- Command completion with various PATH configurations
- Path completion with absolute, relative, and home paths
- Flag completion for known and unknown commands
- Scoring algorithm for different match types

### Integration Tests

- Tab completion in full REPL context
- Completion with large PATH (100+ directories)
- Completion with large directories (1,000+ files)
- Completion with Unicode filenames

### Benchmark Tests

- Command completion latency
- Path completion latency with various directory sizes
- Memory usage of PATH cache

---

## Error Cases

- **PATH not set**: Use default PATH (/usr/bin:/bin:/usr/local/bin)
- **Directory unreadable**: Skip, log warning, continue with other directories
- **Permission denied**: Filter out inaccessible files
- **Timeout exceeded**: Return partial results gathered so far

---

## Example Usage

```rust
use rush::completion::CompletionEngine;

fn main() {
    let engine = CompletionEngine::new();

    // Complete command: "gi" → "git"
    let results = engine.complete("gi", 2);
    assert_eq!(results[0].text, "git");
    assert_eq!(results[0].completion_type, CompletionType::Command);

    // Complete path: "ls /et" → "/etc"
    let results = engine.complete("ls /et", 6);
    assert_eq!(results[0].text, "/etc");
    assert_eq!(results[0].completion_type, CompletionType::Path);

    // Complete flag: "ls -" → "-l", "-a", "-h", etc.
    let results = engine.complete("ls -", 4);
    assert!(results.iter().any(|r| r.text == "-l"));
    assert!(results.iter().any(|r| r.text == "-a"));
}
```

---

## Future Enhancements (Post-MVP)

- Context-aware flag completion (parse --help output)
- Fuzzy matching for commands
- Git-aware path completion (only tracked files)
- Command-specific argument completion
- Custom completion scripts (like bash-completion)
