//! Tiny on-disk cache for semantic compressions, keyed by sha256.

use sha2::{Digest, Sha256};
use std::path::PathBuf;

pub fn cache_key(backend: &str, model: &str, input: &str) -> String {
    let mut h = Sha256::new();
    h.update(backend.as_bytes());
    h.update(b"\0");
    h.update(model.as_bytes());
    h.update(b"\0");
    h.update(input.as_bytes());
    format!("{:x}", h.finalize())
}

fn cache_dir() -> Option<PathBuf> {
    let base = dirs::cache_dir()?;
    let p = base.join("tokenlens").join("semantic");
    std::fs::create_dir_all(&p).ok()?;
    Some(p)
}

pub fn get(key: &str) -> Option<String> {
    let path = cache_dir()?.join(key);
    std::fs::read_to_string(path).ok()
}

pub fn put(key: &str, value: &str) {
    if let Some(dir) = cache_dir() {
        let _ = std::fs::write(dir.join(key), value);
    }
}
