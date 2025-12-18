# Rustation Knowledge Base

**Last updated**: 2025-12-18

Welcome to the rustation knowledge base! This is your central hub for understanding the rustation project architecture, APIs, workflows, and development practices.

---

## Quick Links

**For Contributors**:
- [Technical Debt Analysis](03-complexity-analysis/technical-debt.md) - Current pain points
- [rstn TUI Architecture](01-architecture/rstn-tui-architecture.md) - How the TUI works
- [SDD Workflow Guide](04-sdd-workflow/when-to-use-which.md) - When to use full vs lightweight SDD

**For Users**:
- [System Overview](01-architecture/overview.md) - High-level architecture
- [rush Shell Architecture](01-architecture/rush-shell-architecture.md) - Shell internals
- [MCP Tools Reference](02-api-reference/mcp-tools.md) - Available MCP tools

**For Maintainers**:
- [Refactoring Roadmap](03-complexity-analysis/refactoring-roadmap.md) - Long-term plan
- [rstn Public API](02-api-reference/rstn-public-api.md) - Module documentation

**For Claude Code Integration**:
- [MCP Tools Reference](02-api-reference/mcp-tools.md) - MCP tool schemas
- [Claude CLI Reference](02-api-reference/claude-cli-reference.md) - Condensed CLI flags
- [Claude Code CLI Reference](02-api-reference/claude-code-cli-reference.md) - Complete reference
- [Claude Headless Mode](02-api-reference/claude-headless-mode.md) - Headless patterns

---

## Documentation Structure

```
kb/
├── 00-index.md (this file)           - START HERE
├── 01-architecture/                  - System architecture
│   ├── overview.md                   - High-level system diagram
│   ├── rstn-tui-architecture.md      - TUI layers, event flow, state
│   ├── rush-shell-architecture.md    - Parser, executor, builtins
│   ├── mcp-integration.md            - HTTP server, JSON-RPC, tools
│   └── data-flows.md                 - How data moves through system
├── 02-api-reference/                 - API documentation
│   ├── mcp-tools.md                  - MCP tool schemas
│   ├── claude-cli-reference.md       - Condensed CLI reference
│   ├── claude-code-cli-reference.md  - Complete CLI reference
│   ├── claude-headless-mode.md       - Headless mode patterns
│   ├── rstn-public-api.md            - rstn-core modules (TODO)
│   └── rush-builtins.md              - Shell builtins (TODO)
├── 03-complexity-analysis/           - Current state analysis
│   ├── technical-debt.md             - Complexity hotspots
│   ├── worktree-view-breakdown.md    - 54-field struct analysis
│   ├── app-responsibilities.md       - App struct coupling
│   └── refactoring-roadmap.md        - Long-term refactoring plan
└── 04-sdd-workflow/                  - Development workflow
    ├── full-sdd-template.md          - For complex features
    ├── lightweight-sdd-template.md   - For simple changes
    └── when-to-use-which.md          - Decision flowchart
```

---

## Getting Started

### I'm a new contributor

1. Read [System Overview](01-architecture/overview.md) to understand the big picture
2. Read [Technical Debt Analysis](03-complexity-analysis/technical-debt.md) to understand current challenges
3. Read [SDD Workflow Guide](04-sdd-workflow/when-to-use-which.md) to learn the development process
4. Pick a feature from `specs/` and start coding!

### I'm debugging an issue

1. Check [rstn TUI Architecture](01-architecture/rstn-tui-architecture.md) for event flow
2. Check [MCP Integration](01-architecture/mcp-integration.md) if it's MCP-related
3. Check logs at `~/.rustation/logs/rstn.log`
4. See [Technical Debt](03-complexity-analysis/technical-debt.md) for known issues

### I'm implementing a feature

1. Decide on workflow using [SDD Decision Guide](04-sdd-workflow/when-to-use-which.md)
2. For full SDD: Run `/speckit.specify` → `/speckit.plan` → `/speckit.tasks`
3. For lightweight SDD: Run `/speckit-lite` (coming soon)
4. Follow the implementation plan

### I want to understand the codebase

1. Start with [System Overview](01-architecture/overview.md)
2. Deep dive into [rstn TUI Architecture](01-architecture/rstn-tui-architecture.md) or [rush Shell Architecture](01-architecture/rush-shell-architecture.md)
3. Check [API Reference](02-api-reference/rstn-public-api.md) for module details
4. Read existing specs in `specs/` for feature examples

---

## Key Technologies

- **Rust 1.75+** (edition 2021)
- **ratatui 0.29+** - TUI framework
- **crossterm 0.28** - Terminal I/O
- **tokio** - Async runtime
- **axum** - HTTP server (MCP)
- **serde/serde_json** - Serialization
- **tracing** - Logging

---

## Project Status (as of 2025-12-18)

**rush shell**:
- Status: Maintenance mode
- Completed: Phase 1-6 (35 features)
- Remaining: Phase 7-8 (6 features, then suspend)

**rstn TUI**:
- Status: Active development
- Current focus: Knowledge base organization + refactoring
- Challenge: Technical debt (God Classes, tight coupling)
- Plan: 5-6 month refactoring roadmap

---

## Key Metrics

**Codebase**:
- Total rstn source: 24,491 lines (76 files)
- App.rs: 3,404 lines (target: <500)
- WorktreeView: 4,118 lines (target: <500)

**Documentation**:
- Specs: 250 files, 30,210 lines
- KB docs: 8-10 core documents (target)
- Architecture diagrams: In progress

**Process**:
- SDD success rate: 83% (20/24 features with full SDD completed)
- Test coverage: ~40% (target: 70%+)

---

## Related Documentation

- [CLAUDE.md](../CLAUDE.md) - Development workflow instructions
- [ROADMAP.md](../ROADMAP.md) - rush shell roadmap
- [specs/](../specs/) - Feature specifications
- [.claude/agents/](../.claude/agents/) - Subagent definitions
- [.claude/commands/](../.claude/commands/) - Slash commands

---

## Contributing

Questions or suggestions for the knowledge base?
- Open an issue in the GitHub repository
- Update this documentation and submit a PR
- Ask in the team chat

---

## Changelog

- 2025-12-18: Initial KB structure created
