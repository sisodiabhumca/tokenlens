use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Serialize, Deserialize)]
struct Config {
    monthly_usd: Option<f64>,
}

pub fn run(set_monthly: Option<f64>) -> Result<()> {
    let path = config_path()?;
    let mut cfg: Config = if path.exists() {
        serde_json::from_str(&std::fs::read_to_string(&path)?).unwrap_or_default()
    } else { Config::default() };

    if let Some(usd) = set_monthly {
        cfg.monthly_usd = Some(usd);
        if let Some(p) = path.parent() { std::fs::create_dir_all(p).ok(); }
        std::fs::write(&path, serde_json::to_string_pretty(&cfg)?)?;
        println!("Monthly budget set to ${:.2}", usd);
        return Ok(());
    }

    match cfg.monthly_usd {
        Some(b) => println!("Current monthly budget: ${:.2}", b),
        None => println!("No monthly budget set. Use --set-monthly <USD>."),
    }
    Ok(())
}

fn config_path() -> Result<PathBuf> {
    let base = dirs::config_dir().context("no config dir")?;
    Ok(base.join("tokenlens").join("config.json"))
}
