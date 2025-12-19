# Research: Read Builtin

## Terminal Input Reading

**Decision**: Use `std::io::stdin().read_line()` for basic input, `crossterm` for terminal control

**Rationale**:
- `std::io` is zero-overhead for basic reading
- `crossterm` already in project for terminal manipulation
- No additional dependencies needed

**Alternatives considered**:
- reedline: Too heavyweight for simple line reads
- libc raw mode: Less portable than crossterm

## Silent Mode Implementation

**Decision**: Use `crossterm::terminal::disable_raw_mode()` / `enable_raw_mode()` with echo disabled

**Rationale**:
- crossterm provides cross-platform terminal control
- Already used in the project for reedline integration

**Alternatives considered**:
- termios direct: Platform-specific, less maintainable

## IFS Handling

**Decision**: Default IFS = space, tab, newline; check $IFS variable if set

**Rationale**:
- POSIX standard behavior
- Consistent with bash/zsh

## Timeout Implementation

**Decision**: Use `std::time::Duration` with `std::io::stdin().read_line()` in a thread with timeout

**Rationale**:
- Simple implementation
- No additional async runtime needed for basic timeout

**Alternatives considered**:
- tokio async: Overkill for single read operation
- select/poll: Platform-specific complexity

## Backslash Interpretation

**Decision**: By default, interpret backslash escapes (\n, \t, etc.); -r flag disables

**Rationale**:
- POSIX standard behavior
- Matches bash semantics

## Array Reading (-a)

**Decision**: Defer to later when array variables are fully implemented (feature 037)

**Rationale**:
- Core read functionality works without arrays
- Array syntax requires separate feature work
- Can add -a support when arrays are ready
