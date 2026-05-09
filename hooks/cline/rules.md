# TokenLens — Cline / Roo Code rules

Always run shell commands through `tokenlens` to compress output before it enters context.

```bash
tokenlens git status
tokenlens pnpm install
tokenlens pytest
```

Run `tokenlens gain` to see savings.
