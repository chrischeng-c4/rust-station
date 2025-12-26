# Quick Start

This guide walks you through opening your first project and using Rustation's core features.

## 1. Open a Project

Click the **Open Project** button (or use the `+` in the tab bar) and select a folder containing a git repository.

The project will appear as a tab at the top of the window.

## 2. Navigate Tabs

Rustation has two levels of tabs:

- **Project tabs** (top row): Switch between open projects
- **Worktree tabs** (second row): Switch between git worktrees

## 3. Use the Sidebar

The sidebar provides access to three feature tabs:

| Tab | Description |
|-----|-------------|
| **Tasks** | Run justfile commands |
| **Docker** | Manage containers |
| **Settings** | Configure app |

## 4. Run a Task

1. Click **Tasks** in the sidebar
2. If your project has a `justfile`, you'll see available commands
3. Click **Run** on any command
4. Watch the output stream in real-time

## 5. Manage Docker Services

1. Click **Docker** in the sidebar
2. View status of development containers
3. Click **Start/Stop** to control services
4. Click a service to view its logs

## 6. Work with Worktrees

If you need to work on multiple branches:

1. Click the `+` button in the worktree row
2. Select a branch or create a new one
3. A new worktree tab appears
4. Each worktree has isolated state

## Keyboard Shortcuts

| Action | Shortcut |
|--------|----------|
| Open Project | `Cmd+O` |
| Close Project | `Cmd+W` |
| Switch Project | `Cmd+1-9` |
| Refresh | `Cmd+R` |

## Next Steps

- [Project Management](/features/project-management) - Learn about worktrees
- [Docker](/features/docker) - Set up development containers
- [Tasks](/features/tasks) - Create a justfile
