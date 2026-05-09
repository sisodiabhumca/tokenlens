import { NextResponse } from "next/server";
import { ingest } from "../../../lib/db";

export const runtime = "nodejs"; // pg requires the Node runtime

export async function POST(req: Request) {
  try {
    const body = await req.json();
    if (!Array.isArray(body?.events)) {
      return NextResponse.json({ error: "expected { events: [...] }" }, { status: 400 });
    }
    if (process.env.INGEST_TOKEN) {
      const auth = req.headers.get("authorization") ?? "";
      if (!auth.startsWith("Bearer ") || auth.slice(7) !== process.env.INGEST_TOKEN) {
        return NextResponse.json({ error: "unauthorized" }, { status: 401 });
      }
    }
    const result = await ingest(body.events);
    return NextResponse.json({ ok: true, ...result });
  } catch (e: any) {
    return NextResponse.json({ error: String(e?.message ?? e) }, { status: 500 });
  }
}
