//! Universal Hook Protocol (UHP) v1.
//!
//! One JSON line in, one JSON line out. Used by every TokenLens agent adapter
//! (Claude Code, Codex, Cursor, Perplexity, Windsurf, Cline, …).
//!
//! ## Request
//! ```json
//! { "v": 1, "event": "tool.before", "agent": "claude-code",
//!   "tool": "bash", "payload": {"command": "git diff"},
//!   "meta": {"repo": "/repo", "model": "claude-sonnet-4.5"} }
//! ```
//!
//! ## Response
//! ```json
//! { "action": "rewrite", "payload": {"command": "tokenlens git diff"},
//!   "reason": "structural-rule:git.diff" }
//! ```

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookRequest {
    /// Protocol version. Always 1 for now.
    pub v: u32,
    /// Event name, e.g. "tool.before", "tool.after", "file.read".
    pub event: String,
    /// Originating agent: "claude-code", "codex-cli", "cursor", "perplexity-computer", …
    pub agent: String,
    /// Tool name being invoked: "bash", "shell", "read_file", "fetch", …
    pub tool: String,
    /// Tool-specific payload. Common keys: `command`, `path`, `url`, `args`.
    pub payload: Value,
    /// Free-form metadata: repo path, model id, session id.
    #[serde(default)]
    pub meta: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum HookAction {
    /// Pass through unchanged.
    Allow,
    /// Replace `payload` with the response's payload.
    Rewrite,
    /// Refuse the operation.
    Deny,
    /// Rewrite, but ask the user to confirm before running.
    Ask,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookResponse {
    pub action: HookAction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}
