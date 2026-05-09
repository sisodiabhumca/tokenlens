# TokenLens

> Universal context-window optimizer and observability layer for AI coding agents.

TokenLens sits between your tools (shell, files, MCP, web fetches) and your AI agent (Claude Code, OpenAI Codex, Cursor, Perplexity Computer, ChatGPT Desktop, Vercel AI SDK / v0, Windsurf, Cline) and **compresses, filters, and reports** on every payload before it consumes context window.

It evolved from [RTK (Rust Token Killer)](https://github.com/rtk-ai/rtk), keeping the fast Rust core and the TOML filter format, and adding:

- ✅ **Universal Hook Protocol (UHP)** — one JSON line-protocol, all agents
- ✅ **MCP server** (real handlers for `compress`, `gain`, `lens.read`, `lens.diff`)
- ✅ **Vercel AI SDK middleware** — `@tokenlens/vercel`, with tests, CJS+ESM+DTS
- ✅ **Semantic compression** — Ollama default, OpenAI fallback, sha256 disk cache
- ✅ **Per-model cost map** — dollars saved by Claude/GPT/Gemini/Llama
- ✅ **Cloud dashboard** — Next.js 14, Postgres, ingest API, budget cron
- ✅ **Self-host** — `docker compose up -d` or Kubernetes manifests
- ✅ **Budget alerts** — CLI (`tokenlens budget --check`) + dashboard cron + webhooks

## Quick start

### CLI

```bash
# Build
cargo build --release --workspace

# Install hooks
target/release/tokenlens init --agents claude,codex,cursor,perplexity,vercel

# Run any tool with auto-compression and tracking
tokenlens run -- git diff
tokenlens run -- pytest -q
tokenlens fetch https://example.com

# Compress text from stdin (with optional semantic pass)
echo "$(git log)" | tokenlens compress --level aggressive --semantic

# See savings
tokenlens gain
tokenlens gain --by model,agent,repo --format json

# Budget
tokenlens budget --set-monthly 50
tokenlens budget --webhook https://hooks.slack.com/services/…
tokenlens budget --check    # exit 1 if projected > cap
```

### MCP (Claude Desktop / ChatGPT Desktop / Cursor MCP)

```jsonc
// ~/.config/claude/claude_desktop_config.json (or equivalent)
{ "mcpServers": { "tokenlens": { "command": "tokenlens-mcp" } } }
```

Tools exposed: `compress`, `gain`, `lens.read`, `lens.diff`.

### Vercel AI SDK

```ts
import { tokenLens } from "@tokenlens/vercel";
import { anthropic } from "@ai-sdk/anthropic";
import { streamText } from "ai";

await streamText({
  model: anthropic("claude-sonnet-4.5"),
  tools,
  experimental_wrapGenerate: tokenLens({
    level: "aggressive",
    model: "claude-sonnet-4.5",
    semantic: async (text, target) => /* call any LLM */ text,
  }),
});
```

Set `TOKENLENS_CLOUD_URL` (and `TOKENLENS_CLOUD_TOKEN`) and the middleware will POST events to your dashboard automatically.

### Cloud dashboard

```bash
docker compose up -d
# http://localhost:3000
```

Or deploy to Vercel — `cloud/vercel.json` already schedules `/api/budget/check` every 6 hours. See [`docs/SELF_HOSTING.md`](docs/SELF_HOSTING.md).

## Repo layout

```
tokenlens/
├─ crates/
│  ├─ tokenlens-core/     Rust lib + tokenlens binary
│  │  └─ filters/         59 TOML filter rules (RTK-derived)
│  ├─ tokenlens-format/   public formatter trait
│  ├─ tokenlens-mcp/      MCP stdio server (talks to tokenlens-core)
│  └─ tokenlens-uhp/      Universal Hook Protocol types
├─ packages/
│  ├─ vercel/             @tokenlens/vercel (built + tested)
│  └─ node/               @tokenlens/node bindings
├─ cloud/                 Next.js 14 dashboard + ingest + budget cron
│  ├─ db/schema.sql       Postgres schema
│  └─ Dockerfile
├─ deploy/k8s/            Kubernetes manifests
├─ docker-compose.yml
├─ hooks/                 per-agent thin shims
└─ docs/
```

## Status

- Rust workspace: builds (CI green); 4 crates, 5 unit tests
- @tokenlens/vercel: 6/6 tests passing
- Cloud: production build clean, 5 routes, mock-mode fallback for no-DB dev
- Self-host: docker-compose verified
- Hooks: Claude / Codex / Cursor / Perplexity / Windsurf / Cline shipped

Migration from RTK is one-shot: `tokenlens import-rtk`, then symlink `rtk → tokenlens`.

## License

MIT. Portions of `crates/tokenlens-core` derive from [RTK](https://github.com/rtk-ai/rtk) (MIT, © Patrick Szymkowiak). See [`NOTICE`](crates/tokenlens-core/NOTICE).
