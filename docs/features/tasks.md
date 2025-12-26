# Task Runner

The Tasks tab lets you run [just](https://github.com/casey/just) commands from your project's `justfile`.

## Requirements

- Install `just`: `brew install just`
- Create a `justfile` in your project root

## Tasks Interface

The Tasks tab shows:
- **Command list**: Available tasks from justfile
- **Output panel**: Real-time command output

## Command Cards

Each command displays:

```
┌────────────────────────────────────────────┐
│ build                          [▶ Run]     │
│ Build the project for production           │
└────────────────────────────────────────────┘
```

- **Name**: The justfile recipe name
- **Description**: Comment above the recipe
- **Status**: Idle / Running / Success / Error

## Running Tasks

1. Click the **Run** button on any task
2. Watch output stream in the output panel
3. Status updates to Success (green) or Error (red)

## Writing a Justfile

Create a `justfile` in your project root:

```just
# Build the project
build:
    cargo build --release

# Run all tests
test:
    cargo test

# Format code
fmt:
    cargo fmt

# Start development server
dev:
    pnpm dev
```

### Syntax

```just
# Comment becomes description
recipe-name:
    command1
    command2
```

- Comments directly above a recipe become descriptions
- Recipes can have multiple commands
- Commands run in sequence

## Output Panel

The output panel shows:
- Real-time streaming output
- Both stdout and stderr
- Preserved between command runs (per worktree)

### Controls

| Button | Action |
|--------|--------|
| **Refresh** | Re-parse justfile |
| **Copy** | Copy output to clipboard |
| **Clear** | Clear output panel |

## Per-Worktree State

Each worktree maintains:
- Parsed commands (different justfile per worktree possible)
- Task statuses
- Output history

Switching worktrees preserves your task context.

## Status Indicators

| Status | Color | Meaning |
|--------|-------|---------|
| Idle | Gray | Not run yet |
| Running | Blue | Currently executing |
| Success | Green | Completed successfully |
| Error | Red | Failed with error |

## Troubleshooting

### "No justfile found"

1. Create a `justfile` in project root
2. Click **Refresh** in the Tasks tab
3. Ensure file is named exactly `justfile` (no extension)

### Command fails with "just: command not found"

Install just:
```bash
brew install just
# or
cargo install just
```

### Output not showing

1. Check command produces output
2. Try running directly: `just <command>`
3. Click **Refresh** to reload
