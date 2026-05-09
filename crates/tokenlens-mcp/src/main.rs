//! TokenLens MCP server (stdio transport, JSON-RPC 2.0).
//!
//! Exposes the following tools to MCP clients (ChatGPT Desktop, Claude Desktop, Cursor MCP):
//!
//!   compress    — compress arbitrary text/JSON before it lands in context
//!   gain        — return current TokenLens savings summary
//!   lens.read   — read a file with TokenLens filtering
//!   lens.diff   — compute a token-efficient diff
//!
//! This is a minimal-but-real handler for `initialize`, `tools/list`, and
//! `tools/call`. Replace the stub implementations with calls into
//! `tokenlens-core` once it exposes a library API.

use anyhow::Result;
use serde_json::{json, Value};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

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
            "tools/list" => json!({
                "tools": [
                    tool_def("compress", "Compress arbitrary text or JSON to reduce token count.",
                             json!({"type":"object","required":["text"],
                                    "properties":{"text":{"type":"string"},
                                                  "level":{"type":"string","enum":["minimal","aggressive"]}}})),
                    tool_def("gain", "Return current TokenLens savings summary.",
                             json!({"type":"object","properties":{}})),
                    tool_def("lens.read", "Read a file with TokenLens filtering.",
                             json!({"type":"object","required":["path"],
                                    "properties":{"path":{"type":"string"},
                                                  "level":{"type":"string"}}})),
                    tool_def("lens.diff", "Token-efficient diff between two files or strings.",
                             json!({"type":"object","required":["a","b"],
                                    "properties":{"a":{"type":"string"},"b":{"type":"string"}}})),
                ]
            }),
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
            // Stub: drop blank lines and excessive whitespace.
            let compressed: String = text
                .lines()
                .map(str::trim_end)
                .filter(|l| !l.is_empty())
                .collect::<Vec<_>>()
                .join("\n");
            content_result(&compressed)
        }
        "gain" => content_result("TokenLens savings: (stub)"),
        "lens.read" => {
            let path = args.get("path").and_then(|p| p.as_str()).unwrap_or("");
            content_result(&format!("[lens.read stub] would compress: {path}"))
        }
        "lens.diff" => content_result("[lens.diff stub]"),
        other => content_result(&format!("unknown tool: {other}")),
    }
}

fn content_result(text: &str) -> Value {
    json!({ "content": [{ "type": "text", "text": text }] })
}
