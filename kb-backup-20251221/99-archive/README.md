# Knowledge Base Archive

**Archived**: 2025-12-19
**Reason**: Rustation v2 restart - clean slate approach

---

## What's Archived Here

This directory contains knowledge base documentation from **rustation v1** that no longer reflects the current codebase or architecture.

### Why Archive?

**Project Restart**: Rustation underwent a complete restart in December 2025. All v1 specifications (features 001-065) were archived, and the project is being rebuilt with:
- State-first architecture (Feature 079 as core principle)
- Clean separation of CLI/TUI concerns
- Simplified, observable design
- Fresh start without legacy constraints

**Historical Value**: These documents are preserved for:
- Understanding the evolution of design thinking
- Learning from past complexity issues
- Reference when similar challenges arise
- Project continuity and context

---

## Archive Structure

### `v1-analysis/` - Analysis of v1 Codebase

Documentation analyzing the complexity and technical debt of the v1 implementation:

- **technical-debt.md** (490 lines) - Comprehensive analysis of v1 issues
  - God Classes: App (3,404 LOC), WorktreeView (4,118 LOC)
  - State explosion: 54+ mutable fields
  - 308 unwrap/panic sites
  - ~40% test coverage
  - 5-6 month refactoring roadmap

- **architecture-overview.md** (477 lines) - v1 system architecture
  - High-level component diagram
  - rstn TUI + rush shell structure
  - MCP integration architecture
  - Directory structure
  - Technology stack

### `v1-designs/` - Unimplemented v1 Designs

Design documents created for v1 refactoring that were never implemented due to the v2 restart:

- **worktree-view-redesign.md** (593 lines) - Three-column layout design
  - 20/40/40 column split
  - Two-tier logging system
  - Session ID tracking
  - Copy functions for debug info
  - Never implemented - v2 uses different approach

- **worktree-state-machine.md** (545 lines) - v1 state machine documentation
  - 3-level state hierarchy
  - Complex state transitions
  - 54+ state fields
  - Documents problems with v1 approach

- **rstn-integration-flow.md** (605 lines) - rstn ↔ MCP ↔ Claude research
  - Sequence diagrams
  - Stream parsing flow
  - Research plan (never completed)
  - MCP tool call patterns

- **logging-specification.md** (846 lines) - Logging design spec
  - Two-tier logging (panel + file)
  - Comprehensive checkpoint list
  - Observability requirements
  - Never fully implemented in v1

---

## What Changed in v2

### Core Principles
- **State-first architecture** (Feature 079): All state must be JSON/YAML serializable
- **Simplified design**: Focus on one core workflow at a time
- **Clean slate**: No legacy constraints, rebuild from scratch
- **Testability**: State-based testing (not UI testing)

### Key Differences
| Aspect | v1 (Archived) | v2 (Current) |
|--------|---------------|--------------|
| **Complexity** | 4,118 line WorktreeView | Simplified, modular design |
| **State** | 54+ mutable fields | State-first, serializable |
| **Testing** | ~40% coverage, UI tests | 70%+ coverage, state tests |
| **Architecture** | God Classes, tight coupling | Clean separation, small modules |
| **Approach** | Incremental refactoring | Fresh start |

---

## Using Archived Documents

### ✅ Good Uses
- Understanding why certain v1 approaches were abandoned
- Learning from complexity analysis
- Historical context for design decisions
- Reference for "what not to do"

### ⚠️ Caution
- **Don't implement v1 designs**: They were created for v1 architecture
- **Don't reference as current**: These don't reflect v2 reality
- **Don't copy complexity**: v2 aims to avoid v1 issues

### ❌ Don't Use For
- Current architecture reference (use `kb/02-architecture/` instead)
- Implementation guidance (use current specs in `specs/`)
- API reference (use `kb/03-api-reference/`)

---

## Current KB Organization

For **current** documentation, see:

- **`kb/01-getting-started/`** - Installation, quick start, concepts (v2)
- **`kb/02-architecture/`** - State-first principle, core principles (v2)
- **`kb/03-api-reference/`** - MCP tools, Claude CLI reference (current)
- **`kb/04-development/`** - SDD workflow, contribution guide (v2)

Start at [`kb/00-index.md`](../00-index.md) for navigation.

---

## Restoration

**Q: Can we restore archived content?**
**A:** Yes, but carefully:

1. **Don't restore wholesale**: v2 has different architecture
2. **Extract specific insights**: Complexity analysis, lessons learned
3. **Adapt to v2**: Rewrite for state-first architecture
4. **Update references**: Ensure compatibility with v2

**Process**:
1. Read archived document
2. Identify valuable insights (not implementation details)
3. Create NEW v2 document with adapted content
4. Reference archive for historical context

---

## Archive Metadata

**Total Lines**: ~3,556 lines archived
**Date Range**: Created 2025-12-18, Archived 2025-12-19
**v1 Lifespan**: ~1 day (rapid prototyping phase)
**v1 Specs**: 001-065 (archived to `specs/archive/`)

**Why so short?**: v1 was a learning phase. After building initial features and documentation, we realized the architecture needed a fundamental rethink. Rather than incremental refactoring (5-6 month roadmap), we chose a v2 restart for faster progress.

---

## Questions?

- **"Why archive instead of delete?"** - Historical value, learning, context
- **"Will we ever use this?"** - Insights yes, implementation no
- **"Is v1 a failure?"** - No, it's a learning foundation
- **"What's different in v2?"** - State-first architecture, clean slate
- **"Can I read these docs?"** - Yes, for context and learning

---

## See Also

- **Current docs**: `kb/00-index.md` (start here)
- **v2 core principle**: `kb/02-architecture/state-first.md`
- **Spec archive**: `specs/archive/README.md`
- **v1→v2 transition**: `docs/2025-12-19-v2-restart.md` (if it exists)

---

**Remember**: These documents represent valuable learning, not failed work. They helped us understand what v2 should be.
