# Feature Specification: Arithmetic Expansion

**Feature Branch**: `029-arithmetic-expansion`
**Created**: 2025-12-07
**Status**: Draft
**Input**: User description: "Arithmetic expansion $((expr)), let, (())"

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic Arithmetic Expansion (Priority: P1)

A shell user wants to perform arithmetic calculations inline within commands using the `$((expression))` syntax, allowing them to compute values without external tools like `bc` or `expr`.

**Why this priority**: This is the most common arithmetic syntax in POSIX-compatible shells and is essential for scripting. Nearly all shell scripts requiring math use this syntax.

**Independent Test**: Can be fully tested by running `echo $((2 + 3))` and verifying it outputs `5`.

**Acceptance Scenarios**:

1. **Given** an arithmetic expression `$((5 + 3))`, **When** the shell expands it, **Then** the result `8` is substituted in place.
2. **Given** an expression with variables `x=10; echo $((x * 2))`, **When** expanded, **Then** the output is `20`.
3. **Given** nested arithmetic `$((1 + $((2 * 3))))`, **When** expanded, **Then** the result is `7`.

---

### User Story 2 - Arithmetic Operators (Priority: P1)

A shell user needs access to standard arithmetic operators including addition, subtraction, multiplication, division, modulo, and exponentiation to perform calculations.

**Why this priority**: Without operators, arithmetic expansion has no utility. This is co-equal with basic syntax.

**Independent Test**: Can be tested by evaluating each operator: `echo $((10 / 3))` outputs `3`, `echo $((10 % 3))` outputs `1`.

**Acceptance Scenarios**:

1. **Given** expression `$((10 + 5))`, **When** expanded, **Then** result is `15`.
2. **Given** expression `$((10 - 5))`, **When** expanded, **Then** result is `5`.
3. **Given** expression `$((10 * 5))`, **When** expanded, **Then** result is `50`.
4. **Given** expression `$((10 / 3))`, **When** expanded, **Then** result is `3` (integer division).
5. **Given** expression `$((10 % 3))`, **When** expanded, **Then** result is `1`.
6. **Given** expression `$((2 ** 8))`, **When** expanded, **Then** result is `256`.

---

### User Story 3 - Comparison and Logical Operators (Priority: P2)

A shell user needs comparison and logical operators within arithmetic expressions to evaluate conditions that return 1 (true) or 0 (false).

**Why this priority**: Essential for conditional logic in scripts but can be worked around with `test`/`[` commands.

**Independent Test**: Can be tested with `echo $((5 > 3))` outputting `1` and `echo $((5 < 3))` outputting `0`.

**Acceptance Scenarios**:

1. **Given** expression `$((5 > 3))`, **When** expanded, **Then** result is `1`.
2. **Given** expression `$((5 < 3))`, **When** expanded, **Then** result is `0`.
3. **Given** expression `$((5 == 5))`, **When** expanded, **Then** result is `1`.
4. **Given** expression `$((5 != 3))`, **When** expanded, **Then** result is `1`.
5. **Given** expression `$((5 >= 5))`, **When** expanded, **Then** result is `1`.
6. **Given** expression `$((5 <= 3))`, **When** expanded, **Then** result is `0`.
7. **Given** expression `$((1 && 0))`, **When** expanded, **Then** result is `0`.
8. **Given** expression `$((1 || 0))`, **When** expanded, **Then** result is `1`.
9. **Given** expression `$((!0))`, **When** expanded, **Then** result is `1`.

---

### User Story 4 - Bitwise Operators (Priority: P2)

A shell user working with binary data or flags needs bitwise operations within arithmetic expressions.

**Why this priority**: Important for systems programming scripts but less commonly used than basic arithmetic.

**Independent Test**: Can be tested with `echo $((5 & 3))` outputting `1` and `echo $((5 | 3))` outputting `7`.

**Acceptance Scenarios**:

1. **Given** expression `$((5 & 3))`, **When** expanded, **Then** result is `1` (bitwise AND).
2. **Given** expression `$((5 | 3))`, **When** expanded, **Then** result is `7` (bitwise OR).
3. **Given** expression `$((5 ^ 3))`, **When** expanded, **Then** result is `6` (bitwise XOR).
4. **Given** expression `$((~0))`, **When** expanded, **Then** result is `-1` (bitwise NOT).
5. **Given** expression `$((1 << 4))`, **When** expanded, **Then** result is `16` (left shift).
6. **Given** expression `$((16 >> 2))`, **When** expanded, **Then** result is `4` (right shift).

---

### User Story 5 - Variable Assignment in Arithmetic (Priority: P2)

A shell user wants to assign values to variables within arithmetic expressions, supporting assignment operators like `=`, `+=`, `-=`, etc.

**Why this priority**: Enables concise increment/decrement patterns common in loops.

**Independent Test**: Can be tested with `x=5; echo $((x += 3))` outputting `8` and then `echo $x` outputting `8`.

**Acceptance Scenarios**:

1. **Given** `x=5` and expression `$((x = 10))`, **When** expanded, **Then** result is `10` and `$x` is `10`.
2. **Given** `x=5` and expression `$((x += 3))`, **When** expanded, **Then** result is `8` and `$x` is `8`.
3. **Given** `x=10` and expression `$((x -= 3))`, **When** expanded, **Then** result is `7` and `$x` is `7`.
4. **Given** `x=5` and expression `$((x *= 2))`, **When** expanded, **Then** result is `10` and `$x` is `10`.
5. **Given** `x=10` and expression `$((x /= 2))`, **When** expanded, **Then** result is `5` and `$x` is `5`.
6. **Given** `x=10` and expression `$((x %= 3))`, **When** expanded, **Then** result is `1` and `$x` is `1`.
7. **Given** `x=5` and expression `$((x++))`, **When** expanded, **Then** result is `5` (post-increment) and `$x` is `6`.
8. **Given** `x=5` and expression `$((++x))`, **When** expanded, **Then** result is `6` (pre-increment) and `$x` is `6`.

---

### User Story 6 - The let Builtin (Priority: P2)

A shell user wants to use the `let` builtin to perform arithmetic without command substitution syntax, especially for variable assignments.

**Why this priority**: Provides an alternative syntax preferred by some users and common in bash scripts.

**Independent Test**: Can be tested with `let x=5+3; echo $x` outputting `8`.

**Acceptance Scenarios**:

1. **Given** command `let x=5+3`, **When** executed, **Then** `$x` is `8` and exit status is `0`.
2. **Given** command `let "x = 5 + 3"`, **When** executed, **Then** `$x` is `8` (spaces allowed in quotes).
3. **Given** command `let x=5 y=10`, **When** executed, **Then** `$x` is `5` and `$y` is `10`.
4. **Given** `x=5` and command `let x++`, **When** executed, **Then** `$x` is `6`.
5. **Given** command `let "x=0"`, **When** executed, **Then** exit status is `1` (result is zero/false).
6. **Given** command `let "x=5"`, **When** executed, **Then** exit status is `0` (result is non-zero/true).

---

### User Story 7 - Arithmetic Command (()) (Priority: P3)

A shell user wants to use `(( expression ))` as a command that evaluates arithmetic and sets exit status based on result, useful in conditionals.

**Why this priority**: Syntactic sugar over `let` but commonly used in bash scripts for conditionals.

**Independent Test**: Can be tested with `(( 5 > 3 )) && echo yes` outputting `yes`.

**Acceptance Scenarios**:

1. **Given** command `(( 5 > 3 ))`, **When** executed, **Then** exit status is `0` (true).
2. **Given** command `(( 5 < 3 ))`, **When** executed, **Then** exit status is `1` (false).
3. **Given** `x=5` and command `(( x++ ))`, **When** executed, **Then** `$x` is `6`.
4. **Given** command `if (( 10 > 5 )); then echo yes; fi`, **When** executed, **Then** output is `yes`.
5. **Given** command `(( 0 ))`, **When** executed, **Then** exit status is `1` (zero is false).
6. **Given** command `(( 1 ))`, **When** executed, **Then** exit status is `0` (non-zero is true).

---

### User Story 8 - Ternary Operator (Priority: P3)

A shell user wants to use the ternary conditional operator within arithmetic expressions for inline conditionals.

**Why this priority**: Convenience feature that can be accomplished with other constructs.

**Independent Test**: Can be tested with `echo $((5 > 3 ? 1 : 0))` outputting `1`.

**Acceptance Scenarios**:

1. **Given** expression `$((5 > 3 ? 10 : 20))`, **When** expanded, **Then** result is `10`.
2. **Given** expression `$((5 < 3 ? 10 : 20))`, **When** expanded, **Then** result is `20`.
3. **Given** `x=5` and expression `$((x > 0 ? x : -x))`, **When** expanded, **Then** result is `5`.

---

### User Story 9 - Comma Operator (Priority: P3)

A shell user wants to evaluate multiple expressions in sequence, returning the last value.

**Why this priority**: Rarely used but part of complete arithmetic expression support.

**Independent Test**: Can be tested with `echo $((x=5, y=10, x+y))` outputting `15`.

**Acceptance Scenarios**:

1. **Given** expression `$((1, 2, 3))`, **When** expanded, **Then** result is `3`.
2. **Given** expression `$((x=5, y=10, x+y))`, **When** expanded, **Then** result is `15` and `$x` is `5`, `$y` is `10`.

---

### Edge Cases

- What happens when dividing by zero? Shell displays an error message and the command fails.
- What happens with integer overflow? Behavior follows standard integer semantics (wraps around).
- What happens with undefined variables? Undefined variables evaluate to `0`.
- What happens with non-numeric variable values? Non-numeric strings evaluate to `0`.
- What happens with unbalanced parentheses? Shell displays a syntax error.
- What happens with empty expression `$(())`? Result is `0`.
- What happens with whitespace `$((  ))`? Result is `0`.
- What happens with negative numbers? Negative numbers are supported, e.g., `$((-5))` is `-5`.
- What happens with leading zeros? Numbers with leading zeros are treated as octal (e.g., `$((010))` is `8`).
- What happens with hex numbers? Hex numbers with `0x` prefix are supported (e.g., `$((0x10))` is `16`).

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST support arithmetic expansion syntax `$((expression))` that evaluates the expression and substitutes the integer result.
- **FR-002**: Shell MUST support arithmetic operators: `+` (add), `-` (subtract), `*` (multiply), `/` (integer divide), `%` (modulo), `**` (exponent).
- **FR-003**: Shell MUST support unary operators: `+` (positive), `-` (negative), `!` (logical not), `~` (bitwise not).
- **FR-004**: Shell MUST support comparison operators: `<`, `>`, `<=`, `>=`, `==`, `!=` returning `1` for true, `0` for false.
- **FR-005**: Shell MUST support logical operators: `&&` (and), `||` (or).
- **FR-006**: Shell MUST support bitwise operators: `&` (and), `|` (or), `^` (xor), `<<` (left shift), `>>` (right shift).
- **FR-007**: Shell MUST support assignment operators: `=`, `+=`, `-=`, `*=`, `/=`, `%=`, `&=`, `|=`, `^=`, `<<=`, `>>=`.
- **FR-008**: Shell MUST support increment/decrement operators: `++var`, `var++`, `--var`, `var--`.
- **FR-009**: Shell MUST support ternary operator: `condition ? true_expr : false_expr`.
- **FR-010**: Shell MUST support comma operator for expression sequencing.
- **FR-011**: Shell MUST support parentheses for grouping: `$((( 2 + 3 ) * 4))`.
- **FR-012**: Shell MUST expand variables within arithmetic expressions without requiring `$` prefix.
- **FR-013**: Shell MUST support nested arithmetic expansions: `$((1 + $((2 * 3))))`.
- **FR-014**: Shell MUST implement the `let` builtin that evaluates arithmetic expressions and sets exit status.
- **FR-015**: Shell MUST implement arithmetic command `(( expression ))` that evaluates and sets exit status (0 for non-zero result, 1 for zero result).
- **FR-016**: Shell MUST support octal numbers (leading `0`) and hexadecimal numbers (leading `0x` or `0X`).
- **FR-017**: Shell MUST treat undefined or empty variables as `0` in arithmetic context.
- **FR-018**: Shell MUST treat non-numeric string values as `0` in arithmetic context.
- **FR-019**: Shell MUST display an error and fail the command when division by zero is attempted.
- **FR-020**: Shell MUST follow standard C operator precedence for arithmetic expressions.

### Assumptions

- Integer arithmetic only (no floating-point support, consistent with bash/POSIX behavior).
- 64-bit signed integer range for calculations.
- Variables referenced without `$` inside `$(())` are resolved to their values.
- Operator precedence follows C language conventions.

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All standard arithmetic expressions from bash/POSIX documentation evaluate correctly.
- **SC-002**: 100% of arithmetic test cases pass, covering all operators and edge cases.
- **SC-003**: Existing shell scripts using arithmetic expansion work without modification.
- **SC-004**: Error messages for invalid expressions are clear and identify the problem.
- **SC-005**: Performance is comparable to other operations (arithmetic expressions complete instantly for typical use).
