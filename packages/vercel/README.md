# @tokenlens/vercel

Vercel AI SDK middleware for [TokenLens](../../README.md). Compresses tool results before they enter context.

```ts
import { tokenLens } from "@tokenlens/vercel";
import { anthropic } from "@ai-sdk/anthropic";
import { streamText } from "ai";

const result = await streamText({
  model: anthropic("claude-sonnet-4.5"),
  tools,
  experimental_wrapGenerate: tokenLens({ level: "aggressive" }),
});
```

Options:

- `level`: `"none" | "minimal" | "aggressive"` (default `"minimal"`)
- `semantic`: optional `(text) => Promise<string>` for LLM-based summarization
- `record`: optional callback for analytics ingest

Pairs with `tokenlens gain` and the TokenLens Cloud dashboard.
