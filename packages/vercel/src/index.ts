/**
 * @tokenlens/vercel — Vercel AI SDK middleware.
 *
 * Wraps tool-call results so they go through TokenLens compression before
 * being appended to the message history.
 *
 * ```ts
 * import { tokenLens } from "@tokenlens/vercel";
 * import { anthropic } from "@ai-sdk/anthropic";
 * import { streamText } from "ai";
 *
 * await streamText({
 *   model: anthropic("claude-sonnet-4.5"),
 *   tools,
 *   experimental_wrapGenerate: tokenLens({ level: "aggressive" }),
 * });
 * ```
 */

export type CompressionLevel = "none" | "minimal" | "aggressive";

export interface TokenLensOptions {
  /** Default compression level applied to tool results. */
  level?: CompressionLevel;
  /** Optional semantic compressor (e.g. a function that calls Ollama or gpt-4o-mini). */
  semantic?: (text: string) => Promise<string>;
  /** Custom recorder — defaults to no-op. */
  record?: (event: TokenLensEvent) => void;
}

export interface TokenLensEvent {
  agent: "vercel";
  tool: string;
  inputTokens: number;
  outputTokens: number;
  savedTokens: number;
  model?: string;
}

const NEWLINES = /\n{3,}/g;
const TRAILING_WS = /[ \t]+$/gm;

/** Cheap structural compression: collapse blank lines and trailing whitespace. */
export function structuralCompress(text: string): string {
  return text.replace(TRAILING_WS, "").replace(NEWLINES, "\n\n").trim();
}

/** Approximate token count (1 token ≈ 4 chars for English). */
export function approxTokens(text: string): number {
  return Math.ceil(text.length / 4);
}

/**
 * Build an AI SDK middleware. Currently transforms tool-result text content;
 * extend to handle other content types as the SDK adds them.
 */
export function tokenLens(opts: TokenLensOptions = {}) {
  const level: CompressionLevel = opts.level ?? "minimal";

  return async function middleware<T extends { text?: string }>(input: T): Promise<T> {
    if (!input?.text) return input;
    const before = input.text;
    let after = before;

    if (level !== "none") after = structuralCompress(after);
    if (level === "aggressive" && opts.semantic) {
      try { after = await opts.semantic(after); } catch { /* keep structural result */ }
    }

    opts.record?.({
      agent: "vercel",
      tool: "unknown",
      inputTokens: approxTokens(before),
      outputTokens: approxTokens(after),
      savedTokens: Math.max(0, approxTokens(before) - approxTokens(after)),
    });

    return { ...input, text: after };
  };
}

export default tokenLens;
