#!/usr/bin/env bash
# Display current spec-kit status

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

echo "📊 Spec-Kit Status"
echo "=================="
echo ""

# Constitution
if [ -f "$SPECIFY_DIR/memory/constitution.md" ]; then
    echo "✅ Constitution: Defined"
else
    echo "❌ Constitution: Missing (run /speckit.constitution)"
fi

# Specifications
SPEC_COUNT=$(find "$SPECIFY_DIR/memory" -name "spec-*.md" 2>/dev/null | wc -l | tr -d ' ')
if [ "$SPEC_COUNT" -gt 0 ]; then
    echo "✅ Specifications: $SPEC_COUNT found"
    find "$SPECIFY_DIR/memory" -name "spec-*.md" -exec basename {} \; | sed 's/^/   - /'
else
    echo "❌ Specifications: None (run /speckit.specify)"
fi

echo ""

# Plans
PLAN_COUNT=$(find "$SPECIFY_DIR/memory" -name "plan-*.md" 2>/dev/null | wc -l | tr -d ' ')
if [ "$PLAN_COUNT" -gt 0 ]; then
    echo "✅ Plans: $PLAN_COUNT found"
    find "$SPECIFY_DIR/memory" -name "plan-*.md" -exec basename {} \; | sed 's/^/   - /'
else
    echo "❌ Plans: None (run /speckit.plan)"
fi

echo ""

# Tasks
TASK_COUNT=$(find "$SPECIFY_DIR/memory" -name "tasks-*.md" 2>/dev/null | wc -l | tr -d ' ')
if [ "$TASK_COUNT" -gt 0 ]; then
    echo "✅ Tasks: $TASK_COUNT found"
    find "$SPECIFY_DIR/memory" -name "tasks-*.md" -exec basename {} \; | sed 's/^/   - /'
else
    echo "❌ Tasks: None (run /speckit.tasks)"
fi

echo ""

# Implementation progress
PROGRESS_FILE="$SPECIFY_DIR/memory/.implementation-progress"
if [ -f "$PROGRESS_FILE" ]; then
    CHANGE_COUNT=$(wc -l < "$PROGRESS_FILE" | tr -d ' ')
    echo "🔧 Implementation: $CHANGE_COUNT file modifications tracked"
    echo "   Recent:"
    tail -3 "$PROGRESS_FILE" | sed 's/^/   /'
else
    echo "🔧 Implementation: No tracked progress"
fi

echo ""
echo "Next Steps:"
if [ ! -f "$SPECIFY_DIR/memory/constitution.md" ]; then
    echo "  1. Run /speckit.constitution to establish project principles"
elif [ "$SPEC_COUNT" -eq 0 ]; then
    echo "  1. Run /speckit.specify to document requirements"
elif [ "$PLAN_COUNT" -eq 0 ]; then
    echo "  1. Run /speckit.plan to create technical plans"
elif [ "$TASK_COUNT" -eq 0 ]; then
    echo "  1. Run /speckit.tasks to generate actionable tasks"
else
    echo "  1. Run /speckit.implement to execute tasks"
    echo "  2. Run /speckit.analyze to check consistency"
fi
