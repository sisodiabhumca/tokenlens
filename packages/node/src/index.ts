import { spawn } from "node:child_process";

export interface RewriteResult { rewritten: string; action: "allow" | "rewrite" | "deny" | "ask"; }

/** Spawn `tokenlens rewrite` and return the result. Falls back to allow on error. */
export function rewrite(cmd: string, bin = "tokenlens"): Promise<RewriteResult> {
  return new Promise((resolve) => {
    const child = spawn(bin, ["rewrite", cmd]);
    let out = "";
    child.stdout.on("data", (d) => (out += d.toString()));
    child.on("close", (code) => {
      const rewritten = out.trim() || cmd;
      const action: RewriteResult["action"] =
        code === 0 ? "rewrite" : code === 2 ? "deny" : code === 3 ? "ask" : "allow";
      resolve({ rewritten, action });
    });
    child.on("error", () => resolve({ rewritten: cmd, action: "allow" }));
  });
}
