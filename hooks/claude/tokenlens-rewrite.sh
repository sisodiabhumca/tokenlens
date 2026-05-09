#!/usr/bin/env bash
# TokenLens Claude Code hook (PreToolUse).
# Reads Claude's JSON envelope on stdin, asks `tokenlens rewrite` for an
# RTK-style answer, and emits the rewritten command for Claude to run.
#
# Exit code protocol (mirrors RTK so existing setups work):
#   0 + stdout  rewrite found -> auto-allow
#   1           no equivalent -> pass through
#   2           deny rule
#   3 + stdout  rewrite + ask
set -euo pipefail

if ! command -v jq >/dev/null; then echo "[tokenlens] jq required" >&2; exit 0; fi
if ! command -v tokenlens >/dev/null; then echo "[tokenlens] tokenlens not in PATH" >&2; exit 0; fi

INPUT=$(cat)
CMD=$(jq -r '.tool_input.command // empty' <<<"$INPUT")
[ -z "$CMD" ] && exit 0

REWRITTEN=$(tokenlens rewrite "$CMD" 2>/dev/null) || EXIT_CODE=$?
EXIT_CODE=${EXIT_CODE:-$?}

case "${EXIT_CODE:-0}" in
  0) [ "$CMD" = "$REWRITTEN" ] && exit 0; printf '%s' "$REWRITTEN"; exit 0 ;;
  1) exit 0 ;;
  2) exit 0 ;;
  3) printf '%s' "$REWRITTEN"; exit 3 ;;
  *) exit 0 ;;
esac
