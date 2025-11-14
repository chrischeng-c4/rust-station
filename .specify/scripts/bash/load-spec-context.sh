#!/usr/bin/env bash
# SessionStart hook: Load spec-kit context at session start

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

# Check if constitution exists
if [ -f "$SPECIFY_DIR/memory/constitution.md" ]; then
    echo "📋 Spec-Kit Active"
    echo "Constitution: Loaded"

    # Count specifications
    SPEC_COUNT=$(find "$SPECIFY_DIR/memory" -name "spec-*.md" 2>/dev/null | wc -l | tr -d ' ')
    echo "Specifications: $SPEC_COUNT"

    # Count plans
    PLAN_COUNT=$(find "$SPECIFY_DIR/memory" -name "plan-*.md" 2>/dev/null | wc -l | tr -d ' ')
    echo "Plans: $PLAN_COUNT"

    # Count tasks
    TASK_COUNT=$(find "$SPECIFY_DIR/memory" -name "tasks-*.md" 2>/dev/null | wc -l | tr -d ' ')
    echo "Tasks: $TASK_COUNT"

    echo ""
    echo "Use spec-driven workflow:"
    echo "  /speckit.constitution - Establish principles"
    echo "  /speckit.specify - Create specifications"
    echo "  /speckit.plan - Plan implementation"
    echo "  /speckit.tasks - Generate tasks"
    echo "  /speckit.implement - Execute tasks"
else
    echo "⚠️  No constitution found"
    echo "Start with: /speckit.constitution"
fi

exit 0
