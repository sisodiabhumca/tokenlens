async function getSummary() {
  // In production this would query Postgres. Stub data for now.
  return {
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
  };
}

function fmt(n: number) {
  if (n >= 1e9) return `${(n / 1e9).toFixed(1)}B`;
  if (n >= 1e6) return `${(n / 1e6).toFixed(1)}M`;
  if (n >= 1e3) return `${(n / 1e3).toFixed(1)}K`;
  return `${n}`;
}

export default async function Page() {
  const s = await getSummary();
  const pct = ((s.savedTokens / s.inputTokens) * 100).toFixed(1);
  return (
    <div>
      <h1 style={{ fontSize: 28, marginBottom: 8 }}>Savings overview</h1>
      <p style={{ color: "#9aa0a6", marginTop: 0 }}>Across your team's AI agents in the last 30 days.</p>

      <section style={{ display: "grid", gridTemplateColumns: "repeat(4, 1fr)", gap: 16, margin: "24px 0" }}>
        <Card label="Commands" value={fmt(s.commands)} />
        <Card label="Tokens saved" value={`${fmt(s.savedTokens)} (${pct}%)`} />
        <Card label="Input → Output" value={`${fmt(s.inputTokens)} → ${fmt(s.outputTokens)}`} />
        <Card label="Estimated $ saved" value={`$${s.dollarsSaved.toFixed(2)}`} />
      </section>

      <section style={{ display: "grid", gridTemplateColumns: "1fr 1fr", gap: 16 }}>
        <Pivot title="By model" rows={s.byModel} />
        <Pivot title="By agent" rows={s.byAgent} />
      </section>
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
  const max = Math.max(...rows.map((r) => r.saved));
  return (
    <div style={{ background: "#11151a", border: "1px solid #1f2328", borderRadius: 12, padding: 16 }}>
      <h3 style={{ marginTop: 0 }}>{title}</h3>
      {rows.map((r) => (
        <div key={r.name} style={{ margin: "8px 0" }}>
          <div style={{ display: "flex", justifyContent: "space-between", fontSize: 13 }}>
            <span>{r.name}</span><span>{fmt(r.saved)}</span>
          </div>
          <div style={{ background: "#1f2328", height: 6, borderRadius: 4, marginTop: 4 }}>
            <div style={{ background: "#4f8cff", height: 6, width: `${(r.saved / max) * 100}%`, borderRadius: 4 }} />
          </div>
        </div>
      ))}
    </div>
  );
}
