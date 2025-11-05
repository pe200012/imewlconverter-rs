//! Rime input method format export
//!
//! Format: `word\tcode\trank`
//! Example: `你好\tni hao\t1000`

use crate::export::WordLibraryExport;
use crate::{CodeType, Result, WordLibrary, WordLibraryList};

/// Operating system for line ending configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperatingSystem {
    Windows,
    MacOS,
    Linux,
}

impl OperatingSystem {
    pub fn line_ending(&self) -> &str {
        match self {
            OperatingSystem::Windows => "\r\n",
            OperatingSystem::MacOS => "\r",
            OperatingSystem::Linux => "\n",
        }
    }
}

/// Rime format exporter
pub struct RimeExport {
    code_type: CodeType,
    os: OperatingSystem,
}

impl RimeExport {
    pub fn new() -> Self {
        RimeExport {
            code_type: CodeType::Pinyin,
            os: OperatingSystem::Linux,
        }
    }

    pub fn with_code_type(code_type: CodeType) -> Self {
        RimeExport {
            code_type,
            os: OperatingSystem::Linux,
        }
    }

    pub fn with_os(mut self, os: OperatingSystem) -> Self {
        self.os = os;
        self
    }
}

impl Default for RimeExport {
    fn default() -> Self {
        Self::new()
    }
}

impl WordLibraryExport for RimeExport {
    fn export(&self, word_list: &WordLibraryList) -> Result<Vec<String>> {
        let mut lines = Vec::new();
        let line_ending = self.os.line_ending();

        for word in word_list {
            if let Ok(line) = self.export_line(word) {
                if !line.is_empty() {
                    lines.push(line);
                }
            }
        }

        Ok(vec![lines.join(line_ending)])
    }

    fn export_line(&self, word: &WordLibrary) -> Result<String> {
        let code_str = if self.code_type == CodeType::Pinyin {
            word.get_pinyin_string(" ")
        } else if let Some(code) = word.get_single_code() {
            code.to_string()
        } else {
            return Ok(String::new());
        };

        if code_str.is_empty() {
            return Ok(String::new());
        }

        Ok(format!("{}\t{}\t{}", word.word, code_str, word.rank))
    }

    fn code_type(&self) -> CodeType {
        self.code_type
    }

    fn format_name(&self) -> &str {
        "Rime"
    }

    fn encoding(&self) -> &'static str {
        "utf-8"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Code, WordLibrary};

    #[test]
    fn test_export_line_pinyin() {
        let exporter = RimeExport::new();

        let mut word = WordLibrary::new("你好".to_string());
        word.rank = 1000;
        word.code_type = CodeType::Pinyin;
        word.codes = Code::from_char_list(vec!["ni".to_string(), "hao".to_string()]);

        let line = exporter.export_line(&word).unwrap();
        assert_eq!(line, "你好\tni hao\t1000");
    }

    #[test]
    fn test_export_line_wubi() {
        let exporter = RimeExport::with_code_type(CodeType::Wubi);

        let mut word = WordLibrary::new("你好".to_string());
        word.rank = 1000;
        word.code_type = CodeType::Wubi;
        word.codes = Code::from_single("vqkb".to_string());

        let line = exporter.export_line(&word).unwrap();
        assert_eq!(line, "你好\tvqkb\t1000");
    }

    #[test]
    fn test_line_endings() {
        assert_eq!(OperatingSystem::Windows.line_ending(), "\r\n");
        assert_eq!(OperatingSystem::MacOS.line_ending(), "\r");
        assert_eq!(OperatingSystem::Linux.line_ending(), "\n");
    }
}
