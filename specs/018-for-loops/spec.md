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
