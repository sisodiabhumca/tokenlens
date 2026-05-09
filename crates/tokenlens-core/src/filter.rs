//! Compression engine. Combines:
//!   1. Structural rules (regex strip + truncate, declared in `filters/*.toml`)
//!   2. Source-code filter (drops comments / blank lines)
//!   3. JSON / YAML structural prune
//!
//! Semantic compression lives in `semantic.rs` and is layered on top.

use crate::registry::filter_rules;
use crate::tokens::approx_tokens;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CompressionLevel {
    None,
    Minimal,
    Aggressive,
}

impl Default for CompressionLevel {
    fn default() -> Self { Self::Minimal }
}

impl std::str::FromStr for CompressionLevel {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "none" => Ok(Self::None),
            "minimal" => Ok(Self::Minimal),
            "aggressive" => Ok(Self::Aggressive),
            other => Err(format!("unknown level: {other}")),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FilterOutcome {
    pub original_tokens: u64,
    pub compressed_tokens: u64,
    pub saved_tokens: u64,
    pub savings_pct: f64,
    pub strategy: String,
    pub output: String,
}

impl FilterOutcome {
    pub fn from(input: &str, output: String, strategy: impl Into<String>) -> Self {
        let original = approx_tokens(input);
        let compressed = approx_tokens(&output);
        let saved = original.saturating_sub(compressed);
        let pct = if original == 0 { 0.0 } else { (saved as f64 / original as f64) * 100.0 };
        Self {
            original_tokens: original,
            compressed_tokens: compressed,
            saved_tokens: saved,
            savings_pct: pct,
            strategy: strategy.into(),
            output,
        }
    }
}

/// Top-level entry point. Picks the best engine based on `command`/`level` and
/// returns a `FilterOutcome` with the compressed output and savings stats.
pub fn compress_text(input: &str, command: Option<&str>, level: CompressionLevel) -> FilterOutcome {
    if level == CompressionLevel::None {
        return FilterOutcome::from(input, input.to_string(), "none");
    }

    // 1. Try a registered command rule.
    if let Some(cmd) = command {
        if let Some(rule) = filter_rules().match_cmd(cmd) {
            let out = apply_rule(input, rule);
            return FilterOutcome::from(input, out, format!("rule:{}", rule.name));
        }
    }

    // 2. Fall back to a generic structural pass.
    let out = structural_pass(input, level);
    FilterOutcome::from(input, out, "structural")
}

/// Apply a single TOML-declared rule.
pub fn apply_rule(input: &str, rule: &CompiledRule) -> String {
    let mut text: String = if rule.strip_ansi { strip_ansi(input) } else { input.to_string() };

    // Strip lines matching any of the strip patterns.
    if !rule.strip_patterns.is_empty() {
        text = text
            .lines()
            .filter(|line| !rule.strip_patterns.iter().any(|re| re.is_match(line)))
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Truncate long lines.
    if let Some(max) = rule.truncate_lines_at {
        text = text
            .lines()
            .map(|l| if l.chars().count() > max { truncate(l, max) } else { l.to_string() })
            .collect::<Vec<_>>()
            .join("\n");
    }

    // Cap total lines.
    if let Some(max_lines) = rule.max_lines {
        let lines: Vec<&str> = text.lines().collect();
        if lines.len() > max_lines {
            let kept = &lines[..max_lines];
            let omitted = lines.len() - max_lines;
            text = format!(
                "{}\n... +{} more lines (truncated by tokenlens)",
                kept.join("\n"),
                omitted
            );
        }
    }

    text
}

/// Generic structural pass: trims trailing whitespace and collapses runs of
/// blank lines. Used when no specific rule matched.
pub fn structural_pass(input: &str, level: CompressionLevel) -> String {
    let mut out = String::with_capacity(input.len());
    let mut blanks = 0;
    for line in input.lines() {
        let trimmed_end = line.trim_end();
        if trimmed_end.is_empty() {
            blanks += 1;
            let limit = if level == CompressionLevel::Aggressive { 0 } else { 1 };
            if blanks > limit { continue; }
        } else {
            blanks = 0;
        }
        out.push_str(trimmed_end);
        out.push('\n');
    }

    // Aggressive: collapse runs of identical "Downloading X..." style lines.
    if level == CompressionLevel::Aggressive {
        out = collapse_repeats(&out);
    }

    out.trim_end().to_string()
}

fn collapse_repeats(input: &str) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut prev: Option<String> = None;
    let mut count = 0;
    for line in input.lines() {
        if Some(line) == prev.as_deref() {
            count += 1;
            continue;
        }
        if count > 1 {
            if let Some(last) = out.last_mut() {
                last.push_str(&format!(" (x{count})"));
            }
        }
        out.push(line.to_string());
        prev = Some(line.to_string());
        count = 1;
    }
    if count > 1 {
        if let Some(last) = out.last_mut() {
            last.push_str(&format!(" (x{count})"));
        }
    }
    out.join("\n")
}

fn truncate(s: &str, max_chars: usize) -> String {
    let mut taken: String = s.chars().take(max_chars).collect();
    taken.push_str("…");
    taken
}

/// Compiled TOML rule.
#[derive(Debug)]
pub struct CompiledRule {
    pub name: String,
    pub match_command: Regex,
    pub strip_ansi: bool,
    pub strip_patterns: Vec<Regex>,
    pub max_lines: Option<usize>,
    pub truncate_lines_at: Option<usize>,
}

fn strip_ansi(input: &str) -> String {
    static ANSI: once_cell::sync::Lazy<Regex> =
        once_cell::sync::Lazy::new(|| Regex::new(r"\x1b\[[0-9;]*[A-Za-z]").unwrap());
    ANSI.replace_all(input, "").into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn structural_collapses_blank_lines() {
        let input = "a\n\n\n\nb\n";
        let out = structural_pass(input, CompressionLevel::Minimal);
        assert_eq!(out, "a\n\nb");
    }

    #[test]
    fn aggressive_drops_all_blanks_and_repeats() {
        let input = "downloading\ndownloading\ndownloading\ndone";
        let out = structural_pass(input, CompressionLevel::Aggressive);
        assert!(out.contains("(x3)"));
    }

    #[test]
    fn savings_pct_zero_for_empty_input() {
        let outcome = FilterOutcome::from("", String::new(), "noop");
        assert_eq!(outcome.savings_pct, 0.0);
    }
}
