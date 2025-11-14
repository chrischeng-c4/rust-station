---
name: spec-kit
description: Specification-driven development workflow using GitHub Spec-Kit. Use when starting new features, implementing complex functionality, planning architecture, or when code is being written without clear specifications. Guides through constitution → specify → plan → tasks → implement workflow. Helps maintain consistency between requirements and implementation.
allowed-tools: Read, Grep, Glob
---

# Spec-Kit: Specification-Driven Development

This skill guides you through the GitHub Spec-Kit workflow, ensuring all development is specification-driven rather than ad-hoc.

## When to Use This Skill

Activate this skill when:

1. **Starting new features** without existing specifications
2. **User requests implementation** of complex functionality
3. **Detecting ambiguity** in requirements or approach
4. **Code changes proposed** without clear specifications
5. **Planning architecture** for new components
6. **Inconsistency detected** between stated goals and implementation

## Spec-Kit Workflow Phases

### Phase 1: Constitution (`/speckit.constitution`)
**Purpose**: Establish project principles, values, and constraints

Run this FIRST for new projects or when fundamental principles need clarity. The constitution serves as the foundation for all other specifications.

**Key Questions**:
- What are the project's core values?
- What constraints must we operate within?
- What are the non-negotiables?
- What trade-offs do we prefer?

**Output**: `.specify/memory/constitution.md`

### Phase 2: Specify (`/speckit.specify`)
**Purpose**: Document WHAT needs to be built (not HOW)

Focus on requirements, user stories, and desired outcomes. Avoid implementation details.

**Key Questions**:
- What problem are we solving?
- Who are the users?
- What are the success criteria?
- What are the user stories?
- What are the acceptance criteria?

**Output**: `.specify/memory/spec-*.md` files

### Phase 3: Plan (`/speckit.plan`)
**Purpose**: Develop technical approach and architecture

Translate specifications into technical plans. This is where we decide HOW to implement.

**Key Questions**:
- What architecture should we use?
- What technologies fit our constraints?
- How do we break down the work?
- What are the technical risks?
- What dependencies exist?

**Output**: `.specify/memory/plan-*.md` files

### Phase 4: Tasks (`/speckit.tasks`)
**Purpose**: Break down plans into actionable tasks

Create concrete, implementable tasks from plans.

**Key Questions**:
- What are the discrete steps?
- What's the implementation order?
- What are the dependencies between tasks?
- How do we verify completion?

**Output**: `.specify/memory/tasks-*.md` files

### Phase 5: Implement (`/speckit.implement`)
**Purpose**: Execute tasks and build features

Follow the specifications, plans, and tasks to implement features.

**Key Principles**:
- Reference specifications frequently
- Validate against acceptance criteria
- Update specs if requirements change
- Document decisions and deviations

## Enhancement Commands (Optional)

### `/speckit.clarify`
Run BEFORE planning when requirements are ambiguous. Generates structured questions to de-risk unclear areas.

### `/speckit.analyze`
Run AFTER tasks, BEFORE implementation. Generates cross-artifact consistency report to catch misalignments.

### `/speckit.checklist`
Run AFTER planning. Creates quality validation checklists for requirements completeness.

## Checking Current State

To understand where you are in the spec-driven workflow:

1. **Check for constitution**: Read `.specify/memory/constitution.md`
2. **Check for specifications**: List `.specify/memory/spec-*.md` files
3. **Check for plans**: List `.specify/memory/plan-*.md` files
4. **Check for tasks**: List `.specify/memory/tasks-*.md` files

## Guiding Principles

1. **Specifications before code**: Never implement without specs
2. **Constitution guides everything**: All decisions align with constitutional principles
3. **What before how**: Specifications describe outcomes, plans describe approaches
4. **Traceability**: Every line of code traces back to a specification
5. **Evolving specifications**: Update specs when requirements change

## Example Workflow for Rush Shell

For the rush shell project, a proper workflow would be:

1. `/speckit.constitution` - Establish shell design philosophy
   - POSIX compatibility?
   - Performance vs. features?
   - Plugin architecture?
   - User experience principles?

2. `/speckit.specify` - Document shell features
   - Basic command execution
   - Job control
   - History management
   - Tab completion
   - Configuration system

3. `/speckit.plan` - Plan technical architecture
   - Parser design (recursive descent? PEG?)
   - Execution model (fork/exec? threads?)
   - Plugin system architecture
   - Configuration format

4. `/speckit.tasks` - Break into tasks
   - Implement tokenizer
   - Implement parser
   - Implement command executor
   - Add builtin commands
   - Add history support

5. `/speckit.implement` - Build the shell
   - Execute tasks in order
   - Validate against specifications
   - Update specs as needed

## Integration with Rust Monorepo

This repository is a Rust workspace. When using spec-kit:

- Place rush-specific specs in `.specify/memory/rush/`
- Use workspace-level specs for cross-project concerns
- Reference `Cargo.toml` workspace configuration in plans
- Consider shared dependencies when planning

## Avoiding Common Mistakes

❌ **Don't**: Jump straight to implementation
✅ **Do**: Start with constitution and specifications

❌ **Don't**: Write specs that describe implementation details
✅ **Do**: Write specs that describe requirements and outcomes

❌ **Don't**: Ignore the constitution
✅ **Do**: Validate all decisions against constitutional principles

❌ **Don't**: Let specs become stale
✅ **Do**: Update specs when requirements change

## Next Steps

Based on the current project state:

1. **If no constitution exists**: Run `/speckit.constitution`
2. **If no specifications exist**: Run `/speckit.specify`
3. **If implementing without specs**: STOP and run `/speckit.specify` first
4. **If specs exist but unclear**: Run `/speckit.clarify`
5. **If ready to implement**: Verify specs → plan → tasks are complete, then `/speckit.implement`
