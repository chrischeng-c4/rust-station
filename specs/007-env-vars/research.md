# Research: Environment Variables

**Feature**: 007-env-vars | **Date**: 2025-11-26

## Overview

Research findings for implementing environment variable support in rush shell.

## Technical Decisions

### 1. Variable Storage

**Decision**: Use `HashMap<String, String>` for in-memory storage

**Rationale**:
- O(1) average lookup time for variable expansion
- Matches `std::env::vars()` return type for easy inheritance
- Simple to serialize for child process environment
- No external dependencies needed

**Alternatives Considered**:
- `BTreeMap<String, String>` - Ordered output but slower lookup; rejected for performance
- `IndexMap` - Insertion order preserved; rejected as external dependency
- Direct `std::env` access - No session isolation; rejected for correctness

### 2. Variable Expansion Location

**Decision**: Expand variables in `parser.rs` after tokenization, before segment creation

**Rationale**:
- Tokenizer already handles quote detection
- Clean separation: parsing produces ready-to-execute commands
- Expansion happens once, not repeated for each pipeline segment
- Mirrors how bash/fish handle expansion (post-parse, pre-execute)

**Alternatives Considered**:
- During tokenization - Too early, quote context not complete
- In execute.rs - Too late, would need to pass env through pipeline
- In pipeline.rs - Wrong layer, execution shouldn't modify arguments

### 3. Quote Handling Strategy

**Decision**: Track quote type in tokenizer, expand only in double-quoted or unquoted contexts

**Rationale**:
- POSIX compliance: `'$HOME'` → literal, `"$HOME"` → expanded
- Tokenizer already tracks quote state for other purposes
- Consistent with user expectations from bash/fish experience

**Implementation**:
- Add `QuoteType` enum: `None`, `Single`, `Double`
- Modify `Token::Word` to include quote context
- `expand_variables()` checks quote type before expanding

### 4. Undefined Variable Behavior

**Decision**: Expand undefined variables to empty string (no error)

**Rationale**:
- POSIX shell behavior (bash, sh, fish all do this)
- Enables common patterns like `${VAR:-default}` later
- Less frustrating for users than errors on every typo

**Alternatives Considered**:
- Error on undefined - Too strict, breaks common patterns
- Warning message - Noisy, clutters output
- Configurable behavior - Complexity not justified for MVP

### 5. Child Process Environment

**Decision**: Use `Command::envs()` to pass full environment to child processes

**Rationale**:
- Standard Rust API, well-tested
- Replaces default inherited environment with shell's managed set
- Allows shell to modify environment without affecting parent process

**Implementation**:
```rust
// In pipeline.rs
let env_map = env_manager.as_env_map();
cmd.env_clear().envs(env_map);
```

### 6. Variable Name Validation

**Decision**: Validate on `export`, accept any name on expansion lookup

**Rationale**:
- Fail fast: catch invalid names when user tries to set them
- Forgiving: `$123` just returns empty string (undefined)
- Matches bash behavior

**Validation Regex**: `^[a-zA-Z_][a-zA-Z0-9_]*$`

## Codebase Integration Analysis

### Parser Integration (parser.rs)

**Current Flow**:
```
tokenize_with_pipes() → split_into_segments() → Pipeline::new() → validate()
```

**Modified Flow**:
```
tokenize_with_pipes() → split_into_segments() → expand_variables() → Pipeline::new() → validate()
```

**Key Insight**: The tokenizer already strips quotes but doesn't preserve quote type. Need to enhance `Token::Word` to track this.

### Builtin Integration (builtins/mod.rs)

**Current Pattern**:
```rust
pub fn execute_builtin(executor: &mut CommandExecutor, command: &str, args: &[String]) -> Option<Result<i32>> {
    match command {
        "jobs" => Some(jobs::execute(executor, args)),
        "fg" => Some(fg::execute(executor, args)),
        "bg" => Some(bg::execute(executor, args)),
        _ => None,
    }
}
```

**Addition**:
```rust
"export" => Some(export::execute(executor, args)),
"set" => Some(set::execute(executor, args)),
```

### Environment Access Pattern

**Existing Usage** (command.rs line 65):
```rust
if let Ok(path) = env::var("PATH") {
    // scan PATH directories for commands
}
```

**New Pattern**:
```rust
// In CommandExecutor
pub fn env_manager(&self) -> &EnvironmentManager { &self.env_manager }
pub fn env_manager_mut(&mut self) -> &mut EnvironmentManager { &mut self.env_manager }
```

## Performance Considerations

### Expansion Overhead

**Target**: <1ms per command (SC-002)

**Analysis**:
- Average command: 5 words, 2-3 variables
- HashMap lookup: ~50ns per variable
- String allocation: ~100ns per expansion
- Total estimate: ~0.5μs per command (well under 1ms)

**Optimization Strategy**:
- Pre-allocate result string with estimated capacity
- Use `Cow<str>` to avoid allocation when no expansion needed
- Cache common variables ($HOME, $PATH) if profiling shows need

### Memory Impact

**Estimate**:
- Typical environment: 50-100 variables
- Average variable: 30 bytes name + 100 bytes value
- Total: ~13KB for environment storage

**Constitution Compliance**: Well under 10MB baseline memory limit

## Edge Cases Documented

| Case | Input | Expected Output | Notes |
|------|-------|-----------------|-------|
| Basic expansion | `echo $HOME` | `echo /Users/user` | Standard case |
| Braced expansion | `echo ${HOME}` | `echo /Users/user` | Disambiguation |
| Adjacent text | `echo ${HOME}_backup` | `echo /Users/user_backup` | Braces required |
| Undefined | `echo $UNDEFINED` | `echo ` | Empty string |
| Escaped | `echo \$HOME` | `echo $HOME` | Literal dollar |
| Single quotes | `echo '$HOME'` | `echo $HOME` | No expansion |
| Double quotes | `echo "$HOME"` | `echo /Users/user` | Expands |
| Empty value | `export EMPTY=` | `$EMPTY` → `` | Empty, not unset |
| Invalid name | `export 123=foo` | Error | Rejected |
| In path | `ls $HOME/docs` | `ls /Users/user/docs` | Mid-argument |
| Multiple | `echo $A$B` | `echo valuea valueb` | Sequential |

## Dependencies

**Required**: None (standard library only)

**std modules used**:
- `std::env` - Initial environment inheritance
- `std::collections::HashMap` - Variable storage
- `std::process::Command` - Environment passing to children

## Testing Strategy

### Unit Tests (expansion.rs)
- Test each edge case from table above
- Test variable name validation
- Test quote handling

### Integration Tests (env_vars.rs)
- Test `export VAR=value && echo $VAR`
- Test child process inheritance
- Test `set` output format
- Test interaction with redirections

### Performance Tests
- Benchmark expansion with various input sizes
- Verify <1ms overhead requirement
