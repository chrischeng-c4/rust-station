#!/usr/bin/env bash
# Stop hook: Check if work is properly documented in specs

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

# Check if implementation progress was tracked
PROGRESS_FILE="$SPECIFY_DIR/memory/.implementation-progress"

if [ -f "$PROGRESS_FILE" ]; then
    # Work was done - remind about spec-kit workflow
    SPEC_COUNT=$(find "$SPECIFY_DIR/memory" -name "spec-*.md" 2>/dev/null | wc -l | tr -d ' ')

    if [ "$SPEC_COUNT" -eq 0 ]; then
        echo "💡 Reminder: Document your work with /speckit.specify"
    else
        echo "💡 Consider running /speckit.analyze to verify spec alignment"
    fi
fi

exit 0
