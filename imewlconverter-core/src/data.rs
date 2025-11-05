//! Core data structures for the IME converter
//!
//! This module defines the fundamental data types used throughout the converter:
//! - `WordLibrary`: Represents a dictionary entry (word + encoding + frequency)
//! - `Code`: Flexible encoding representation supporting various encoding schemes
//! - `CodeType`: Enumeration of supported encoding types

use serde::{Deserialize, Serialize};
use std::fmt;

/// Type of encoding used for the dictionary entry
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CodeType {
    /// User-defined phrase
    UserDefinePhrase,
    /// Wubi 86
    Wubi,
    /// Wubi 98
    Wubi98,
    /// Wubi New Age
    WubiNewAge,
    /// Zhengma
    Zhengma,
    /// Cangjie
    Cangjie,
    /// Unknown encoding
    Unknown,
    /// User-defined encoding
    UserDefine,
    /// Pinyin
    Pinyin,
    /// Yongma
    Yong,
    /// Qingsong Erbi
    QingsongErbi,
    /// Chaoqiang Erbi 30-key
    ChaoqiangErbi,
    /// Chaoqing Yinxin (Erbi)
    ChaoqingYinxin,
    /// English
    English,
    /// Internal code
    InnerCode,
    /// Xiandai Erbi
    XiandaiErbi,
    /// Zhuyin (Bopomofo)
    Zhuyin,
    /// Terra Pinyin
    TerraPinyin,
    /// Chaoyin
    Chaoyin,
    /// No encoding
    NoCode,
}

impl fmt::Display for CodeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeType::Pinyin => write!(f, "Pinyin"),
            CodeType::Wubi => write!(f, "Wubi86"),
            CodeType::Wubi98 => write!(f, "Wubi98"),
            CodeType::English => write!(f, "English"),
            _ => write!(f, "{:?}", self),
        }
    }
}

/// Represents the encoding of a word or phrase
///
/// The structure is `Vec<Vec<String>>` where:
/// - For one-char-one-code: `codes[n][0]` = nth character's code
/// - For one-char-multi-code: `codes[n]` = nth character's possible codes
/// - For one-word-one-code: `codes[0][0]` = the word's single code
/// - For one-word-multi-code: `codes[0]` = the word's possible codes
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Code(pub Vec<Vec<String>>);

impl Code {
    /// Create a new empty code
    pub fn new() -> Self {
        Code(Vec::new())
    }

    /// Create a code from a single string (one-word-one-code)
    pub fn from_single(code: String) -> Self {
        Code(vec![vec![code]])
    }

    /// Create a code from multiple strings (one-word-multi-code)
    pub fn from_multiple(codes: Vec<String>) -> Self {
        Code(vec![codes])
    }

    /// Create a code from character codes (one-char-one-code or one-char-multi-code)
    pub fn from_chars(char_codes: Vec<Vec<String>>) -> Self {
        Code(char_codes)
    }

    /// Create from a list of codes where each code is for one character
    pub fn from_char_list(codes: Vec<String>) -> Self {
        Code(codes.into_iter().map(|c| vec![c]).collect())
    }

    /// Get the first code (most common usage)
    pub fn get_single_code(&self) -> Option<&str> {
        self.0.first()?.first().map(|s| s.as_str())
    }

    /// Get the default code (first code of each character)
    pub fn get_default_codes(&self) -> Vec<&str> {
        self.0
            .iter()
            .filter_map(|codes| codes.first().map(|s| s.as_str()))
            .collect()
    }

    /// Check if the code is empty
    pub fn is_empty(&self) -> bool {
        self.0.is_empty() || self.0.iter().all(|codes| codes.is_empty())
    }

    /// Get number of characters/parts
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Convert to string with separator
    pub fn to_string_with_separator(&self, separator: &str) -> String {
        self.get_default_codes().join(separator)
    }

    /// Perform Cartesian product for polyphonic characters
    /// Returns all possible combinations
    pub fn cartesian_product(&self) -> Vec<String> {
        if self.0.is_empty() {
            return vec![];
        }

        let mut result = vec![String::new()];

        for codes in &self.0 {
            if codes.is_empty() {
                continue;
            }
            let mut new_result = Vec::new();
            for existing in &result {
                for code in codes {
                    let mut new_str = existing.clone();
                    new_str.push_str(code);
                    new_result.push(new_str);
                }
            }
            result = new_result;
        }

        result
    }

    /// Cartesian product with separator
    pub fn cartesian_product_with_separator(&self, separator: &str) -> Vec<String> {
        if self.0.is_empty() {
            return vec![];
        }

        let mut result = vec![String::new()];

        for (i, codes) in self.0.iter().enumerate() {
            if codes.is_empty() {
                continue;
            }
            let mut new_result = Vec::new();
            for existing in &result {
                for code in codes {
                    let mut new_str = existing.clone();
                    if i > 0 && !new_str.is_empty() {
                        new_str.push_str(separator);
                    }
                    new_str.push_str(code);
                    new_result.push(new_str);
                }
            }
            result = new_result;
        }

        result
    }
}

impl From<Vec<Vec<String>>> for Code {
    fn from(codes: Vec<Vec<String>>) -> Self {
        Code(codes)
    }
}

impl From<Vec<String>> for Code {
    fn from(codes: Vec<String>) -> Self {
        Code::from_char_list(codes)
    }
}

/// Represents a dictionary entry
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WordLibrary {
    /// The word or phrase (汉字)
    pub word: String,

    /// Word frequency/rank (词频)
    pub rank: i32,

    /// Type of encoding
    pub code_type: CodeType,

    /// The encoding(s) for this word
    pub codes: Code,

    /// Whether this is an English word
    pub is_english: bool,
}

impl WordLibrary {
    /// Create a new WordLibrary entry
    pub fn new(word: String) -> Self {
        WordLibrary {
            word,
            rank: 0,
            code_type: CodeType::Pinyin,
            codes: Code::new(),
            is_english: false,
        }
    }

    /// Create with rank
    pub fn with_rank(word: String, rank: i32) -> Self {
        WordLibrary {
            word,
            rank,
            code_type: CodeType::Pinyin,
            codes: Code::new(),
            is_english: false,
        }
    }

    /// Set the code for this word
    pub fn set_code(&mut self, code_type: CodeType, codes: Code) {
        self.code_type = code_type;
        self.codes = codes;
    }

    /// Get pinyin string with separator
    pub fn get_pinyin_string(&self, separator: &str) -> String {
        if self.code_type == CodeType::Pinyin || self.code_type == CodeType::TerraPinyin {
            self.codes.to_string_with_separator(separator)
        } else {
            String::new()
        }
    }

    /// Get the single code (for formats like Wubi)
    pub fn get_single_code(&self) -> Option<&str> {
        self.codes.get_single_code()
    }

    /// Check if the word has valid codes
    pub fn has_code(&self) -> bool {
        !self.codes.is_empty()
    }

    /// Get word length (number of characters)
    pub fn len(&self) -> usize {
        self.word.chars().count()
    }

    /// Check if word is empty
    pub fn is_empty(&self) -> bool {
        self.word.is_empty()
    }
}

impl fmt::Display for WordLibrary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "WordLibrary {{ word: {}, codes: {:?}, rank: {} }}",
            self.word,
            self.codes.get_default_codes(),
            self.rank
        )
    }
}

/// A list of WordLibrary entries
pub type WordLibraryList = Vec<WordLibrary>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code_single() {
        let code = Code::from_single("test".to_string());
        assert_eq!(code.get_single_code(), Some("test"));
    }

    #[test]
    fn test_code_cartesian() {
        let code = Code(vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ]);
        let result = code.cartesian_product();
        assert_eq!(result, vec!["ac", "ad", "bc", "bd"]);
    }

    #[test]
    fn test_code_cartesian_with_separator() {
        let code = Code(vec![
            vec!["ni".to_string(), "nv".to_string()],
            vec!["hao".to_string()],
        ]);
        let result = code.cartesian_product_with_separator("'");
        assert_eq!(result, vec!["ni'hao", "nv'hao"]);
    }

    #[test]
    fn test_word_library() {
        let mut word = WordLibrary::new("你好".to_string());
        word.rank = 1000;
        word.set_code(
            CodeType::Pinyin,
            Code::from_char_list(vec!["ni".to_string(), "hao".to_string()]),
        );

        assert_eq!(word.word, "你好");
        assert_eq!(word.rank, 1000);
        assert_eq!(word.get_pinyin_string("'"), "ni'hao");
    }
}
