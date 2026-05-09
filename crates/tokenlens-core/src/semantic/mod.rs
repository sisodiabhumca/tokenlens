//! Semantic compression — calls a local or hosted LLM to summarize big text.
//!
//! Backends:
//!   - `OllamaBackend`        (default; localhost Ollama)
//!   - `OpenAIBackend`        (OpenAI / OpenAI-compatible APIs incl. Together, Groq)
//!
//! Backends share a tiny disk-backed cache keyed by sha256(input + backend tag),
//! so identical payloads compress only once across sessions.

pub mod cache;

use anyhow::Result;
use serde_json::json;

pub trait SemanticBackend: Send + Sync {
    fn name(&self) -> &str;
    fn summarize(&self, text: &str, target_tokens: u64) -> Result<String>;
}

pub struct OllamaBackend {
    pub host: String,
    pub model: String,
}

impl Default for OllamaBackend {
    fn default() -> Self {
        Self {
            host: std::env::var("OLLAMA_HOST").unwrap_or_else(|_| "http://localhost:11434".into()),
            model: std::env::var("TOKENLENS_OLLAMA_MODEL").unwrap_or_else(|_| "llama3.2".into()),
        }
    }
}

impl SemanticBackend for OllamaBackend {
    fn name(&self) -> &str { "ollama" }
    fn summarize(&self, text: &str, target_tokens: u64) -> Result<String> {
        let key = cache::cache_key(self.name(), &self.model, text);
        if let Some(cached) = cache::get(&key) { return Ok(cached); }

        let prompt = format!(
            "Compress the following command output to roughly {target_tokens} tokens. \
             Preserve errors, file paths, and key findings. Drop boilerplate, banners, \
             and progress lines. Output only the compressed text.\n\n---\n{text}\n---"
        );

        let resp = ureq::post(&format!("{}/api/generate", self.host))
            .timeout(std::time::Duration::from_secs(60))
            .send_json(json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
            }))?;
        let v: serde_json::Value = resp.into_json()?;
        let out = v.get("response").and_then(|s| s.as_str()).unwrap_or("").to_string();
        cache::put(&key, &out);
        Ok(out)
    }
}

pub struct OpenAIBackend {
    pub base_url: String,
    pub api_key: String,
    pub model: String,
}

impl OpenAIBackend {
    pub fn from_env() -> Option<Self> {
        let api_key = std::env::var("OPENAI_API_KEY").ok()?;
        Some(Self {
            base_url: std::env::var("OPENAI_BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".into()),
            api_key,
            model: std::env::var("TOKENLENS_OPENAI_MODEL")
                .unwrap_or_else(|_| "gpt-4o-mini".into()),
        })
    }
}

impl SemanticBackend for OpenAIBackend {
    fn name(&self) -> &str { "openai" }
    fn summarize(&self, text: &str, target_tokens: u64) -> Result<String> {
        let key = cache::cache_key(self.name(), &self.model, text);
        if let Some(cached) = cache::get(&key) { return Ok(cached); }

        let resp = ureq::post(&format!("{}/chat/completions", self.base_url))
            .timeout(std::time::Duration::from_secs(60))
            .set("Authorization", &format!("Bearer {}", self.api_key))
            .send_json(json!({
                "model": self.model,
                "messages": [
                    {"role": "system",
                     "content": "You compress command output for AI coding assistants. Preserve errors and key facts; drop boilerplate."},
                    {"role": "user",
                     "content": format!("Target ~{target_tokens} tokens.\n\n{text}")},
                ],
            }))?;
        let v: serde_json::Value = resp.into_json()?;
        let out = v["choices"][0]["message"]["content"]
            .as_str().unwrap_or("").to_string();
        cache::put(&key, &out);
        Ok(out)
    }
}

/// Pick the default backend based on env: prefer Ollama if reachable, else OpenAI if keyed.
pub fn default_backend() -> Option<Box<dyn SemanticBackend>> {
    let ollama = OllamaBackend::default();
    // Cheap reachability check — don't fail the whole pipeline if Ollama is down.
    if ureq::get(&format!("{}/api/tags", ollama.host))
        .timeout(std::time::Duration::from_millis(300))
        .call()
        .is_ok()
    {
        return Some(Box::new(ollama));
    }
    OpenAIBackend::from_env().map(|b| Box::new(b) as Box<dyn SemanticBackend>)
}
