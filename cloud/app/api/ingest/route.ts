import { NextResponse } from "next/server";

/**
 * Ingest endpoint for the TokenLens core's cloud recorder.
 * In production: write batched events to Postgres / Neon.
 */
export async function POST(req: Request) {
  try {
    const body = await req.json();
    if (!Array.isArray(body?.events)) {
      return NextResponse.json({ error: "expected { events: [] }" }, { status: 400 });
    }
    // TODO: persist to Postgres. For now, count and ack.
    return NextResponse.json({ ok: true, received: body.events.length });
  } catch (e: any) {
    return NextResponse.json({ error: String(e?.message ?? e) }, { status: 500 });
  }
}
