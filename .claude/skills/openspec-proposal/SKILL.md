---
name: openspec-proposal
description: Scaffold a new OpenSpec change proposal with spec deltas, tasks, and design docs. Use when the user requests a new feature, capability, architecture change, or says "create a proposal", "spec this out", "plan a feature", or mentions adding requirements. For non-trivial changes (>500 LOC, >5 files, new capabilities).
---

# OpenSpec Proposal Skill

Create specification-driven change proposals following the OpenSpec workflow.

## When to Use This Skill

Automatically use when the user:
- Requests a **new feature** or **capability**
- Asks to **add/modify requirements**
- Says "**create a proposal**", "**spec this**", "**plan this feature**"
- Mentions **architecture changes** or **new patterns**
- Describes work that's **non-trivial** (>500 LOC, >5 files, complex logic)

**Do NOT use** for:
- Simple bug fixes (< 100 LOC, single file)
- Documentation-only changes
- Quick tweaks or refactoring

---

## Instructions

### Guardrails
- Favor straightforward, minimal implementations first; add complexity only when clearly required
- Keep changes tightly scoped to the requested outcome
- Refer to `openspec/AGENTS.md` if you need OpenSpec conventions or clarifications
- Identify vague or ambiguous details and ask follow-up questions
- **Do NOT write code** during the proposal stage—only create design documents

### Steps

1. **Review context**
   - Read `openspec/project.md` to understand project conventions
   - Run `openspec list` and `openspec list --specs` to see existing changes and specs
   - Use `rg` or `ls` to inspect related code/docs
   - Note any gaps requiring clarification

2. **Create proposal structure**
   - Choose a unique verb-led `change-id` (e.g., `add-docker-compose`, `refactor-mcp-tools`)
   - Scaffold under `openspec/changes/<id>/`:
     - `proposal.md` - Overview and rationale
     - `tasks.md` - Ordered implementation checklist
     - `design.md` - Architecture decisions (if complex)

3. **Define spec deltas**
   - Create `changes/<id>/specs/<capability>/spec.md` for each new capability
   - Use `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`
   - Each requirement needs at least one `#### Scenario:` with WHEN/THEN bullets
   - Cross-reference related capabilities when relevant

4. **Draft tasks**
   - Break down into small, verifiable work items
   - Ensure tasks deliver user-visible progress
   - Include validation steps (tests, tooling)
   - Highlight dependencies or parallelizable work

5. **Validate**
   - Run `openspec validate <id> --strict`
   - Fix all validation errors before presenting to user
   - Use `openspec show <id> --json --deltas-only` to inspect details

---

## Examples

### Example: User Request
```
User: "I want to add Docker Compose support to the project management feature"
```

**Your response:**
1. Ask clarifying questions about requirements
2. Create `openspec/changes/add-docker-compose/`
3. Draft spec deltas for new Docker Compose capabilities
4. Create ordered task list
5. Validate with `openspec validate add-docker-compose --strict`
6. Present proposal for approval

### Example: Proposal Structure
```
openspec/changes/add-docker-compose/
├── proposal.md          # Why, what, scope
├── tasks.md             # [ ] Task checklist
├── design.md            # Architecture decisions (if needed)
└── specs/
    └── docker-compose/
        └── spec.md      # ## ADDED Requirements
```

---

## Reference

- Search existing requirements: `rg -n "Requirement:|Scenario:" openspec/specs`
- Explore codebase: `rg <keyword>`, `ls`, or direct file reads
- Validation help: `openspec show <spec> --type spec`
- Full workflow: See `openspec/AGENTS.md`

---

## After Completion

Present the proposal to the user:
1. Summarize the change
2. List spec deltas created
3. Show task count and highlights
4. Confirm validation passed
5. Ask: "Should I proceed with implementation?" (triggers `openspec-apply` skill)
