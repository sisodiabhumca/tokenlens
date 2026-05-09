//! UHP server: reads JSON lines on stdin, writes JSON responses on stdout.

use anyhow::Result;
use std::io::{BufRead, Write};
use tokenlens_uhp::{HookAction, HookRequest, HookResponse};

pub fn recv() -> Result<()> {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut out = stdout.lock();

    for line in stdin.lock().lines() {
        let line = line?;
        if line.trim().is_empty() { continue; }

        let req: HookRequest = match serde_json::from_str(&line) {
            Ok(r) => r,
            Err(e) => {
                let resp = HookResponse {
                    action: HookAction::Allow,
                    payload: None,
                    reason: Some(format!("parse error: {e}")),
                };
                writeln!(out, "{}", serde_json::to_string(&resp)?)?;
                continue;
            }
        };

        let resp = handle(req);
        writeln!(out, "{}", serde_json::to_string(&resp)?)?;
        out.flush()?;
    }
    Ok(())
}

fn handle(req: HookRequest) -> HookResponse {
    if req.tool == "bash" || req.tool == "shell" {
        if let Some(cmd) = req.payload.get("command").and_then(|v| v.as_str()) {
            if let Some(rewritten) = stub_rewrite(cmd) {
                let mut payload = req.payload.clone();
                payload["command"] = serde_json::Value::String(rewritten);
                return HookResponse {
                    action: HookAction::Rewrite,
                    payload: Some(payload),
                    reason: Some("structural-stub".into()),
                };
            }
        }
    }
    HookResponse { action: HookAction::Allow, payload: None, reason: None }
}

fn stub_rewrite(cmd: &str) -> Option<String> {
    const PREFIXES: &[&str] = &["git ", "cargo ", "npm ", "pnpm ", "pytest", "tsc"];
    for p in PREFIXES {
        if cmd.starts_with(p) && !cmd.starts_with("tokenlens ") {
            return Some(format!("tokenlens {}", cmd));
        }
    }
    None
}
