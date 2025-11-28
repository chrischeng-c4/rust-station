# Implementation Plan: Environment Variables

**Feature:** 014-environment-variables
**Created:** 2025-11-28

## Implementation Approach

Environment variables will be implemented with:
1. **VariableManager** - HashMap-based storage with export tracking
2. **set/export/unset** builtins - Manage variables
3. **Variable Expansion** - Expand $VAR before command execution
4. **Special Variables** - Support $$, $?, $0, etc.

## Architecture

### Module Structure
```
crates/rush/src/executor/
├── mod.rs          # Add VariableManager to CommandExecutor
├── execute.rs      # Add variable expansion before command parsing
├── variables.rs    # New: VariableManager implementation
└── builtins/
    ├── mod.rs      # Register set, export, unset
    ├── set.rs      # New: set builtin
    ├── export.rs   # New: export builtin
    └── unset.rs    # New: unset builtin
```

### VariableManager

```rust
pub struct VariableManager {
    variables: HashMap<String, String>,
    exported: HashSet<String>,
}

impl VariableManager {
    pub fn new() -> Self
    pub fn set(&mut self, name: String, value: String) -> Result<()>
    pub fn get(&self, name: &str) -> Option<&str>
    pub fn remove(&mut self, name: &str) -> bool
    pub fn export(&mut self, name: &str) -> Result<()>
    pub fn is_exported(&self, name: &str) -> bool
    pub fn list(&self) -> Vec<(&str, &str)>
    pub fn list_exported(&self) -> Vec<(&str, &str)>
    pub fn to_env_vec(&self) -> Vec<String>  // For passing to subshells
}
```

## Implementation Steps

### Step 1: Create variables.rs Module

**File:** `crates/rush/src/executor/variables.rs`

Implement VariableManager with:
- HashMap<String, String> for storage
- HashSet<String> for tracking exported vars
- Methods: set, get, remove, export, is_exported, list
- Validation for variable names (alphanumeric + underscore)
- Unit tests (10+ tests)

### Step 2: Integrate into CommandExecutor

**File:** `crates/rush/src/executor/mod.rs` or `execute.rs`

Add to CommandExecutor struct:
```rust
pub struct CommandExecutor {
    pipeline_executor: PipelineExecutor,
    job_manager: JobManager,
    alias_manager: AliasManager,
    variable_manager: VariableManager,  // New
    last_exit_code: i32,                // Track for $?
}
```

Add accessor methods:
```rust
pub fn variable_manager(&self) -> &VariableManager
pub fn variable_manager_mut(&mut self) -> &mut VariableManager
pub fn set_last_exit_code(&mut self, code: i32)
pub fn last_exit_code(&self) -> i32
```

### Step 3: Implement Variable Expansion

**File:** `crates/rush/src/executor/execute.rs`

Add new module `crates/rush/src/executor/expansion.rs`:

```rust
pub fn expand_variables(
    input: &str,
    executor: &CommandExecutor,
) -> Result<String> {
    // Expand $VAR, ${VAR}, $$, $?, $0, $#
}
```

Algorithm:
1. Iterate through input string
2. When encountering '$':
   - If followed by '{': extract until '}', look up variable
   - If followed by digit/special: extract special variable
   - Otherwise: extract identifier, look up variable
3. Replace with value (or empty string if not found)
4. Handle escape sequences (\$)

Integration in execute():
```rust
pub fn execute(&mut self, line: &str) -> Result<i32> {
    // ... existing code ...

    // Expand variables BEFORE parsing
    let expanded = expand_variables(line, self)?;

    // Parse expanded line
    let pipeline = parse_pipeline(&expanded)?;

    // ... rest of execution ...
}
```

### Step 4: Create set Builtin

**File:** `crates/rush/src/executor/builtins/set.rs`

```rust
pub fn set(executor: &mut CommandExecutor, args: &[String]) -> Result<i32> {
    if args.is_empty() {
        // List all variables
        let vars = executor.variable_manager().list();
        for (name, value) in vars {
            println!("{}={}", name, value);
        }
        return Ok(0);
    }

    // set NAME=value or set NAME
    for arg in args {
        if let Some(pos) = arg.find('=') {
            let name = &arg[..pos];
            let value = &arg[pos + 1..];
            // Validate name
            executor.variable_manager_mut().set(name.to_string(), value.to_string())?;
        } else {
            // Show specific variable
            if let Some(value) = executor.variable_manager().get(arg) {
                println!("{}={}", arg, value);
            } else {
                eprintln!("rush: set: {}: not set", arg);
                return Ok(1);
            }
        }
    }
    Ok(0)
}
```

Tests:
- Set and retrieve variable
- List all variables
- Show specific variable
- Show non-existent variable (error)
- Variable with spaces in value

### Step 5: Create export Builtin

**File:** `crates/rush/src/executor/builtins/export.rs`

```rust
pub fn export(executor: &mut CommandExecutor, args: &[String]) -> Result<i32> {
    if args.is_empty() {
        // List exported variables
        let vars = executor.variable_manager().list_exported();
        for (name, value) in vars {
            println!("export {}={}", name, value);
        }
        return Ok(0);
    }

    for arg in args {
        if let Some(pos) = arg.find('=') {
            // export NAME=value
            let name = &arg[..pos];
            let value = &arg[pos + 1..];
            executor.variable_manager_mut().set(name.to_string(), value.to_string())?;
            executor.variable_manager_mut().export(name)?;
        } else {
            // export NAME (mark as exported)
            executor.variable_manager_mut().export(arg)?;
        }
    }
    Ok(0)
}
```

Tests:
- Export new variable
- Export existing variable
- List exported variables
- Variable visible to subshells

### Step 6: Create unset Builtin

**File:** `crates/rush/src/executor/builtins/unset.rs`

```rust
pub fn unset(executor: &mut CommandExecutor, args: &[String]) -> Result<i32> {
    if args.is_empty() {
        eprintln!("rush: unset: usage: unset name [name ...]");
        return Ok(1);
    }

    let mut exit_code = 0;
    for name in args {
        if !executor.variable_manager_mut().remove(name) {
            eprintln!("rush: unset: {}: not found", name);
            exit_code = 1;
        }
    }
    Ok(exit_code)
}
```

Tests:
- Unset existing variable
- Unset non-existent variable (error)
- Unset multiple variables

### Step 7: Register Builtins

**File:** `crates/rush/src/executor/builtins/mod.rs`

Add module declarations:
```rust
pub mod export;
pub mod set;
pub mod unset;
```

Add to match statement:
```rust
pub fn execute_builtin(executor: &mut CommandExecutor, command: &str, args: &[String]) -> Option<Result<i32>> {
    match command {
        "set" => Some(set::set(executor, args)),
        "export" => Some(export::export(executor, args)),
        "unset" => Some(unset::unset(executor, args)),
        "cd" => Some(cd::execute(executor, args)),
        // ... rest ...
    }
}
```

### Step 8: Add Integration Tests

**File:** `crates/rush/tests/integration/variables_tests.rs`

Tests:
- Set and use variable in command
- Export variable to subshell
- Unset variable
- Variable expansion in different contexts
- Special variables ($$, $?, $0)
- Variable with spaces
- Recursive variable expansion

## Testing Strategy

### Unit Tests (in each module)
- VariableManager: 15+ tests
  - set/get/remove
  - export tracking
  - list operations
  - name validation

- set builtin: 8+ tests
  - set NAME=value
  - set NAME
  - set (list all)
  - Error cases

- export builtin: 8+ tests
  - export NAME=value
  - export NAME (existing var)
  - export (list exported)

- unset builtin: 5+ tests
  - unset single
  - unset multiple
  - error cases

- Variable expansion: 15+ tests
  - $VAR basic
  - ${VAR} syntax
  - $$, $?, $0
  - Non-existent vars
  - In quoted strings

### Integration Tests
- Set variable and use in command
- Export to subprocess
- Multiple variables
- Complex expansion cases

## Special Variables Implementation

| Variable | Value | Example |
|----------|-------|---------|
| `$$` | Process ID | `$$` → 12345 |
| `$?` | Last exit code | `$?` → 0 |
| `$0` | Shell name | `$0` → rush |
| `$#` | Arg count | `$#` → 2 |
| `$1`, `$2`, ... | Args | `$1` → first_arg |

Implementation:
- `$$` - Get via `std::process::id()`
- `$?` - Store last_exit_code in CommandExecutor
- `$0` - Return literal "rush"
- `$#` and `$N` - Not used in interactive shell, for scripts

## Error Handling

| Error | Message | Exit Code |
|-------|---------|-----------|
| Invalid var name | `rush: set: NAME: invalid identifier` | 1 |
| Not set | `rush: set: NAME: not set` | 1 |
| Not found | `rush: unset: NAME: not found` | 1 |
| No arguments | `rush: unset: usage: ...` | 1 |

## Success Criteria

- [ ] All 4 user stories implemented
- [ ] VariableManager fully tested (15+ tests)
- [ ] All builtins fully tested (20+ tests)
- [ ] Variable expansion works in all contexts
- [ ] Special variables work correctly
- [ ] Exported variables visible to subshells
- [ ] No clippy warnings
- [ ] Formatted with cargo fmt
- [ ] All tests pass

## Dependencies

- `std::collections::{HashMap, HashSet}` - Variable storage
- `std::process::id()` - For $$ expansion
- Existing parser and command execution infrastructure

No new external dependencies required.
