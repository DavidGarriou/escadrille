#!/usr/bin/env bash
# PostToolUse hook (Edit|MultiEdit|Write): format the Rust file that was just changed.
# Reads the tool call JSON on stdin. Never blocks Claude: always exits 0.
file="$(jq -r '.tool_input.file_path // empty' 2>/dev/null)"
[[ "$file" == *.rs && -f "$file" ]] || exit 0
rustfmt --edition 2024 --quiet "$file" 2>/dev/null
exit 0
