//! `tokenlens read <file>` — read a file with token-aware filtering.

use crate::filter::{compress_text, CompressionLevel, FilterOutcome};
use anyhow::Result;
use std::path::Path;

pub fn read_file(path: &Path, level: CompressionLevel, max_lines: Option<usize>) -> Result<FilterOutcome> {
    let bytes = std::fs::read(path)?;
    let text = String::from_utf8_lossy(&bytes).to_string();
    let limited = match max_lines {
        Some(n) => {
            let mut out: String = text.lines().take(n).collect::<Vec<_>>().join("\n");
            let total = text.lines().count();
            if total > n {
                out.push_str(&format!("\n... +{} more lines", total - n));
            }
            out
        }
        None => text,
    };
    let cmd = format!("read {}", path.display());
    Ok(compress_text(&limited, Some(&cmd), level))
}
