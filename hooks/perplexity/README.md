# Perplexity Computer + TokenLens

Perplexity Computer runs bash in a sandbox. To use TokenLens:

1. Install the binary inside the sandbox (or rely on a system-wide install):

   ```bash
   curl -fsSL https://raw.githubusercontent.com/sisodiabhumca/tokenlens/main/scripts/install.sh | bash
   ```

2. Wrap `pplx` calls so search/fetch/snippets results are compressed before
   landing in context:

   ```bash
   tokenlens pplx search web "query"
   tokenlens pplx content fetch "https://example.com"
   ```

3. Add a Perplexity skill `tokenlens` that documents the wrapping rule and
   exposes `tokenlens gain` so agents can self-report savings.

A reference skill is in [`skill.md`](./skill.md).
