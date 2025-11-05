//! ZiGuang Pinyin (Purple Light) format import
//!
//! Format: `code=word rank` or `code word rank`
//! Example: `ni'hao=你好 1000` or `ni'hao 你好 1000`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// ZiGuang Pinyin format importer
pub struct ZiguangPinyinImport;

impl ZiguangPinyinImport {
    pub fn new() -> Self {
        ZiguangPinyinImport
    }
}

impl Default for ZiguangPinyinImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for ZiguangPinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        // Handle both formats: "code=word rank" and "code word rank"
        let (code_str, word, rank) = if line.contains('=') {
            // Format: code=word rank
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 1 {
                return Ok(None);
            }

            let code_word: Vec<&str> = parts[0].split('=').collect();
            if code_word.len() != 2 {
                return Ok(None);
            }

            let rank = if parts.len() > 1 {
                parts[1].parse::<i32>().unwrap_or(0)
            } else {
                0
            };

            (code_word[0], code_word[1], rank)
        } else {
            // Format: code word rank
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                return Ok(None);
            }

            let rank = if parts.len() > 2 {
                parts[2].parse::<i32>().unwrap_or(0)
            } else {
                0
            };

            (parts[0], parts[1], rank)
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
        "gbk"
    }
}

impl WordLibraryImport for ZiguangPinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line_equals_format() {
        let importer = ZiguangPinyinImport::new();
        let result = importer.import_line("ni'hao=你好 1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }

    #[test]
    fn test_import_line_space_format() {
        let importer = ZiguangPinyinImport::new();
        let result = importer.import_line("zhong'guo 中国 500").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "中国");
        assert_eq!(word.rank, 500);
    }

    #[test]
    fn test_import_line_no_rank() {
        let importer = ZiguangPinyinImport::new();
        let result = importer.import_line("ni'hao=你好").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.rank, 0);
    }
}
