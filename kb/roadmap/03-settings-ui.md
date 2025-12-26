---
title: "Settings UI"
description: "Planned: Settings configuration interface"
category: roadmap
status: partial
last_updated: 2025-12-26
version: 3.0.0
---

# Settings UI (Partial)

## Current State

**Backend**: Complete
- `GlobalSettings` struct defined
- `SetTheme` action implemented
- `SetProjectPath` action implemented
- Persistence working

**Frontend**: Not implemented
- Settings tab shows "Coming Soon"
- No UI components for settings

---

## Planned UI

```
┌─────────────────────────────────────────────────────────┐
│ Settings                                                │
├─────────────────────────────────────────────────────────┤
│                                                         │
│ Appearance                                              │
│ ─────────────────────────────────────────────────────── │
│ Theme:     [System ▼]                                   │
│            ○ Light  ○ Dark  ● System                    │
│                                                         │
│ Paths                                                   │
│ ─────────────────────────────────────────────────────── │
│ Default Project Path:                                   │
│ [/Users/chris/projects          ] [Browse...]           │
│                                                         │
│ About                                                   │
│ ─────────────────────────────────────────────────────── │
│ Version: 0.1.0                                          │
│ Electron: 28.0.0                                        │
│ React: 19.0.0                                           │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

---

## Settings Structure

```typescript
interface GlobalSettings {
  theme: 'system' | 'light' | 'dark'
  default_project_path: string | null
}
```

---

## Actions (Already Implemented)

| Action | Payload | Description |
|--------|---------|-------------|
| `SetTheme` | `{ theme: Theme }` | Change theme |
| `SetProjectPath` | `{ path: string }` | Set default path |

---

## Implementation Tasks

### 1. SettingsPage.tsx
- Form layout with sections
- Theme radio buttons or dropdown
- Path input with browse button
- About section with versions

### 2. Theme System
- Apply theme to document root
- Persist preference
- Respect system preference when "System" selected

### 3. Path Browser
- Use `dialogApi.openFolder()`
- Validate path exists
- Show current value

---

## UI Components Needed

| Component | Purpose |
|-----------|---------|
| `ThemeSelector` | Radio group for theme |
| `PathInput` | Text input + browse button |
| `SettingsSection` | Grouped settings with header |

---

## References

- [State Topology](../implemented/02-state-topology.md)
- [Persistence](../implemented/03-persistence.md)
