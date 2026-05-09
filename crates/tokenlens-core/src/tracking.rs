//! Local SQLite tracker.

use crate::recorder::Event;
use anyhow::{Context, Result};
use rusqlite::{params, Connection};
use serde::Serialize;
use std::path::PathBuf;

#[derive(Debug, Default, Serialize)]
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
    pub conn: Connection,
}

impl Tracker {
    pub fn open_default() -> Result<Self> {
        let path = default_db_path()?;
        if let Some(p) = path.parent() { std::fs::create_dir_all(p).ok(); }
        let conn = Connection::open(&path).with_context(|| format!("opening {}", path.display()))?;
        conn.execute_batch(SCHEMA)?;
        Ok(Self { conn })
    }

    pub fn insert_event(&self, e: &Event) -> Result<()> {
        self.conn.execute(
            "INSERT INTO events (ts, cmd, input_tokens, output_tokens, saved_tokens,
                                 dollars_saved, agent, model, repo)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                e.ts, e.cmd, e.input_tokens as i64, e.output_tokens as i64,
                e.saved_tokens as i64, e.dollars_saved, e.agent, e.model, e.repo
            ],
        )?;
        Ok(())
    }

    pub fn summary(&self) -> Result<Summary> {
        let row: (i64, i64, i64, i64, f64) = self.conn.query_row(
            "SELECT COALESCE(COUNT(*),0),
                    COALESCE(SUM(input_tokens),0),
                    COALESCE(SUM(output_tokens),0),
                    COALESCE(SUM(saved_tokens),0),
                    COALESCE(SUM(dollars_saved),0.0)
             FROM events",
            [], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;
        Ok(Summary {
            commands: row.0 as u64,
            input_tokens: row.1 as u64,
            output_tokens: row.2 as u64,
            saved_tokens: row.3 as u64,
            dollars_saved: row.4,
        })
    }

    pub fn summary_since(&self, since_ts: i64) -> Result<Summary> {
        let row: (i64, i64, i64, i64, f64) = self.conn.query_row(
            "SELECT COALESCE(COUNT(*),0),
                    COALESCE(SUM(input_tokens),0),
                    COALESCE(SUM(output_tokens),0),
                    COALESCE(SUM(saved_tokens),0),
                    COALESCE(SUM(dollars_saved),0.0)
             FROM events WHERE ts >= ?1",
            params![since_ts], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?, r.get(4)?)),
        )?;
        Ok(Summary {
            commands: row.0 as u64,
            input_tokens: row.1 as u64,
            output_tokens: row.2 as u64,
            saved_tokens: row.3 as u64,
            dollars_saved: row.4,
        })
    }

    pub fn pivot(&self, col: &str) -> Result<Vec<(String, u64)>> {
        if !matches!(col, "model" | "agent" | "repo") {
            anyhow::bail!("invalid pivot column: {col}");
        }
        let sql = format!(
            "SELECT COALESCE({col},'<none>'), COALESCE(SUM(saved_tokens),0)
             FROM events GROUP BY {col} ORDER BY 2 DESC LIMIT 20"
        );
        let mut stmt = self.conn.prepare(&sql)?;
        Ok(stmt
            .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)? as u64)))?
            .collect::<Result<Vec<_>, _>>()?)
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
        None => dirs::data_local_dir().context("no local data dir")?.join("rtk").join("tracking.db"),
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

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn insert_and_summary_roundtrip() {
        let dir = tempdir().unwrap();
        let conn = Connection::open(dir.path().join("t.db")).unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        let t = Tracker { conn };
        t.insert_event(&Event {
            ts: 1, cmd: "git diff".into(), input_tokens: 100, output_tokens: 10,
            saved_tokens: 90, dollars_saved: 0.01,
            agent: Some("claude-code".into()), model: Some("claude-sonnet-4.5".into()),
            repo: Some("/r".into()),
        }).unwrap();
        let s = t.summary().unwrap();
        assert_eq!(s.commands, 1);
        assert_eq!(s.saved_tokens, 90);
    }
}
