# TokenLens end-to-end harness

Simulates real agent traffic (Claude Code, Codex CLI, Cursor, Perplexity Computer)
against the `tokenlens hook recv` UHP listener and asserts that:

1. Noisy commands (`cargo build`, `git status`, `npm test`, etc.) come back with
   `action: "rewrite"`.
2. Quiet commands (`echo hi`, `cat README.md`) come back with `action: "allow"`.
3. The exit code from `tokenlens hook recv` matches the documented contract:
   `0=rewrite, 1=passthrough, 2=deny, 3=ask`.

## Run locally

```bash
# build first (CI does this in the rust job)
cargo build --release -p tokenlens-core --bin tokenlens

# then run the harness
TOKENLENS_BIN=./target/release/tokenlens bash tests/e2e/run.sh
```

## Run in CI

The harness is invoked by the `e2e` job in `.github/workflows/ci.yml` after
the rust build completes.

## Fixtures

Each fixture is a pair of files:

| input              | description                                  |
| ------------------ | -------------------------------------------- |
| `*.in.json`        | One UHP `HookRequest` JSON line.             |
| `*.expect.json`    | Asserted fields of the `HookResponse`.       |

The runner only checks the fields present in `*.expect.json`; extra fields
(e.g. `payload.command`, `reason`) are ignored unless specified.
