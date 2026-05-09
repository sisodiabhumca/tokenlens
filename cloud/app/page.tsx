import { getSummary } from "../lib/db";

function fmt(n: number) {
  if (n >= 1e9) return `${(n / 1e9).toFixed(1)}B`;
  if (n >= 1e6) return `${(n / 1e6).toFixed(1)}M`;
  if (n >= 1e3) return `${(n / 1e3).toFixed(1)}K`;
  return `${n}`;
}

export default async function Page() {
  const s = await getSummary(30);
  const pct = s.inputTokens ? ((s.savedTokens / s.inputTokens) * 100).toFixed(1) : "0";

  return (
    <div>
      <div style={{ display: "flex", justifyContent: "space-between", alignItems: "baseline" }}>
        <h1 style={{ fontSize: 28, marginBottom: 8 }}>Savings overview</h1>
        <span style={{ color: "#9aa0a6" }}>last 30 days</span>
      </div>

      <section style={{ display: "grid", gridTemplateColumns: "repeat(4, 1fr)", gap: 16, margin: "24px 0" }}>
        <Card label="Commands" value={fmt(s.commands)} />
        <Card label="Tokens saved" value={`${fmt(s.savedTokens)} (${pct}%)`} />
        <Card label="Input → Output" value={`${fmt(s.inputTokens)} → ${fmt(s.outputTokens)}`} />
        <Card label="Estimated $ saved" value={`$${s.dollarsSaved.toFixed(2)}`} />
      </section>

      <section style={{ display: "grid", gridTemplateColumns: "1fr 1fr 1fr", gap: 16 }}>
        <Pivot title="By model" rows={s.byModel} />
        <Pivot title="By agent" rows={s.byAgent} />
        <Pivot title="By repo" rows={s.byRepo} />
      </section>

      <p style={{ marginTop: 32, color: "#9aa0a6", fontSize: 13 }}>
        Send events: <code>POST /api/ingest</code> with{" "}
        <code>{`{ "events": [{ ts, cmd, input_tokens, output_tokens, saved_tokens, dollars_saved, agent, model, repo }] }`}</code>.
      </p>
    </div>
  );
}

function Card({ label, value }: { label: string; value: string }) {
  return (
    <div style={{ background: "#11151a", border: "1px solid #1f2328", borderRadius: 12, padding: 16 }}>
      <div style={{ color: "#9aa0a6", fontSize: 12, textTransform: "uppercase", letterSpacing: 0.5 }}>{label}</div>
      <div style={{ fontSize: 22, marginTop: 6 }}>{value}</div>
    </div>
  );
}

function Pivot({ title, rows }: { title: string; rows: { name: string; saved: number }[] }) {
  const max = Math.max(1, ...rows.map((r) => r.saved));
  return (
    <div style={{ background: "#11151a", border: "1px solid #1f2328", borderRadius: 12, padding: 16 }}>
      <h3 style={{ marginTop: 0 }}>{title}</h3>
      {rows.length === 0 && <div style={{ color: "#9aa0a6", fontSize: 13 }}>No data yet.</div>}
      {rows.map((r) => (
        <div key={r.name} style={{ margin: "8px 0" }}>
          <div style={{ display: "flex", justifyContent: "space-between", fontSize: 13 }}>
            <span style={{ overflow: "hidden", textOverflow: "ellipsis", whiteSpace: "nowrap", maxWidth: 220 }}>{r.name}</span>
            <span>{fmt(r.saved)}</span>
          </div>
          <div style={{ background: "#1f2328", height: 6, borderRadius: 4, marginTop: 4 }}>
            <div style={{ background: "#4f8cff", height: 6, width: `${(r.saved / max) * 100}%`, borderRadius: 4 }} />
          </div>
        </div>
      ))}
    </div>
  );
}
