---
description: Display comprehensive Spec-Kit status including constitution, specifications, plans, tasks, and implementation progress
argument-hint: ""
---

Display the current state of the Spec-Kit workflow for this project.

Check and report:
1. **Constitution**: Whether project principles are defined
2. **Specifications**: Count and list all spec files
3. **Plans**: Count and list all plan files
4. **Tasks**: Count and list all task files
5. **Implementation Progress**: Track file modifications

Read the following to assess status:
- `.specify/memory/constitution.md` - Check if exists
- `.specify/memory/spec-*.md` - Count and list
- `.specify/memory/plan-*.md` - Count and list
- `.specify/memory/tasks-*.md` - Count and list
- `.specify/memory/.implementation-progress` - Check tracked changes

Format the output clearly with:
- ✅ for completed items
- ❌ for missing items
- 📊 for counts and lists
- 💡 for suggested next steps

Based on what exists, suggest the appropriate next step in the workflow:
- No constitution → `/speckit.constitution`
- No specs → `/speckit.specify`
- No plans → `/speckit.plan`
- No tasks → `/speckit.tasks`
- All exist → `/speckit.implement` or `/speckit.analyze`

Keep output concise but informative.
