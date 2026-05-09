import { NextResponse } from "next/server";
import { getPool } from "../../../lib/db";

export const runtime = "nodejs";

export async function GET() {
  const p = getPool();
  if (!p) return NextResponse.json({ budgets: [] });
  const r = await p.query("SELECT project_id, monthly_usd, alert_pct, webhook_url FROM budgets ORDER BY project_id");
  return NextResponse.json({ budgets: r.rows });
}

export async function POST(req: Request) {
  const p = getPool();
  if (!p) return NextResponse.json({ error: "no DATABASE_URL" }, { status: 503 });
  const body = await req.json();
  const { project_id, monthly_usd, alert_pct = 0.8, webhook_url = null } = body ?? {};
  if (!project_id || typeof monthly_usd !== "number") {
    return NextResponse.json({ error: "project_id and monthly_usd required" }, { status: 400 });
  }
  await p.query(
    `INSERT INTO budgets (project_id, monthly_usd, alert_pct, webhook_url, updated_at)
     VALUES ($1, $2, $3, $4, NOW())
     ON CONFLICT (project_id)
     DO UPDATE SET monthly_usd = EXCLUDED.monthly_usd,
                   alert_pct = EXCLUDED.alert_pct,
                   webhook_url = EXCLUDED.webhook_url,
                   updated_at = NOW()`,
    [project_id, monthly_usd, alert_pct, webhook_url]
  );
  return NextResponse.json({ ok: true });
}
