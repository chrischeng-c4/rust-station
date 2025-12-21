# TUI Keybindings Reference

This document is the Source of Truth for all keyboard shortcuts in the rstn TUI.

```yaml
# Keybindings Schema
# - context: Global | Worktree | Input | ...
#   key: The key or key combination (e.g., 'q', 'Ctrl-c', 'Enter')
#   action: Internal action ID
#   description: Human-readable description

keybindings:
  # Global Actions
  - context: Global
    key: "q"
    action: Quit
    description: "Quit rstn"
  
  - context: Global
    key: "Ctrl-c"
    action: ForceQuit
    description: "Force quit rstn"

  # Navigation
  - context: Worktree
    key: "j"
    action: MoveDown
    description: "Move selection down"
  
  - context: Worktree
    key: "k"
    action: MoveUp
    description: "Move selection up"

  - context: Worktree
    key: "Tab"
    action: NextPane
    description: "Switch focus between Commands and Content panels"

  # Workflow Triggers
  - context: Worktree
    key: "Enter"
    action: ExecuteSelected
    description: "Trigger the selected workflow command"

  # Panel Specific
  - context: Content
    key: "h"
    action: ScrollLeft
    description: "Scroll content left"
  
  - context: Content
    key: "l"
    action: ScrollRight
    description: "Scroll content right"
```

## Logic Implementation
The TUI event loop in `crates/rstn/src/tui/app.rs` and individual views in `crates/rstn/src/tui/views/` must implement these mappings.
