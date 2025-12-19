# Specification: Environment Variables

**Feature ID:** 014-environment-variables
**Status:** Draft
**Created:** 2025-11-28
**Updated:** 2025-11-28

## Overview

Implement environment variable support for the rush shell, including:
- Setting variables (`set NAME=value`)
- Exporting variables to subshells (`export NAME=value`)
- Removing variables (`unset NAME`)
- Expanding variables in commands (`$VAR`, `${VAR}`)
- Listing variables (`set` without arguments)

Environment variables are fundamental to shell functionality, allowing shells to pass configuration and state to programs and subshells.

## Motivation

Environment variables are essential for:
1. **Shell Configuration** - HOME, PATH, SHELL, TERM, etc.
2. **Program Communication** - Passing settings to external commands
3. **Shell Scripting** - Storing and manipulating data
4. **User Environment** - Customizing shell behavior
5. **Cross-Shell Communication** - Exported variables visible to subshells

Without environment variable support, rush cannot run most real-world shell scripts or commands that depend on environment configuration.

## User Stories

### US1: Set Local Variables
**As a** shell user
**I want to** set variables for use in the current shell session
**So that** I can store and reference data within commands

**Acceptance Criteria:**
- `set NAME=value` stores variable in shell
- `set NAME` shows value if set, error if not set
- `set` without arguments lists all variables
- Variables with spaces in values can be quoted: `set MSG="hello world"`
- Variables can reference other variables: `set FULL=$FIRST$LAST`
- Exit code 0 on success, 1 on error

**Examples:**
```bash
$ set greeting="hello"
$ set name="world"
$ echo $greeting $name
hello world
$ set MYVAR
error: MYVAR: not set
$ set
greeting=hello
name=world
```

### US2: Export Variables to Subshells
**As a** shell user
**I want to** export variables so subshells can access them
**So that** environment settings propagate to child processes

**Acceptance Criteria:**
- `export NAME=value` sets variable and marks as exported
- `export NAME` marks existing variable as exported
- Exported variables visible to subshells (external commands)
- Non-exported variables NOT visible to subshells
- `export` without arguments lists exported variables
- Works with variable expansion: `export PATH=$PATH:/new/path`

**Examples:**
```bash
$ export MY_VAR="hello"
$ env | grep MY_VAR
MY_VAR=hello
$ export CUSTOM="/custom/path"
$ /usr/bin/env | grep CUSTOM
CUSTOM=/custom/path
```

### US3: Remove Variables
**As a** shell user
**I want to** remove variables I no longer need
**So that** they don't persist or affect other commands

**Acceptance Criteria:**
- `unset NAME` removes variable from shell
- `unset NAME1 NAME2 ...` removes multiple variables
- `unset` without arguments is error (unlike bash)
- Removing non-existent variable returns exit code 1
- Removed variables not visible to `set` or `export` listing

**Examples:**
```bash
$ set myvar="test"
$ set myvar
test
$ unset myvar
$ set myvar
error: myvar: not set
```

### US4: Variable Expansion in Commands
**As a** shell user
**I want to** use variables in command arguments
**So that** I can dynamically construct commands and arguments

**Acceptance Criteria:**
- `$VARNAME` expands to variable value
- `${VARNAME}` expands to variable value
- `$$` expands to shell PID
- `$?` expands to last exit code
- `$#` expands to number of positional arguments
- `$0` expands to shell name (rush)
- `$1`, `$2`, ... expand to command-line arguments (for scripts)
- Non-existent variables expand to empty string
- Variable expansion works in quoted strings
- Can escape with backslash: `\$VAR` → literal `$VAR`

**Examples:**
```bash
$ set dir="/tmp"
$ cd $dir
$ pwd
/tmp
$ echo "Directory: ${dir}"
Directory: /tmp
$ echo $nonexistent
(empty line)
$ echo "test=$nonexistent"
test=
```

## Technical Requirements

### Storage
1. **Local Variables** - HashMap stored in CommandExecutor
2. **Exported Flag** - Track which variables are exported
3. **Type** - All variables are strings (like POSIX shells)

### Variable Expansion
1. **When to Expand:**
   - Before command execution
   - In double-quoted strings
   - NOT in single-quoted strings
   - NOT in variable names themselves (set $VAR=value is error)

2. **Expansion Order:**
   - Variable expansion happens BEFORE command parsing
   - After variable expansion, command is re-parsed
   - Must handle recursive expansion (var1=$var2 when var2=$var3)

3. **Special Variables:**
   - `$0` → "rush"
   - `$$` → Process ID
   - `$?` → Last exit code (from CommandExecutor)
   - `$#` → Number of positional args
   - `$1...$9` → Positional arguments (for scripts)

### Built-in Commands

**set**
- `set` - List all variables (sorted)
- `set NAME` - Show specific variable
- `set NAME=value` - Set variable
- `set NAME="value with spaces"` - Support quoted values

**export**
- `export` - List exported variables
- `export NAME` - Mark existing variable as exported
- `export NAME=value` - Set and export variable
- `export NAME="value"` - Support quoted values

**unset**
- `unset NAME` - Remove variable
- `unset NAME1 NAME2 ...` - Remove multiple
- Error if no arguments
- Non-zero exit if variable doesn't exist

## Success Metrics

1. **Functionality:**
   - All 4 user stories pass acceptance tests
   - Variable expansion works in all contexts
   - Exported variables visible to subshells

2. **Compatibility:**
   - Behavior matches bash for common cases
   - Error messages are clear
   - Exit codes follow POSIX conventions

3. **Testing:**
   - Unit tests for each builtin (set, export, unset)
   - Tests for variable expansion
   - Tests for special variables ($?, $$, $0)
   - Integration tests for subshell inheritance

## Out of Scope

The following features are NOT included:
- Special parameter expansion (`${VAR:-default}`, `${VAR#pattern}`)
- Array variables
- Function-local variables
- Variable aliasing
- Read-only variables (readonly builtin)
- Command substitution in variable names
- Positional parameter assignment

These may be added in future iterations.

## Dependencies

- Rust HashMap for variable storage
- CommandExecutor for accessing last exit code and environment
- Parser modifications to handle variable expansion
- Environment variable access via `std::env`

## Timeline

**Estimated Effort:** 4-5 hours
- Specification: 30 min (this document)
- Implementation: 2.5-3 hours
  - Variable storage: 30 min
  - set/export/unset builtins: 1 hour
  - Variable expansion: 1-1.5 hours
- Testing: 45 min - 1 hour
- Documentation: 15-30 min

**Target Completion:** Current or next session
