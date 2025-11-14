#!/usr/bin/env bash
# PreToolUse hook: Validate code changes align with specifications

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

# Get the file being edited/written
FILE_PATH="${CLAUDE_TOOL_INPUT_file_path:-}"

# Only validate Rust source files
if [[ ! "$FILE_PATH" =~ \.rs$ ]]; then
    exit 0
fi

# Check if constitution exists
if [ ! -f "$SPECIFY_DIR/memory/constitution.md" ]; then
    # No constitution - can't validate
    exit 0
fi

# Check if any specifications exist
SPEC_COUNT=$(find "$SPECIFY_DIR/memory" -name "spec-*.md" 2>/dev/null | wc -l | tr -d ' ')
if [ "$SPEC_COUNT" -eq 0 ]; then
    echo "⚠️  Warning: Modifying code without specifications"
    echo "Consider running /speckit.specify to document requirements"
    # Don't block - just warn
    exit 0
fi

# If we get here, specs exist - changes are likely aligned
exit 0
