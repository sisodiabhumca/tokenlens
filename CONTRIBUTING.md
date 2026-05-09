# Contributing to TokenLens

Thanks for taking the time to contribute. TokenLens is in public beta — bug
reports, filter rules, agent adapters, and docs are all welcome.

## Ground rules

- By participating, you agree to follow our [Code of Conduct](./CODE_OF_CONDUCT.md).
- Security issues: see [SECURITY.md](./SECURITY.md). Please do **not** open
  public issues for security reports.
- File [issues](https://github.com/sisodiabhumca/tokenlens/issues)
  using the templates in `.github/ISSUE_TEMPLATE/`.

## Repo layout

```
crates/         # Rust workspace: tokenlens-core (CLI), tokenlens-format,
                # tokenlens-mcp, tokenlens-uhp
packages/       # JS workspace: @tokenlens/vercel, @tokenlens/node
cloud/          # Next.js dashboard + ingest API
web/            # marketing site (tokenlens-seven.vercel.app)
hooks/          # per-agent shell hooks (claude, codex, cursor, perplexity, …)
crates/tokenlens-core/filters/  # 59 RTK-derived TOML filter rules
tests/e2e/      # UHP simulation harness
```

## Development setup

```bash
# Rust
cargo build --workspace
cargo test  --workspace

# JS
pnpm install
pnpm --filter @tokenlens/vercel test
pnpm --filter ./cloud build
pnpm --filter ./web build

# end-to-end
TOKENLENS_BIN=./target/debug/tokenlens bash tests/e2e/run.sh
```

## Adding a filter rule

1. Drop a TOML file into `crates/tokenlens-core/filters/`. The CI filter job
   parses every file and compiles the regexes.
2. Run `bash scripts/regenerate-embedded.sh` to refresh
   `crates/tokenlens-core/src/registry/embedded.rs`.
3. Commit both files together.

## Adding a new agent hook

1. Create `hooks/<agent>/hook.sh` that reads UHP JSON on stdin and execs
   `tokenlens hook recv`.
2. Add a fixture pair under `tests/e2e/fixtures/<agent>-*.{in,expect}.json`.
3. Document the wiring step in `README.md` under "Wire up an agent".

## Pull requests

- Keep PRs focused; split unrelated changes.
- Fill out the PR template — it asks for what changed, how to test, and
  whether docs/CHANGELOG were updated.
- CI must be green before merge: `rust`, `js`, `filters`, `e2e`.
- Update `CHANGELOG.md` for user-visible changes.
