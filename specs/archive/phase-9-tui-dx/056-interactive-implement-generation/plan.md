# Implementation Plan: Interactive Implement Generation

**Branch**: `056-interactive-implement-generation` | **Date**: 2025-12-17 | **Spec**: [spec.md](./spec.md)

## Summary

Add implementation execution workflow to TUI. Users can view tasks, execute them via Claude CLI, and track completion status with persistence to tasks.md.

## Technical Context

**Language/Version**: Rust 1.75+ (edition 2021)
**Primary Dependencies**: ratatui, crossterm, tokio (already in workspace)
**Testing**: cargo test with TestBackend
**Target Platform**: macOS (MVP), Linux (future)
**Project Type**: Enhancement to `rstn` TUI crate

## What Already Exists (from 055)

- `ParsedTask` struct - parses task lines from markdown
- `TaskListState` - selection, reordering, serialization
- Task rendering with colored markers
- j/k navigation, J/K reordering in Tasks phase

## What This Feature Adds

1. **Implement Phase Entry**: New "Implement" command in SDD workflow
2. **Task Execution**: Run Claude CLI for selected task
3. **Status Tracking**: Mark tasks complete, update tasks.md
4. **Progress View**: Show overall implementation progress

## Constitution Check

| Principle | Status | Evidence |
|-----------|--------|----------|
| I. Performance-First | PASS | Async task execution, non-blocking UI |
| II. Zero-Config | PASS | Uses existing tasks.md format |
| III. Progressive Complexity | PASS | Simple execute action, optional auto-advance |
| IV. Modern UX | PASS | Visual progress, keyboard shortcuts |
| V. Rust-Native | PASS | Pure ratatui, async tokio |

## Design

### Workflow

```
1. User selects "Implement" from SDD menu (hotkey: i)
2. TUI loads tasks.md, displays task list
3. User selects task (j/k), presses Enter to execute
4. Claude CLI runs with task context
5. On completion, task marked [X] in memory and saved to tasks.md
6. Auto-advance to next incomplete task (optional)
```

### State Extensions

```rust
// Add to SpecifyState or create ImplementState
pub struct ImplementState {
    pub task_list: TaskListState,      // Reuse from 055
    pub executing_task: Option<usize>, // Currently running task index
    pub execution_output: String,      // Claude CLI output
    pub auto_advance: bool,            // Move to next task on completion
}
```

### Key Bindings (Implement phase)

| Key | Action |
|-----|--------|
| `j`/`↓` | Select next task |
| `k`/`↑` | Select previous task |
| `Enter` | Execute selected task |
| `x` | Toggle task complete (manual) |
| `a` | Toggle auto-advance |
| `Esc` | Exit implement mode |

### Task Execution Flow

```rust
async fn execute_task(task: &ParsedTask, feature_dir: &Path) -> Result<String, String> {
    // 1. Build context from spec.md, plan.md, tasks.md
    // 2. Format prompt: "Implement task: {task.description}"
    // 3. Run: claude --print --system-prompt-file implement.md "{prompt}"
    // 4. Return output for display
}
```

### Persistence

- On task completion: update task.completed = true
- Save entire TaskListState to tasks.md via to_markdown()
- Atomic write with temp file + rename

## Deployment Strategy

**Single PR** - Feature builds directly on 055 (~200-300 lines)

1. Add ImplementState with task execution methods
2. Add hotkey 'i' to trigger Implement phase
3. Add Enter handler to execute task
4. Add persistence (save on completion)
5. Tests

## Files to Modify

1. `crates/rstn/src/tui/views/worktree.rs` - ImplementState, execution logic
2. `crates/rstn/src/tui/app.rs` - Add execute_implement_task() method

## Complexity Tracking

No constitution violations expected - reuses existing patterns.
