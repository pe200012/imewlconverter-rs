//! libpinyin format import
//!
//! Format: `word rank code1 code2...`
//! Example: `你好 1000 ni hao` (space-separated pinyin)

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// libpinyin format importer
pub struct LibpinyinImport;

impl LibpinyinImport {
    pub fn new() -> Self {
        LibpinyinImport
    }
}

impl Default for LibpinyinImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for LibpinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            return Ok(None);
        }

        let word = parts[0];
        let rank = parts[1].parse::<i32>().unwrap_or(0);
        let codes: Vec<String> = parts[2..].iter().map(|s| s.to_string()).collect();

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = rank;
        wl.code_type = CodeType::Pinyin;
        wl.codes = Code::from_char_list(codes);

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for LibpinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = LibpinyinImport::new();
        let result = importer.import_line("你好 1000 ni hao").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string(" "), "ni hao");
    }

    #[test]
    fn test_import_line_multi_char() {
        let importer = LibpinyinImport::new();
        let result = importer
            .import_line("中华人民共和国 500 zhong hua ren min gong he guo")
            .unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "中华人民共和国");
        assert_eq!(word.rank, 500);
    }
}
