//! Budget management + alert logic.
//!
//! `tokenlens budget` shows current cap and month-to-date spend.
//! `tokenlens budget check` emits a non-zero exit code (and an optional
//! webhook POST) when projected month-end spend exceeds the cap.

use crate::tracking::Tracker;
use anyhow::{Context, Result};
use chrono::{Datelike, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
pub struct Config {
    pub monthly_usd: Option<f64>,
    pub alert_pct: Option<f64>,        // 0.0–1.0; default 0.8
    pub webhook_url: Option<String>,   // POST { project, month, used, cap, projected }
}

impl Config {
    pub fn load() -> Result<Self> {
        let p = config_path()?;
        if !p.exists() { return Ok(Self::default()); }
        Ok(serde_json::from_str(&std::fs::read_to_string(p)?).unwrap_or_default())
    }

    pub fn save(&self) -> Result<()> {
        let p = config_path()?;
        if let Some(parent) = p.parent() { std::fs::create_dir_all(parent).ok(); }
        std::fs::write(p, serde_json::to_string_pretty(self)?)?;
        Ok(())
    }
}

pub fn show() -> Result<()> {
    let cfg = Config::load()?;
    match cfg.monthly_usd {
        Some(b) => println!("Monthly cap: ${:.2}", b),
        None => println!("No monthly budget set. Try: tokenlens budget --set-monthly 50"),
    }
    if let Some(pct) = cfg.alert_pct { println!("Alert threshold: {:.0}%", pct * 100.0); }
    if let Some(url) = &cfg.webhook_url { println!("Webhook: {url}"); }
    Ok(())
}

pub fn set_monthly(usd: f64) -> Result<()> {
    let mut cfg = Config::load()?;
    cfg.monthly_usd = Some(usd);
    cfg.save()?;
    println!("Monthly budget set to ${:.2}", usd);
    Ok(())
}

pub fn set_webhook(url: String) -> Result<()> {
    let mut cfg = Config::load()?;
    cfg.webhook_url = Some(url.clone());
    cfg.save()?;
    println!("Webhook set: {url}");
    Ok(())
}

pub fn check() -> Result<()> {
    let cfg = Config::load()?;
    let cap = match cfg.monthly_usd {
        Some(c) => c,
        None => { println!("No budget set; nothing to check."); return Ok(()); }
    };
    let alert_pct = cfg.alert_pct.unwrap_or(0.8);
    let tracker = Tracker::open_default()?;

    let now = Utc::now();
    let month_start = Utc.with_ymd_and_hms(now.year(), now.month(), 1, 0, 0, 0).single()
        .context("invalid month start")?;
    let used = tracker.summary_since(month_start.timestamp())?.dollars_saved;

    // "dollars_saved" is the avoided cost; spend is what was *not* saved.
    // Projected spend = used / fraction-of-month-elapsed.
    let secs_in_month = days_in_month(now.year(), now.month()) as i64 * 86_400;
    let elapsed = (now.timestamp() - month_start.timestamp()).max(1) as f64;
    let fraction = elapsed / secs_in_month as f64;
    let projected = if fraction > 0.0 { used / fraction } else { 0.0 };

    println!("Month-to-date avoided cost: ${:.2}", used);
    println!("Projected month-end:        ${:.2}", projected);
    println!("Cap:                        ${:.2}", cap);

    let over = projected >= cap;
    let warn = projected >= cap * alert_pct;

    if over || warn {
        let level = if over { "OVER" } else { "WARN" };
        println!("[{level}] Projected to {} {:.0}% of cap.",
                 if over { "exceed" } else { "approach" },
                 (projected / cap) * 100.0);
        if let Some(url) = &cfg.webhook_url {
            let body = serde_json::json!({
                "level": level,
                "month": format!("{:04}-{:02}", now.year(), now.month()),
                "used": used, "projected": projected, "cap": cap,
            });
            if let Err(e) = ureq::post(url).timeout(std::time::Duration::from_secs(10)).send_json(body) {
                eprintln!("[tokenlens] webhook failed: {e}");
            }
        }
        if over { std::process::exit(1); }
    }
    Ok(())
}

fn config_path() -> Result<PathBuf> {
    let base = dirs::config_dir().context("no config dir")?;
    Ok(base.join("tokenlens").join("config.json"))
}

fn days_in_month(y: i32, m: u32) -> u32 {
    let next = if m == 12 { Utc.with_ymd_and_hms(y + 1, 1, 1, 0, 0, 0) }
               else { Utc.with_ymd_and_hms(y, m + 1, 1, 0, 0, 0) };
    let this = Utc.with_ymd_and_hms(y, m, 1, 0, 0, 0).single().unwrap();
    ((next.single().unwrap() - this).num_days()) as u32
}
