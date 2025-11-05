//! PinyinJiaJia format import
//!
//! Format: `word	code	rank`
//! Example: `你好	ni'hao	1000`
//! Similar to Sina but with slightly different encoding

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// PinyinJiaJia format importer
pub struct PinyinJiajiaImport;

impl PinyinJiajiaImport {
    pub fn new() -> Self {
        PinyinJiajiaImport
    }
}

impl Default for PinyinJiajiaImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for PinyinJiajiaImport {
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
        "gbk"
    }
}

impl WordLibraryImport for PinyinJiajiaImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = PinyinJiajiaImport::new();
        let result = importer.import_line("你好\tni'hao\t1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }
}
