---
title: "Settings UI"
description: "Spec: Unified Global and Worktree Settings"
category: roadmap
status: planned
version: 1.1.0
---

# Feature Spec: Settings UI

## 1. Overview

**Goal**: Provide a unified interface for configuring both Global (app-wide) and Worktree (context-specific) settings.
**Core Value**: Allow users to customize their experience and configure environment-specific tool behaviors.

## 2. User Stories

1. **Theme**: As a user, I want to switch between Light, Dark, and System themes.
2. **Defaults**: As a user, I want to set a default directory for new projects.
3. **Worktree Specifics**: As a user, I want to override the `just` binary path for a specific legacy worktree.

## 3. UI Design

### Location
- **View**: `SettingsPage`
- **Access**: "Settings" tab in the left sidebar (Worktree Scope).

### Layout
```
+---------------------------------------------------------------+
| Settings                                                      |
+-----------------------+---------------------------------------+
| [ General           ] |  Global Settings                      |
| [ Editor            ] |  -----------------------------------  |
| [ Worktree          ] |  Theme                                |
| [ About             ] |  (o) System  ( ) Light  ( ) Dark      |
|                       |                                       |
|                       |  Default Project Path                 |
|                       |  [ /Users/chris/dev       ] [Browse]  |
|                       |                                       |
|                       |  Worktree Overrides (feature/login)   |
|                       |  -----------------------------------  |
|                       |  Justfile Path                        |
|                       |  [ ./justfile             ]           |
|                       |                                       |
|                       |  MCP Server Port                      |
|                       |  [ Auto                   ]           |
+-----------------------+---------------------------------------+
```

## 4. State Architecture

### Global Settings (AppState)
```rust
pub struct GlobalSettings {
    pub theme: Theme, // System, Light, Dark
    pub default_project_path: Option<String>,
    pub notifications_enabled: bool,
}
```

### Worktree Settings (WorktreeState)
*New struct needed*
```rust
pub struct WorktreeSettings {
    pub justfile_path: Option<String>, // Default: "justfile"
    pub mcp_port_override: Option<u16>,
    pub terminal_font_size: Option<u8>,
}
```

## 5. Actions & API

| Action | Payload | Description |
|--------|---------|-------------|
| `SetTheme` | `{ theme: Theme }` | Update global theme |
| `SetGlobalSetting` | `{ key: string, value: any }` | Generic setter |
| `SetWorktreeSetting` | `{ worktree_id: string, key: string, value: any }` | Override |

## 6. Implementation Plan

### Phase 1: Global Settings
- Implement `GlobalSettings` persistence.
- Create `SettingsPage` with "General" tab.
- Implement Theme toggle using `document.documentElement.classList`.

### Phase 2: Worktree Settings
- Add `settings` field to `WorktreeState`.
- Add "Worktree" tab to `SettingsPage`.
- Implement overrides logic in backend (e.g. `get_effective_setting(key)`).

## 7. Edge Cases
- **Theme Sync**: System theme changes should auto-update app if "System" is selected.
- **Persistence**: Settings must survive app restart.
- **Migration**: Adding new settings fields should not break existing JSON.