/**
 * Postgres client. Uses DATABASE_URL (Neon, Supabase, or local).
 * Falls back to in-memory mock data when DATABASE_URL is unset, so the
 * dashboard renders during local dev without a database.
 */
import { Pool } from "pg";

let pool: Pool | null = null;

export function getPool(): Pool | null {
  if (!process.env.DATABASE_URL) return null;
  if (!pool) {
    pool = new Pool({
      connectionString: process.env.DATABASE_URL,
      ssl: process.env.DATABASE_SSL === "false" ? false : { rejectUnauthorized: false },
    });
  }
  return pool;
}

export interface Summary {
  commands: number;
  inputTokens: number;
  outputTokens: number;
  savedTokens: number;
  dollarsSaved: number;
  byModel: { name: string; saved: number }[];
  byAgent: { name: string; saved: number }[];
  byRepo: { name: string; saved: number }[];
}

const MOCK: Summary = {
  commands: 12480,
  inputTokens: 48_200_000,
  outputTokens: 5_900_000,
  savedTokens: 42_300_000,
  dollarsSaved: 184.32,
  byModel: [
    { name: "claude-sonnet-4.5", saved: 22_400_000 },
    { name: "gpt-5", saved: 11_900_000 },
    { name: "gemini-2.5-pro", saved: 5_300_000 },
  ],
  byAgent: [
    { name: "claude-code", saved: 19_800_000 },
    { name: "cursor", saved: 9_400_000 },
    { name: "codex-cli", saved: 6_200_000 },
    { name: "perplexity-computer", saved: 4_100_000 },
    { name: "vercel-ai-sdk", saved: 2_800_000 },
  ],
  byRepo: [{ name: "/repos/tokenlens", saved: 18_400_000 }],
};

export async function getSummary(daysBack = 30): Promise<Summary> {
  const p = getPool();
  if (!p) return MOCK;

  const since = `NOW() - INTERVAL '${daysBack} days'`;
  const totals = await p.query(
    `SELECT COUNT(*)::bigint AS commands,
            COALESCE(SUM(input_tokens),0)::bigint AS input_tokens,
            COALESCE(SUM(output_tokens),0)::bigint AS output_tokens,
            COALESCE(SUM(saved_tokens),0)::bigint AS saved_tokens,
            COALESCE(SUM(dollars_saved),0)::float AS dollars_saved
       FROM events WHERE ts >= ${since}`
  );
  const t = totals.rows[0];

  const pivot = (col: string) =>
    p.query(
      `SELECT COALESCE(${col}, '<none>') AS name,
              COALESCE(SUM(saved_tokens),0)::bigint AS saved
         FROM events WHERE ts >= ${since}
        GROUP BY 1 ORDER BY saved DESC LIMIT 20`
    );

  const [m, a, r] = await Promise.all([pivot("model"), pivot("agent"), pivot("repo")]);
  const num = (v: any) => Number(v ?? 0);
  const rows = (rs: any) => rs.rows.map((row: any) => ({ name: row.name, saved: num(row.saved) }));

  return {
    commands: num(t.commands),
    inputTokens: num(t.input_tokens),
    outputTokens: num(t.output_tokens),
    savedTokens: num(t.saved_tokens),
    dollarsSaved: num(t.dollars_saved),
    byModel: rows(m),
    byAgent: rows(a),
    byRepo: rows(r),
  };
}

export async function ingest(events: any[]) {
  const p = getPool();
  if (!p) return { persisted: 0, mock: true };
  const client = await p.connect();
  try {
    await client.query("BEGIN");
    for (const e of events) {
      await client.query(
        `INSERT INTO events (ts, cmd, input_tokens, output_tokens, saved_tokens,
                             dollars_saved, agent, model, repo)
         VALUES (to_timestamp($1), $2, $3, $4, $5, $6, $7, $8, $9)`,
        [
          e.ts ?? Math.floor(Date.now() / 1000),
          e.cmd ?? "",
          e.input_tokens ?? 0,
          e.output_tokens ?? 0,
          e.saved_tokens ?? 0,
          e.dollars_saved ?? 0,
          e.agent ?? null,
          e.model ?? null,
          e.repo ?? null,
        ]
      );
    }
    await client.query("COMMIT");
  } catch (err) {
    await client.query("ROLLBACK");
    throw err;
  } finally {
    client.release();
  }
  return { persisted: events.length };
}
