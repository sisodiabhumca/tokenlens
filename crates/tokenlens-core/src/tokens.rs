//! Token estimation. Uses the rough rule "1 token ≈ 4 chars" for English/code.
//! Replace with a tokenizer per provider when accuracy matters.

#[inline]
pub fn approx_tokens(text: &str) -> u64 {
    if text.is_empty() { 0 } else { ((text.len() as u64) + 3) / 4 }
}
