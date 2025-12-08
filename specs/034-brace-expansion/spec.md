# Feature Specification: Brace Expansion

**Feature Branch**: `034-brace-expansion`
**Created**: 2025-12-08
**Status**: Draft
**Dependencies**: 009-globbing

## Overview

Implement brace expansion, a shell feature that generates multiple text strings from a pattern containing braces. Brace expansion occurs before any other expansion and produces a list of words by expanding comma-separated alternatives or numeric/character sequences.

## Syntax

### Comma-Separated List
- `{a,b,c}` - Expands to: `a b c`
- `pre{a,b}post` - Expands to: `preapost prebpost`

### Numeric Sequence
- `{1..5}` - Expands to: `1 2 3 4 5`
- `{5..1}` - Expands to: `5 4 3 2 1` (reverse)
- `{1..10..2}` - Expands to: `1 3 5 7 9` (with step)
- `{01..05}` - Expands to: `01 02 03 04 05` (zero-padded)

### Character Sequence
- `{a..e}` - Expands to: `a b c d e`
- `{e..a}` - Expands to: `e d c b a` (reverse)
- `{a..z..2}` - Expands to: `a c e g i k m o q s u w y` (with step)

### Nested Braces
- `{a,{b,c}}` - Expands to: `a b c`
- `{a,b{1,2}}` - Expands to: `a b1 b2`

## Examples

```bash
# Basic comma expansion
echo {cat,dog,bird}
# Output: cat dog bird

# With prefix and suffix
echo file{1,2,3}.txt
# Output: file1.txt file2.txt file3.txt

# Numeric ranges
echo {1..5}
# Output: 1 2 3 4 5

# Reverse range
echo {5..1}
# Output: 5 4 3 2 1

# With step
echo {0..10..2}
# Output: 0 2 4 6 8 10

# Zero-padded numbers
echo {001..005}
# Output: 001 002 003 004 005

# Character ranges
echo {a..e}
# Output: a b c d e

# Creating multiple directories
mkdir -p project/{src,tests,docs}

# Multiple file extensions
ls *.{js,ts,json}

# Nested braces
echo {a,b{1,2},c}
# Output: a b1 b2 c

# Combining patterns
echo {a..c}{1..3}
# Output: a1 a2 a3 b1 b2 b3 c1 c2 c3
```

## Expansion Order

Brace expansion MUST occur before other expansions in this order:
1. **Brace expansion** (this feature)
2. Tilde expansion
3. Parameter/variable expansion
4. Command substitution
5. Arithmetic expansion
6. Word splitting
7. Filename expansion (globbing)

## Functional Requirements

### Core Expansion
- **FR-001**: System MUST expand `{a,b,c}` to separate words `a`, `b`, `c`
- **FR-002**: System MUST preserve preamble and postscript: `pre{a,b}post` → `preapost prebpost`
- **FR-003**: System MUST support empty alternatives: `{a,,b}` → `a` `` `b`

### Numeric Sequences
- **FR-004**: System MUST expand `{x..y}` for integers x and y
- **FR-005**: System MUST support reverse sequences when y < x
- **FR-006**: System MUST support step increment `{x..y..incr}`
- **FR-007**: System MUST preserve leading zeros: `{01..03}` → `01 02 03`
- **FR-008**: System MUST use the widest number's padding for all outputs

### Character Sequences
- **FR-009**: System MUST expand `{a..z}` for single ASCII characters
- **FR-010**: System MUST support reverse character sequences
- **FR-011**: System MUST support step increment for characters

### Nesting
- **FR-012**: System MUST support nested brace expansions
- **FR-013**: Nested braces MUST expand from innermost to outermost

### Edge Cases
- **FR-014**: Single element `{a}` MUST NOT expand (literal `{a}`)
- **FR-015**: Unmatched braces MUST be treated literally
- **FR-016**: Braces inside quotes MUST NOT expand
- **FR-017**: Escaped braces `\{` MUST be treated literally
- **FR-018**: Invalid sequences `{a..1}` (mixing char/number) MUST be treated literally

## Edge Cases

- Empty braces `{}` - treated literally as `{}`
- Single item `{a}` - treated literally as `{a}` (no expansion)
- Unbalanced braces `{a,b` - treated literally
- Mixed type sequence `{a..5}` - treated literally (no expansion)
- Very large ranges `{1..1000000}` - should work but may be slow
- Negative numbers `{-5..5}` - should expand correctly
- Braces in quotes `"{a,b}"` - no expansion, literal string
- Escaped braces `\{a,b\}` - no expansion, literal braces

## Success Criteria

- **SC-001**: All comma-separated brace expansions produce correct word lists
- **SC-002**: All numeric sequences expand correctly including reverse and stepped
- **SC-003**: All character sequences expand correctly including reverse and stepped
- **SC-004**: Zero-padding is preserved and applied consistently
- **SC-005**: Nested braces expand correctly from inside out
- **SC-006**: Edge cases (quoted, escaped, invalid) are handled without errors
- **SC-007**: Brace expansion occurs before glob expansion
- **SC-008**: All existing tests continue to pass
