//! `tokenlens fetch <url>` — fetch a URL and compress the body.

use crate::filter::{compress_text, CompressionLevel, FilterOutcome};
use anyhow::{Context, Result};

pub fn fetch_url(url: &str, level: CompressionLevel) -> Result<FilterOutcome> {
    let resp = ureq::get(url)
        .timeout(std::time::Duration::from_secs(20))
        .call()
        .with_context(|| format!("GET {url}"))?;
    let body = resp.into_string()?;
    Ok(compress_text(&body, Some(&format!("fetch {url}")), level))
}
