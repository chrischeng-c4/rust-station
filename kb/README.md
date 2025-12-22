# rustation v2 Engineering Handbook

**Target Audience**: Contributors, Developers, Maintainers.
**For Users**: See [`docs/`](../docs/README.md) for installation and usage guides.

---

## Architecture 🏗️
*Core design principles and internal models*

| Document | Description | Status |
|----------|-------------|--------|
| [Overview](architecture/00-overview.md) | The Three Pillars: State-First, CLI/TUI, Testing | 🟢 Active |
| [State-First Principle](architecture/01-state-first-principle.md) | Core principle: all state serializable | 🟢 Active |
| [State-First MVI](architecture/02-state-first-mvi.md) | **Runtime Model**: Msg → Reduce → State → Render | 🟢 Active |
| [Serialization](architecture/03-serialization.md) | Rules, patterns, anti-patterns | 🟢 Active |
| [Testing State](architecture/04-testing-state.md) | Round-trip, transitions, invariants | 🟢 Active |
| [Migration from v1](architecture/05-migration-from-v1.md) | v1 problems → v2 solutions | 🟢 Active |
| [State Topology](architecture/06-state-topology.md) | Structure of the AppState tree | 🟢 Active |
| [Layout Management](architecture/07-layout-management.md) | Layout as State & Workflow-Driven Layouts | 🟢 Active |

---

## Workflow & Standards 🛠️
*Development processes and guidelines*

| Document | Description | Status |
|----------|-------------|--------|
| [SDD Workflow](workflow/sdd-workflow.md) | Specification-Driven Development guide | 🟢 Implemented |
| [Testing Guide](workflow/testing-guide.md) | How to write state & MVI tests | 🟢 Implemented |
| [Debugging](workflow/debugging.md) | State inspection, logs, troubleshooting | 🟢 Implemented |
| [Contribution Guide](workflow/contribution-guide.md) | PR requirements, code style | 🟢 Implemented |

---

## Internals ⚙️
*Deep dives into subsystems*

| Document | Description | Status |
|----------|-------------|--------|
| [MCP Tools](internals/mcp/tools.md) | Internal MCP tool schemas and protocol | 🟢 Implemented |

---

## Legend

- 🟢 **Active/Implemented** - Current source of truth
- 🟡 **Draft** - Work in progress
- 🔴 **Deprecated** - Kept for reference only

---

## Documentation Principles

This `kb/` directory is the **Source of Truth** for the codebase.
- **Code follows KB**: If code contradicts KB, code is wrong (or KB needs update).
- **KB-First**: Design changes must be documented here *before* implementation.
