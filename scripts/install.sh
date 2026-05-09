#!/usr/bin/env bash
# TokenLens installer.
# Detects platform, downloads the matching prebuilt archive from the latest
# GitHub Release, installs the `tokenlens` binary into $TOKENLENS_BIN
# (default: ~/.local/bin), and copies hook scripts to $TOKENLENS_HOOKS
# (default: ~/.tokenlens/hooks).
#
# Usage:
#   curl -fsSL https://raw.githubusercontent.com/sisodiabhumca/tokenlens/main/scripts/install.sh | bash
#   TOKENLENS_VERSION=v0.2.0 bash scripts/install.sh
set -euo pipefail

REPO="${TOKENLENS_REPO:-sisodiabhumca/tokenlens}"
VERSION="${TOKENLENS_VERSION:-latest}"
BIN_DIR="${TOKENLENS_BIN:-$HOME/.local/bin}"
HOOK_DIR="${TOKENLENS_HOOKS:-$HOME/.tokenlens/hooks}"

uname_s="$(uname -s)"
uname_m="$(uname -m)"

case "$uname_s-$uname_m" in
  Linux-x86_64)   target="x86_64-unknown-linux-gnu" ;;
  Linux-aarch64)  target="aarch64-unknown-linux-gnu" ;;
  Linux-arm64)    target="aarch64-unknown-linux-gnu" ;;
  Darwin-x86_64)  target="x86_64-apple-darwin" ;;
  Darwin-arm64)   target="aarch64-apple-darwin" ;;
  *)
    echo "tokenlens: unsupported platform $uname_s-$uname_m" >&2
    echo "  Supported: Linux x86_64/aarch64, macOS x86_64/arm64." >&2
    echo "  Build from source: cargo install --git https://github.com/$REPO tokenlens-core" >&2
    exit 1
    ;;
esac

archive="tokenlens-$target.tar.gz"
if [ "$VERSION" = "latest" ]; then
  url="https://github.com/$REPO/releases/latest/download/$archive"
else
  url="https://github.com/$REPO/releases/download/$VERSION/$archive"
fi

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

echo "[tokenlens] downloading $url"
if ! curl -fsSL "$url" -o "$tmp/$archive"; then
  echo "tokenlens: download failed. Check that release $VERSION exists for $target." >&2
  exit 1
fi

tar -xzf "$tmp/$archive" -C "$tmp"
extracted="$tmp/tokenlens-$target"

mkdir -p "$BIN_DIR" "$HOOK_DIR"
install -m 0755 "$extracted/tokenlens" "$BIN_DIR/tokenlens"
if [ -d "$extracted/hooks" ]; then
  cp -R "$extracted/hooks/." "$HOOK_DIR/"
fi

echo "[tokenlens] installed: $BIN_DIR/tokenlens"
echo "[tokenlens] hooks:     $HOOK_DIR"
case ":$PATH:" in
  *":$BIN_DIR:"*) ;;
  *) echo "[tokenlens] note: add $BIN_DIR to PATH (e.g. export PATH=\"$BIN_DIR:\$PATH\")" ;;
esac
echo "[tokenlens] next: tokenlens init --agents claude,codex,cursor,perplexity"
