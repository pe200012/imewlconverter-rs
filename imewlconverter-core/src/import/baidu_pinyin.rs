/// Baidu Pinyin text format (.txt)
/// Format: word\tpinyin'\trank (for Chinese) or word\trank (for English)
/// Example: 你好\tni'hao'\t1000
use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{CodeType, Result, WordLibrary};

pub struct BaiduPinyinImport;

impl WordLibraryTextImport for BaiduPinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            return Ok(None);
        }

        let word = parts[0];
        let mut wl = WordLibrary::new(word.to_string());

        if parts.len() == 2 {
            // English word: word\trank
            wl.rank = parts[1].parse().unwrap_or(0);
            wl.code_type = CodeType::English;
        } else if parts.len() >= 3 {
            // Chinese word: word\tpinyin'\trank
            let pinyin_str = parts[1];
            let pinyin: Vec<String> = pinyin_str
                .split('\'')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            wl.code_type = CodeType::Pinyin;
            wl.rank = parts[2].parse().unwrap_or(0);
            wl.codes = crate::Code::from_char_list(pinyin);
        }

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "utf-16le" // Baidu uses UTF-16LE
    }
}

impl WordLibraryImport for BaiduPinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_chinese_word() {
        let importer = BaiduPinyinImport;

        let result = importer.import_line("你好\tni'hao'\t1000").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "你好");
        assert_eq!(wl.get_pinyin_string("'"), "ni'hao");
        assert_eq!(wl.rank, 1000);
        assert_eq!(wl.code_type, CodeType::Pinyin);
    }

    #[test]
    fn test_import_english_word() {
        let importer = BaiduPinyinImport;

        let result = importer.import_line("hello\t500").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "hello");
        assert_eq!(wl.rank, 500);
        assert_eq!(wl.code_type, CodeType::English);
    }

    #[test]
    fn test_import_line_complex() {
        let importer = BaiduPinyinImport;

        let result = importer
            .import_line("中华人民共和国\tzhong'hua'ren'min'gong'he'guo'\t5000")
            .unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "中华人民共和国");
        assert_eq!(wl.rank, 5000);
        assert_eq!(wl.codes.0.len(), 7);
    }
}
