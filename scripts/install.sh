#!/usr/bin/env bash
# TokenLens installer (placeholder).
# Real installer should detect platform, fetch the right release tarball,
# install to /usr/local/bin/tokenlens, then run `tokenlens init`.
set -euo pipefail
echo "[tokenlens] installer placeholder. Build from source for now:"
echo "  cargo build --release -p tokenlens-core"
echo "  ln -s \"$(pwd)/target/release/tokenlens\" /usr/local/bin/tokenlens"
echo "  tokenlens init --agents claude,codex,cursor,perplexity"
