#!/bin/bash
INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

[ -z "$FILE_PATH" ] || [ "$FILE_PATH" = "null" ] && exit 0

cd "$CLAUDE_PROJECT_DIR" || exit 0
command -v qlty &>/dev/null || exit 0

# Auto-format (never fails)
qlty fmt "$FILE_PATH" 2>/dev/null

# Lint check
OUTPUT=$(qlty check "$FILE_PATH" --level=medium 2>&1)
EXIT_CODE=$?

if [ $EXIT_CODE -ne 0 ]; then
  echo "$OUTPUT" >&2
  exit 2
fi

exit 0
