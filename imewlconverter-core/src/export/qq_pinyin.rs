//! QQ Pinyin text format export
//!
//! Format: `pinyin word rank`
//! Example: `ni'hao 你好 1000`

use crate::export::WordLibraryExport;
use crate::{CodeType, Error, Result, WordLibrary, WordLibraryList};

/// QQ Pinyin text format exporter
pub struct QQPinyinExport;

impl QQPinyinExport {
    pub fn new() -> Self {
        QQPinyinExport
    }
}

impl Default for QQPinyinExport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryExport for QQPinyinExport {
    fn export(&self, word_list: &WordLibraryList) -> Result<Vec<String>> {
        if word_list.is_empty() {
            return Ok(vec![String::new()]);
        }

        let mut lines = Vec::new();

        // Export all but the last word normally
        for word in &word_list[..word_list.len() - 1] {
            if let Ok(line) = self.export_line(word) {
                if !line.is_empty() {
                    lines.push(line);
                }
            }
        }

        // Last line has special format: includes duplicate pinyin and rank
        if let Some(last) = word_list.last() {
            let line = self.export_line(last)?;
            if !line.is_empty() {
                let pinyin = last.get_pinyin_string("'");
                lines.push(format!("{}, {} {}", line, pinyin, last.rank));
            }
        }

        Ok(vec![lines.join("\r\n")])
    }

    fn export_line(&self, word: &WordLibrary) -> Result<String> {
        if word.code_type != CodeType::Pinyin {
            return Err(Error::InvalidFormat(
                "QQ Pinyin export requires Pinyin encoding".to_string(),
            ));
        }

        let pinyin = word.get_pinyin_string("'");
        if pinyin.is_empty() {
            return Ok(String::new());
        }

        Ok(format!("{} {} {}", pinyin, word.word, word.rank))
    }

    fn code_type(&self) -> CodeType {
        CodeType::Pinyin
    }

    fn format_name(&self) -> &str {
        "QQ Pinyin"
    }

    fn encoding(&self) -> &'static str {
        "utf-16le"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Code, WordLibrary};

    #[test]
    fn test_export_line() {
        let exporter = QQPinyinExport::new();

        let mut word = WordLibrary::new("你好".to_string());
        word.rank = 1000;
        word.code_type = CodeType::Pinyin;
        word.codes = Code::from_char_list(vec!["ni".to_string(), "hao".to_string()]);

        let line = exporter.export_line(&word).unwrap();
        assert_eq!(line, "ni'hao 你好 1000");
    }

    #[test]
    fn test_export_list() {
        let exporter = QQPinyinExport::new();

        let mut word1 = WordLibrary::new("你好".to_string());
        word1.rank = 1000;
        word1.code_type = CodeType::Pinyin;
        word1.codes = Code::from_char_list(vec!["ni".to_string(), "hao".to_string()]);

        let mut word2 = WordLibrary::new("世界".to_string());
        word2.rank = 500;
        word2.code_type = CodeType::Pinyin;
        word2.codes = Code::from_char_list(vec!["shi".to_string(), "jie".to_string()]);

        let result = exporter.export(&vec![word1, word2]).unwrap();
        assert_eq!(result.len(), 1);
        assert!(result[0].contains("ni'hao 你好 1000"));
        assert!(result[0].contains("shi'jie 世界 500, shi'jie 500"));
    }
}
