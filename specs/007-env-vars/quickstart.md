# Quickstart: Environment Variables Implementation

**Feature**: 007-env-vars | **Date**: 2025-11-26

## Prerequisites

- Rush shell codebase checked out
- Rust 1.75+ installed
- Branch `007-env-vars` checked out

## Implementation Order

### PR #1: Foundation (US1 + US4)

**Goal**: Environment inheritance and `$VAR` expansion working

#### Step 1: Create EnvironmentManager

**File**: `crates/rush/src/executor/environment.rs` (NEW)

```rust
//! Environment variable management for rush shell

use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnvError {
    #[error("Invalid variable name '{0}'")]
    InvalidName(String),
    #[error("Variable name cannot be empty")]
    EmptyName,
}

pub struct EnvironmentManager {
    variables: HashMap<String, String>,
}

impl EnvironmentManager {
    pub fn new() -> Self {
        // Inherit from parent process
        Self {
            variables: std::env::vars().collect(),
        }
    }

    pub fn get(&self, name: &str) -> Option<&str> {
        self.variables.get(name).map(|s| s.as_str())
    }

    pub fn set(&mut self, name: String, value: String) -> Result<(), EnvError> {
        if name.is_empty() {
            return Err(EnvError::EmptyName);
        }
        if !is_valid_variable_name(&name) {
            return Err(EnvError::InvalidName(name));
        }
        self.variables.insert(name, value);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.variables.iter()
    }

    pub fn as_env_map(&self) -> &HashMap<String, String> {
        &self.variables
    }
}

fn is_valid_variable_name(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {}
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}
```

#### Step 2: Add to executor/mod.rs

```rust
pub mod environment;
pub use environment::{EnvironmentManager, EnvError};
```

#### Step 3: Add expand_variables to parser.rs

```rust
use crate::executor::EnvironmentManager;

/// Expand environment variables in a string
pub fn expand_variables_in_string(input: &str, env: &EnvironmentManager) -> String {
    let mut result = String::with_capacity(input.len());
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\\' {
            // Handle escaped characters
            if let Some(&next) = chars.peek() {
                if next == '$' {
                    result.push('$');
                    chars.next();
                    continue;
                }
            }
            result.push(c);
        } else if c == '$' {
            // Variable expansion
            let var_name = if chars.peek() == Some(&'{') {
                // ${VAR} syntax
                chars.next(); // consume '{'
                let name: String = chars.by_ref().take_while(|&c| c != '}').collect();
                name
            } else {
                // $VAR syntax
                let name: String = chars
                    .by_ref()
                    .take_while(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .collect();
                name
            };

            if let Some(value) = env.get(&var_name) {
                result.push_str(value);
            }
            // Undefined variables expand to empty string
        } else {
            result.push(c);
        }
    }

    result
}

/// Expand variables in all pipeline segments
pub fn expand_variables(segments: &mut [PipelineSegment], env: &EnvironmentManager) {
    for segment in segments {
        // Expand program name (rare but possible)
        segment.program = expand_variables_in_string(&segment.program, env);

        // Expand all arguments
        for arg in &mut segment.args {
            *arg = expand_variables_in_string(arg, env);
        }

        // Expand redirection paths
        for redir in &mut segment.redirections {
            redir.file_path = expand_variables_in_string(&redir.file_path, env);
        }
    }
}
```

#### Step 4: Integrate into CommandExecutor

**File**: `crates/rush/src/executor/execute.rs`

```rust
use crate::executor::environment::EnvironmentManager;

pub struct CommandExecutor {
    job_manager: JobManager,
    pipeline_executor: PipelineExecutor,
    env_manager: EnvironmentManager,  // ADD THIS
}

impl CommandExecutor {
    pub fn new() -> Self {
        Self {
            job_manager: JobManager::new(),
            pipeline_executor: PipelineExecutor::new(),
            env_manager: EnvironmentManager::new(),  // ADD THIS
        }
    }

    pub fn env_manager(&self) -> &EnvironmentManager {
        &self.env_manager
    }

    pub fn env_manager_mut(&mut self) -> &mut EnvironmentManager {
        &mut self.env_manager
    }

    pub fn execute(&mut self, line: &str) -> Result<i32> {
        let mut pipeline = parse_pipeline(line)?;

        // ADD: Expand variables before execution
        expand_variables(&mut pipeline.segments, &self.env_manager);

        // ... rest of execute
    }
}
```

#### Step 5: Write Unit Tests

**File**: `crates/rush/src/executor/parser.rs` (add to tests module)

```rust
#[cfg(test)]
mod env_tests {
    use super::*;
    use crate::executor::EnvironmentManager;

    #[test]
    fn test_basic_expansion() {
        let mut env = EnvironmentManager::new();
        env.set("FOO".to_string(), "bar".to_string()).unwrap();

        let result = expand_variables_in_string("echo $FOO", &env);
        assert_eq!(result, "echo bar");
    }

    #[test]
    fn test_braced_expansion() {
        let mut env = EnvironmentManager::new();
        env.set("HOME".to_string(), "/Users/test".to_string()).unwrap();

        let result = expand_variables_in_string("${HOME}_backup", &env);
        assert_eq!(result, "/Users/test_backup");
    }

    #[test]
    fn test_undefined_var() {
        let env = EnvironmentManager::new();
        let result = expand_variables_in_string("$UNDEFINED", &env);
        assert_eq!(result, "");
    }

    #[test]
    fn test_escaped_dollar() {
        let env = EnvironmentManager::new();
        let result = expand_variables_in_string("\\$HOME", &env);
        assert_eq!(result, "$HOME");
    }
}
```

---

### PR #2: Export Builtin (US2)

**Goal**: `export VAR=value` sets variables for child processes

#### Step 1: Create export.rs

**File**: `crates/rush/src/executor/builtins/export.rs` (NEW)

```rust
//! 'export' built-in command

use crate::error::{Result, RushError};
use crate::executor::execute::CommandExecutor;
use crate::executor::parser::expand_variables_in_string;

pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32> {
    if args.is_empty() {
        // No args: list all variables (like set)
        let env = executor.env_manager();
        let mut vars: Vec<_> = env.iter().collect();
        vars.sort_by_key(|(k, _)| k.as_str());
        for (name, value) in vars {
            println!("{}={}", name, value);
        }
        return Ok(0);
    }

    for arg in args {
        // Parse VAR=value
        let (name, value) = match arg.split_once('=') {
            Some((n, v)) => (n.to_string(), v.to_string()),
            None => {
                return Err(RushError::Execution(format!(
                    "export: invalid syntax '{}', expected VAR=value",
                    arg
                )));
            }
        };

        // Expand variables in value
        let expanded_value = expand_variables_in_string(&value, executor.env_manager());

        // Set the variable
        executor
            .env_manager_mut()
            .set(name.clone(), expanded_value)
            .map_err(|e| RushError::Execution(e.to_string()))?;
    }

    Ok(0)
}
```

#### Step 2: Register in builtins/mod.rs

```rust
pub mod export;

pub fn execute_builtin(...) -> Option<Result<i32>> {
    match command {
        // ... existing
        "export" => Some(export::execute(executor, args)),
        _ => None,
    }
}
```

#### Step 3: Pass environment to child processes

**File**: `crates/rush/src/executor/pipeline.rs`

```rust
// In execute_single and MultiCommandExecution::spawn:
fn spawn_command(segment: &PipelineSegment, env: &HashMap<String, String>) -> Result<Child> {
    let mut cmd = Command::new(&segment.program);
    cmd.args(&segment.args);

    // Clear inherited env, use shell's managed environment
    cmd.env_clear().envs(env);

    // ... rest of spawn logic
}
```

---

### PR #3: Set Builtin (US3)

**Goal**: `set` lists all environment variables

#### Step 1: Create set.rs

**File**: `crates/rush/src/executor/builtins/set.rs` (NEW)

```rust
//! 'set' built-in command

use crate::error::Result;
use crate::executor::execute::CommandExecutor;

pub fn execute(executor: &mut CommandExecutor, _args: &[String]) -> Result<i32> {
    let env = executor.env_manager();
    let mut vars: Vec<_> = env.iter().collect();
    vars.sort_by_key(|(k, _)| k.as_str());

    for (name, value) in vars {
        println!("{}={}", name, value);
    }

    Ok(0)
}
```

#### Step 2: Register in builtins/mod.rs

```rust
pub mod set;

pub fn execute_builtin(...) -> Option<Result<i32>> {
    match command {
        // ... existing
        "set" => Some(set::execute(executor, args)),
        _ => None,
    }
}
```

---

### PR #4: Polish & Edge Cases

**Goal**: Handle edge cases, add integration tests

#### Key Edge Cases to Handle

1. Quote handling (single vs double)
2. Invalid variable names
3. Empty values
4. Multiple variables in one line

#### Integration Test Example

**File**: `crates/rush/tests/env_vars.rs`

```rust
use rush::executor::CommandExecutor;

#[test]
fn test_export_and_echo() {
    let mut executor = CommandExecutor::new();

    // Export a variable
    let result = executor.execute("export TEST_VAR=hello");
    assert!(result.is_ok());

    // Check it's in the environment
    assert_eq!(
        executor.env_manager().get("TEST_VAR"),
        Some("hello")
    );
}
```

---

## Verification Checklist

After each PR, verify:

- [ ] `cargo build` succeeds
- [ ] `cargo test` passes
- [ ] `cargo clippy` has no warnings
- [ ] `cargo fmt --check` passes

### Manual Testing

```bash
# After PR #1
cargo run -p rush
rush> echo $HOME
/Users/username

# After PR #2
rush> export FOO=bar
rush> echo $FOO
bar

# After PR #3
rush> set
HOME=/Users/username
FOO=bar
...
```

---

## Common Issues

### Issue: Variables not expanding

**Check**:
1. Is `expand_variables()` called before builtin check?
2. Is the tokenizer preserving `$` character?

### Issue: Child process doesn't see variables

**Check**:
1. Is `cmd.env_clear().envs()` being called?
2. Is the environment map being passed through pipeline?

### Issue: Invalid variable name accepted

**Check**:
1. Is `is_valid_variable_name()` being called in `set()`?
2. Check regex: `^[a-zA-Z_][a-zA-Z0-9_]*$`
