# TokenLens

> Universal context-window optimizer and observability layer for AI coding agents.

TokenLens sits between your tools (shell, files, MCP servers, web fetches) and your AI agent (Claude Code, Codex, Cursor, Perplexity Computer, ChatGPT, Vercel AI SDK, Windsurf, Cline) and **compresses, filters, and reports** on every payload before it consumes context window.

It's a successor to [RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk) — keeping its fast Rust core and TOML filter registry, and adding:

- **Universal Hook Protocol (UHP)** — one JSON line-protocol replaces N per-agent shell scripts
- **MCP server** — drop-in for Claude Desktop, ChatGPT Desktop, Cursor MCP
- **Vercel AI SDK middleware** — `@tokenlens/vercel` wraps `streamText` / `generateText`
- **Semantic compression** — local LLMs or hosted summarizers (not just regex rules)
- **Per-model cost mapping** — dollars saved, not just tokens
- **Team dashboard** (Next.js, Vercel-deployable) with budget alerts

## Status

🚧 **Early scaffold** — the Rust core is RTK-derived and builds; the JS packages and dashboard are stubs that compile and run but need the full integration layer.

## Repo layout

```
tokenlens/
├─ crates/
│  ├─ tokenlens-core/    # Rust binary, RTK-derived
│  ├─ tokenlens-format/  # public formatter crate
│  ├─ tokenlens-mcp/     # MCP server
│  └─ tokenlens-uhp/     # Universal Hook Protocol
├─ packages/
│  ├─ vercel/            # @tokenlens/vercel — AI SDK middleware
│  └─ node/              # @tokenlens/node — bindings to the Rust core
├─ cloud/                # Next.js dashboard (deploy to Vercel)
├─ hooks/                # per-agent thin shims (claude, codex, cursor, perplexity, …)
└─ docs/
```

## Supported agents

| Agent | Mechanism | Status |
|---|---|---|
| Claude Code | UHP shell hook | ✅ stub |
| OpenAI Codex CLI | UHP shell hook | ✅ stub |
| Cursor (CLI + editor) | UHP shell hook | ✅ stub |
| Perplexity Computer | `pplx`-aware bash wrapper + skill | ✅ stub |
| ChatGPT Desktop | MCP server | ✅ stub |
| Claude Desktop | MCP server | ✅ stub |
| Vercel AI SDK / v0 | npm middleware | ✅ stub |
| Windsurf / Cline / Kilocode / Antigravity | UHP shell hook | ✅ stub |

## Quick start

```bash
# Build the core
cargo build --release -p tokenlens-core

# Install hooks for the agents you use
./target/release/tokenlens init --agents claude,codex,cursor,perplexity

# See savings
tokenlens gain
tokenlens gain --by model --by repo

# Run the MCP server (for ChatGPT/Claude Desktop)
tokenlens mcp serve

# Dev the dashboard
pnpm install
pnpm --filter ./cloud dev
```

## Migration from RTK

```bash
tokenlens import-rtk           # imports ~/.local/share/rtk/tracking.db
ln -s "$(which tokenlens)" /usr/local/bin/rtk   # existing hooks keep working
```

All RTK TOML filters in `src/filters/*.toml` load unchanged.

## License

MIT. Portions of `crates/tokenlens-core` derive from [RTK](https://github.com/rtk-ai/rtk) (MIT, © Patrick Szymkowiak). See [`NOTICE`](crates/tokenlens-core/NOTICE).
