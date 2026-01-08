# Project Management

Rustation supports opening multiple projects and working with git worktrees.

## Navigation Hierarchy

Rustation uses a three-layer navigation hierarchy for clear organization:

### Level 0: Global Utilities

Located in the top-right corner of Project Tabs, always accessible regardless of which project or worktree is active:

- **Copy**: Copy current screen to clipboard as image
- **Screenshot**: Save screenshot as file
- **Download**: Download files or export data
- **Notifications**: View application notifications
- **Logs**: View application logs
- **Docker**: Manage all containers (cross-project)
- **Settings**: Application-wide settings

### Level 1: Projects

Each project appears as a tab at the top. See [Project Tabs](#project-tabs) for details.

### Level 2: Worktrees

When a project is active, worktree tabs appear below the project tabs. See [Worktree Tabs](#worktree-tabs) for details.

### Complete Navigation Structure

```
┌────────────────────────────────────────────────────────────┐
│ Level 1: [my-app] [api-server] [+]  │📋 📸 📥 🔔 📊 🐳 ⚙️│ ← Global Icons (Level 0)
├────────────────────────────────────────────────────────────┤
│ Level 2: [main] [feature-x] [+]  │ Env │                  │
├────────────┬───────────────────────────────────────────────┤
│ Sidebar    │ Content Area                                  │
│ 📝 Tasks    │                                               │
│ 💻 Terminal │                                               │
│ 📂 Explorer │                                               │
│ 🤖 Chat    │                                               │
│ 🔌 MCP     │                                               │
└────────────┴───────────────────────────────────────────────┘
```

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

Each open project appears as a tab at the top of the window. The right side contains 7 Global Icon Buttons for cross-project utilities:

```
┌──────────────────────────────────────────────────────────────┐
│ [*my-app] [api-server] [shared-lib] [+]  │📋 📸 📥 🔔 📊 🐳 ⚙️│
└──────────────────────────────────────────────────────────────┘
```

- **Active project** is highlighted with background
- **Modified indicator** (`*`) shows unsaved changes
- Click a tab to switch projects
- Click `x` to close a project
- **Global Icon Buttons** (right side) are always accessible across all projects

## Git Worktrees

Git worktrees allow working on multiple branches simultaneously without stashing.

### What is a Worktree?

A worktree is a separate checkout of your repository at a different path. Each worktree:
- Has its own working directory
- Can be on a different branch
- Shares git history with the main repository

### Worktree Tabs

When a project is open, the second row shows worktree tabs. The right side contains an "Env" tab for project-scoped environment file management:

```
┌──────────────────────────────────────────────────────┐
│ [main] [feature/auth] [bugfix/123] [+]  │ Env │      │
└──────────────────────────────────────────────────────┘
```

- **Env tab**: Manage environment files (.env) that sync to all worktrees in this project

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
