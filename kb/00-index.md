# Rustation Knowledge Base

**Last updated**: 2025-12-19

**🎯 rustation v2 - Fresh Start**: All v1 specs (001-065) archived. Starting clean with state-first architecture.

Welcome to the rustation knowledge base! This is your central hub for understanding the rustation project architecture, APIs, workflows, and development practices.

---

## 🆕 What's New in v2

**Project Restart** (2025-12-19): Rustation has been restarted with a clean slate approach:
- **State-first architecture**: All state must be JSON/YAML serializable (Feature 079)
- **Simplified design**: Focus on core workflows, avoid complexity
- **Clean separation**: CLI/TUI share business logic, different interfaces
- **Testability**: State-based testing, not UI testing

**v1 archived**: All previous specs and design docs moved to [`99-archive/`](99-archive/) for historical reference.

---

## Quick Links

**For Contributors**:
- [State Serializability](01-architecture/state-serializability.md) - **🎯 Core principle**: State as JSON/YAML
- [SDD Workflow Guide](04-sdd-workflow/when-to-use-which.md) - When to use full vs lightweight SDD
- [MCP Tools Reference](02-api-reference/mcp-tools.md) - Available MCP tools

**For Users**:
- [MCP Tools Reference](02-api-reference/mcp-tools.md) - Available MCP tools
- Installation guide (coming soon)

**For Claude Code Integration**:
- [MCP Tools Reference](02-api-reference/mcp-tools.md) - MCP tool schemas
- [Claude CLI Reference](02-api-reference/claude-cli-reference.md) - Condensed CLI flags
- [Claude Code CLI Reference](02-api-reference/claude-code-cli-reference.md) - Complete reference
- [Claude Headless Mode](02-api-reference/claude-headless-mode.md) - Headless patterns

**v1 Archive** (historical reference only):
- [Archive README](99-archive/) - What's archived and why
- v1 complexity analysis, design docs (not for implementation)

---

## Documentation Structure

```
kb/
├── 00-index.md (this file)           - START HERE
├── 01-architecture/                  - System architecture
│   └── state-serializability.md      - **🎯 Core principle**: State as JSON/YAML
├── 02-api-reference/                 - API documentation
│   ├── mcp-tools.md                  - MCP tool schemas
│   ├── claude-cli-reference.md       - Condensed CLI reference
│   ├── claude-code-cli-reference.md  - Complete CLI reference
│   └── claude-headless-mode.md       - Headless mode patterns
├── 04-sdd-workflow/                  - Development workflow
│   ├── full-sdd-template.md          - For complex features
│   ├── lightweight-sdd-template.md   - For simple changes
│   └── when-to-use-which.md          - Decision flowchart
└── 99-archive/                       - v1 documentation (archived)
    ├── README.md                     - What's archived and why
    ├── v1-analysis/                  - v1 complexity analysis
    │   ├── technical-debt.md         - v1 God Classes, state explosion
    │   └── architecture-overview.md  - v1 system architecture
    └── v1-designs/                   - Unimplemented v1 designs
        ├── worktree-view-redesign.md
        ├── worktree-state-machine.md
        ├── rstn-integration-flow.md
        └── logging-specification.md
```

**Note**: v2 docs will be created as needed. Start simple, avoid over-documentation.

---

## Getting Started

### I'm a new contributor (v2)

1. Read [State Serializability](01-architecture/state-serializability.md) - **The v2 core principle**
2. Read [SDD Workflow Guide](04-sdd-workflow/when-to-use-which.md) - Development process
3. Look at recent commits to see v2 patterns
4. Check `specs/` for active features (v1 archived)

### I'm debugging an issue (v2)

1. Check logs at `~/.rustation/logs/rstn.log` or `~/.rstn/logs/`
2. Review [MCP Tools Reference](02-api-reference/mcp-tools.md) if MCP-related
3. Use `--save-state` / `--load-state` to reproduce issues
4. Review recent code in `crates/rstn/src/`

### I'm implementing a feature (v2)

1. **State-first**: Define state structs FIRST (must be serializable)
2. Write state tests BEFORE implementation
3. Use [SDD Workflow](04-sdd-workflow/when-to-use-which.md) for planning
4. For SDD: Run `/speckit.specify` → `/speckit.plan` → `/speckit.tasks`
5. Ensure all state tests pass

### I want to understand the codebase (v2)

1. Start with [State Serializability](01-architecture/state-serializability.md) - Core principle
2. Read `CLAUDE.md` for CLI/TUI architecture pattern
3. Explore `crates/rstn/src/tui/state/` for state management
4. Check `crates/rstn/tests/` for testing patterns
5. (Optional) Read [v1 Archive](99-archive/) for historical context

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

## Project Status (as of 2025-12-19)

**🆕 rustation v2 - Fresh Start**:
- Status: **Active restart** (v1 archived 2025-12-19)
- Core principle: **State-first architecture** (Feature 079)
- Approach: Clean slate, avoid v1 complexity
- All v1 specs (001-065) archived to `specs/archive/`

**rush shell**:
- Status: Maintenance mode (unchanged)
- Completed: Phase 1-6 (35 features)
- Remaining: Phase 7-8 (6 features, then suspend)

**rstn TUI** (v2):
- Status: Rebuilding from scratch
- Current focus: State management, testing infrastructure
- Philosophy: Simple, observable, testable
- No v1 technical debt to carry forward

---

## Key Metrics (v2)

**Codebase** (post-restart):
- v1 archived, v2 metrics TBD
- Target: Keep modules small (<500 lines)
- Target: State structs simple (<15 fields)

**Documentation**:
- KB docs: 7 active files (~3,700 lines)
- v1 archive: 6 files (~3,556 lines)
- Specs: v1 archived (001-065), v2 TBD

**Process**:
- State-first testing: **Mandatory** for all features
- Test coverage target: 70%+ (state + integration)
- SDD workflow: Continued from v1

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

- 2025-12-19: **v2 Restart** - Archived all v1 content (6 docs, ~3,556 lines) to `99-archive/`
  - Removed broken references from index
  - Updated Quick Links to reflect v2 focus
  - Simplified documentation structure
  - Added v2 guidance throughout
- 2025-12-18: Added state-serializability.md - Core architecture principle (Feature 079)
- 2025-12-18: Initial KB structure created
