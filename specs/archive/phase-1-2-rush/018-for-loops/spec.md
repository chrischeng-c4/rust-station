<<<<<<< HEAD
# Feature Specification: For Loop Iteration (for/in/do/done)

**Feature Branch**: `018-for-loops`
**Feature Number**: 018 (Rush Feature Roadmap)
**Created**: 2025-12-06
**Status**: Draft
**Priority**: P0 (Critical - Foundation for scripting)
**Dependencies**: Feature 017 (if/then/else - required for complex loop logic)

## User Scenarios & Testing *(mandatory)*

### User Story 1 - Basic List Iteration (Priority: P1)

A user wants to iterate over a simple list of words or filenames using a for loop. They need to execute a block of commands for each item in the list.

**Why this priority**: Core use case for loops. Most shell scripts need to iterate over lists. Essential for MVP.

**Independent Test**: Can execute `for item in one two three; do echo $item; done` and verify output contains all three items.

**Acceptance Scenarios**:

1. **Given** a shell with for/in/do/done implemented, **When** executing `for x in a b c; do echo $x; done`, **Then** outputs "a", "b", "c" on separate lines
2. **Given** a for loop with explicit list, **When** the list is empty `for x in; do echo $x; done`, **Then** loop body never executes
3. **Given** a for loop with shell variables, **When** executing `for x in $HOME /tmp; do echo $x; done`, **Then** variables are expanded before iteration

---

### User Story 2 - Command Substitution in Loops (Priority: P1)

A user needs to iterate over results from commands like `ls`, `grep`, or `find`. The for loop must support command substitution and globbing to generate the list dynamically.

**Why this priority**: Essential for real scripts; static lists alone are too limiting. Required for file processing.

**Independent Test**: Can execute `for f in $(ls); do echo $f; done` or `for f in *.txt; do echo $f; done` and verify iteration over command output/glob results.

**Acceptance Scenarios**:

1. **Given** a for loop with command substitution, **When** executing `for file in $(find /tmp -type f); do echo $file; done`, **Then** iterates over all found files
2. **Given** a for loop with globbing, **When** executing `for file in /etc/*.conf; do echo $file; done`, **Then** iterates over all .conf files
3. **Given** combined globbing and variables, **When** executing `for f in $dir/*.sh; do echo $f; done`, **Then** expands $dir and applies glob pattern

---

### User Story 3 - Nested Loops (Priority: P2)

A user needs to nest for loops to iterate over multiple dimensions (e.g., processing files in multiple directories).

**Why this priority**: Important for realistic scripts; enables matrix operations. Secondary but highly valuable.

**Independent Test**: Can execute nested for loops and verify proper iteration over both dimensions.

**Acceptance Scenarios**:

1. **Given** nested for loops, **When** executing `for dir in /tmp /var; do for f in $dir/*; do echo $f; done; done`, **Then** correctly iterates directories then files
2. **Given** nested loops with variable isolation, **When** using `i` in outer and inner loop, **Then** each loop has proper variable scope

---

### User Story 4 - Loop Variable Expansion (Priority: P1)

A user needs to access the loop variable within the loop body and expand it properly in various contexts (command args, variable expansion, string substitution).

**Why this priority**: Essential; without proper variable handling, loops are unusable. Blocks all other stories.

**Independent Test**: Can access loop variable in various expansion contexts and verify correct substitution.

**Acceptance Scenarios**:

1. **Given** a for loop with variable, **When** executing `for x in a b; do echo $x; done`, **Then** $x is correctly substituted each iteration
2. **Given** loop variable in word expansion, **When** executing `for f in a b; do echo "$f.txt"; done`, **Then** produces "a.txt" "b.txt"
3. **Given** loop variable in command argument, **When** executing `for f in 1 2; do touch /tmp/file$f; done`, **Then** creates /tmp/file1 and /tmp/file2

---

### User Story 5 - Range Expansion (Priority: P2)

A user needs to iterate over numeric or character ranges without explicitly listing all values.

**Why this priority**: Convenient feature; provides shell script expressiveness. Secondary to basic iteration.

**Independent Test**: Can execute `for i in {1..3}; do echo $i; done` and verify numeric range expansion.

**Acceptance Scenarios**:

1. **Given** numeric range, **When** executing `for i in {1..5}; do echo $i; done`, **Then** outputs 1 2 3 4 5
2. **Given** range with step, **When** executing `for i in {1..10..2}; do echo $i; done`, **Then** outputs 1 3 5 7 9
3. **Given** character range, **When** executing `for c in {a..c}; do echo $c; done`, **Then** outputs a b c

---

### User Story 6 - Loop with Arrays (Priority: P2)

A user needs to iterate over shell array variables created with feature 011 (array variables).

**Why this priority**: Important for advanced scripts; builds on array support. Secondary but enables data structure iteration.

**Independent Test**: Can iterate over shell arrays using for loop: `for x in "${arr[@]}"; do echo $x; done`.

**Acceptance Scenarios**:

1. **Given** array variable, **When** executing `arr=(a b c); for x in "${arr[@]}"; do echo $x; done`, **Then** iterates over all array elements
2. **Given** array with spaces, **When** array contains "hello world", **Then** treats as single element if properly quoted

---

### Edge Cases

- What happens with an empty list? → Loop never executes (valid behavior)
- What if loop variable name conflicts with existing variable? → Loop variable shadows existing variable, restored after loop
- What if command substitution in list fails? → Should error appropriately
- What if list contains special characters or spaces? → Proper word splitting/quoting required
- What about loop variable scope? → Should be local to loop, not persist after loop exits
- What if `for` appears at EOF with missing `done`? → Should error or wait for more input in interactive mode

## Requirements *(mandatory)*

### Functional Requirements

- **FR-001**: Shell MUST implement `for` keyword that initiates a loop with a loop variable
- **FR-002**: Shell MUST implement `in` keyword that specifies the list to iterate over
- **FR-003**: Shell MUST implement `do` keyword that marks the beginning of loop body
- **FR-004**: Shell MUST implement `done` keyword that closes a for loop
- **FR-005**: Shell MUST expand the list expression before iteration (command substitution, globbing, variable expansion)
- **FR-006**: Shell MUST iterate over each word/item in the expanded list, assigning it to the loop variable
- **FR-007**: Shell MUST execute the loop body for each list item, with loop variable set to current item
- **FR-008**: Shell MUST support nested for loops with proper variable scoping
- **FR-009**: Shell MUST support empty lists (loop body never executes, valid behavior)
- **FR-010**: Shell MUST support loop variable expansion in all contexts (command args, variable references, etc.)
- **FR-011**: Shell MUST support implicit list expansion (for x in *; without explicit in list, iterate over command line arguments)
- **FR-012**: Parser MUST recognize for/in/do/done as keywords, not command names
- **FR-013**: Loop variable scope MUST be isolated to loop body (variable changes don't persist after loop exits)
- **FR-014**: Shell MUST properly handle word splitting and quoting in list expressions

### Key Entities

- **ForStatement**: A loop construct containing a variable name, list expression, and command list (loop body)
  - Attributes: var_name (string), list_expression (command/expansion), body (command list)
  - Relationships: can be nested within other loops, if statements, or function bodies

## Success Criteria *(mandatory)*

### Measurable Outcomes

- **SC-001**: All 6 user stories have passing acceptance scenarios
- **SC-002**: Shell can execute bash/sh scripts containing for/in/do/done loops with 100% POSIX compliance
- **SC-003**: Performance: for loop parsing <1ms, loop execution overhead <5ms per item vs direct commands
- **SC-004**: Support unlimited loop nesting depth (limited only by available memory)
- **SC-005**: Parser correctly rejects 100% of malformed for statements with clear error messages
- **SC-006**: All existing tests pass; zero regressions introduced

## Assumptions

- Shell has command substitution support (feature 010) and globbing (feature 009)
- Variable expansion infrastructure exists (feature 014)
- Parser and executor from feature 001 (MVP) are extensible with loop constructs
- POSIX-compliant for loop syntax is the target (not bash-specific extensions)
- Loop variable does not need to exist before loop; it is created by the for statement

## Constraints & Dependencies

- **Depends on**: Feature 001 (MVP), Feature 009 (globbing), Feature 010 (command substitution), Feature 014 (variables)
- **Blocks**: Features 022-023 (break/continue statements require loop support)
- **Scope**: Limited to for/in/do/done syntax; range expansion is optional enhancement in feature 030
- **Shell Compatibility**: POSIX for loop syntax; bash-specific syntax (C-style for loops) out of scope
=======
# Feature 018: For/In/Do/Done Loops

**Feature ID**: 018
**Category**: Control Flow
**Priority**: High (follows immediately after conditionals)
**Dependencies**: Feature 017 (Conditional Control Flow)

## Overview

Implement POSIX-compliant `for` loops allowing iteration over word lists with command execution for each iteration. This is a fundamental control flow construct for shell scripting.

## User Stories

### US1: Basic For Loop (for/in/do/done)

**Title**: Execute commands for each word in a list
**Priority**: Critical (P1)

**Description**:
As a shell user, I want to iterate over a list of words using a for loop so I can execute commands repeatedly with different values.

**Acceptance Criteria**:
- `for var in word1 word2 ... wordN; do commands; done` syntax works correctly
- Loop variable is bound to each word sequentially
- Loop body executes for each word
- Final exit code is the exit code of the last executed command
- Loop variable persists after loop completion with the last value

**Example**:
```bash
$ for name in Alice Bob Charlie; do echo "Hello $name"; done
Hello Alice
Hello Bob
Hello Charlie
```

**Edge Cases**:
- Empty word list (loop doesn't execute, exit code 0)
- Single word in list
- Words with spaces (when quoted)
- Special characters in words (globbing should not occur in explicit word list)
- Variable expansion in word list
- Command substitution in word list

---

### US2: Default Word List (Positional Parameters)

**Title**: Iterate over positional parameters when no word list provided
**Priority**: High (P2)

**Description**:
As a shell user, I want to iterate over positional parameters using `for var; do` syntax (without explicit word list) when no arguments are provided.

**Acceptance Criteria**:
- `for var; do commands; done` iterates over `$@` (all positional parameters)
- `for var; do commands; done` is equivalent to `for var in "$@"; do commands; done`
- Works correctly with 0, 1, or multiple positional parameters
- Proper handling of parameters with spaces and special characters
- Exit code semantics match US1

**Example**:
```bash
$ function iterate() { for item; do echo $item; done; }
$ iterate apple banana cherry
apple
banana
cherry
```

**Edge Cases**:
- No positional parameters (loop doesn't execute)
- Single positional parameter
- Parameters containing spaces
- Parameters starting with special characters ($, *, etc.)

---

### US3: Word List Expansion

**Title**: Support variable expansion and command substitution in word list
**Priority**: High (P2)

**Description**:
As a shell user, I want variable expansion and command substitution to work in the word list so I can dynamically generate iteration values.

**Acceptance Criteria**:
- Variable expansion (`$VAR`, `${VAR}`) works in word list
- Command substitution (`$(cmd)` or `` `cmd` ``) works in word list
- Results are split on word boundaries (IFS)
- Brace expansion works in word list
- Globbing does NOT occur in word list (words are used as-is after expansion)
- Proper quoting prevents unwanted expansion

**Example**:
```bash
$ names="John Jane"
$ for person in $names extra; do echo $person; done
John
Jane
extra

$ for file in $(ls *.txt); do echo "Processing $file"; done
Processing file1.txt
Processing file2.txt
```

**Edge Cases**:
- Empty variable expansion (no words added)
- Variable containing multiple words (should split)
- Command substitution returning multiple lines (should split)
- Nested expansions

---

### US4: Loop Variable Scoping

**Title**: Loop variable binding and scoping rules
**Priority**: Medium (P3)

**Description**:
As a shell user, I want proper variable scoping behavior so that loop variables don't unexpectedly shadow or corrupt existing variables.

**Acceptance Criteria**:
- Loop variable is created if it doesn't exist
- Loop variable overwrites existing variable with same name
- After loop completion, loop variable retains its last assigned value
- If loop doesn't execute (empty word list), loop variable is not modified
- Loop variable is in the current shell scope (not subshell)
- Loop variable is visible to all commands in the loop body

**Example**:
```bash
$ x="original"
$ for x in one two three; do echo $x; done
one
two
three
$ echo $x
three

$ for y in; do echo "never"; done
$ echo "y is: $y"
y is:
```

**Edge Cases**:
- Loop variable shadows function parameter
- Multiple nested loops with same variable name
- Loop variable used in nested conditional or loop

---

### US5: Complex Loop Bodies

**Title**: Support complex command sequences in loop body
**Priority**: High (P2)

**Description**:
As a shell user, I want to use complex command sequences including pipes, redirections, and nested control structures in the loop body.

**Acceptance Criteria**:
- Simple command (single command) works in loop body
- Command pipeline works (`cmd1 | cmd2 | cmd3`)
- Output redirection works (`>`, `>>`, `<`)
- Multiple commands separated by semicolons work
- Nested conditionals work (if/then/else/fi inside for loop)
- Nested loops work (for/while/until inside for loop)
- Command groups work `{ cmd1; cmd2; }`
- Subshells work `( cmd1; cmd2 )`
- Exit code reflects the exit code of the last command in the loop body

**Example**:
```bash
$ for num in 1 2 3; do
>   if [ $num -eq 2 ]; then
>     echo "Found two"
>   else
>     echo "Number: $num"
>   fi
> done
Number: 1
Found two
Number: 3

$ for file in *.txt; do
>   lines=$(wc -l < "$file")
>   echo "$file has $lines lines"
> done
```

**Edge Cases**:
- Loop body is a pipeline (exit code is from last command)
- Loop body has subshell (variables modified in subshell don't affect outer scope)
- Loop body uses break/continue (feature 022/023)

---

## Technical Requirements

### Parser Requirements
- Recognize `for` keyword at statement level
- Parse `var` (identifier)
- Parse optional `in` keyword followed by word list
- Parse `do` keyword
- Parse command list (loop body) recursively
- Parse `done` keyword
- Handle newlines and semicolons as statement separators
- Proper error reporting for malformed syntax

### Execution Requirements
- Expand word list using current shell expansion rules (parameters, command substitution, brace expansion)
- For each word in the expanded list:
  - Bind loop variable to the word
  - Execute loop body
  - Capture exit code
- Return exit code of last iteration (or 0 if no iterations)
- Handle break/continue signals (when implemented in features 022/023)

### Integration Points
- Feature 001 (Command execution) - execute commands in loop body
- Feature 017 (Conditionals) - support nested if statements
- Feature 019 (while/until loops) - parallel implementation for while/until
- Feature 022 (break) - implement break statement support
- Feature 023 (continue) - implement continue statement support

## Success Metrics

- ✅ All 5 user stories fully implemented
- ✅ 45+ test cases (unit and integration combined)
  - US1: 8 tests (basic, empty list, single word, special chars, expansions)
  - US2: 8 tests (positional parameters with 0, 1, N args)
  - US3: 12 tests (variable expansion, command substitution, globbing prevention)
  - US4: 8 tests (variable scoping, shadowing, persistence)
  - US5: 9 tests (pipes, redirects, nested structures)
- ✅ POSIX compatibility verified against reference shell behavior
- ✅ Performance: loops with 100+ iterations complete in <100ms
- ✅ All tests pass with 100% code coverage for for-loop implementation
- ✅ Documentation includes usage examples and edge case handling

## POSIX Specification Reference

From POSIX.1-2017 (Shell and Utilities):

```
For Loop:
  for name in [word ...] ; do list ; done
  for name ; do list ; done          (equivalent to: for name in "$@" ; do list ; done)
```

This feature implements the core for-loop construct as specified in POSIX, excluding the `in` keyword variants that reference shell functions (feature 021).

## Architecture Notes

The for loop implementation will follow the pattern established by Feature 017 (conditionals):

1. **Parser** (`executor/loop.rs` or `executor/for_loop.rs`)
   - Recursive descent parsing with keyword detection
   - Word list expansion using existing expansion utilities
   - Proper error handling and reporting

2. **AST** (`executor/mod.rs`)
   - `ForLoop` struct containing:
     - variable name (String)
     - word list (Vec<String> or expression)
     - body (Vec<Command>)

3. **Executor** (`executor/execute.rs`)
   - Detect `for` keyword
   - Delegate to for-loop handler
   - Iterate and execute body

4. **Integration** (`repl/mod.rs`)
   - Support multiline for loops in REPL
   - Completion tracking for nested structures

## Constraints

- Must maintain single-threaded execution model
- Must preserve variable scope rules (no subshells unless explicitly requested)
- Word list expansion order matters (left to right, with proper IFS handling)
- Must handle signals properly (SIGINT, SIGTERM)
- Exit codes must follow POSIX semantics

## Notes

- Feature 018 builds directly on Feature 017 (conditional control flow)
- Break and continue (features 022/023) will be implemented as separate features that integrate with the for loop body execution
- While and until loops (Feature 019) will use similar infrastructure
- This feature is essential for any real shell usage (scripting, batch operations)

---

**Created**: 2025-12-06
**Status**: Specification Complete
**Next Phase**: Planning (create plan.md)
>>>>>>> 025-subshells
