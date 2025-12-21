# rustation v2 Knowledge Base

**Last updated**: 2025-12-21

**🎯 rustation v2 - Fresh Start**: State-first architecture.

---

## Quick Navigation by Role

**New Users**: Start with [Tutorials](#tutorials) → [Installation](01-tutorials/installation.md) → [Quick Start](01-tutorials/quick-start.md)

**Developers**: See [How-to Guides](#how-to-guides) → [SDD Workflow](02-how-to-guides/sdd-workflow.md) → [Testing](02-how-to-guides/testing-guide.md)

**API Consumers**: See [Reference](#reference) → [MCP Tools](04-reference/mcp/tools.md) → [Claude Code CLI](04-reference/claude-code/cli-reference.md)

**Contributors**: See [Contribute](#contribute) → [Contribution Guide](05-contribute/contribution-guide.md)

---

## Tutorials 🎓
*Learning-oriented guides that take you by the hand*

| Document | Description | Status |
|----------|-------------|--------|
| [Installation](01-tutorials/installation.md) | Install rstn on your system | 🟢 Implemented |
| [Quick Start](01-tutorials/quick-start.md) | Run your first session in 10 minutes | 🟢 Implemented |

---

## How-to Guides 🛠️
*Problem-oriented recipes for specific tasks*

| Document | Description | Status |
|----------|-------------|--------|
| [SDD Workflow](02-how-to-guides/sdd-workflow.md) | Full vs Lightweight SDD decision guide | 🟢 Implemented |
| [Testing Guide](02-how-to-guides/testing-guide.md) | State tests, round-trip, transitions | 🟢 Implemented |
| [Debugging](02-how-to-guides/debugging.md) | State inspection, logs, common issues | 🟢 Implemented |

---

## Concepts 💡
*Understanding-oriented explanations of key ideas*

| Document | Description | Status |
|----------|-------------|--------|
| [Overview](03-concepts/overview.md) | Core concepts: state-first, CLI/TUI, SDD, MCP | 🟢 Implemented |
| [Architecture](03-concepts/architecture.md) | Three pillars: state-first, CLI/TUI, testing | 🟢 Implemented |
| [Prompt Workflow](03-concepts/prompt-workflow.md) | Prompt Claude architecture | 🟢 Implemented |
| **State-First Architecture** | | |
| └─ [Overview](03-concepts/state-first/overview.md) | Core principle: all state serializable | 🟢 Implemented |
| └─ [Serialization](03-concepts/state-first/serialization.md) | Rules, patterns, anti-patterns | 🟢 Implemented |
| └─ [Testing](03-concepts/state-first/testing.md) | Round-trip, transitions, invariants | 🟢 Implemented |
| └─ [Migration](03-concepts/state-first/migration.md) | v1 problems → v2 solutions | 🟢 Implemented |

---

## Reference 📚
*Information-oriented technical descriptions*

### MCP Reference
| Document | Description | Status |
|----------|-------------|--------|
| [Tools](04-reference/mcp/tools.md) | MCP tool schemas | 🟢 Implemented |

### Claude Code Integration
| Document | Description | Status |
|----------|-------------|--------|
| [CLI Reference](04-reference/claude-code/cli-reference.md) | Complete CLI reference | 🟢 Implemented |
| [Headless Mode](04-reference/claude-code/headless-mode.md) | Headless mode patterns | 🟢 Implemented |
| [Communication](04-reference/claude-code/communication.md) | rstn ↔ Claude Code channels | 🟢 Implemented |
| **Hooks** (Deprecated) | | |
| └─ [Overview](04-reference/claude-code/hooks/overview.md) | What are hooks, when to use | 🔴 Deprecated |
| └─ [Configuration](04-reference/claude-code/hooks/configuration.md) | Hook setup | 🔴 Deprecated |
| └─ [Events](04-reference/claude-code/hooks/events.md) | Hook lifecycle | 🔴 Deprecated |
| └─ [Examples](04-reference/claude-code/hooks/examples.md) | Real-world examples | 🔴 Deprecated |
| └─ [Troubleshooting](04-reference/claude-code/hooks/troubleshooting.md) | Debug hooks | 🔴 Deprecated |
| **Advanced Features** | | |
| └─ [Overview](04-reference/claude-code/advanced/overview.md) | Feature matrix | 🟢 Implemented |
| └─ [Permission Modes](04-reference/claude-code/advanced/permission-modes.md) | Plan/Auto/Ask modes | 🟢 Implemented |
| └─ [MCP Interaction](04-reference/claude-code/advanced/mcp-interaction.md) | Mini TUI mode | 🟢 Implemented |
| └─ [Multi-File Context](04-reference/claude-code/advanced/multi-file-context.md) | --context flag | 🟢 Implemented |
| └─ [Stream Message](04-reference/claude-code/advanced/stream-message.md) | Extended message struct | 🟢 Implemented |
| └─ [Session Management](04-reference/claude-code/advanced/session-management.md) | Dual-layer sessions | 🟢 Implemented |
| └─ [Edit Approval](04-reference/claude-code/advanced/edit-approval.md) | Diff preview | 🟢 Implemented |
| └─ [Cancellation](04-reference/claude-code/advanced/cancellation.md) | Ctrl+C / Esc | 🟢 Implemented |
| └─ [Cost Tracking](04-reference/claude-code/advanced/cost-tracking.md) | Real-time cost | 🟢 Implemented |
| └─ [Error Handling](04-reference/claude-code/advanced/error-handling.md) | MCP errors | 🟢 Implemented |
| └─ [UI Improvements](04-reference/claude-code/advanced/ui-improvements.md) | Progress indicators | 🟢 Implemented |

---

## Contribute 🚀
*Development-oriented guides for contributors*

| Document | Description | Status |
|----------|-------------|--------|
| [Contribution Guide](05-contribute/contribution-guide.md) | Setup, workflow, PR requirements | 🟢 Implemented |

---

## Legend

- 🟢 **Implemented** - Content current, feature shipped
- 🟡 **Draft** - Documentation written, feature not yet implemented
- 🔴 **Deprecated** - Feature exists but being phased out

---

## Documentation Principles

This KB follows the **Divio Documentation System**:

1. **Tutorials** - *Learning-oriented*: Teach by doing
2. **How-to Guides** - *Problem-oriented*: Solve specific issues
3. **Concepts** - *Understanding-oriented*: Explain why and how
4. **Reference** - *Information-oriented*: Look up facts
5. **Contribute** - *Development-oriented*: Build the project

**Target file size**: <500 lines per document (for maintainability)

---

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - Development workflow instructions
- [specs/](../specs/) - Feature specifications

---

## Contributing to KB

Questions or suggestions?
- Open an issue in the GitHub repository
- Update documentation and submit a PR
- Follow the file size guideline (<500 lines)
