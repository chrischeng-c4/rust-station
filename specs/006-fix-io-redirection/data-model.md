# Data Model: Fix I/O Redirection Bug

**Feature**: 006-fix-io-redirection
**Date**: 2025-11-22

## Overview

This document defines the data structures required to fix the I/O redirection bug. The core issue is that `PipelineSegment` lacks a field to store redirection information parsed from the command line.

## Core Entities

### PipelineSegment (MODIFIED)

**Purpose**: Represents a single command in a pipeline with all its execution context.

**Current Structure** (BROKEN):
```rust
pub struct PipelineSegment {
    /// Command name (e.g., "ls", "grep")
    pub program: String,

    /// Command arguments (e.g., ["-la"], ["txt"])
    pub args: Vec<String>,

    /// Position in pipeline (0-indexed)
    pub index: usize,
}
```

**New Structure** (FIXED):
```rust
pub struct PipelineSegment {
    /// Command name (e.g., "ls", "grep")
    pub program: String,

    /// Command arguments (e.g., ["-la"], ["txt"])
    pub args: Vec<String>,

    /// Position in pipeline (0-indexed)
    pub index: usize,

    /// I/O redirections for this command
    /// NEW FIELD - stores parsed redirections from command line
    pub redirections: Vec<Redirection>,
}
```

**Attributes**:
- `program`: String - Name or path of executable
- `args`: Vec<String> - Arguments passed to executable
- `index`: usize - Position in pipeline (0 = first command)
- `redirections`: Vec<Redirection> - **NEW** - List of I/O redirections

**Relationships**:
- Contained by: `Pipeline` (collection of segments)
- Contains: Multiple `Redirection` instances (0 to N)

**Validation Rules**:
- `program` must be non-empty
- `index` must be valid for containing pipeline
- `redirections` can be empty (no redirections is valid)
- Multiple redirections for same stream type: last one wins (POSIX behavior)

**Constructor Update**:
```rust
impl PipelineSegment {
    pub fn new(program: String, args: Vec<String>, index: usize, redirections: Vec<Redirection>) -> Self {
        Self { program, args, index, redirections }
    }
}
```

---

### Redirection (EXISTING - NO CHANGES)

**Purpose**: Represents a single I/O redirection for a command.

**Structure**:
```rust
pub struct Redirection {
    /// Type of redirection (Output, Append, Input)
    pub redir_type: RedirectionType,

    /// Target file path
    pub file_path: String,
}
```

**Attributes**:
- `redir_type`: RedirectionType enum - Kind of redirection
- `file_path`: String - Path to file for redirection

**Validation Rules**:
- `file_path` must be non-empty
- File path validation done at execution time (not during parsing)

**Enum: RedirectionType**:
```rust
pub enum RedirectionType {
    /// Output redirection (>)
    /// Creates/truncates file, redirects stdout
    Output,

    /// Append redirection (>>)
    /// Creates/appends to file, redirects stdout
    Append,

    /// Input redirection (<)
    /// Opens file for reading, redirects stdin
    Input,
}
```

---

### Pipeline (EXISTING - MINOR CHANGE)

**Purpose**: Collection of pipeline segments representing a complete command line.

**Structure**:
```rust
pub struct Pipeline {
    /// Individual commands in the pipeline
    pub segments: Vec<PipelineSegment>,

    /// Original user input
    pub raw_input: String,

    /// Run in background (ends with &)
    pub background: bool,
}
```

**Impact of Change**:
- No structural changes to `Pipeline`
- Segments now carry redirection information
- No changes to pipeline validation or execution flow

---

## Data Flow

### Parsing Flow (UPDATED)

```text
User Input: "echo hello > file.txt"
    ↓
tokenize_with_pipes() → [Word("echo"), Word("hello"), RedirectOut, Word("file.txt")]
    ↓
split_into_segments() → PipelineSegment {
                            program: "echo",
                            args: ["hello"],
                            index: 0,
                            redirections: [Redirection {
                                redir_type: RedirectionType::Output,
                                file_path: "file.txt"
                            }]
                        }
    ↓
Pipeline { segments: [segment], raw_input: "...", background: false }
```

### Execution Flow (UPDATED)

```text
Pipeline with redirections
    ↓
PipelineExecutor::spawn()
    ↓
For each segment:
  1. Get program, args, redirections from segment
  2. Create std::process::Command
  3. For each redirection in segment.redirections:
       - Output: Open file (create/truncate) → Command::stdout()
       - Append: Open file (create/append) → Command::stdout()
       - Input: Open file (read) → Command::stdin()
  4. Spawn process
    ↓
Process runs with redirected I/O
```

---

## Migration Impact

### Backward Compatibility

✅ **Fully backward compatible**

Existing code that creates `PipelineSegment` without redirections:
```rust
// Old code (still works)
PipelineSegment::new("ls".to_string(), vec!["-la".to_string()], 0, vec![])
//                                                                      ^^^^
//                                                                      empty vec = no redirections
```

### Required Changes

1. **`executor/mod.rs`**:
   - Add `redirections` field to struct definition
   - Update `new()` constructor signature

2. **`executor/parser.rs`**:
   - Update `split_into_segments()` to extract redirections from tokens
   - Pass redirections to `PipelineSegment::new()`

3. **`executor/pipeline.rs`**:
   - Update `execute_single_command()` to use `segment.redirections`
   - Remove redundant `extract_redirections_from_args()` call

4. **All test files**:
   - Update `PipelineSegment::new()` calls to include empty `vec![]` for redirections
   - Or use default/builder pattern if we add one

### Performance Impact

**None** - Vec<Redirection> is zero-cost when empty:
- Empty vec: 24 bytes (pointer + length + capacity)
- No heap allocation until redirections added
- Commands without redirections: zero overhead

---

## Example Usage

### Command Without Redirections
```rust
let segment = PipelineSegment::new(
    "ls".to_string(),
    vec!["-la".to_string()],
    0,
    vec![]  // No redirections
);
```

### Command With Output Redirection
```rust
let segment = PipelineSegment::new(
    "echo".to_string(),
    vec!["hello".to_string()],
    0,
    vec![Redirection {
        redir_type: RedirectionType::Output,
        file_path: "/tmp/output.txt".to_string(),
    }]
);
```

### Command With Multiple Redirections
```rust
let segment = PipelineSegment::new(
    "cat".to_string(),
    vec![],
    0,
    vec![
        Redirection {
            redir_type: RedirectionType::Input,
            file_path: "/tmp/input.txt".to_string(),
        },
        Redirection {
            redir_type: RedirectionType::Output,
            file_path: "/tmp/output.txt".to_string(),
        },
    ]
);
```

---

## Validation & Constraints

### At Parse Time
- Redirection syntax validated by parser (already works)
- File paths extracted from tokens
- Multiple redirections allowed (last wins for same stream)

### At Execution Time
- File existence checked for input redirections
- File permissions checked for all redirections
- Parent directory must exist (no auto-creation)
- Clear error messages for all failures

### POSIX Compliance
- Multiple redirections for same stream: last one wins
- Example: `echo test > file1.txt > file2.txt` → writes to `file2.txt` only
- Standard behavior for all POSIX shells

---

## Testing Data

### Test Fixtures

**Valid Redirections**:
```rust
// Output
Redirection { redir_type: Output, file_path: "/tmp/test.txt" }

// Append
Redirection { redir_type: Append, file_path: "/tmp/log.txt" }

// Input
Redirection { redir_type: Input, file_path: "/tmp/input.txt" }
```

**Invalid Cases** (should error at execution):
- Input from non-existent file
- Output to file without write permission
- Output to directory path

**Edge Cases**:
- Empty file after redirection (valid)
- Redirection to `/dev/null` (valid, special file)
- Redirection to `/dev/stdin`, `/dev/stdout` (valid)
- Multiple redirections same type (last wins)

---

## Summary

**Key Change**: Add `redirections: Vec<Redirection>` field to `PipelineSegment`

**Impact**: Minimal - backward compatible, zero performance overhead, enables full I/O redirection support

**Next Step**: Generate tasks (`/speckit.tasks`) to implement the changes outlined in plan.md
