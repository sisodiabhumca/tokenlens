//! Generic subprocess runner: execute a command, capture stdout/stderr,
//! pass the combined output through the compression engine, return both
//! the original and compressed payloads plus a `FilterOutcome`.

use crate::filter::{compress_text, CompressionLevel, FilterOutcome};
use anyhow::Result;
use std::process::{Command, Stdio};

#[derive(Debug)]
pub struct ProxyResult {
    pub status: i32,
    pub raw: String,
    pub outcome: FilterOutcome,
}

/// Run `program args...`, combining stdout+stderr, then compress.
pub fn run_proxied(program: &str, args: &[String], level: CompressionLevel) -> Result<ProxyResult> {
    let output = Command::new(program)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let mut raw = String::with_capacity(stdout.len() + stderr.len());
    raw.push_str(&stdout);
    if !stderr.is_empty() {
        if !raw.ends_with('\n') { raw.push('\n'); }
        raw.push_str(&stderr);
    }

    let cmd_line = format!("{} {}", program, args.join(" "));
    let outcome = compress_text(&raw, Some(&cmd_line), level);

    Ok(ProxyResult {
        status: output.status.code().unwrap_or(-1),
        raw,
        outcome,
    })
}
