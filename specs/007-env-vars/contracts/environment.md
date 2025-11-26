# API Contract: Environment Manager

**Feature**: 007-env-vars | **Date**: 2025-11-26

## Module: `executor::environment`

### Public API

```rust
//! Environment variable management for rush shell
//!
//! This module provides the `EnvironmentManager` struct which handles
//! all environment variable operations including inheritance, expansion,
//! and passing to child processes.

use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur during environment operations
#[derive(Debug, Error)]
pub enum EnvError {
    /// Variable name doesn't match `[a-zA-Z_][a-zA-Z0-9_]*`
    #[error("Invalid variable name '{0}': must start with letter or underscore, contain only alphanumeric and underscore")]
    InvalidName(String),

    /// Variable name is empty
    #[error("Variable name cannot be empty")]
    EmptyName,
}

/// Manages environment variables for a shell session
///
/// # Example
///
/// ```rust
/// use rush::executor::environment::EnvironmentManager;
///
/// let mut env = EnvironmentManager::new();
/// assert!(env.get("HOME").is_some()); // Inherited from system
///
/// env.set("MY_VAR".to_string(), "my_value".to_string()).unwrap();
/// assert_eq!(env.get("MY_VAR"), Some("my_value"));
/// ```
pub struct EnvironmentManager {
    variables: HashMap<String, String>,
}

impl EnvironmentManager {
    /// Create a new EnvironmentManager, inheriting all variables from parent process
    ///
    /// # Returns
    ///
    /// A new `EnvironmentManager` with all system environment variables
    ///
    /// # Example
    ///
    /// ```rust
    /// let env = EnvironmentManager::new();
    /// // HOME, PATH, USER, etc. are available
    /// ```
    pub fn new() -> Self;

    /// Get the value of an environment variable
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name to look up
    ///
    /// # Returns
    ///
    /// `Some(&str)` if the variable is defined, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// let env = EnvironmentManager::new();
    /// if let Some(home) = env.get("HOME") {
    ///     println!("Home directory: {}", home);
    /// }
    /// ```
    pub fn get(&self, name: &str) -> Option<&str>;

    /// Set or update an environment variable
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name (must be valid identifier)
    /// * `value` - The value to set
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, `Err(EnvError)` if name is invalid
    ///
    /// # Errors
    ///
    /// * `EnvError::EmptyName` - if name is empty
    /// * `EnvError::InvalidName` - if name doesn't match `[a-zA-Z_][a-zA-Z0-9_]*`
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut env = EnvironmentManager::new();
    /// env.set("MY_VAR".to_string(), "value".to_string())?;
    ///
    /// // Invalid names are rejected
    /// assert!(env.set("123".to_string(), "value".to_string()).is_err());
    /// ```
    pub fn set(&mut self, name: String, value: String) -> Result<(), EnvError>;

    /// Remove an environment variable
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name to remove
    ///
    /// # Returns
    ///
    /// `Some(String)` with the old value if it existed, `None` otherwise
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut env = EnvironmentManager::new();
    /// env.set("TEMP".to_string(), "value".to_string())?;
    /// let old = env.remove("TEMP");
    /// assert_eq!(old, Some("value".to_string()));
    /// ```
    pub fn remove(&mut self, name: &str) -> Option<String>;

    /// Iterate over all environment variables
    ///
    /// # Returns
    ///
    /// Iterator yielding `(&String, &String)` pairs of (name, value)
    ///
    /// # Example
    ///
    /// ```rust
    /// let env = EnvironmentManager::new();
    /// for (name, value) in env.iter() {
    ///     println!("{}={}", name, value);
    /// }
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)>;

    /// Get environment as HashMap for passing to child processes
    ///
    /// # Returns
    ///
    /// Reference to internal HashMap, suitable for `Command::envs()`
    ///
    /// # Example
    ///
    /// ```rust
    /// let env = EnvironmentManager::new();
    /// let mut cmd = Command::new("echo");
    /// cmd.env_clear().envs(env.as_env_map());
    /// ```
    pub fn as_env_map(&self) -> &HashMap<String, String>;

    /// Get the number of environment variables
    pub fn len(&self) -> usize;

    /// Check if environment is empty
    pub fn is_empty(&self) -> bool;
}

impl Default for EnvironmentManager {
    fn default() -> Self {
        Self::new()
    }
}
```

### Internal Functions

```rust
/// Validate that a variable name follows POSIX conventions
///
/// Valid names: `[a-zA-Z_][a-zA-Z0-9_]*`
///
/// # Arguments
///
/// * `name` - The variable name to validate
///
/// # Returns
///
/// `true` if valid, `false` otherwise
fn is_valid_variable_name(name: &str) -> bool;
```

---

## Module: `executor::parser` (additions)

### New Public Function

```rust
/// Expand environment variables in pipeline segments
///
/// Replaces `$VAR` and `${VAR}` patterns with their values from the
/// environment manager. Respects quote context: single-quoted strings
/// are not expanded.
///
/// # Arguments
///
/// * `segments` - Mutable reference to pipeline segments
/// * `env` - Reference to environment manager
///
/// # Behavior
///
/// - `$VAR` expands to variable value or empty string if undefined
/// - `${VAR}` expands with explicit boundaries (e.g., `${VAR}suffix`)
/// - `\$` produces literal `$` (escaped)
/// - Single-quoted strings are not expanded
/// - Double-quoted strings are expanded
///
/// # Example
///
/// ```rust
/// // Given env["HOME"] = "/Users/user"
///
/// // Input: ["echo", "$HOME"]
/// // Output: ["echo", "/Users/user"]
///
/// // Input: ["echo", "${HOME}_backup"]
/// // Output: ["echo", "/Users/user_backup"]
///
/// // Input: ["echo", "'$HOME'"]  (single quoted)
/// // Output: ["echo", "$HOME"]  (no expansion)
/// ```
pub fn expand_variables(segments: &mut [PipelineSegment], env: &EnvironmentManager);
```

### New Internal Function

```rust
/// Expand variables in a single string
///
/// # Arguments
///
/// * `input` - String potentially containing `$VAR` or `${VAR}` patterns
/// * `env` - Environment manager for lookups
/// * `expand` - Whether to actually expand (false for single-quoted)
///
/// # Returns
///
/// String with variables expanded (or unchanged if expand=false)
fn expand_string(input: &str, env: &EnvironmentManager, expand: bool) -> String;
```

---

## Module: `executor::builtins::export`

### Public Function

```rust
//! 'export' built-in command
//!
//! Sets environment variables that will be passed to child processes.

use crate::error::Result;
use crate::executor::execute::CommandExecutor;

/// Execute the 'export' builtin command
///
/// # Arguments
///
/// * `executor` - Mutable reference to command executor
/// * `args` - Command arguments (VAR=value pairs)
///
/// # Returns
///
/// * `Ok(0)` on success
/// * `Err(RushError)` on invalid variable name or syntax
///
/// # Syntax
///
/// ```text
/// export VAR=value           # Set VAR to "value"
/// export VAR="value"         # Set VAR to "value" (quotes stripped)
/// export VAR=$OTHER          # Set VAR to value of OTHER
/// export VAR=                # Set VAR to empty string
/// export                     # List all exported variables (like set)
/// ```
///
/// # Errors
///
/// * Invalid variable name (doesn't match `[a-zA-Z_][a-zA-Z0-9_]*`)
/// * Missing `=` in argument
///
/// # Example
///
/// ```text
/// rush> export PATH=$PATH:/custom/bin
/// rush> export MY_VAR=hello
/// rush> echo $MY_VAR
/// hello
/// ```
pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32>;
```

---

## Module: `executor::builtins::set`

### Public Function

```rust
//! 'set' built-in command
//!
//! Lists all shell environment variables.

use crate::error::Result;
use crate::executor::execute::CommandExecutor;

/// Execute the 'set' builtin command
///
/// # Arguments
///
/// * `executor` - Mutable reference to command executor
/// * `args` - Command arguments (currently unused)
///
/// # Returns
///
/// * `Ok(0)` always (listing cannot fail)
///
/// # Output Format
///
/// Prints all variables in `NAME=value` format, one per line,
/// sorted alphabetically by name.
///
/// # Example
///
/// ```text
/// rush> set
/// HOME=/Users/username
/// PATH=/usr/bin:/bin
/// USER=username
/// ...
/// ```
pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32>;
```

---

## Integration with CommandExecutor

### Additions to `execute.rs`

```rust
impl CommandExecutor {
    /// Get reference to environment manager
    pub fn env_manager(&self) -> &EnvironmentManager;

    /// Get mutable reference to environment manager
    pub fn env_manager_mut(&mut self) -> &mut EnvironmentManager;
}
```

### Additions to `builtins/mod.rs`

```rust
pub fn execute_builtin(
    executor: &mut CommandExecutor,
    command: &str,
    args: &[String],
) -> Option<Result<i32>> {
    match command {
        "jobs" => Some(jobs::execute(executor, args)),
        "fg" => Some(fg::execute(executor, args)),
        "bg" => Some(bg::execute(executor, args)),
        "export" => Some(export::execute(executor, args)),  // NEW
        "set" => Some(set::execute(executor, args)),        // NEW
        _ => None,
    }
}
```

---

## Error Handling Contract

All environment-related errors go through `RushError`:

```rust
// In error.rs, add variant:
pub enum RushError {
    // ... existing variants ...

    /// Environment variable error
    #[error("Environment error: {0}")]
    Environment(#[from] EnvError),
}
```
