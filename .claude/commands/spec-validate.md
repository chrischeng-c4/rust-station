---
description: Run comprehensive spec-kit consistency analysis using the spec-analyzer subagent
argument-hint: ""
---

Perform a comprehensive consistency analysis of all spec-kit artifacts by delegating to the **spec-analyzer** subagent.

The spec-analyzer will:
1. Check constitutional adherence across all artifacts
2. Validate specification coverage and completeness
3. Verify plan-to-task alignment
4. Assess implementation alignment with specs
5. Identify conflicts and inconsistencies
6. Generate actionable recommendations

Tell the user you're delegating to the spec-analyzer subagent, then invoke the Task tool with subagent_type=general-purpose and prompt:

```
You are the spec-analyzer subagent. Perform a comprehensive consistency analysis of this Spec-Kit project.

Read and analyze:
1. .specify/memory/constitution.md
2. All .specify/memory/spec-*.md files
3. All .specify/memory/plan-*.md files
4. All .specify/memory/tasks-*.md files
5. Rust source code in crates/rush/src/

Generate a detailed consistency report covering:
- Constitutional adherence
- Specification coverage
- Plan-to-task alignment
- Implementation alignment
- Cross-artifact conflicts
- Recommendations (prioritized)

Format as specified in your spec-analyzer instructions.
```

After the subagent completes, summarize the key findings for the user.
