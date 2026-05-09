//! Token-savings tracker (SQLite, 90-day retention).
//!
//! Schema-compatible target with RTK's `tracking.db` so `import-rtk` can
//! copy rows in place. New columns (`model`, `agent`, `repo`) are added
//! with NULL defaults.

use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Serialize, Default)]
pub struct Summary {
    pub commands: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub saved_tokens: u64,
    pub dollars_saved: f64,
}

impl Summary {
    pub fn savings_pct(&self) -> f64 {
        if self.input_tokens == 0 { 0.0 }
        else { (self.saved_tokens as f64) / (self.input_tokens as f64) * 100.0 }
    }
}

pub struct Tracker {
    conn: Connection,
}

impl Tracker {
    pub fn open_default() -> Result<Self> {
        let path = default_db_path()?;
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        let conn = Connection::open(&path)
            .with_context(|| format!("opening tracking db at {}", path.display()))?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn summary(&self) -> Result<Summary> {
        let mut s = Summary::default();
        let row: (i64, i64, i64, i64, f64) = self.conn.query_row(
            "SELECT COALESCE(COUNT(*),0),
                    COALESCE(SUM(input_tokens),0),
                    COALESCE(SUM(output_tokens),0),
                    COALESCE(SUM(saved_tokens),0),
                    COALESCE(SUM(dollars_saved),0.0)
             FROM events",
            [],
            |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;
        s.commands = row.0 as u64;
        s.input_tokens = row.1 as u64;
        s.output_tokens = row.2 as u64;
        s.saved_tokens = row.3 as u64;
        s.dollars_saved = row.4;
        Ok(s)
    }

    pub fn pivot_model(&self) -> Result<Vec<(String, u64)>> { self.pivot("model") }
    pub fn pivot_repo(&self) -> Result<Vec<(String, u64)>> { self.pivot("repo") }
    pub fn pivot_agent(&self) -> Result<Vec<(String, u64)>> { self.pivot("agent") }

    fn pivot(&self, col: &str) -> Result<Vec<(String, u64)>> {
        let sql = format!(
            "SELECT COALESCE({col},'<none>'), COALESCE(SUM(saved_tokens),0)
             FROM events GROUP BY {col} ORDER BY 2 DESC LIMIT 20"
        );
        let mut stmt = self.conn.prepare(&sql)?;
        let rows = stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as u64)))?
            .collect::<Result<Vec<_>, _>>()?;
        Ok(rows)
    }

    #[allow(dead_code)]
    pub fn record(
        &self,
        cmd: &str,
        input: u64,
        output: u64,
        agent: Option<&str>,
        model: Option<&str>,
        repo: Option<&str>,
    ) -> Result<()> {
        let saved = input.saturating_sub(output);
        self.conn.execute(
            "INSERT INTO events (ts, cmd, input_tokens, output_tokens, saved_tokens,
                                 dollars_saved, agent, model, repo)
             VALUES (strftime('%s','now'), ?1, ?2, ?3, ?4, 0.0, ?5, ?6, ?7)",
            params![cmd, input as i64, output as i64, saved as i64, agent, model, repo],
        )?;
        Ok(())
    }
}

const SCHEMA: &str = r#"
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ts INTEGER NOT NULL,
    cmd TEXT NOT NULL,
    input_tokens INTEGER NOT NULL DEFAULT 0,
    output_tokens INTEGER NOT NULL DEFAULT 0,
    saved_tokens INTEGER NOT NULL DEFAULT 0,
    dollars_saved REAL NOT NULL DEFAULT 0.0,
    agent TEXT,
    model TEXT,
    repo TEXT
);
CREATE INDEX IF NOT EXISTS idx_events_ts ON events(ts);
CREATE INDEX IF NOT EXISTS idx_events_model ON events(model);
CREATE INDEX IF NOT EXISTS idx_events_repo ON events(repo);
"#;

fn default_db_path() -> Result<PathBuf> {
    let base = dirs::data_local_dir().context("no local data dir")?;
    Ok(base.join("tokenlens").join("tracking.db"))
}

pub fn import_rtk(from: Option<&str>) -> Result<()> {
    let src = match from {
        Some(p) => PathBuf::from(p),
        None => {
            let base = dirs::data_local_dir().context("no local data dir")?;
            base.join("rtk").join("tracking.db")
        }
    };
    if !src.exists() {
        anyhow::bail!("RTK db not found at {}", src.display());
    }
    let dst = default_db_path()?;
    if let Some(p) = dst.parent() { std::fs::create_dir_all(p).ok(); }
    std::fs::copy(&src, &dst)?;
    println!("Imported RTK tracking db -> {}", dst.display());
    Ok(())
}
