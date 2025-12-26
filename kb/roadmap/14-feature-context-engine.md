---
title: "Intelligent Context Engine"
description: "Spec: Context Aggregation for AI Workflows"
category: roadmap
status: planned
version: 1.0.0
---

# Feature Spec: Intelligent Context Engine

## 1. Overview

**Goal**: Automatically gather, rank, and format the most relevant information from the project state to send to the LLM.
**Core Value**: Reduces hallucination and manual copy-pasting. "It just knows" what you are working on.

## 2. Context Sources

The engine will aggregate data from:

1.  **File System**:
    - `active_file`: The file currently open in the editor/viewer.
    - `directory_structure`: Tree view (depth=2) to show layout.
2.  **Runtime State**:
    - `docker_errors`: Logs from failing containers.
    - `task_output`: Result of the last run `just` command.
3.  **Git State**:
    - `git_diff`: Unstaged changes (what the user is working on *right now*).
    - `branch_name`: Current feature context.

## 3. Architecture

### Context Context Object
```rust
pub struct AIContext {
    pub open_files: Vec<FileContext>,
    pub terminal_last_output: Option<String>,
    pub git_status: String,
    pub active_errors: Vec<String>,
}

pub struct FileContext {
    pub path: String,
    pub content: String, // Truncated if too large
    pub cursor_line: Option<usize>,
}
```

### Token Budgeting
- **Limit**: e.g., 20k tokens.
- **Strategy**: Priority Queue.
  1. `active_file` (Highest)
  2. `git_diff`
  3. `docker_errors`
  4. `directory_tree` (Lowest)

## 4. Implementation Plan

### Phase 1: Gatherers
- Implement `ContextGatherer` trait.
- Implement `GitGatherer`, `FileGatherer`, `DockerGatherer`.

### Phase 2: Orchestrator
- `ContextEngine::build(budget: usize) -> String`.
- Logic to assemble the final System Prompt string.

## 5. API
- Internal Rust API used by `mcp_server` and `chat_workflow`.
- Not directly exposed to frontend, but results are visible in "Inspector".
