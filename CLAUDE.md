# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Repository Overview

This is a Rust monorepo workspace called `rust-station` that contains multiple Rust projects. The primary project is **rush**, a shell implementation written in Rust designed to replace traditional shells like zsh, bash, and fish.

## Workspace Structure

```
rust-station/
├── Cargo.toml          # Workspace root configuration
├── crates/             # All projects live here
│   └── rush/          # Shell implementation
└── target/            # Shared build output (gitignored)
```

The workspace uses Cargo's workspace feature with `resolver = "2"`. All projects are organized under `crates/` and share common workspace-level configurations.

## Common Commands

### Building

```bash
# Build all workspace members
cargo build

# Build in release mode
cargo build --release

# Build a specific project
cargo build -p rush

# Build and run rush
cargo run -p rush
```

### Testing

```bash
# Run all tests in the workspace
cargo test

# Run tests for a specific project
cargo test -p rush

# Run a specific test
cargo test -p rush test_name
```

### Linting and Formatting

```bash
# Check code with clippy
cargo clippy --all-targets --all-features

# Format all code
cargo fmt

# Check formatting without modifying files
cargo fmt -- --check
```

### Working with Dependencies

```bash
# Add a workspace-level dependency (edit Cargo.toml [workspace.dependencies])
# Then reference it in a crate's Cargo.toml with:
# dependency-name.workspace = true

# Add a project-specific dependency
cd crates/rush
cargo add <dependency-name>
```

### Cleaning

```bash
# Clean all build artifacts
cargo clean
```

## Workspace Configuration

The root `Cargo.toml` defines workspace-level settings that all member crates inherit:
- **version**: 0.1.0
- **edition**: 2021
- **resolver**: Version 2 (newer dependency resolver)

Common dependencies available to all workspace members are defined in `[workspace.dependencies]` including:
- tokio (async runtime)
- serde/serde_json (serialization)
- anyhow/thiserror (error handling)
- tracing/tracing-subscriber (logging)

## Adding New Projects to the Workspace

New projects are automatically included via the `members = ["crates/*"]` glob pattern:

```bash
cd crates
cargo new --bin project-name    # For a binary
cargo new --lib project-name    # For a library
```

The new project will automatically become part of the workspace.

## Spec-Kit: Specification-Driven Development

This repository uses [GitHub Spec-Kit](https://github.com/github/spec-kit), a toolkit for spec-driven development where specifications drive the implementation rather than being written after the fact.

### Spec-Kit Workflow

The recommended workflow follows these phases:

1. **Establish Principles** - `/speckit.constitution`
   - Define project values, constraints, and governing principles
   - Creates the foundation for all subsequent specifications

2. **Create Specifications** - `/speckit.specify`
   - Document requirements, user stories, and what needs to be built
   - Focus on "what" not "how"

3. **Plan Implementation** - `/speckit.plan`
   - Develop technical approach and architecture decisions
   - Translate specifications into technical plans

4. **Generate Tasks** - `/speckit.tasks`
   - Break down plans into concrete, actionable tasks
   - Create implementation checklist

5. **Implement** - `/speckit.implement`
   - Execute tasks to build features
   - Follow the specification and plan

### Enhancement Commands (Optional)

These commands improve quality and reduce risk:

- `/speckit.clarify` - Ask structured questions to de-risk ambiguous areas (run before planning)
- `/speckit.analyze` - Generate cross-artifact consistency report (after tasks, before implementation)
- `/speckit.checklist` - Create quality validation checklists (after planning)

### Spec-Kit Directory Structure

- `.specify/` - Specification artifacts and project memory
  - `memory/` - Constitution and project state
  - `templates/` - Spec document templates
  - `scripts/` - Utility scripts for workflow automation
- `.claude/commands/` - Slash commands for spec-kit workflow

### Using Spec-Kit with Rush

When developing the rush shell, follow the spec-driven approach:
1. Start with `/speckit.constitution` to establish shell design principles
2. Use `/speckit.specify` to document shell features and requirements
3. Plan architecture with `/speckit.plan`
4. Break down into tasks with `/speckit.tasks`
5. Implement features with `/speckit.implement`

This ensures all development is traceable back to specifications and maintains consistency with project principles.

### Spec-Kit Claude Code Integration

This project includes full integration between Spec-Kit and Claude Code through skills, subagents, and hooks for autonomous spec-driven development.

#### Spec-Kit Skill (Autonomous Activation)

A Claude Code skill (`.claude/skills/spec-kit/`) automatically detects when to use the spec-driven workflow. Claude will proactively suggest spec-kit when:
- Starting new features without specifications
- Implementing complex functionality
- Detecting ambiguous requirements
- Proposing code changes without clear specs
- Planning architecture

You don't need to manually invoke spec-kit—Claude will recognize when it's appropriate and guide you through the workflow.

#### Specialized Subagents

Four specialized subagents are available for different phases of spec-driven development:

**1. `spec-writer`** - Specification authoring expert
- Use when: Creating or refining requirements and specifications
- Specializes in: Writing clear, implementation-agnostic specs
- Focus: WHAT needs to be built (not HOW)
- Tools: Read, Write, Edit (limited to spec files)

**2. `spec-analyzer`** - Cross-artifact consistency validator
- Use when: Checking alignment between specs, plans, and code
- Specializes in: Finding gaps, conflicts, and inconsistencies
- Focus: Ensuring traceability across all artifacts
- Tools: Read, Grep, Glob, Bash (read-only)

**3. `spec-planner`** - Technical planning expert
- Use when: Designing architecture and making technical decisions
- Specializes in: Rust best practices, monorepo architecture
- Focus: HOW to implement specifications
- Tools: Read, Write, Edit, Grep, Glob

**4. `spec-implementer`** - Implementation expert
- Use when: Writing code following specs and plans
- Specializes in: Spec-aligned implementation in Rust
- Focus: Building features that match specifications
- Tools: All tools (full implementation capability)

**Invoking Subagents:**
- Claude automatically delegates to appropriate subagents based on task
- You can explicitly request: "Use spec-writer to document the parser requirements"
- Each subagent maintains its own context to avoid polluting main conversation

#### Automated Hooks

Five hooks automate the spec-driven workflow:

**1. SessionStart Hook**
- **Triggers**: When Claude Code session starts
- **Action**: Displays spec-kit status (constitution, specs, plans, tasks)
- **Purpose**: Immediate visibility into project state
- **Script**: `.specify/scripts/bash/load-spec-context.sh`

**2. UserPromptSubmit Hook**
- **Triggers**: When you submit a prompt containing implementation keywords
- **Action**: Warns if specs/plans are missing before implementation
- **Purpose**: Prevents implementation without specifications
- **Script**: `.specify/scripts/bash/inject-spec-context.sh`

**3. PreToolUse Hook (Edit/Write)**
- **Triggers**: Before editing or creating Rust files
- **Action**: Validates that specifications exist for code changes
- **Purpose**: Ensures code changes are specification-driven
- **Script**: `.specify/scripts/bash/validate-spec-alignment.sh`

**4. PostToolUse Hook (Edit/Write)**
- **Triggers**: After editing or creating Rust files in rush project
- **Action**: Tracks implementation progress
- **Purpose**: Maintains history of what's been implemented
- **Script**: `.specify/scripts/bash/update-spec-memory.sh`

**5. Stop Hook**
- **Triggers**: When Claude finishes responding
- **Action**: Reminds to document work and run consistency checks
- **Purpose**: Ensures work is properly captured in specifications
- **Script**: `.specify/scripts/bash/check-spec-documentation.sh`

**Disabling Hooks:**
If hooks cause issues, disable with: `.claude/settings.local.json` → `"disableAllHooks": true`

#### Helper Scripts

Utility scripts for spec-kit management:

```bash
# Check current spec-kit status
.specify/scripts/bash/spec-status.sh

# Manually load spec context (also runs on SessionStart)
.specify/scripts/bash/load-spec-context.sh
```

#### How Integration Works

1. **Session Start**: Hook displays spec-kit status and suggests next steps
2. **Planning Phase**: Skill activates if you start implementing without specs
3. **Specification Writing**: Claude may delegate to `spec-writer` subagent
4. **Technical Planning**: Claude may delegate to `spec-planner` subagent
5. **Implementation**: Hooks warn if trying to code without specs
6. **Validation**: `spec-analyzer` subagent checks cross-artifact consistency
7. **Completion**: Hook reminds to document and validate work

#### Best Practices with Integration

**Let Claude Guide You:**
- Trust the skill to activate when needed
- Follow suggestions to run `/speckit.*` commands
- Pay attention to hook warnings

**Use Subagents Explicitly When Needed:**
- "Use spec-writer to create a specification for command parsing"
- "Have spec-analyzer check if my code aligns with specifications"
- "Ask spec-planner to design the plugin architecture"

**Monitor Hook Output:**
- SessionStart shows where you are in the workflow
- PreToolUse warns before misaligned changes
- Stop reminds to validate and document

**Iterate on Specifications:**
- Specs can evolve as you learn more
- Update specs when requirements change
- Run `/speckit.analyze` periodically to check alignment

#### Troubleshooting

**Skill not activating:**
- Ensure `.claude/skills/spec-kit/SKILL.md` exists
- Check skill description matches your use case
- Try explicitly mentioning "specification" in your request

**Subagent not delegating:**
- Claude chooses when to delegate automatically
- You can explicitly request a subagent
- Check subagent `.md` files exist in `.claude/agents/`

**Hooks causing issues:**
- Check script permissions: `ls -la .specify/scripts/bash/`
- Review hook output for errors
- Temporarily disable: `"disableAllHooks": true`

**Scripts failing:**
- Ensure scripts are executable: `chmod +x .specify/scripts/bash/*.sh`
- Check script paths in `.claude/settings.local.json`
- Run scripts manually to debug: `.specify/scripts/bash/spec-status.sh`

## Rush Shell Project

Located in `crates/rush/`, this is a shell implementation being developed as an alternative to traditional Unix shells. It's a binary project with its entry point at `crates/rush/src/main.rs`.
