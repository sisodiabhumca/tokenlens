//! `tokenlens gain` — drop-in compatible with `rtk gain`, plus new pivots.

use anyhow::Result;
use colored::Colorize;

use crate::tracking::Tracker;

pub fn gain(by: &[String], format: &str) -> Result<()> {
    let tracker = Tracker::open_default()?;
    let summary = tracker.summary()?;

    if format == "json" {
        println!("{}", serde_json::to_string_pretty(&summary)?);
        return Ok(());
    }

    println!("{}", "TokenLens — savings summary".bold());
    println!("Total commands : {}", summary.commands);
    println!("Input tokens   : {}", fmt(summary.input_tokens));
    println!("Output tokens  : {}", fmt(summary.output_tokens));
    println!(
        "Tokens saved   : {} ({:.1}%)",
        fmt(summary.saved_tokens),
        summary.savings_pct()
    );
    println!("Est. $ saved   : ${:.2}", summary.dollars_saved);

    if !by.is_empty() {
        println!();
        for dim in by {
            match dim.as_str() {
                "model" => print_pivot("Model", &tracker.pivot_model()?),
                "repo" => print_pivot("Repo", &tracker.pivot_repo()?),
                "agent" => print_pivot("Agent", &tracker.pivot_agent()?),
                other => eprintln!("[tokenlens] unknown pivot: {other}"),
            }
        }
    }
    Ok(())
}

fn print_pivot(label: &str, rows: &[(String, u64)]) {
    println!("{} breakdown:", label.bold());
    for (k, v) in rows {
        println!("  {:<32} {}", k, fmt(*v));
    }
}

fn fmt(n: u64) -> String {
    let n = n as f64;
    if n >= 1.0e9 { format!("{:.1}B", n / 1.0e9) }
    else if n >= 1.0e6 { format!("{:.1}M", n / 1.0e6) }
    else if n >= 1.0e3 { format!("{:.1}K", n / 1.0e3) }
    else { format!("{n:.0}") }
}
