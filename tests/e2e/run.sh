#!/usr/bin/env bash
# TokenLens end-to-end harness.
#
# For each fixture pair (NAME.in.json, NAME.expect.json) under fixtures/:
#   1. pipe NAME.in.json into `tokenlens hook recv`
#   2. parse stdout as JSON
#   3. assert every key in NAME.expect.json matches
#
# Exits non-zero if any fixture fails.
set -uo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
FIX="$HERE/fixtures"
BIN="${TOKENLENS_BIN:-}"

if [ -z "$BIN" ]; then
  if [ -x "$HERE/../../target/release/tokenlens" ]; then
    BIN="$HERE/../../target/release/tokenlens"
  elif [ -x "$HERE/../../target/debug/tokenlens" ]; then
    BIN="$HERE/../../target/debug/tokenlens"
  else
    echo "tokenlens binary not found; build it first or set TOKENLENS_BIN" >&2
    exit 2
  fi
fi

if ! command -v jq >/dev/null 2>&1; then
  echo "jq is required for the e2e harness" >&2
  exit 2
fi

pass=0
fail=0
failed_names=()

shopt -s nullglob
for inp in "$FIX"/*.in.json; do
  name="$(basename "$inp" .in.json)"
  expect="$FIX/$name.expect.json"
  if [ ! -f "$expect" ]; then
    echo "[skip] $name (no .expect.json)"
    continue
  fi

  actual_out="$(cat "$inp" | "$BIN" hook recv 2>/dev/null || true)"
  if [ -z "$actual_out" ]; then
    echo "[FAIL] $name — empty response from tokenlens hook recv"
    fail=$((fail+1))
    failed_names+=("$name")
    continue
  fi

  ok=1
  while IFS= read -r key; do
    expected_val="$(jq -c --arg k "$key" '.[$k]' "$expect")"
    actual_val="$(echo "$actual_out" | jq -c --arg k "$key" '.[$k]')"
    if [ "$expected_val" != "$actual_val" ]; then
      echo "[FAIL] $name — key=$key expected=$expected_val actual=$actual_val"
      ok=0
    fi
  done < <(jq -r 'keys[]' "$expect")

  if [ "$ok" -eq 1 ]; then
    echo "[pass] $name"
    pass=$((pass+1))
  else
    fail=$((fail+1))
    failed_names+=("$name")
  fi
done

echo
echo "e2e: $pass passed, $fail failed"
if [ "$fail" -gt 0 ]; then
  printf '  failed: %s\n' "${failed_names[@]}"
  exit 1
fi

# ---------- gain-pipeline check ----------
# Pipe a known-rewritable command through hook recv, then assert the tracker
# DB has at least one row and `tokenlens gain --format json` reports it.
echo
echo "e2e: verifying gain pipeline"
TMP_DB="$(mktemp -t tokenlens.gain.XXXXXX.db)"
rm -f "$TMP_DB"
export TOKENLENS_DB="$TMP_DB"
export TOKENLENS_AGENT="e2e"

gain_before_json="$("$BIN" gain --format json 2>/dev/null || echo '{}')"
gain_before="$(echo "$gain_before_json" | jq -r '.commands // 0')"

rewrite_in='{"tool":"bash","payload":{"command":"git log --oneline -5"}}'
echo "$rewrite_in" | "$BIN" hook recv >/dev/null 2>&1 || true

gain_after_json="$("$BIN" gain --format json 2>/dev/null)"
gain_after="$(echo "$gain_after_json" | jq -r '.commands // 0')"

if [ "$gain_after" -gt "$gain_before" ]; then
  echo "[pass] gain pipeline: commands $gain_before -> $gain_after"
else
  echo "[FAIL] gain pipeline: commands did not increment ($gain_before -> $gain_after)"
  echo "       json: $gain_after_json"
  rm -f "$TMP_DB"
  exit 1
fi

rm -f "$TMP_DB"
unset TOKENLENS_DB TOKENLENS_AGENT
