---
name: tokenlens
description: Compress shell, search, and fetch outputs in Perplexity Computer to save context window. Load when running long-output commands like git diff, search, or fetch.
---

# TokenLens for Perplexity Computer

Always wrap output-heavy commands with `tokenlens`:

```bash
tokenlens git diff
tokenlens pplx search web "query"
tokenlens pplx content fetch "https://example.com"
tokenlens pytest -q
```

Inspect savings:

```bash
tokenlens gain
tokenlens gain --by model --by repo
```

If a command isn't supported, TokenLens falls through cleanly.
