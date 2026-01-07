# Engineering Handbook

**Target Audience**: Contributors, Developers, Maintainers

This handbook contains architecture decisions, development guides, and internal documentation for contributing to rustation.

> **For Feature Specifications**: See [`openspec/specs/`](../openspec/specs/)
> **For User Documentation**: See [`docs/`](../docs/)

---

## 🏗️ Architecture

*Why we built things this way*

| Document | Description |
|----------|-------------|
| [00. Overview](architecture/00-overview.md) | Three pillars: State-First, Frontend/Backend Separation, Workflow-Driven UI |
| [01. UI Components](architecture/01-ui-component-architecture.md) | Material Design 3 (MUI) architecture |
| [02. State Topology](architecture/02-state-topology.md) | AppState tree structure |
| [03. Persistence](architecture/03-persistence.md) | Save/load application state |
| [07. Testing](architecture/07-testing.md) | Testing patterns and strategies |

---

## 🔧 Development Workflow

*How to contribute*

| Document | Description |
|----------|-------------|
| [Contribution Guide](workflow/contribution-guide.md) | Dev environment setup, PR workflow |
| [Definition of Done](workflow/definition-of-done.md) | Feature completion checklist |

---

## 🔍 Internals

*Implementation details*

| Document | Description |
|----------|-------------|
| [MCP Tools](internals/mcp/tools.md) | MCP server implementation |
| [File Reader](internals/file-reader.md) | Safe file reading implementation |

---

## 🧪 Experimental

*Features in early prototyping phase*

| Document | Description |
|----------|-------------|
| [A2UI](experimental/a2ui.md) | Server-Driven UI protocol exploration |

---

## Quick Reference

### Tech Stack
- **Desktop**: Electron
- **Frontend**: React 19 + Vite + MUI (Material UI v7)
- **Backend**: Rust (napi-rs)
- **State**: Rust AppState (JSON-serializable)

### Commands
```bash
# Development
cd desktop && pnpm dev

# Build
cd packages/core && pnpm build
cd desktop && pnpm build

# Tests
cargo test                    # Rust tests
pnpm test                     # React tests
pnpm test:e2e                 # Playwright E2E
```

### Key Directories
```
packages/core/src/    # Rust napi-rs module
desktop/src/          # Electron + React app
openspec/specs/       # Feature specifications (see OpenSpec)
dev-docs/                   # This documentation (contribution guide)
docs/                 # User documentation
```

---

## Related Documentation

- **Feature Specifications**: [`openspec/specs/`](../openspec/specs/) - What features do (Requirements + Scenarios)
- **User Manual**: [`docs/`](../docs/) - How to use rustation (Guides + Tutorials)
- **Project Context**: [`openspec/project.md`](../openspec/project.md) - Tech stack, conventions, constraints
