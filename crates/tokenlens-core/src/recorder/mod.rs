//! Recorder trait — pluggable analytics sink.
//!
//! Two built-in implementations:
//!   * `SqliteRecorder` — local SQLite (used by `tokenlens gain`)
//!   * `CloudRecorder`  — POSTs batched events to a TokenLens Cloud `/api/ingest`
//!
//! Custom recorders (OTLP, Datadog, Prometheus) live in downstream crates.

use crate::tracking::Tracker;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub ts: i64,
    pub cmd: String,
    pub input_tokens: u64,
    pub output_tokens: u64,
    pub saved_tokens: u64,
    pub dollars_saved: f64,
    pub agent: Option<String>,
    pub model: Option<String>,
    pub repo: Option<String>,
}

pub trait Recorder: Send + Sync {
    fn record(&self, event: &Event) -> anyhow::Result<()>;
}

pub struct SqliteRecorder {
    pub tracker: Tracker,
}

impl Recorder for SqliteRecorder {
    fn record(&self, e: &Event) -> anyhow::Result<()> {
        self.tracker.insert_event(e)
    }
}

pub struct CloudRecorder {
    pub endpoint: String,
    pub api_key: Option<String>,
    /// Buffer events; flush on drop or when this many accumulate.
    pub batch_size: usize,
    inner: std::sync::Mutex<Vec<Event>>,
}

impl CloudRecorder {
    pub fn new(endpoint: impl Into<String>, api_key: Option<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            api_key,
            batch_size: 32,
            inner: std::sync::Mutex::new(Vec::new()),
        }
    }

    pub fn flush(&self) -> anyhow::Result<()> {
        let mut buf = self.inner.lock().expect("lock");
        if buf.is_empty() { return Ok(()); }
        let body = serde_json::json!({ "events": &*buf });
        let mut req = ureq::post(&self.endpoint)
            .timeout(std::time::Duration::from_secs(10));
        if let Some(k) = &self.api_key {
            req = req.set("Authorization", &format!("Bearer {k}"));
        }
        match req.send_json(body) {
            Ok(_) => { buf.clear(); Ok(()) }
            Err(e) => Err(anyhow::anyhow!("cloud ingest failed: {e}")),
        }
    }
}

impl Recorder for CloudRecorder {
    fn record(&self, e: &Event) -> anyhow::Result<()> {
        let len = {
            let mut buf = self.inner.lock().expect("lock");
            buf.push(e.clone());
            buf.len()
        };
        if len >= self.batch_size {
            self.flush()?;
        }
        Ok(())
    }
}

impl Drop for CloudRecorder {
    fn drop(&mut self) {
        let _ = self.flush();
    }
}
