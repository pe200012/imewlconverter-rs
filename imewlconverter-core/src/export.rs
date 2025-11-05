//! Export traits and implementations for various IME formats

use crate::{CodeType, Result, WordLibrary, WordLibraryList};

pub mod qq_pinyin;
pub mod rime;

/// Trait for exporting word libraries to files
pub trait WordLibraryExport {
    /// Export a word library list to string(s)
    /// Returns a vector because some formats split into multiple files
    fn export(&self, word_list: &WordLibraryList) -> Result<Vec<String>>;

    /// Export a single word to a line
    fn export_line(&self, word: &WordLibrary) -> Result<String>;

    /// Get the code type this exporter expects
    fn code_type(&self) -> CodeType;

    /// Get the format name
    fn format_name(&self) -> &str;

    /// Get the text encoding (e.g., UTF-8, GBK)
    fn encoding(&self) -> &'static str {
        "utf-8"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_export_trait_exists() {
        // Just test that the trait compiles
    }
}
