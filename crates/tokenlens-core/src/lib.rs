//! TokenLens core library.
//!
//! Public API used by the `tokenlens` binary, `tokenlens-mcp`, and any other
//! crate that wants programmatic access to compression, rewriting, and
//! token-savings tracking.

pub mod budget;
pub mod cmds;
pub mod filter;
pub mod hook;
pub mod recorder;
pub mod registry;
pub mod rewrite;
pub mod semantic;
pub mod tokens;
pub mod tracking;

pub use filter::{compress_text, CompressionLevel, FilterOutcome};
pub use registry::{Rewrite, RewriteAction};
pub use tokens::approx_tokens;
pub use tracking::{Summary, Tracker};
