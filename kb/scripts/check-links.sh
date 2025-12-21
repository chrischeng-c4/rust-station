#!/bin/bash
# Check for broken internal links in KB

set -euo pipefail

KB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REDIRECT_MAP="$KB_DIR/.redirects.json"

echo "Checking KB links in: $KB_DIR"
echo

# Find all markdown files
find "$KB_DIR" -name "*.md" -type f | while read -r file; do
  # Extract markdown links: [text](path.md)
  grep -oP '\[.*?\]\(\K[^)]+\.md[^)]*' "$file" 2>/dev/null || true | while read -r link; do
    # Skip external links
    if [[ "$link" =~ ^https?:// ]]; then
      continue
    fi

    # Resolve relative path
    link_dir=$(dirname "$file")
    target=$(python3 -c "import os; print(os.path.normpath(os.path.join('$link_dir', '$link')))" 2>/dev/null || echo "$link")

    # Check if target exists
    if [[ ! -f "$target" ]]; then
      # Check redirect map
      relative_link=${link#$KB_DIR/}
      if [[ -f "$REDIRECT_MAP" ]] && jq -e ".\"$relative_link\"" "$REDIRECT_MAP" >/dev/null 2>&1; then
        new_path=$(jq -r ".\"$relative_link\"" "$REDIRECT_MAP")
        echo "REDIRECT: $file → $link → kb/$new_path"
      else
        echo "BROKEN: $file → $link (NOT FOUND)"
      fi
    fi
  done
done

echo
echo "Link check complete."
