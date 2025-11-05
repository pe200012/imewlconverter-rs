//! Chinese Pyim format import
//!
//! Format: `code word1 word2 word3`
//! Example: `ni'hao 你好 尼好`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// Chinese Pyim format importer
pub struct ChinesePyimImport;

impl ChinesePyimImport {
    pub fn new() -> Self {
        ChinesePyimImport
    }
}

impl Default for ChinesePyimImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for ChinesePyimImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(None);
        }

        let code_str = parts[0];

        // Each word in the line shares the same code
        // For simplicity, we'll return None here and implement full parsing
        // in the import_from_file method
        // For now, just take the first word
        let word = parts[1];

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = 0; // Default rank
        wl.code_type = CodeType::Pinyin;

        // Parse pinyin code (apostrophe separated)
        let codes: Vec<String> = code_str.split('\'').map(|s| s.to_string()).collect();
        wl.codes = Code::from_char_list(codes);

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for ChinesePyimImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        let content = read_file_with_encoding_str(path, self.default_encoding())?;
        let mut result = Vec::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let code_str = parts[0];
            let codes: Vec<String> = code_str.split('\'').map(|s| s.to_string()).collect();

            // Process all words in this line
            for (i, word) in parts[1..].iter().enumerate() {
                let mut wl = WordLibrary::new(word.to_string());
                wl.rank = (parts.len() - i) as i32; // Higher rank for earlier words
                wl.code_type = CodeType::Pinyin;
                wl.codes = Code::from_char_list(codes.clone());
                result.push(wl);
            }
        }

        Ok(result)
    }
}

// Helper function from import.rs
use crate::import::read_file_with_encoding_str;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = ChinesePyimImport::new();
        let result = importer.import_line("ni'hao 你好").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }
}
