# Project Management

Rustation supports opening multiple projects and working with git worktrees.

## Opening Projects

### From Menu

1. Click the `+` button in the project tab bar
2. Select **Open Project...**
3. Choose a folder containing a git repository

### From Recent Projects

1. Click the `+` button in the project tab bar
2. See recent projects in the dropdown
3. Click to open

## Project Tabs

Each open project appears as a tab at the top of the window:

```
┌────────────────────────────────────────────────┐
│ [*my-app] [api-server] [shared-lib] [+]        │
└────────────────────────────────────────────────┘
```

- **Active project** is highlighted with background
- **Modified indicator** (`*`) shows unsaved changes
- Click a tab to switch projects
- Click `x` to close a project

## Git Worktrees

Git worktrees allow working on multiple branches simultaneously without stashing.

### What is a Worktree?

A worktree is a separate checkout of your repository at a different path. Each worktree:
- Has its own working directory
- Can be on a different branch
- Shares git history with the main repository

### Worktree Tabs

When a project is open, the second row shows worktree tabs:

```
┌────────────────────────────────────────────────┐
│ [main] [feature/auth] [bugfix/123] [+]         │
└────────────────────────────────────────────────┘
```

### Creating a Worktree

1. Click the `+` button in the worktree row
2. Choose **From existing branch** or **Create new branch**
3. Select or enter the branch name

The new worktree is created in a sibling directory:
```
/projects/my-app/           ← Main repository
/projects/my-app-feature/   ← Worktree for feature branch
```

### Per-Worktree State

Each worktree maintains isolated state:
- Active tab (Tasks/Docker/Settings)
- Selected Docker service
- Task output

Switching worktrees preserves your context.

## State Persistence

Rustation remembers:
- Recently opened projects (up to 10)
- Global settings (theme, paths)

Projects are **not** automatically reopened on launch.
