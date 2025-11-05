//! FitInput format import
//!
//! Format: `word,code,rank`
//! Example: `你好,ni'hao,1000`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// FitInput format importer
pub struct FitInputImport;

impl FitInputImport {
    pub fn new() -> Self {
        FitInputImport
    }
}

impl Default for FitInputImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for FitInputImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() < 2 {
            return Ok(None);
        }

        let word = parts[0];
        let code_str = parts[1];
        let rank = if parts.len() >= 3 {
            parts[2].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = rank;
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

impl WordLibraryImport for FitInputImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = FitInputImport::new();
        let result = importer.import_line("你好,ni'hao,1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }
}
