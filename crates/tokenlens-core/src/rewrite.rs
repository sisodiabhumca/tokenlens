//! Thin CLI front for the rewrite registry. Mirrors RTK's exit-code protocol
//! so existing hooks keep working without changes.

use crate::registry::{rewrite_command, RewriteAction};
use anyhow::Result;

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
            println!("{}", r.command);
            std::process::exit(0);
        }
        RewriteAction::Deny => std::process::exit(2),
        RewriteAction::Ask => {
            println!("{}", r.command);
            std::process::exit(3);
        }
    }
}
