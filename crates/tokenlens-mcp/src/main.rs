//! TokenLens MCP server. Stdio JSON-RPC 2.0. Real implementations now —
//! delegates to `tokenlens-core` for compression and analytics.

use anyhow::Result;
use serde_json::{json, Value};
use std::path::PathBuf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use tokenlens_core::filter::{compress_text, CompressionLevel};
use tokenlens_core::tracking::Tracker;

#[tokio::main]
async fn main() -> Result<()> {
    let stdin = tokio::io::stdin();
    let mut reader = BufReader::new(stdin).lines();
    let mut stdout = tokio::io::stdout();

    while let Some(line) = reader.next_line().await? {
        if line.trim().is_empty() { continue; }
        let req: Value = match serde_json::from_str(&line) {
            Ok(v) => v,
            Err(_) => continue,
        };
        let id = req.get("id").cloned().unwrap_or(Value::Null);
        let method = req.get("method").and_then(|m| m.as_str()).unwrap_or("");
        let result = match method {
            "initialize" => json!({
                "protocolVersion": "2024-11-05",
                "serverInfo": { "name": "tokenlens-mcp", "version": env!("CARGO_PKG_VERSION") },
                "capabilities": { "tools": {} }
            }),
            "tools/list" => tools_list(),
            "tools/call" => handle_call(&req),
            other => json!({ "error": format!("unknown method: {other}") }),
        };
        let resp = json!({ "jsonrpc": "2.0", "id": id, "result": result });
        let line = serde_json::to_string(&resp)?;
        stdout.write_all(line.as_bytes()).await?;
        stdout.write_all(b"\n").await?;
        stdout.flush().await?;
    }
    Ok(())
}

fn tools_list() -> Value {
    json!({
        "tools": [
            tool_def("compress",
                "Compress text/JSON using the TokenLens engine. Optional `command` hint picks a TOML rule.",
                json!({
                    "type": "object",
                    "required": ["text"],
                    "properties": {
                        "text": {"type": "string"},
                        "command": {"type": "string"},
                        "level": {"type": "string", "enum": ["none", "minimal", "aggressive"]}
                    }
                })),
            tool_def("gain", "Return current TokenLens savings summary.",
                json!({"type": "object", "properties": {}})),
            tool_def("lens.read", "Read a file with TokenLens filtering.",
                json!({
                    "type": "object",
                    "required": ["path"],
                    "properties": {
                        "path": {"type": "string"},
                        "level": {"type": "string"},
                        "max_lines": {"type": "integer"}
                    }
                })),
            tool_def("lens.diff", "Token-efficient diff between two strings.",
                json!({
                    "type": "object",
                    "required": ["a", "b"],
                    "properties": {"a": {"type": "string"}, "b": {"type": "string"}}
                })),
        ]
    })
}

fn tool_def(name: &str, desc: &str, schema: Value) -> Value {
    json!({ "name": name, "description": desc, "inputSchema": schema })
}

fn handle_call(req: &Value) -> Value {
    let params = req.get("params").cloned().unwrap_or(Value::Null);
    let name = params.get("name").and_then(|n| n.as_str()).unwrap_or("");
    let args = params.get("arguments").cloned().unwrap_or(Value::Null);

    match name {
        "compress" => {
            let text = args.get("text").and_then(|t| t.as_str()).unwrap_or("");
            let cmd = args.get("command").and_then(|c| c.as_str());
            let level = args.get("level").and_then(|l| l.as_str()).unwrap_or("minimal");
            let level: CompressionLevel = level.parse().unwrap_or_default();
            let outcome = compress_text(text, cmd, level);
            json!({
                "content": [
                    { "type": "text", "text": outcome.output },
                    { "type": "text", "text": format!(
                        "[tokenlens] strategy={} saved={} ({:.1}%)",
                        outcome.strategy, outcome.saved_tokens, outcome.savings_pct) }
                ]
            })
        }
        "gain" => {
            match Tracker::open_default().and_then(|t| t.summary()) {
                Ok(s) => content(&format!(
                    "Commands: {} | Saved: {} ({:.1}%) | $ saved: ${:.2}",
                    s.commands, s.saved_tokens, s.savings_pct(), s.dollars_saved
                )),
                Err(e) => content(&format!("error: {e}")),
            }
        }
        "lens.read" => {
            let path = args.get("path").and_then(|p| p.as_str()).unwrap_or("");
            let level = args.get("level").and_then(|l| l.as_str()).unwrap_or("minimal");
            let max_lines = args.get("max_lines").and_then(|n| n.as_u64()).map(|n| n as usize);
            let level: CompressionLevel = level.parse().unwrap_or_default();
            match tokenlens_core::cmds::read::read_file(&PathBuf::from(path), level, max_lines) {
                Ok(o) => content(&o.output),
                Err(e) => content(&format!("error reading {path}: {e}")),
            }
        }
        "lens.diff" => {
            let a = args.get("a").and_then(|s| s.as_str()).unwrap_or("");
            let b = args.get("b").and_then(|s| s.as_str()).unwrap_or("");
            content(&simple_diff(a, b))
        }
        other => content(&format!("unknown tool: {other}")),
    }
}

fn content(text: &str) -> Value {
    json!({ "content": [{ "type": "text", "text": text }] })
}

/// Tiny line-level diff. Replace with a real LCS diff later.
fn simple_diff(a: &str, b: &str) -> String {
    let av: Vec<&str> = a.lines().collect();
    let bv: Vec<&str> = b.lines().collect();
    let mut out = String::new();
    let max = av.len().max(bv.len());
    for i in 0..max {
        match (av.get(i), bv.get(i)) {
            (Some(x), Some(y)) if x == y => {}
            (Some(x), Some(y)) => {
                out.push_str(&format!("- {x}\n+ {y}\n"));
            }
            (Some(x), None) => out.push_str(&format!("- {x}\n")),
            (None, Some(y)) => out.push_str(&format!("+ {y}\n")),
            _ => {}
        }
    }
    if out.is_empty() { "(no diff)".into() } else { out }
}
