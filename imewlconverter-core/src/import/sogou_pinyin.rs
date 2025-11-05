/// Sogou Pinyin text format (.txt)
/// Format: 'pinyin word
/// Example: 'ni'hao 你好
use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{CodeType, Result, WordLibrary};

pub struct SogouPinyinImport;

impl WordLibraryTextImport for SogouPinyinImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        let line = line.trim();

        // Lines starting with ' are dictionary entries
        if !line.starts_with('\'') {
            return Ok(None);
        }

        // Parse: 'pinyin word
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Ok(None);
        }

        let pinyin_str = parts[0];
        let word = parts[1];

        // Split pinyin by apostrophes
        let pinyin: Vec<String> = pinyin_str
            .split('\'')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();

        let mut wl = WordLibrary::new(word.to_string());
        wl.code_type = CodeType::Pinyin;
        wl.rank = 1;
        wl.codes = crate::Code::from_char_list(pinyin);

        Ok(Some(wl))
    }

    fn default_encoding(&self) -> &'static str {
        "gbk"
    }
}

impl WordLibraryImport for SogouPinyinImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_import_line() {
        let importer = SogouPinyinImport;

        let result = importer.import_line("'ni'hao 你好").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "你好");
        assert_eq!(wl.get_pinyin_string("'"), "ni'hao");
        assert_eq!(wl.rank, 1);
    }

    #[test]
    fn test_import_line_complex() {
        let importer = SogouPinyinImport;

        let result = importer
            .import_line("'zhong'hua'ren'min'gong'he'guo 中华人民共和国")
            .unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "中华人民共和国");
        assert_eq!(wl.codes.0.len(), 7); // 7 characters
    }

    #[test]
    fn test_import_line_skip_non_dict() {
        let importer = SogouPinyinImport;

        let result = importer.import_line("# Comment line").unwrap();
        assert!(result.is_none());

        let result = importer.import_line("").unwrap();
        assert!(result.is_none());
    }
}
