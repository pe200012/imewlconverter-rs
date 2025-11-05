//! QQ Wubi format import
//!
//! Format: `code word rank`
//! Example: `vqkb 你好 1000`

use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{Code, CodeType, Result, WordLibrary};

/// QQ Wubi format importer
pub struct QQWubiImport;

impl QQWubiImport {
    pub fn new() -> Self {
        QQWubiImport
    }
}

impl Default for QQWubiImport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryTextImport for QQWubiImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            return Ok(None);
        }

        let code_str = parts[0];
        let word = parts[1];
        let rank = if parts.len() >= 3 {
            parts[2].parse::<i32>().unwrap_or(0)
        } else {
            0
        };

        let mut wl = WordLibrary::new(word.to_string());
        wl.rank = rank;
        wl.code_type = CodeType::Wubi;
        wl.codes = Code::from_single(code_str.to_string());

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for QQWubiImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = QQWubiImport::new();
        let result = importer.import_line("vqkb 你好 1000").unwrap();

        assert!(result.is_some());
        let word = result.unwrap();
        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_single_code(), Some("vqkb"));
    }
}
