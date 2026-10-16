#!/usr/bin/env bash
# Stop hook: refuse to end the turn until docs/carnet-de-bord.md has been
# updated during this turn, committed and pushed (rule in CLAUDE.md).
# Blocks at most once per turn (stop_hook_active) to avoid an endless loop.
input="$(cat)"
[[ "$(jq -r '.stop_hook_active // false' <<<"$input" 2>/dev/null)" == "true" ]] && exit 0

root="${CLAUDE_PROJECT_DIR:-.}"
log="docs/carnet-de-bord.md"
marker="$root/.claude/.state/turn-start"
cd "$root" 2>/dev/null || exit 0
[[ -f "$marker" ]] || exit 0

block() {
  jq -n --arg r "$1" '{decision: "block", reason: $r}'
  exit 0
}

if [[ ! -f "$log" ]]; then
  block "Carnet de bord : $log est introuvable. Recrée-le (voir CLAUDE.md, « Carnet de bord »)."
fi
if [[ ! "$log" -nt "$marker" ]]; then
  block "Carnet de bord : $log n'a pas été mis à jour pendant cet échange. Consigne la question du joueur, ta réponse, les actions et les décisions (format Q/R/A/D/⚠), puis commite et pousse."
fi
if [[ -n "$(git status --porcelain -- "$log" 2>/dev/null)" ]]; then
  block "Carnet de bord : $log contient des modifications non commitées. Commite et pousse-les."
fi
if git rev-parse --abbrev-ref '@{u}' >/dev/null 2>&1 &&
  [[ -n "$(git log --oneline '@{u}..HEAD' -- "$log" 2>/dev/null)" ]]; then
  block "Carnet de bord : le commit du carnet n'est pas poussé. Pousse la branche."
fi
exit 0
