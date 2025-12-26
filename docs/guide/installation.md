# Installation

## Requirements

- **macOS** 12.0 (Monterey) or later
- **Docker Desktop** (for container management features)
- **just** command runner (optional, for task features)

## Install via DMG

1. Download the latest `.dmg` from [Releases](https://github.com/user/rustation/releases)
2. Open the DMG file
3. Drag **Rustation** to your Applications folder
4. Launch from Applications or Spotlight

## Install via Homebrew

```bash
brew tap user/rustation
brew install --cask rustation
```

## Build from Source

### Prerequisites

- Node.js 20+
- pnpm 8+
- Rust 1.75+

### Steps

```bash
# Clone repository
git clone https://github.com/user/rustation.git
cd rustation

# Install dependencies
pnpm install

# Build Rust core
cd packages/core && pnpm build && cd ../..

# Run in development
cd apps/desktop && pnpm dev

# Or build for production
cd apps/desktop && pnpm build:mac
```

## Verify Installation

Launch Rustation from your Applications folder. You should see:

1. An empty window with "Open Project" button
2. A sidebar with Tasks, Docker, and Settings icons

If Docker is running, the Docker tab will show available services.

## Optional: Install just

Rustation uses [just](https://github.com/casey/just) for task running. Install it to enable the Tasks tab:

```bash
# macOS
brew install just

# Or via cargo
cargo install just
```

## Troubleshooting

### App won't open (unidentified developer)

```bash
# Allow the app in System Preferences > Security & Privacy
# Or run:
xattr -d com.apple.quarantine /Applications/Rustation.app
```

### Docker tab shows "Docker Not Available"

1. Ensure Docker Desktop is running
2. Check Docker CLI: `docker ps`
3. Restart Rustation

### Tasks tab shows "No justfile found"

Create a `justfile` in your project root:

```just
# Build the project
build:
    echo "Building..."

# Run tests
test:
    echo "Testing..."
```
