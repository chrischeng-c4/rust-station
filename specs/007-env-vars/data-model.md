# Data Model: Environment Variables

**Feature**: 007-env-vars | **Date**: 2025-11-26

## Core Entities

### EnvironmentManager

Primary struct for managing shell environment variables.

```rust
/// Manages environment variables for the shell session
pub struct EnvironmentManager {
    /// All environment variables (inherited + user-set)
    variables: HashMap<String, String>,
}

impl EnvironmentManager {
    /// Create new manager, inheriting from parent process
    pub fn new() -> Self;

    /// Get variable value, returns None if undefined
    pub fn get(&self, name: &str) -> Option<&str>;

    /// Set or update a variable
    pub fn set(&mut self, name: String, value: String) -> Result<(), EnvError>;

    /// Remove a variable (for future `unset` command)
    pub fn remove(&mut self, name: &str) -> Option<String>;

    /// Get all variables as iterator
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)>;

    /// Export as HashMap for Command::envs()
    pub fn as_env_map(&self) -> &HashMap<String, String>;

    /// Count of variables
    pub fn len(&self) -> usize;
}
```

### EnvError

Error type for environment operations.

```rust
/// Errors that can occur during environment operations
#[derive(Debug, thiserror::Error)]
pub enum EnvError {
    #[error("Invalid variable name: {0}")]
    InvalidName(String),

    #[error("Variable name cannot be empty")]
    EmptyName,
}
```

### QuoteType (Enhancement to Token)

Track quote context for proper expansion behavior.

```rust
/// Type of quoting used for a token
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QuoteType {
    /// No quotes: `word` - variables expand
    None,
    /// Single quotes: `'word'` - no expansion
    Single,
    /// Double quotes: `"word"` - variables expand
    Double,
}

/// Enhanced Word token with quote information
pub struct WordToken {
    pub value: String,
    pub quote_type: QuoteType,
}
```

## Entity Relationships

```
┌─────────────────────────────────────────────────────────┐
│                   CommandExecutor                        │
│  ┌─────────────────────────────────────────────────┐    │
│  │  env_manager: EnvironmentManager                │    │
│  │  job_manager: JobManager                        │    │
│  │  pipeline_executor: PipelineExecutor            │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          │
                          │ owns
                          ▼
┌─────────────────────────────────────────────────────────┐
│                  EnvironmentManager                      │
│  ┌─────────────────────────────────────────────────┐    │
│  │  variables: HashMap<String, String>             │    │
│  │    "HOME" → "/Users/username"                   │    │
│  │    "PATH" → "/usr/bin:/bin:..."                 │    │
│  │    "USER" → "username"                          │    │
│  │    ...                                          │    │
│  └─────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────┘
                          │
                          │ used by
                          ▼
┌─────────────────────────────────────────────────────────┐
│                   expand_variables()                     │
│  Input: PipelineSegment with raw args                   │
│  Output: PipelineSegment with expanded args             │
│                                                          │
│  "$HOME/docs" + env["HOME"]="/Users/u"                  │
│       → "/Users/u/docs"                                 │
└─────────────────────────────────────────────────────────┘
```

## State Transitions

### Variable Lifecycle

```
                    ┌──────────────┐
                    │   Undefined  │
                    └──────────────┘
                           │
                           │ export VAR=value
                           ▼
                    ┌──────────────┐
                    │   Defined    │◄────┐
                    └──────────────┘     │
                           │             │
                           │ export VAR=new_value
                           └─────────────┘
```

### Quote State Machine (in tokenizer)

```
State: QuoteType::None
  │
  ├─ ' (single quote) ──► State: QuoteType::Single
  │                              │
  │                              └─ ' (close) ──► State: QuoteType::None
  │
  └─ " (double quote) ──► State: QuoteType::Double
                                 │
                                 └─ " (close) ──► State: QuoteType::None
```

## Validation Rules

### Variable Name Validation (FR-010)

```rust
/// Validate variable name follows POSIX conventions
fn is_valid_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }

    let mut chars = name.chars();

    // First character: letter or underscore
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }

    // Remaining: letters, digits, or underscores
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
```

**Valid names**: `HOME`, `_private`, `VAR_123`, `a`
**Invalid names**: `123`, `-foo`, `var-name`, ``, `foo.bar`

### Variable Value Rules

- Any UTF-8 string is valid
- Empty string is a valid value (different from undefined)
- No length limit enforced (but memory constraints apply)
- Special characters allowed (spaces, newlines, etc.)

## Data Flow Examples

### Example 1: Basic Expansion

```
Input: "echo $HOME"

Tokenize:
  Token::Word { value: "echo", quote_type: None }
  Token::Word { value: "$HOME", quote_type: None }

Expand (quote_type != Single):
  Token::Word { value: "echo", quote_type: None }
  Token::Word { value: "/Users/username", quote_type: None }

Execute:
  program: "echo"
  args: ["/Users/username"]
```

### Example 2: Single Quote (No Expansion)

```
Input: "echo '$HOME'"

Tokenize:
  Token::Word { value: "echo", quote_type: None }
  Token::Word { value: "$HOME", quote_type: Single }

Expand (quote_type == Single, skip):
  Token::Word { value: "echo", quote_type: None }
  Token::Word { value: "$HOME", quote_type: Single }

Execute:
  program: "echo"
  args: ["$HOME"]  // Literal string
```

### Example 3: Export and Use

```
Input: "export FOO=bar"

Builtin detected: export
Parse args: ["FOO=bar"]
Split on '=': name="FOO", value="bar"
Validate name: OK
env_manager.set("FOO", "bar")
Return: Ok(0)

Later input: "echo $FOO"
expand_variables: env_manager.get("FOO") → Some("bar")
Execute: echo "bar"
```

## Memory Layout

```
EnvironmentManager (stack: 24 bytes typical)
  └─ variables: HashMap (heap)
       ├─ "HOME" (5 bytes) → "/Users/username" (16 bytes)
       ├─ "PATH" (4 bytes) → "/usr/bin:/bin:..." (100+ bytes)
       ├─ "USER" (4 bytes) → "username" (8 bytes)
       └─ ... (~50-100 entries typical)

Total heap estimate: ~10-15KB for typical environment
```

## Thread Safety

**Current design**: Single-threaded (shell REPL is synchronous)

**Future consideration**: If rush becomes multi-threaded:
- Wrap `EnvironmentManager` in `Arc<RwLock<_>>`
- Use `RwLock::read()` for expansion (many readers)
- Use `RwLock::write()` for export (exclusive writer)
