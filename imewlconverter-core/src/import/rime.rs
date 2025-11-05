//! Rime input method format import
//!
//! Format: `word\tcode\trank`
//! Example: `你好\tni hao\t1000`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// Rime format importer
pub struct RimeImport {
    code_type: CodeType,
}

impl RimeImport {
    pub fn new() -> Self {
        RimeImport {
            code_type: CodeType::Pinyin,
        }
    }

    pub fn with_code_type(code_type: CodeType) -> Self {
        RimeImport { code_type }
    }
}

impl Default for RimeImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for RimeImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split('\t').collect();

        if parts.len() < 2 {
            return Ok(None);
        }

        let word = parts[0];
        let code = parts[1];
        let rank = if parts.len() >= 3 {
            parts[2].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = rank;
        wl.code_type = self.code_type;

        // Parse code based on type
        if self.code_type == CodeType::Pinyin {
            // Split by space for pinyin
            let codes: Vec<String> = code.split_whitespace().map(|s| s.to_string()).collect();
            wl.codes = Code::from_char_list(codes);
        } else {
            // For other code types, treat as single code
            wl.codes = Code::from_single(code.to_string());
        }

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for RimeImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line_pinyin() {
        let importer = RimeImport::new();
        let result = importer.import_line("你好\tni hao\t1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string(" "), "ni hao");
    }

    #[test]
    fn test_import_line_wubi() {
        let importer = RimeImport::with_code_type(CodeType::Wubi);
        let result = importer.import_line("你好\tvqkb\t1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_single_code(), Some("vqkb"));
    }

    #[test]
    fn test_import_line_no_rank() {
        let importer = RimeImport::new();
        let result = importer.import_line("你好\tni hao").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.rank, 0);
    }
}
