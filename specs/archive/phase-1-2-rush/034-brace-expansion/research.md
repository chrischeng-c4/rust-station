# Research: Brace Expansion

**Feature**: 034-brace-expansion
**Date**: 2025-12-08

## Existing Codebase Analysis

### Expansion Pipeline Location

**File**: `crates/rush/src/executor/execute.rs`

Current pipeline order (lines 137-149):
1. Alias expansion (`alias_manager.expand()`)
2. Variable expansion (`expand_variables()`)
3. Arithmetic expansion (`expand_arithmetic()`)
4. Glob expansion (`glob_expand()`)
5. Command parsing (`parse_pipeline()`)

**Integration Point**: Insert brace expansion between alias and variable expansion (line ~138).

### Module Pattern Reference

**Arithmetic Expansion** (`crates/rush/src/executor/arithmetic/`):
- `mod.rs` - Module coordination and exports
- `expander.rs` - Main `expand_arithmetic()` entry point
- `lexer.rs` - Tokenization
- `parser.rs` - AST construction
- `evaluator.rs` - Expression evaluation

**Decision**: Follow this pattern for brace expansion with 4 files:
- `mod.rs`, `expander.rs`, `lexer.rs`, `parser.rs`

### Quote Handling

**File**: `crates/rush/src/executor/parser.rs` (lines 391-422)

Quote tracking pattern:
```rust
let mut in_single_quote = false;
let mut in_double_quote = false;
let mut escape_next = false;
```

**Decision**: Replicate this pattern in brace lexer. Single quotes and double quotes both prevent brace expansion.

### Glob Expansion Reference

**File**: `crates/rush/src/executor/glob.rs`

Key functions:
- `glob_expand()` (line 42) - Main entry point
- `expand_single_pattern()` (line 129) - Per-pattern expansion

Quote handling approach:
- Tracks quote state during scan
- Quoted patterns returned unchanged
- Escaped characters treated literally

**Decision**: Use similar scanning approach for brace detection.

## Bash Reference Behavior

### Expansion Order

From bash manual, brace expansion is performed BEFORE all other expansions:
1. **Brace expansion** ← First
2. Tilde expansion
3. Parameter/variable expansion
4. Command substitution
5. Arithmetic expansion
6. Word splitting
7. Filename expansion (globbing)

**Decision**: Place brace expansion immediately after alias expansion, before variable expansion.

### Comma Lists

```bash
$ echo {a,b,c}
a b c

$ echo pre{a,b}post
preapost prebpost

$ echo {a,,b}    # Empty element
a  b
```

**Decision**: Empty elements produce empty strings in output.

### Numeric Sequences

```bash
$ echo {1..5}
1 2 3 4 5

$ echo {5..1}      # Reverse
5 4 3 2 1

$ echo {1..10..2}  # Step
1 3 5 7 9

$ echo {01..05}    # Zero-padding
01 02 03 04 05

$ echo {001..100}  # Width from widest
001 002 ... 100

$ echo {-5..5}     # Negative numbers
-5 -4 -3 -2 -1 0 1 2 3 4 5
```

**Decision**:
- Step defaults to 1 (or -1 for reverse)
- Padding width = max(len(start), len(end))
- Negative numbers supported

### Character Sequences

```bash
$ echo {a..e}
a b c d e

$ echo {e..a}      # Reverse
e d c b a

$ echo {a..z..2}   # Step
a c e g i k m o q s u w y
```

**Decision**: Use ASCII values for character iteration. Step applies to ASCII distance.

### Nested Braces

```bash
$ echo {a,b{1,2},c}
a b1 b2 c

$ echo {{a,b},{c,d}}
a b c d

$ echo {a,{b,{c,d}}}
a b c d
```

**Decision**: Expand innermost braces first, then outer. Recursive algorithm.

### Non-Expansion Cases

```bash
$ echo {a}         # Single element - no expansion
{a}

$ echo {}          # Empty - no expansion
{}

$ echo {a,b        # Unmatched - literal
{a,b

$ echo \{a,b\}     # Escaped - literal
{a,b}

$ echo '{a,b}'     # Single quoted - literal
{a,b}

$ echo "{a,b}"     # Double quoted - literal
{a,b}

$ echo {a..z       # Invalid sequence - literal
{a..z

$ echo {a..5}      # Mixed types - literal
{a..5}
```

**Decision**: All these cases return the input unchanged.

## Algorithm Design

### Scanning Algorithm

```
function find_brace_pattern(input):
    for each character c at position i:
        if escaped or in_quote:
            continue
        if c == '{':
            find matching '}'
            if found and is_valid_pattern:
                return (start, end, content)
    return None
```

### Validity Checking

A brace pattern `{content}` is valid if:
1. Contains comma (list): `{a,b}` → valid
2. Contains `..` (sequence): `{1..5}` → check sequence validity
3. Otherwise: invalid (e.g., `{a}`, `{}`)

Sequence validity:
- `{int..int}` → valid numeric
- `{int..int..int}` → valid numeric with step
- `{char..char}` → valid character (single ASCII chars)
- `{char..char..int}` → valid character with step
- Otherwise → invalid

### Expansion Algorithm

```
function expand_word(word):
    pattern = find_brace_pattern(word)
    if pattern is None:
        return [word]

    preamble = word[0:pattern.start]
    postscript = word[pattern.end+1:]
    content = pattern.content

    alternatives = expand_content(content)

    results = []
    for alt in alternatives:
        expanded = expand_word(preamble + alt + postscript)
        results.extend(expanded)

    return results
```

### Cartesian Product

For adjacent braces like `{a,b}{1,2}`:
1. First find leftmost brace pattern
2. Expand it: `a{1,2}`, `b{1,2}`
3. Recursively expand each result
4. Result: `a1 a2 b1 b2`

## Performance Considerations

### Large Ranges

`{1..1000000}` generates 1 million strings.

**Mitigation**:
- Use iterator/lazy generation where possible
- Pre-allocate vector with estimated capacity
- No explicit limit (let user control)

### Deeply Nested

`{a,{b,{c,{d,{e,f}}}}}` - recursion depth = nesting level.

**Mitigation**:
- Typical nesting is 1-2 levels
- Rust stack handles reasonable depths
- No explicit limit needed

### Memory

`{a..z}{0..9}{a..z}` = 26 × 10 × 26 = 6,760 strings.

**Mitigation**:
- Estimate result size from pattern analysis
- Pre-allocate with capacity

## Decisions Summary

| Topic | Decision | Rationale |
|-------|----------|-----------|
| Pipeline position | After alias, before variable | Matches bash order |
| Module structure | 4 files (mod, expander, lexer, parser) | Matches arithmetic pattern |
| Quote handling | Both quote types prevent expansion | Matches existing glob behavior |
| Empty elements | Produce empty strings | Matches bash |
| Zero padding | Width from widest number | Matches bash |
| Nested expansion | Recursive, innermost first | Matches bash |
| Invalid patterns | Return literal | Matches bash |
| Performance limits | None explicit | Let user control |
