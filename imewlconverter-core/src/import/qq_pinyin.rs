//! QQ Pinyin text format import
//!
//! Format: `pinyin word rank`
//! Example: `ni'hao 你好 1000`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// QQ Pinyin text format importer
pub struct QQPinyinImport;

impl QQPinyinImport {
    pub fn new() -> Self {
        QQPinyinImport
    }
}

impl Default for QQPinyinImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for QQPinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        // Handle lines with comma (take first part only)
        let line = if let Some(comma_pos) = line.find(',') {
            &line[..comma_pos]
        } else {
            line
        };

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(None);
        }

        let pinyin = parts[0];
        let word = parts[1];
        let rank = if parts.len() >= 3 {
            parts[2].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        // Parse pinyin - split by apostrophe
        let pinyin_parts: Vec<String> = pinyin
            .split('\'')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = rank;
        wl.code_type = CodeType::Pinyin;
        wl.codes = Code::from_char_list(pinyin_parts);

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-16le"
    }
}

impl WordLibraryImport for QQPinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line_simple() {
        let importer = QQPinyinImport::new();
        let result = importer.import_line("ni'hao 你好 1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }

    #[test]
    fn test_import_line_with_comma() {
        let importer = QQPinyinImport::new();
        let result = importer
            .import_line("ni'hao 你好 1000, additional")
            .unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
    }

    #[test]
    fn test_import_line_no_rank() {
        let importer = QQPinyinImport::new();
        let result = importer.import_line("ni'hao 你好").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 0);
    }
}
