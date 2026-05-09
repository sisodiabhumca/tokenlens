# TokenLens — Codex CLI

**Rule:** prefix shell commands with `tokenlens` to compress their output before it enters context.

```bash
tokenlens git status
tokenlens cargo test
tokenlens npm run build
tokenlens pytest -q
```

## Meta commands

```bash
tokenlens gain                  # savings analytics
tokenlens gain --by model       # group by model
tokenlens budget --set-monthly 50
```

If a command is already wrapped, leave it alone. If it doesn't have a TokenLens equivalent (rare), run it directly.
