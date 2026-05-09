//! Universal Hook Protocol server. Reads JSON lines on stdin, writes JSON
//! lines on stdout. Used by every TokenLens agent adapter.

use crate::registry::{rewrite_command, RewriteAction};
use crate::tracking;
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
        let resp = match serde_json::from_str::<HookRequest>(&line) {
            Ok(req) => handle(req),
            Err(e) => HookResponse {
                action: HookAction::Allow,
                payload: None,
                reason: Some(format!("parse error: {e}")),
            },
        };
        writeln!(out, "{}", serde_json::to_string(&resp)?)?;
        out.flush()?;
    }
    Ok(())
}

fn handle(req: HookRequest) -> HookResponse {
    if matches!(req.tool.as_str(), "bash" | "shell") {
        if let Some(cmd) = req.payload.get("command").and_then(|v| v.as_str()) {
            let r = rewrite_command(cmd);
            return match r.action {
                RewriteAction::Rewrite | RewriteAction::Ask => {
                    // Record one tracker event per rewrite so `tokenlens gain`
                    // reflects hook activity. The wrapped command records its
                    // own saved-token tally when it later runs through
                    // `tokenlens run …`, so we deliberately log 0 here — this
                    // is a counter of agent-side rewrites, not a savings claim.
                    let _ = tracking::record(format!("hook:{}", cmd), 0, 0, 0);
                    let mut payload = req.payload.clone();
                    payload["command"] = serde_json::Value::String(r.command);
                    HookResponse {
                        action: if r.action == RewriteAction::Ask { HookAction::Ask } else { HookAction::Rewrite },
                        payload: Some(payload),
                        reason: Some(r.reason.into()),
                    }
                }
                RewriteAction::Deny => HookResponse {
                    action: HookAction::Deny,
                    payload: None,
                    reason: Some(r.reason.into()),
                },
                RewriteAction::Allow => HookResponse {
                    action: HookAction::Allow,
                    payload: None,
                    reason: Some(r.reason.into()),
                },
            };
        }
    }
    HookResponse { action: HookAction::Allow, payload: None, reason: None }
}
