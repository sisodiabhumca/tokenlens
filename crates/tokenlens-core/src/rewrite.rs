//! Command-rewrite engine.
//!
//! Mirrors RTK's exit-code protocol so existing hooks keep working:
//!   exit 0 + stdout  -> rewrite found, auto-allow
//!   exit 1           -> no equivalent, pass through
//!   exit 2           -> deny rule matched
//!   exit 3 + stdout  -> rewrite + ask
//!
//! This is a stub registry. The full RTK registry (~3,900 lines) is intended
//! to be ported under `src/registry/` with attribution.

use anyhow::Result;
use std::process::ExitCode;

pub fn run(cmd: String) -> Result<()> {
    let trimmed = cmd.trim();
    if trimmed.is_empty() {
        std::process::exit(1);
    }

    // Already wrapped — emit as-is, exit 0.
    if trimmed.starts_with("tokenlens ") || trimmed.starts_with("rtk ") {
        println!("{}", trimmed);
        std::process::exit(0);
    }

    if let Some(rewritten) = stub_registry(trimmed) {
        println!("{}", rewritten);
        std::process::exit(0);
    }

    // No equivalent.
    std::process::exit(1);
}

/// Tiny placeholder registry — replace with the ported RTK registry.
fn stub_registry(cmd: &str) -> Option<String> {
    const PREFIXES: &[&str] = &[
        "git ", "cargo ", "npm ", "pnpm ", "yarn ", "pytest", "ruff ",
        "ls ", "tree ", "grep ", "find ", "cat ", "tsc", "next ",
    ];
    for p in PREFIXES {
        if cmd.starts_with(p) {
            return Some(format!("tokenlens {}", cmd));
        }
    }
    None
}

// Compile-time assurance that ExitCode stays in scope for future use.
#[allow(dead_code)]
const _: fn() = || {
    let _ = ExitCode::SUCCESS;
};
