# Changelog

## 0.2.0 — M1–M4 complete

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
