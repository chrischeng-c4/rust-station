#!/usr/bin/env bash
# UserPromptSubmit hook: Inject spec context when implementing features

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

# Check if user prompt contains implementation keywords
USER_PROMPT="${CLAUDE_USER_PROMPT:-}"

if [[ "$USER_PROMPT" =~ (implement|build|create|add|write|develop|code) ]]; then
    # Check if constitution exists
    if [ ! -f "$SPECIFY_DIR/memory/constitution.md" ]; then
        echo "⚠️  No constitution - consider running /speckit.constitution first"
        exit 0
    fi

    # Check if any specifications exist
    SPEC_COUNT=$(find "$SPECIFY_DIR/memory" -name "spec-*.md" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$SPEC_COUNT" -eq 0 ]; then
        echo "⚠️  No specifications found - consider running /speckit.specify before implementing"
        exit 0
    fi

    # Check if plans exist
    PLAN_COUNT=$(find "$SPECIFY_DIR/memory" -name "plan-*.md" 2>/dev/null | wc -l | tr -d ' ')
    if [ "$PLAN_COUNT" -eq 0 ]; then
        echo "⚠️  No implementation plans - consider running /speckit.plan before implementing"
        exit 0
    fi

    # All good - spec-driven workflow is being followed
    echo "✅ Spec-driven context available"
fi

exit 0
