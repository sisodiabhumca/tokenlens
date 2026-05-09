# TokenLens Design Doc

## Why a successor to RTK?

[RTK](https://github.com/rtk-ai/rtk) is excellent at **shell-command compression** for Claude Code. Reading the source confirms a clean architecture: `clap` CLI → per-tool handlers in `src/cmds/` → filter strategies (`None`/`Minimal`/`Aggressive`) → SQLite analytics → thin shell hooks per agent that delegate to `rtk rewrite`. ~3,900-line registry in `src/discover/registry.rs` is the rewrite source of truth.

Gaps that motivated TokenLens:

1. **Shell-only.** Modern agent context bloat comes from file reads, MCP tool responses, web fetches, and search results — not just `git diff`.
2. **No cross-agent protocol.** Each agent has a hand-rolled shell script.
3. **No coverage** for Perplexity Computer, ChatGPT Desktop, or the Vercel AI SDK.
4. **Structural compression only.** No semantic layer for prose, search hits, or HTML.
5. **Local analytics only.** No team-level dashboard, no per-model cost view, no budget alerts.

## Architecture

### 1. `tokenlens-core` (Rust)
Forked from RTK. Keeps:
- `cmds/` — per-tool handlers (git, cargo, npm, pytest, terraform, …)
- `filters/*.toml` — declarative filter rules
- `parser::TokenFormatter` — promoted to public crate `tokenlens-format`
- SQLite tracking, but recorder is now a trait so we can swap in OTLP / cloud sinks.

Adds:
- `tokenlens mcp serve` subcommand (delegates to `tokenlens-mcp` crate)
- `tokenlens hook recv` — UHP server mode, reads JSON lines on stdin
- `tokenlens budget` — per-month spend cap and alerts
- `tokenlens gain --by model|repo|agent` — new pivots

### 2. Universal Hook Protocol (UHP)

Every supported agent integration sends a JSON line over stdin and gets a JSON line back. One protocol, N adapters.

Request:
```json
{
  "v": 1,
  "event": "tool.before",
  "agent": "claude-code",
  "tool": "bash",
  "payload": { "command": "git diff" },
  "meta": { "repo": "/Users/me/repo", "model": "claude-sonnet-4.5" }
}
```

Response:
```json
{
  "action": "rewrite",
  "payload": { "command": "tokenlens git diff" },
  "reason": "structural-rule:git.diff"
}
```

`action` ∈ `{allow, rewrite, deny, ask}` mirrors RTK's exit-code protocol but is now data instead of a side channel.

### 3. Compression engines

| Engine | When | Where |
|---|---|---|
| Structural | Known CLIs (git, cargo, …) | `tokenlens-core::filters` |
| Schema-aware | JSON / YAML / XML | `tokenlens-core::parser` with `keep_paths` |
| Diff-aware | File reads with prior cached version | `tokenlens-core::diff` (new) |
| Semantic | Big stdout, search hits, web pages | Pluggable: Ollama, gpt-4o-mini, claude-haiku |
| Retrieval cache | Repeated identical inputs | sha256 → compressed blob in SQLite |

### 4. TokenLens Cloud (optional)

Next.js app under `cloud/`. Deploys to Vercel.
- Ingest endpoint accepts batched events from `tokenlens-core`'s cloud recorder
- Dashboard pages: Savings, Repos, Agents, Models, Budget, Team
- Auth via Vercel + Postgres (Neon recommended)
- Self-host: docker-compose with Postgres + the same Next.js app

## Module ownership

| Module | Owner | Tests |
|---|---|---|
| `crates/tokenlens-core` | shared | port RTK tests |
| `crates/tokenlens-format` | shared | unit |
| `crates/tokenlens-mcp` | server eng | integration with Claude Desktop |
| `crates/tokenlens-uhp` | shared | golden JSON fixtures per agent |
| `packages/vercel` | JS eng | vitest + AI SDK eval |
| `cloud/` | full-stack | Playwright |

## Roadmap (90-day)

- **M1 (week 2)**: core builds, `tokenlens gain` works, RTK DB import, Claude + Codex hooks via UHP
- **M2 (week 4)**: MCP server, Vercel middleware, Cursor + Perplexity hooks
- **M3 (week 8)**: semantic compression (Ollama default), per-model cost map, dashboard MVP
- **M4 (week 12)**: budget alerts, self-host docker, public beta
