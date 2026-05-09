import { describe, it, expect } from "vitest";
import { approxTokens, structuralCompress, tokenLens } from "../src/index";

describe("structuralCompress", () => {
  it("collapses excessive blank lines", () => {
    const out = structuralCompress("a\n\n\n\nb");
    expect(out).toBe("a\n\nb");
  });
  it("trims trailing whitespace", () => {
    expect(structuralCompress("a  \nb")).toBe("a\nb");
  });
});

describe("tokenLens middleware", () => {
  it("compresses .text and records an event", async () => {
    const events: any[] = [];
    const mw = tokenLens({ level: "minimal", record: (e) => { events.push(e); } });
    const out = await mw({ text: "x  \n\n\n\ny" });
    expect(out.text).toBe("x\n\ny");
    expect(events).toHaveLength(1);
    expect(events[0].savedTokens).toBeGreaterThanOrEqual(0);
  });
  it("returns the input unchanged when no .text", async () => {
    const mw = tokenLens();
    const obj = { foo: 1 } as any;
    expect(await mw(obj)).toBe(obj);
  });
});

describe("approxTokens", () => {
  it("zero for empty", () => expect(approxTokens("")).toBe(0));
  it("rounds up", () => expect(approxTokens("abcd")).toBe(1));
});
