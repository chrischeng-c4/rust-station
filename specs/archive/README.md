# Archived Specifications

This directory contains completed feature specifications that have been implemented and merged.

## Archive Structure

- `phase-1-2-rush/` - Rush shell features (Phase 1-2: MVP & Control Flow)
  - Features 001-035: Complete and merged
  
- `phase-9-tui-dx/` - TUI & Developer Experience features (Phase 9)
  - Features 051-065: Complete and merged
  - Includes: Interactive workflows, MCP architecture, prompt management

## Why Archive?

Per project policy: **Specs without test/verification solutions are archived once implementation is complete or abandoned.**

Active specs remain in `specs/` directory. Archived specs serve as historical reference.

## Restoration

To reference an archived spec:
```bash
# View archived spec
cat specs/archive/phase-9-tui-dx/060-mcp-server-infrastructure/spec.md

# Restore to active (if needed)
mv specs/archive/<phase>/<feature>/ specs/
```
