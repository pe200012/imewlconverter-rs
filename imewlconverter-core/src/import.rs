//! Import traits and implementations for various IME formats

use crate::{Result, WordLibrary};

// Import implementations
pub mod baidu_pinyin;
pub mod chinese_pyim;
pub mod fit_input;
pub mod google_pinyin;
pub mod libpinyin;
pub mod ms_pinyin;
pub mod pinyin_jiajia;
pub mod qq_pinyin;
pub mod qq_wubi;
pub mod rime;
pub mod sina_pinyin;
pub mod sogou_pinyin;
pub mod sogou_scel;
pub mod wubi;
pub mod ziguang_pinyin;

// Re-exports
pub use baidu_pinyin::BaiduPinyinImport;
pub use chinese_pyim::ChinesePyimImport;
pub use fit_input::FitInputImport;
pub use google_pinyin::GooglePinyinImport;
pub use libpinyin::LibpinyinImport;
pub use ms_pinyin::MsPinyinImport;
pub use pinyin_jiajia::PinyinJiajiaImport;
pub use qq_pinyin::QQPinyinImport;
pub use qq_wubi::QQWubiImport;
pub use rime::RimeImport;
pub use sina_pinyin::SinaPinyinImport;
pub use sogou_pinyin::SogouPinyinImport;
pub use sogou_scel::SogouScelImport;
pub use wubi::{Wubi86Import, Wubi98Import, WubiNewAgeImport};
pub use ziguang_pinyin::ZiguangPinyinImport;

/// Trait for importing word libraries from files
pub trait WordLibraryImport {
    /// Import from a file path, returns a vector of WordLibrary entries
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>>;
}

/// Trait for text-based import formats that can process line-by-line
pub trait WordLibraryTextImport {
    /// Import a single line, returns Some(WordLibrary) if valid, None if should skip
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>>;

    /// Get the default text encoding (e.g., "utf-8", "gbk", "utf-16le")
    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }

    /// Read and parse entire file with encoding
    fn read_file_with_encoding(&self, path: &str, encoding_name: &str) -> Result<Vec<WordLibrary>> {
        let content = read_file_with_encoding_str(path, encoding_name)?;
        let mut result = Vec::new();

        for line in content.lines() {
            if let Some(wl) = self.import_line(line)? {
                result.push(wl);
            }
        }

        Ok(result)
    }
}

/// Helper function to read file with encoding detection
pub fn read_file_with_encoding_str(path: &str, encoding_name: &str) -> Result<String> {
    use encoding_rs::Encoding;
    use std::fs;

    let bytes = fs::read(path)?;

    // Get encoding
    let encoding = if encoding_name == "utf-8" {
        encoding_rs::UTF_8
    } else if encoding_name == "gbk" {
        encoding_rs::GBK
    } else if encoding_name == "big5" {
        encoding_rs::BIG5
    } else if encoding_name == "utf-16le" {
        encoding_rs::UTF_16LE
    } else if encoding_name == "utf-16be" {
        encoding_rs::UTF_16BE
    } else {
        Encoding::for_label(encoding_name.as_bytes()).unwrap_or(encoding_rs::UTF_8)
    };

    let (result, _, had_errors) = encoding.decode(&bytes);
    if had_errors {
        eprintln!("Warning: encoding errors detected when reading file");
    }

    Ok(result.into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_file_utf8() {
        // This would require actual test files
        // Just test that the function exists and compiles
    }
}
