---
title: "Keybinding Specification"
description: "Authoritative mapping of keys to internal actions"
category: architecture
status: active
last_updated: 2025-12-23
version: 1.0.0
tags: [architecture, keybindings, input]
weight: 8
---

# Keybinding Specification

This document is the **Source of Truth** for all keyboard shortcuts in the rstn TUI. Implementation MUST follow this specification.

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

  - context: Global
    key: "y"
    action: CopyVisual
    description: "Copy current view content to clipboard"

  - context: Global
    key: "Y"
    action: CopyState
    description: "Copy full application state to clipboard"

  # Navigation (Tab Bar)
  - context: Global
    key: "1"
    action: SwitchToWorktree
    description: "Switch to Worktree view"

  - context: Global
    key: "2"
    action: SwitchToDashboard
    description: "Switch to Dashboard view"

  - context: Global
    key: "3"
    action: SwitchToSettings
    description: "Switch to Settings view"

  # Worktree Navigation
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
The TUI event loop in `rstn/tui/app.py` and state reducers in `rstn/reduce/` must implement these mappings.