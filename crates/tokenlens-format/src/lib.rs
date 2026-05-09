//! Token-efficient formatting trait. Promoted from RTK's `parser::formatter`.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormatMode {
    /// Summary only.
    Compact,
    /// Include details.
    Verbose,
    /// Symbols and abbreviations.
    Ultra,
}

impl FormatMode {
    pub fn from_verbosity(v: u8) -> Self {
        match v { 0 => Self::Compact, 1 => Self::Verbose, _ => Self::Ultra }
    }
}

pub trait TokenFormatter {
    fn format_compact(&self) -> String;
    fn format_verbose(&self) -> String { self.format_compact() }
    fn format_ultra(&self) -> String { self.format_compact() }
    fn format(&self, mode: FormatMode) -> String {
        match mode {
            FormatMode::Compact => self.format_compact(),
            FormatMode::Verbose => self.format_verbose(),
            FormatMode::Ultra => self.format_ultra(),
        }
    }
}
