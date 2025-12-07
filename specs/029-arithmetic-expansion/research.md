# Research: Arithmetic Expansion

**Feature**: 029-arithmetic-expansion
**Date**: 2025-12-07

## Research Summary

No external research required - arithmetic expansion follows well-documented POSIX/bash standards. Implementation decisions based on codebase analysis.

## Decisions

### 1. Expression Parser Strategy

**Decision**: Pratt parser (operator-precedence parsing)
**Rationale**:
- Handles operator precedence elegantly
- Easy to extend with new operators
- Natural fit for arithmetic expressions
- Used successfully in many expression parsers

**Alternatives Considered**:
- Recursive descent: More verbose, harder to manage precedence
- Parser combinators: Would add dependency, overkill for arithmetic

### 2. AST Representation

**Decision**: Enum-based expression tree
**Rationale**:
- Type-safe representation
- Pattern matching for evaluation
- No heap allocations for simple expressions
- Follows Rust idioms

### 3. Integer Type

**Decision**: `i64` for all arithmetic
**Rationale**:
- Matches bash behavior (64-bit on modern systems)
- Sufficient for typical shell scripting
- Supports negative numbers
- No overflow checking (matches bash)

### 4. Variable Resolution

**Decision**: Resolve at evaluation time via VariableManager
**Rationale**:
- Consistent with existing variable expansion
- Allows assignments to update variables mid-expression
- Variables without `$` resolved automatically in arithmetic context

### 5. Error Handling

**Decision**: Return `Result<i64, ArithmeticError>` from evaluator
**Rationale**:
- Division by zero returns error
- Syntax errors return error
- Non-critical issues (undefined var) use default value (0)

### 6. Integration Order

**Decision**: Arithmetic expansion AFTER variable expansion, BEFORE command substitution
**Rationale**:
- Variables in expressions already expanded to values
- Command substitution may contain arithmetic (nested)
- Matches bash behavior

## Codebase Patterns to Follow

### Pattern 1: Module Structure
Follow `substitution/` module pattern:
- `mod.rs` - Public API and types
- `lexer.rs` - Tokenization
- `parser.rs` - AST building
- `evaluator.rs` - Execution
- `expander.rs` - String replacement integration

### Pattern 2: Builtin Registration
Follow existing builtins pattern in `builtins/mod.rs`:
```rust
"let" => Some(let_cmd::execute(executor, args)),
```

### Pattern 3: Error Types
Use existing `RushError` variants:
- `RushError::Syntax` for parse errors
- `RushError::Execution` for runtime errors

### Pattern 4: Testing
- Unit tests with `#[cfg(test)]` in source files
- Integration tests in `tests/integration/`
- Use `CommandExecutor::new()` and `.execute()` pattern

## References

- POSIX Shell Command Language: Arithmetic Expansion
- Bash Reference Manual: Shell Arithmetic
- Existing rush code: `substitution/`, `builtins/test.rs`
