#!/usr/bin/env bash
# PostToolUse hook: Update spec-kit memory after code changes

set -euo pipefail

PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../" && pwd)"
SPECIFY_DIR="$PROJECT_ROOT/.specify"

# Get the file that was edited/written
FILE_PATH="${CLAUDE_TOOL_INPUT_file_path:-}"

# Only track Rust source files in rush project
if [[ "$FILE_PATH" =~ crates/rush/.*\.rs$ ]]; then
    # Track that rush implementation is progressing
    PROGRESS_FILE="$SPECIFY_DIR/memory/.implementation-progress"

    # Create or update progress tracking
    mkdir -p "$(dirname "$PROGRESS_FILE")"
    echo "$(date -u +"%Y-%m-%dT%H:%M:%SZ") - Modified: $FILE_PATH" >> "$PROGRESS_FILE"
fi

exit 0
