//! `tokenlens doctor` -- one-shot self-diagnosis for "gain returns zero" type
//! reports. Prints the resolved DB path, whether it's writable, and whether
//! recording an event survives a round-trip through the tracker.

use crate::tracking::{self, Tracker};
use anyhow::Result;
use std::path::PathBuf;

pub fn run() -> Result<()> {
    println!("TokenLens -- doctor");
    println!("version        : {}", env!("CARGO_PKG_VERSION"));

    // Resolve DB path the same way Tracker::open_default does.
    let env_db = std::env::var("TOKENLENS_DB").ok();
    let resolved: PathBuf = match &env_db {
        Some(p) if !p.is_empty() => PathBuf::from(p),
        _ => match dirs::data_local_dir() {
            Some(base) => base.join("tokenlens").join("tracking.db"),
            None => {
                println!("db path        : <unable to resolve dirs::data_local_dir()>");
                anyhow::bail!("no local data dir");
            }
        },
    };
    println!("db path        : {}", resolved.display());
    if let Some(p) = &env_db {
        println!("  source       : TOKENLENS_DB={}", p);
    } else {
        println!("  source       : platform default (dirs::data_local_dir)");
    }

    // Check parent dir.
    if let Some(parent) = resolved.parent() {
        println!("parent dir     : {}", parent.display());
        match std::fs::create_dir_all(parent) {
            Ok(()) => println!("  writable     : yes (created if missing)"),
            Err(e) => println!("  writable     : NO ({e})"),
        }
    }

    // Check pre-existing file size.
    match std::fs::metadata(&resolved) {
        Ok(m) => println!("file size      : {} bytes", m.len()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            println!("file size      : <not yet created>");
        }
        Err(e) => println!("file size      : <stat failed: {e}>"),
    }

    // Try to open + read summary.
    let pre = match Tracker::open_default() {
        Ok(t) => match t.summary() {
            Ok(s) => {
                println!(
                    "summary        : commands={} input={} output={} saved={} dollars=${:.4}",
                    s.commands, s.input_tokens, s.output_tokens, s.saved_tokens, s.dollars_saved
                );
                s.commands
            }
            Err(e) => {
                println!("summary        : FAILED ({e})");
                0
            }
        },
        Err(e) => {
            println!("open tracker   : FAILED ({e})");
            0
        }
    };

    // Round-trip: write a sentinel event and confirm the count grew.
    let sentinel = format!("doctor:roundtrip-{}", chrono::Utc::now().timestamp());
    print!("round-trip     : ");
    match tracking::record(&sentinel, 0, 0, 0) {
        Ok(()) => match Tracker::open_default().and_then(|t| t.summary()) {
            Ok(s) if s.commands > pre => println!("OK (commands {} -> {})", pre, s.commands),
            Ok(s) => println!("WROTE but commands still {} (was {}) -- schema/path mismatch?", s.commands, pre),
            Err(e) => println!("WROTE but read-back failed: {e}"),
        },
        Err(e) => println!("WRITE FAILED: {e}"),
    }

    // Env hints.
    println!();
    println!("env:");
    for k in ["TOKENLENS_DB", "TOKENLENS_AGENT", "TOKENLENS_MODEL", "TOKENLENS_DEBUG", "TOKENLENS_CLOUD_URL"] {
        match std::env::var(k) {
            Ok(v) => println!("  {} = {}", k, v),
            Err(_) => println!("  {} = <unset>", k),
        }
    }

    println!();
    println!("Tip: if 'commands' stays the same after the round-trip, the DB path is");
    println!("     unwritable or being shadowed. Try TOKENLENS_DB=/tmp/tl.db tokenlens doctor.");
    Ok(())
}
