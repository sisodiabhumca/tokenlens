//! Per-tool command handlers. Each handler runs the underlying CLI and
//! pipes its output through the TokenLens compression engine before printing.

pub mod fetch;
pub mod read;
pub mod runner;

pub use runner::{ProxyResult, run_proxied};
