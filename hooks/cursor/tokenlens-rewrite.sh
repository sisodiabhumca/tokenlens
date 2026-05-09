#!/usr/bin/env bash
# TokenLens Cursor hook. Same UHP protocol as Claude.
set -euo pipefail
exec "$(dirname "$0")/../claude/tokenlens-rewrite.sh"
