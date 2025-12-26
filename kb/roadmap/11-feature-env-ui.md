---
title: "Env Management UI"
description: "Spec: UI for Three-Scope Environment Manager"
category: roadmap
status: planned
version: 1.0.0
---

# Feature Spec: Env Management UI

## 1. Overview

**Goal**: Provide a user interface to manage the "Project Scope" environment variables defined in the Three-Scope Model.
**Core Value**: Simplify keeping `.env` files in sync across multiple worktrees (e.g., `main` vs `feature/auth`).

## 2. User Stories

1. **View Patterns**: As a user, I want to see which files are being tracked (e.g., `.env`, `.envrc`) so I know what will be copied.
2. **Manual Copy**: As a user, I want to manually copy env files from one worktree to another when I've made changes.
3. **Auto-Copy Toggle**: As a user, I want to enable/disable "Auto-Copy on Worktree Create" for this project.
4. **History**: As a user, I want to see a log of recent copy operations to verify success.

## 3. UI Design

### Location
- **View**: `EnvPage`
- **Access**: "Env" button on the second top bar (Project Scope).

### Layout
```
+---------------------------------------------------------------+
| Project: my-app [Auto-Copy: ON]                               |
+---------------------------------------------------------------+
|                                                               |
|  [ Manual Sync ]                                              |
|  Source: [ main ▼ ]  -->  Target: [ feature/login ▼ ]         |
|  Files: .env, .env.local                                      |
|  [ Copy Now ]                                                 |
|                                                               |
|  [ Configuration ]                                            |
|  Tracked Patterns:                                            |
|  [ .env        ] [x]                                          |
|  [ .envrc      ] [x]                                          |
|  [ + Add Pattern ]                                            |
|                                                               |
|  [ Recent Activity ]                                          |
|  • 10:00 AM - Copied .env from main to feature/login (Success)|
|  • 09:30 AM - Copied .envrc from main to dev (Partial)        |
|                                                               |
+---------------------------------------------------------------+
```

## 4. State Architecture

### Existing State (ProjectState)
```rust
pub struct EnvConfig {
    pub tracked_patterns: Vec<String>,
    pub auto_copy_enabled: bool,
    pub source_worktree: Option<String>, // Default source
}
```

### New UI State (AppState/ActiveView)
No new persistent state needed. UI state (dropdown selections) can be local to React component or transient in `AppState` if we want to preserve it during navigation.

**Transient State (React)**:
- `selectedSource`: string
- `selectedTarget`: string

## 5. Actions & API

| Action | Payload | Description |
|--------|---------|-------------|
| `SetEnvTrackedPatterns` | `{ patterns: Vec<String> }` | Update list |
| `SetEnvAutoCopy` | `{ enabled: bool }` | Toggle auto-copy |
| `CopyEnvFiles` | `{ from: path, to: path }` | Trigger copy |

## 6. Implementation Plan

### Phase 1: Components
- `EnvPatternList`: Editable list of strings.
- `WorktreeSelector`: Dropdown to select worktree path (display branch name).
- `CopyHistory`: List component displaying `notifications` filtered by type `EnvCopy`.

### Phase 2: Integration
- Connect `EnvPage` to `ProjectState.env_config`.
- Wire up `CopyEnvFiles` action.
- Display toast notifications on result.

## 7. Testing Strategy

- **Unit (Rust)**:
  - Test `copy_env_files` with mocked filesystem.
  - Test pattern matching logic.
- **Component (React)**:
  - Render `EnvPage` with mock state.
  - Fire events, verify `window.api.dispatch` calls.
