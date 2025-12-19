# Quickstart: Arithmetic Expansion

**Feature**: 029-arithmetic-expansion
**Date**: 2025-12-07

## Overview

This feature adds arithmetic capabilities to the rush shell:
- `$((expression))` - Arithmetic expansion (substitutes result)
- `let` - Builtin for arithmetic assignments
- `(( ))` - Arithmetic command (sets exit status)

## Quick Examples

### Basic Arithmetic
```bash
# Addition, subtraction, multiplication
echo $((5 + 3))      # Output: 8
echo $((10 - 4))     # Output: 6
echo $((6 * 7))      # Output: 42

# Division and modulo (integer only)
echo $((17 / 5))     # Output: 3
echo $((17 % 5))     # Output: 2

# Exponentiation
echo $((2 ** 10))    # Output: 1024
```

### Variables in Arithmetic
```bash
x=10
echo $((x + 5))      # Output: 15
echo $((x * 2))      # Output: 20

# No $ needed inside $(())
y=3
echo $((x + y))      # Output: 13
```

### Assignments
```bash
# Simple assignment
echo $((x = 5))      # Output: 5, sets x=5

# Compound assignments
echo $((x += 3))     # Output: 8, x is now 8
echo $((x -= 2))     # Output: 6, x is now 6
echo $((x *= 2))     # Output: 12, x is now 12

# Increment/decrement
echo $((x++))        # Output: 12 (post), x becomes 13
echo $((++x))        # Output: 14 (pre), x becomes 14
```

### Comparisons (returns 1 for true, 0 for false)
```bash
echo $((5 > 3))      # Output: 1 (true)
echo $((5 < 3))      # Output: 0 (false)
echo $((5 == 5))     # Output: 1 (true)
echo $((5 != 5))     # Output: 0 (false)
```

### Logical Operators
```bash
echo $((1 && 1))     # Output: 1 (true AND true)
echo $((1 && 0))     # Output: 0 (true AND false)
echo $((0 || 1))     # Output: 1 (false OR true)
echo $((!0))         # Output: 1 (NOT false)
```

### Bitwise Operators
```bash
echo $((5 & 3))      # Output: 1 (bitwise AND)
echo $((5 | 3))      # Output: 7 (bitwise OR)
echo $((5 ^ 3))      # Output: 6 (bitwise XOR)
echo $((1 << 4))     # Output: 16 (left shift)
echo $((16 >> 2))    # Output: 4 (right shift)
```

### The `let` Builtin
```bash
let x=5+3
echo $x              # Output: 8

let "y = 10 * 2"     # Quotes allow spaces
echo $y              # Output: 20

let a=1 b=2 c=3      # Multiple assignments
echo $((a + b + c))  # Output: 6

# Exit status based on result
let "x = 0"          # Exit status: 1 (false)
let "x = 5"          # Exit status: 0 (true)
```

### Arithmetic Command `(( ))`
```bash
# Use in conditionals
if (( x > 5 )); then
    echo "x is greater than 5"
fi

# Increment in loops
x=0
while (( x < 5 )); do
    echo $x
    (( x++ ))
done

# Exit status: 0 if non-zero, 1 if zero
(( 5 > 3 )) && echo "yes"  # Output: yes
(( 5 < 3 )) || echo "no"   # Output: no
```

### Advanced: Ternary and Comma
```bash
# Ternary operator
x=10
echo $((x > 5 ? 100 : 0))  # Output: 100

# Comma operator (evaluates all, returns last)
echo $((a=1, b=2, a+b))    # Output: 3
```

## Number Formats

```bash
# Decimal (default)
echo $((42))         # Output: 42

# Octal (leading 0)
echo $((010))        # Output: 8

# Hexadecimal (0x prefix)
echo $((0x10))       # Output: 16
echo $((0xFF))       # Output: 255
```

## Error Handling

```bash
# Division by zero
echo $((5 / 0))      # Error: division by zero

# Undefined variables default to 0
echo $((undefined))  # Output: 0

# Syntax errors
echo $((5 +))        # Error: syntax error
```

## Development

### Build and Test
```bash
cargo build
cargo test arithmetic
```

### Key Files
- `src/executor/arithmetic/` - Core arithmetic module
- `src/executor/builtins/let_cmd.rs` - let builtin
- `tests/integration/arithmetic_*_tests.rs` - Tests
