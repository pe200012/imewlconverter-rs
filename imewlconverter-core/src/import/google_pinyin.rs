/// Google Pinyin text format (.txt)
/// Format: word\trank\tpinyin1 pinyin2 pinyin3
/// Example: 你好\t1000\tni hao
use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{CodeType, Result, WordLibrary};

pub struct GooglePinyinImport;

impl WordLibraryTextImport for GooglePinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();
        if line.is_empty() {
            return Ok(None);
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 3 {
            return Ok(None);
        }

        let word = parts[0];
        let rank: i32 = parts[1].parse().unwrap_or(0);
        let pinyin_str = parts[2];

        // Split pinyin by spaces
        let pinyin: Vec<String> = pinyin_str
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let mut wl = WordLibrary::new(word.to_string());
        wl.code_type = CodeType::Pinyin;
        wl.rank = rank;
        wl.codes = crate::Code::from_char_list(pinyin);

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "gbk"
    }
}

impl WordLibraryImport for GooglePinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = GooglePinyinImport;

        let result = importer.import_line("你好\t1000\tni hao").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "你好");
        assert_eq!(wl.get_pinyin_string(" "), "ni hao");
        assert_eq!(wl.rank, 1000);
    }

    #[test]
    fn test_import_line_complex() {
        let importer = GooglePinyinImport;

        let result = importer
            .import_line("中华人民共和国\t5000\tzhong hua ren min gong he guo")
            .unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "中华人民共和国");
        assert_eq!(wl.rank, 5000);
        assert_eq!(wl.codes.0.len(), 7);
    }

    #[test]
    fn test_empty_line() {
        let importer = GooglePinyinImport;

        let result = importer.import_line("").unwrap();
        assert!(result.is_none());
    }
}
