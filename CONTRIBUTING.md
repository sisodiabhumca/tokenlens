# Contributing to TokenLens

Thanks for your interest. The project is in early scaffold phase — most of the value will come from porting RTK's filter registry and adding the semantic-compression engines.

## Areas that need help

- Port RTK's `src/cmds/` per-tool handlers into `crates/tokenlens-core/src/cmds/` with attribution preserved.
- Build out `crates/tokenlens-mcp` (currently stub).
- Add Vercel AI SDK middleware tests against real models.
- Build the dashboard's Postgres schema and ingest pipeline.
- Write golden-fixture tests for each agent hook.

## Dev loop

```bash
# Rust
cargo build --workspace
cargo test --workspace

# JS
pnpm install
pnpm -r build
```

## License

By contributing you agree to release your contribution under the MIT License.
