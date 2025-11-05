/// Resource loading for embedded dictionary files
use crate::data::CodeType;
use crate::error::Error;
use std::collections::HashMap;

/// Character encoding information from ChineseCode.txt
#[derive(Debug, Clone)]
pub struct ChineseCode {
    pub unicode: String,       // U+4E00
    pub character: char,       // 一
    pub wubi86: Vec<String>,   // ggll
    pub wubi98: Vec<String>,   // ggll
    pub wubi_new: Vec<String>, // ggll
    pub pinyin: Vec<String>,   // yi1
    pub frequency: f64,        // 37283.98
}

impl ChineseCode {
    /// Get codes by type
    pub fn get_codes(&self, code_type: &CodeType) -> Vec<String> {
        match code_type {
            CodeType::Pinyin => self.pinyin.clone(),
            CodeType::Wubi => self.wubi86.clone(),
            CodeType::Wubi98 => self.wubi98.clone(),
            CodeType::WubiNewAge => self.wubi_new.clone(),
            _ => vec![],
        }
    }
}

/// Multi-tone word pronunciation from WordPinyin.txt
#[derive(Debug, Clone)]
pub struct WordPinyin {
    pub word: String,
    pub pinyin: String, // with apostrophes like 'jiao'gai
}

/// Resource manager for all embedded dictionaries
pub struct ResourceManager {
    chinese_code: HashMap<char, ChineseCode>,
    word_pinyin: HashMap<String, WordPinyin>,
    zhengma: HashMap<char, Vec<String>>,
    cangjie: HashMap<char, Vec<String>>,
    zhuyin: HashMap<char, Vec<String>>,
}

impl ResourceManager {
    /// Load all resources
    pub fn new() -> Result<Self, Error> {
        let chinese_code = Self::load_chinese_code()?;
        let word_pinyin = Self::load_word_pinyin()?;
        let zhengma = Self::load_simple_dict(include_str!("../resources/Zhengma.txt"))?;
        let cangjie = Self::load_simple_dict(include_str!("../resources/Cangjie5.txt"))?;
        let zhuyin = Self::load_simple_dict(include_str!("../resources/Zhuyin.txt"))?;

        Ok(Self {
            chinese_code,
            word_pinyin,
            zhengma,
            cangjie,
            zhuyin,
        })
    }

    /// Get character codes by type
    pub fn get_char_codes(&self, ch: char, code_type: &CodeType) -> Option<Vec<String>> {
        match code_type {
            CodeType::Pinyin | CodeType::Wubi | CodeType::Wubi98 | CodeType::WubiNewAge => self
                .chinese_code
                .get(&ch)
                .map(|code| code.get_codes(code_type)),
            CodeType::Zhengma => self.zhengma.get(&ch).cloned(),
            CodeType::Cangjie => self.cangjie.get(&ch).cloned(),
            CodeType::Zhuyin => self.zhuyin.get(&ch).cloned(),
            _ => None,
        }
    }

    /// Get word pinyin (for polyphonic words)
    pub fn get_word_pinyin(&self, word: &str) -> Option<String> {
        self.word_pinyin.get(word).map(|wp| wp.pinyin.clone())
    }

    /// Get character frequency
    pub fn get_frequency(&self, ch: char) -> Option<f64> {
        self.chinese_code.get(&ch).map(|code| code.frequency)
    }

    /// Load ChineseCode.txt
    /// Format: U+4E00\t一\tggll\tggll\tggll\tyi1\t37283.98
    fn load_chinese_code() -> Result<HashMap<char, ChineseCode>, Error> {
        let content = include_str!("../resources/ChineseCode.txt");
        let mut map = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 7 {
                continue; // Skip malformed lines
            }

            let character = parts[1]
                .chars()
                .next()
                .ok_or_else(|| Error::Parse("Empty character field".into()))?;

            let code = ChineseCode {
                unicode: parts[0].to_string(),
                character,
                wubi86: Self::split_codes(parts[2]),
                wubi98: Self::split_codes(parts[3]),
                wubi_new: Self::split_codes(parts[4]),
                pinyin: Self::split_codes(parts[5]),
                frequency: parts[6].parse().unwrap_or(0.0),
            };

            map.insert(character, code);
        }

        Ok(map)
    }

    /// Load WordPinyin.txt
    /// Format: 'jiao'gai 校改
    fn load_word_pinyin() -> Result<HashMap<String, WordPinyin>, Error> {
        let content = include_str!("../resources/WordPinyin.txt");
        let mut map = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() < 2 {
                continue;
            }

            let pinyin = parts[0].to_string();
            let word = parts[1].to_string();

            map.insert(word.clone(), WordPinyin { word, pinyin });
        }

        Ok(map)
    }

    /// Load simple dictionary format (char\tcode1,code2,...)
    fn load_simple_dict(content: &str) -> Result<HashMap<char, Vec<String>>, Error> {
        let mut map = HashMap::new();

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() < 2 {
                continue;
            }

            let character = parts[0]
                .chars()
                .next()
                .ok_or_else(|| Error::Parse("Empty character field".into()))?;

            let codes = Self::split_codes(parts[1]);
            map.insert(character, codes);
        }

        Ok(map)
    }

    /// Split codes by comma (handles multiple pronunciations)
    fn split_codes(s: &str) -> Vec<String> {
        if s.is_empty() {
            return vec![];
        }
        s.split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

impl Default for ResourceManager {
    fn default() -> Self {
        Self::new().expect("Failed to load resources")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_resources() {
        let manager = ResourceManager::new().unwrap();

        // Test ChineseCode - 一
        let codes = manager.get_char_codes('一', &CodeType::Pinyin);
        assert!(codes.is_some());
        let pinyin = codes.unwrap();
        assert!(!pinyin.is_empty());
        println!("一 pinyin: {:?}", pinyin);

        // Test frequency
        let freq = manager.get_frequency('一');
        assert!(freq.is_some());
        println!("一 frequency: {}", freq.unwrap());

        // Test Wubi
        let wubi = manager.get_char_codes('一', &CodeType::Wubi);
        assert!(wubi.is_some());
        println!("一 wubi: {:?}", wubi.unwrap());
    }

    #[test]
    fn test_word_pinyin() {
        let manager = ResourceManager::new().unwrap();

        // Test polyphonic word
        let pinyin = manager.get_word_pinyin("校改");
        if let Some(p) = pinyin {
            println!("校改 pinyin: {}", p);
            assert!(p.contains("jiao"));
        }
    }

    #[test]
    fn test_other_encodings() {
        let manager = ResourceManager::new().unwrap();

        // Test Zhengma
        let zhengma = manager.get_char_codes('一', &CodeType::Zhengma);
        println!("一 zhengma: {:?}", zhengma);

        // Test Cangjie
        let cangjie = manager.get_char_codes('一', &CodeType::Cangjie);
        println!("一 cangjie: {:?}", cangjie);
    }
}
