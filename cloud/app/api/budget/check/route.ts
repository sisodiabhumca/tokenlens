import { NextResponse } from "next/server";
import { getPool } from "../../../../lib/db";

export const runtime = "nodejs";

/**
 * Scheduled-call endpoint (Vercel Cron). Iterates every project's budget,
 * computes month-to-date avoided cost and projected month-end, posts to the
 * project's webhook when projected ≥ alert_pct * cap.
 */
export async function GET(req: Request) {
  // Vercel Cron sends an Authorization: Bearer $CRON_SECRET header.
  const expected = process.env.CRON_SECRET;
  if (expected) {
    const auth = req.headers.get("authorization") ?? "";
    if (auth !== `Bearer ${expected}`) {
      return NextResponse.json({ error: "unauthorized" }, { status: 401 });
    }
  }
  const p = getPool();
  if (!p) return NextResponse.json({ checked: 0, mock: true });

  const budgets = await p.query("SELECT project_id, monthly_usd, alert_pct, webhook_url FROM budgets");
  const alerts: any[] = [];
  for (const b of budgets.rows) {
    const where = b.project_id ? "WHERE project_id = $1" : "";
    const params = b.project_id ? [b.project_id] : [];
    const used = await p.query(
      `SELECT COALESCE(SUM(dollars_saved),0)::float AS used
         FROM events ${where}
        AND ts >= date_trunc('month', NOW())`.replace("WHERE", where ? "WHERE" : "WHERE 1=1 AND"),
      params
    );
    const u = Number(used.rows[0].used ?? 0);
    const dim = await p.query(
      `SELECT EXTRACT(EPOCH FROM (NOW() - date_trunc('month', NOW())))::float AS elapsed,
              EXTRACT(EPOCH FROM (date_trunc('month', NOW()) + INTERVAL '1 month' - date_trunc('month', NOW())))::float AS total`
    );
    const elapsed = Number(dim.rows[0].elapsed) || 1;
    const total = Number(dim.rows[0].total) || 1;
    const projected = u / (elapsed / total);
    const over = projected >= b.monthly_usd;
    const warn = projected >= b.monthly_usd * (b.alert_pct ?? 0.8);
    if ((over || warn) && b.webhook_url) {
      try {
        await fetch(b.webhook_url, {
          method: "POST",
          headers: { "content-type": "application/json" },
          body: JSON.stringify({
            project_id: b.project_id,
            level: over ? "OVER" : "WARN",
            used: u,
            projected,
            cap: b.monthly_usd,
          }),
        });
      } catch { /* swallow webhook errors */ }
    }
    alerts.push({ project_id: b.project_id, used: u, projected, cap: b.monthly_usd, level: over ? "OVER" : warn ? "WARN" : "OK" });
  }
  return NextResponse.json({ checked: alerts.length, alerts });
}
