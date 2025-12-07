# CLAUDE.md

## Language Preference

Respond in English (U.S.) by default. Use Traditional Chinese only when user writes in Traditional Chinese.

## Repository Overview

Rust monorepo workspace containing **rush** - a shell implementation replacing zsh/bash/fish.

```
rust-station/
├── Cargo.toml          # Workspace root
├── crates/rush/        # Shell implementation
├── specs/              # Feature specifications
│   └── features.json   # Master feature catalog (001-044)
└── target/             # Build output (gitignored)
```

## Spec-Driven Development Workflow

Use spec-kit commands for all feature development:

```
/speckit.specify  → spec.md      # Define requirements
/speckit.clarify  → refine spec  # Ask clarifying questions
/speckit.plan     → plan.md      # Design architecture
/speckit.tasks    → tasks.md     # Generate task breakdown
/speckit.analyze  → validation   # Check consistency
/speckit.checklist → checklist   # QA checklist
/speckit.implement → code+tests  # Implement feature
/speckit.review   → PR review    # Verify against spec
```

### Quick Status

```bash
/spec-status      # Full status
/spec-check       # Quick check
```

## Common Commands

```bash
# Build & Test
cargo build && cargo test
cargo clippy --all-targets --all-features

# GitHub CLI
gh issue create --title "Feature: {name}" --body-file spec.md
gh pr create --title "{description}" --body "Closes #{issue}"
```

## Commit Format

```bash
git commit -m "feat(NNN): description"
```

## Technologies

- Rust 1.75+ (edition 2021)
- reedline 0.26+ (line editing)
- tokio, serde, anyhow/thiserror, tracing

## Test Coverage

- 670+ passing tests
- All tests complete in <1 second

## Active Technologies
- Rust 1.75+ (edition 2021) + No new dependencies (pure Rust implementation) (029-arithmetic-expansion)
- N/A (uses existing VariableManager) (029-arithmetic-expansion)

## Recent Changes
- 029-arithmetic-expansion: Added Rust 1.75+ (edition 2021) + No new dependencies (pure Rust implementation)
