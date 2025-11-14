---
description: Quick spec-kit status check with minimal output
argument-hint: ""
---

Provide a quick one-line status of the Spec-Kit workflow.

Check:
- `.specify/memory/constitution.md` - exists?
- `.specify/memory/spec-*.md` - count
- `.specify/memory/plan-*.md` - count
- `.specify/memory/tasks-*.md` - count

Output format (single line):
```
Spec-Kit: Constitution [✅/❌] | Specs: N | Plans: N | Tasks: N | Next: [command]
```

Example outputs:
```
Spec-Kit: Constitution ✅ | Specs: 3 | Plans: 2 | Tasks: 5 | Next: /speckit.implement
Spec-Kit: Constitution ❌ | Specs: 0 | Plans: 0 | Tasks: 0 | Next: /speckit.constitution
```

This is for quick status checks without detailed output.
