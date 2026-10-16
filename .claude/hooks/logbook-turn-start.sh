#!/usr/bin/env bash
# UserPromptSubmit hook: record when the player's turn started, so the Stop hook
# can check that docs/carnet-de-bord.md was updated during this turn.
# Never blocks Claude: always exits 0.
state_dir="${CLAUDE_PROJECT_DIR:-.}/.claude/.state"
mkdir -p "$state_dir" 2>/dev/null && touch "$state_dir/turn-start" 2>/dev/null
exit 0
