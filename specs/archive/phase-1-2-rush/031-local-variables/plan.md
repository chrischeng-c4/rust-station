# Implementation Plan: Local Variables

**Branch**: `031-local-variables` | **Date**: 2025-12-07 | **Spec**: [spec.md](./spec.md)

## Summary

Implement the `local` shell builtin for declaring function-scoped variables. This enables proper variable scoping in shell functions.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: None (pure Rust)
**Storage**: Scope stack in VariableManager
**Testing**: cargo test
**Target Platform**: macOS
**Performance Goals**: <1ms scope push/pop

## Constitution Check

| Principle | Status | Notes |
|-----------|--------|-------|
| I. Performance-First | PASS | Simple stack operations |
| II. Zero-Config | PASS | Works automatically in functions |
| III. Progressive Complexity | PASS | Simple usage, powerful scoping |
| IV. Modern UX | PASS | Standard bash-compatible behavior |
| V. Rust-Native | PASS | Pure Rust, no dependencies |

## Implementation Design

### Architecture

1. **Add scope stack to VariableManager**:
   - `scope_stack: Vec<HashMap<String, Option<Variable>>>` - saved values for each scope
   - `push_scope()` - enter new scope (function entry)
   - `pop_scope()` - restore previous scope (function exit)
   - `set_local(name, value)` - mark variable as local to current scope

2. **Add function depth tracking to CommandExecutor**:
   - `function_depth: usize` - tracks nested function calls
   - Increment on function entry, decrement on exit

3. **Create `local` builtin**:
   - Parse `local var=value` or `local var` syntax
   - Error if not in function context (function_depth == 0)
   - Save current value (if any) to scope stack
   - Set new value in variables

### Key Changes

**variables.rs**:
```rust
pub struct VariableManager {
    variables: HashMap<String, Variable>,
    exported: HashSet<String>,
    scope_stack: Vec<HashMap<String, Option<Variable>>>,  // NEW
}

impl VariableManager {
    pub fn push_scope(&mut self);  // NEW
    pub fn pop_scope(&mut self);   // NEW
    pub fn set_local(&mut self, name: String, value: String) -> Result<()>;  // NEW
}
```

**builtins/local.rs** (NEW):
```rust
pub fn execute(executor: &mut CommandExecutor, args: &[String]) -> Result<i32>
```

### Exit Status

- 0: Success
- 1: Used outside function context
- 2: Invalid variable name
