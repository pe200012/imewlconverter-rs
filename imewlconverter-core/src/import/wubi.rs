/// Wubi input method formats (86/98/NewAge)
/// Format: word\tcode1 code2 code3\trank
/// Example: 你好\tni hao\t1000
use crate::import::{WordLibraryImport, WordLibraryTextImport};
use crate::{CodeType, Result, WordLibrary};

/// Wubi 86 format importer
pub struct Wubi86Import;

impl WordLibraryTextImport for Wubi86Import {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        parse_wubi_line(line, CodeType::Wubi)
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for Wubi86Import {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

/// Wubi 98 format importer
pub struct Wubi98Import;

impl WordLibraryTextImport for Wubi98Import {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        parse_wubi_line(line, CodeType::Wubi98)
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for Wubi98Import {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

/// Wubi New Age format importer
pub struct WubiNewAgeImport;

impl WordLibraryTextImport for WubiNewAgeImport {
    fn import_line(&self, line: &str) -> Result<Option<WordLibrary>> {
        parse_wubi_line(line, CodeType::WubiNewAge)
    }

    fn default_encoding(&self) -> &'static str {
        "utf-8"
    }
}

impl WordLibraryImport for WubiNewAgeImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        self.read_file_with_encoding(path, self.default_encoding())
    }
}

/// Common parsing logic for Wubi formats
/// Format can be: word\tcode or word code
fn parse_wubi_line(line: &str, code_type: CodeType) -> Result<Option<WordLibrary>> {
    let line = line.trim();
    if line.is_empty() || line.starts_with('#') {
        return Ok(None);
    }

    // Try tab-separated first
    let parts: Vec<&str> = if line.contains('\t') {
        line.split('\t').collect()
    } else {
        line.split_whitespace().collect()
    };

    if parts.len() < 2 {
        return Ok(None);
    }

    let word = parts[0];
    let code_str = parts[1];

    // Wubi codes can be space-separated or continuous
    let codes: Vec<String> = if code_str.contains(' ') {
        code_str.split_whitespace().map(|s| s.to_string()).collect()
    } else {
        // For single character, it's one code; for words, split by character length
        vec![code_str.to_string()]
    };

    let rank = if parts.len() >= 3 {
        parts[2].parse().unwrap_or(0)
    } else {
        0
    };

    let mut wl = WordLibrary::new(word.to_string());
    wl.code_type = code_type;
    wl.rank = rank;
    wl.codes = crate::Code::from_char_list(codes);

    Ok(Some(wl))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wubi86_import() {
        let importer = Wubi86Import;

        let result = importer.import_line("你\twq").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "你");
        assert_eq!(wl.code_type, CodeType::Wubi);
    }

    #[test]
    fn test_wubi98_import() {
        let importer = Wubi98Import;

        let result = importer.import_line("好\tvb").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "好");
        assert_eq!(wl.code_type, CodeType::Wubi98);
    }

    #[test]
    fn test_wubi_with_rank() {
        let importer = Wubi86Import;

        let result = importer.import_line("你好\twqvb\t1000").unwrap();
        assert!(result.is_some());

        let wl = result.unwrap();
        assert_eq!(wl.word, "你好");
        assert_eq!(wl.rank, 1000);
    }
}
