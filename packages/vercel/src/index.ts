/**
 * @tokenlens/vercel — Vercel AI SDK middleware.
 *
 * Compresses tool-call results and large text blobs before they're appended
 * to the message history. Optional semantic backend can call any LLM.
 */

export type CompressionLevel = "none" | "minimal" | "aggressive";

export interface TokenLensEvent {
  agent: "vercel";
  cmd: string;
  inputTokens: number;
  outputTokens: number;
  savedTokens: number;
  dollarsSaved?: number;
  model?: string;
}

export interface TokenLensOptions {
  level?: CompressionLevel;
  /** Optional semantic compressor — invoked after structural compression. */
  semantic?: (text: string, targetTokens: number) => Promise<string>;
  /** Per-event recorder. Default: POST to TOKENLENS_CLOUD_URL if set. */
  record?: (event: TokenLensEvent) => void | Promise<void>;
  /** Used only for cost calculation when `record` is the default. */
  model?: string;
}

const NEWLINES = /\n{3,}/g;
const TRAILING_WS = /[ \t]+$/gm;

export function structuralCompress(text: string): string {
  return text.replace(TRAILING_WS, "").replace(NEWLINES, "\n\n").trim();
}

export function approxTokens(text: string): number {
  return Math.ceil(text.length / 4);
}

const PRICE_PER_M: Record<string, number> = {
  "claude-opus": 15.0, "claude-sonnet": 3.0, "claude-haiku": 0.8,
  "gpt-4o": 5.0, "gpt-4o-mini": 0.15, "gpt-5": 8.0,
  "gemini-2.5-pro": 3.5, "gemini": 1.25,
};

function dollarsFor(savedTokens: number, model?: string): number {
  if (!model) return (savedTokens / 1_000_000) * 2.0;
  const key = Object.keys(PRICE_PER_M).find((k) => model.includes(k));
  return (savedTokens / 1_000_000) * (key ? PRICE_PER_M[key] : 2.0);
}

async function defaultRecorder(event: TokenLensEvent): Promise<void> {
  const url = process.env.TOKENLENS_CLOUD_URL;
  if (!url) return;
  const token = process.env.TOKENLENS_CLOUD_TOKEN;
  try {
    await fetch(url, {
      method: "POST",
      headers: {
        "content-type": "application/json",
        ...(token ? { authorization: `Bearer ${token}` } : {}),
      },
      body: JSON.stringify({
        events: [{
          ts: Math.floor(Date.now() / 1000),
          cmd: event.cmd,
          input_tokens: event.inputTokens,
          output_tokens: event.outputTokens,
          saved_tokens: event.savedTokens,
          dollars_saved: event.dollarsSaved ?? 0,
          agent: event.agent,
          model: event.model ?? null,
        }],
      }),
    });
  } catch { /* don't break the app on telemetry errors */ }
}

export function tokenLens(opts: TokenLensOptions = {}) {
  const level: CompressionLevel = opts.level ?? "minimal";
  const recorder = opts.record ?? defaultRecorder;

  /** Generic transformer for any object that has a `text` field. */
  return async function middleware<T extends { text?: string }>(input: T): Promise<T> {
    if (!input?.text) return input;
    const before = input.text;
    let after = before;
    if (level !== "none") after = structuralCompress(after);
    if (level === "aggressive" && opts.semantic) {
      const target = Math.max(200, Math.floor(approxTokens(after) / 2));
      try { after = await opts.semantic(after, target); } catch { /* keep structural result */ }
    }
    const inTok = approxTokens(before);
    const outTok = approxTokens(after);
    const saved = Math.max(0, inTok - outTok);
    await recorder({
      agent: "vercel",
      cmd: "ai-sdk:tool-result",
      inputTokens: inTok,
      outputTokens: outTok,
      savedTokens: saved,
      dollarsSaved: dollarsFor(saved, opts.model),
      model: opts.model,
    });
    return { ...input, text: after };
  };
}

export default tokenLens;
