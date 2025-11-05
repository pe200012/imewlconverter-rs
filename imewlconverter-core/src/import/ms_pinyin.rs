//! Microsoft Pinyin format import
//!
//! Format: `code rank word` (space or tab separated)
//! Example: `ni'hao 1000 你好`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// Microsoft Pinyin format importer
pub struct MsPinyinImport;

impl MsPinyinImport {
    pub fn new() -> Self {
        MsPinyinImport
    }
}

impl Default for MsPinyinImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for MsPinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with("//") {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 3 {
            return Ok(None);
        }

        let code_str = parts[0];
        let rank = parts[1].parse::<i32>().unwrap_or(0);
        let word = parts[2];

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

impl WordLibraryImport for MsPinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = MsPinyinImport::new();
        let result = importer.import_line("ni'hao 1000 你好").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }

    #[test]
    fn test_import_line_tab_separated() {
        let importer = MsPinyinImport::new();
        let result = importer.import_line("zhong'guo\t500\t中国").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "中国");
        assert_eq!(word.rank, 500);
    }

    #[test]
    fn test_skip_comments() {
        let importer = MsPinyinImport::new();
        assert!(importer.import_line("# comment").unwrap().is_none());
        assert!(importer.import_line("// comment").unwrap().is_none());
        assert!(importer.import_line("").unwrap().is_none());
    }
}
