# Rush Shell Specifications

Feature specifications for the rush shell project.

## Quick Reference

- **Feature List**: See [features.json](features.json) for complete feature catalog (001-044)
- **Current Status**: Phases 1-4 complete (001-026), Phases 5-8 planned (027-044)

## Directory Structure

```
specs/
├── features.json           # Master feature catalog
├── NNN-feature-name/
│   ├── spec.md             # What to build (requirements)
│   ├── plan.md             # How to build (architecture)
│   ├── tasks.md            # Implementation tasks
│   └── checklist.md        # QA checklist (optional)
└── README.md
```

## Feature Status

| Phase | Features | Status |
|-------|----------|--------|
| 1 | 001-016: Core & MVP | Complete |
| 2 | 017-026: Control Flow | Complete |
| 5 | 027-031: Scripting Foundations | Planned |
| 6 | 032-035: Parameter Power | Planned |
| 7 | 036-039: Shell Control | Planned |
| 8 | 040-044: Advanced Features | Planned |

## Workflow

Uses **spec-kit** for specification-driven development:

```
/speckit.specify  → spec.md
/speckit.plan     → plan.md
/speckit.tasks    → tasks.md
/speckit.implement → code + tests
```

See [CLAUDE.md](../CLAUDE.md) for full workflow details.

## Adding Features

1. Create directory: `specs/NNN-feature-name/`
2. Run `/speckit.specify` with feature description
3. Run `/speckit.plan` to create implementation plan
4. Run `/speckit.tasks` to generate task breakdown
5. Update `features.json` with new entry
