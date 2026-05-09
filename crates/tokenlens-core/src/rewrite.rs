//! Thin CLI front for the rewrite registry. Mirrors RTK's exit-code protocol
//! so existing hooks keep working without changes.

use crate::registry::{rewrite_command, RewriteAction};
use crate::tracking;
use anyhow::Result;

fn debug_log_failure(e: anyhow::Error) {
    if std::env::var("TOKENLENS_DEBUG").as_deref() == Ok("1") {
        eprintln!("[tokenlens] tracker write failed: {e}");
    }
}

pub fn run(cmd: String) -> Result<()> {
    if cmd.trim().is_empty() {
        std::process::exit(1);
    }
    let r = rewrite_command(&cmd);
    match r.action {
        RewriteAction::Allow => {
            // already-wrapped is exit 0 with the same line; passthrough is exit 1.
            if r.reason == "already-wrapped" {
                println!("{}", r.command);
                std::process::exit(0);
            }
            std::process::exit(1);
        }
        RewriteAction::Rewrite => {
            // Best-effort: log the rewrite event so `tokenlens gain` reflects
            // exit-code-protocol activity. Saved-token math happens later when
            // the wrapped command runs through `tokenlens run`.
            if let Err(e) = tracking::record(format!("rewrite:{}", cmd), 0, 0, 0) {
                debug_log_failure(e);
            }
            println!("{}", r.command);
            std::process::exit(0);
        }
        RewriteAction::Deny => std::process::exit(2),
        RewriteAction::Ask => {
            if let Err(e) = tracking::record(format!("rewrite-ask:{}", cmd), 0, 0, 0) {
                debug_log_failure(e);
            }
            println!("{}", r.command);
            std::process::exit(3);
        }
    }
}
