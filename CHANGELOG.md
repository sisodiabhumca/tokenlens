# Changelog

## 0.2.2 — `doctor` + observable tracker

- **New: `tokenlens doctor`.** One-shot self-diagnosis when `gain` shows
  unexpected zeros. Prints the resolved DB path and source, parent-dir
  writability, file size, current summary, and a write→read round-trip with
  a sentinel event — plus all `TOKENLENS_*` env vars currently set.
- **`TOKENLENS_DEBUG=1` stderr logging.** Every tracker-write site
  (`hook recv`, `rewrite`, `run`, `fetch`, `read`, `compress`) now reports
  failures to stderr when the env var is set, instead of silently swallowing
  them. Default behavior unchanged.
- **`hook recv` parse errors print to stderr.** Previously the JSON-encoded
  `reason` field on stdout was the only signal an input line was rejected
  — easy to miss when piping the response to `/dev/null` or `jq`. Errors now
  also print `[tokenlens] hook recv: parse error: …` on stderr.

## 0.2.1 — `gain` tracker fix

- **Fix: `tokenlens gain` no longer always reports zero.** Tracker writes were
  previously only triggered by `tokenlens run …`. The hook handler
  (`tokenlens hook recv`), the exit-code rewriter (`tokenlens rewrite`), and
  the per-tool commands (`tokenlens fetch`, `tokenlens read`,
  `tokenlens compress`) now all record events to the local SQLite tracker.
- Add `TOKENLENS_DB` env var to override the tracker file location (used by
  the e2e harness; handy for power users who want a per-project tracker).
- New e2e check pipes a rewrite through `hook recv` and asserts
  `tokenlens gain --format json` increments.
- Move `dollars_for(…)` into `tokenlens_core::tracking` so the hook handler,
  the run-proxy path, and downstream recorders share the same pricing table.

## 0.2.0 — Public beta

### Releases & install
- `cargo-dist` config publishes prebuilt binaries on every tagged release
  (Linux x86_64+aarch64, macOS x86_64+arm64, Windows x86_64).
- `.github/workflows/release.yml` builds, archives, checksums, and publishes
  to the GitHub Release.
- `scripts/install.sh` detects platform and installs to `~/.local/bin` plus
  `~/.tokenlens/hooks`.
- `Formula/tokenlens.rb` placeholder for the `sisodiabhumca/homebrew-tokenlens`
  tap (SHA256s replaced once v0.2.0 ships).

### Marketing site
- New `web/` workspace package: Next.js 14 App Router, deployed to Vercel
  at https://tokenlens-seven.vercel.app.
- Landing page with hero, six feature cards, supported-agent grid, and
  copy-pasteable install instructions.

### End-to-end harness
- `tests/e2e/run.sh` pipes UHP fixtures through `tokenlens hook recv` and
  asserts the response. Five fixtures cover Claude Code, Codex CLI, Cursor,
  and Perplexity Computer.
- Wired into the `rust` CI job after the release build.

### Repo polish
- `SECURITY.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant 2.1),
  `CONTRIBUTING.md`.
- `.github/ISSUE_TEMPLATE/{bug_report.yml,feature_request.yml,config.yml}`
  and `.github/PULL_REQUEST_TEMPLATE.md`.

## 0.2.0-pre — M1–M4 complete

### Core
- Library + binary split (`tokenlens-core` exposes a public Rust API).
- Compression engine with structural pass + 59 embedded TOML filter rules
  (ported from RTK with attribution).
- Per-tool handlers: `run`, `read`, `fetch`, `compress` (stdin or `--text`).
- Semantic compression: `OllamaBackend` + `OpenAIBackend`, sha256 disk cache.
- Recorder trait with `SqliteRecorder` and `CloudRecorder` (batched POST).
- Per-model cost map (Claude/GPT/Gemini/Llama).
- Budget config (cap, alert pct, webhook) and `tokenlens budget --check`
  with month-end projection + alert webhook.

### MCP
- `tokenlens-mcp` is a real JSON-RPC stdio server now: `compress`, `gain`,
  `lens.read`, `lens.diff` all wired to the core library.

### Vercel
- `@tokenlens/vercel` ships a real middleware:
  - structural compression
  - optional semantic pass
  - automatic ingest POST to `TOKENLENS_CLOUD_URL`
  - per-model cost calculation
  - 6 unit tests (vitest), CJS + ESM + DTS build

### Cloud
- Postgres schema, ingest endpoint with bearer-token gate.
- Budget endpoint (`GET`/`POST /api/budget`) and cron checker
  (`/api/budget/check`) with webhook fan-out.
- Mock-data fallback when `DATABASE_URL` is unset.
- Dashboard: model / agent / repo pivots.

### Self-host
- `docker-compose.yml` with healthcheck, schema auto-applied.
- `cloud/Dockerfile` (multi-stage, Node 20 alpine).
- Kubernetes manifests under `deploy/k8s/`.
- `docs/SELF_HOSTING.md`.

## 0.1.0 — Initial scaffold
- Monorepo skeleton, MIT license, RTK NOTICE, design doc, hooks, CI.
