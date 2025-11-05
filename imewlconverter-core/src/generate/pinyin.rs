//! Pinyin code generator
//!
//! Generates Pinyin codes for Chinese characters

use crate::generate::CodeGenerator;
use crate::resource::ResourceManager;
use crate::{Code, CodeType, Error, Result, WordLibrary};
use std::sync::Arc;

/// Pinyin generator
pub struct PinyinGenerator {
    /// Resource manager with all dictionaries
    resources: Arc<ResourceManager>,
}

impl PinyinGenerator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            resources: Arc::new(ResourceManager::new()?),
        })
    }

    /// Initialize with existing resource manager (for sharing)
    pub fn with_resources(resources: Arc<ResourceManager>) -> Self {
        Self { resources }
    }

    /// Get default pinyin for a character (first pronunciation)
    pub fn get_default_pinyin(&self, c: char) -> Result<String> {
        self.resources
            .get_char_codes(c, &CodeType::Pinyin)
            .and_then(|pinyins| pinyins.first().cloned())
            .ok_or(Error::CharacterNotFound(c))
    }

    /// Check if a character has multiple pronunciations
    pub fn is_polyphonic(&self, c: char) -> bool {
        self.resources
            .get_char_codes(c, &CodeType::Pinyin)
            .map(|pinyins| pinyins.len() > 1)
            .unwrap_or(false)
    }

    /// Get pinyin for a word, handling polyphonic words
    fn get_word_pinyin(&self, word: &str) -> Option<String> {
        // First check if there's a specific pronunciation for this word
        if let Some(pinyin) = self.resources.get_word_pinyin(word) {
            return Some(pinyin);
        }

        // Otherwise, concatenate character pinyin
        let mut result = String::new();
        for ch in word.chars() {
            if let Some(pinyin) = self.resources.get_char_codes(ch, &CodeType::Pinyin) {
                if !result.is_empty() {
                    result.push('\'');
                }
                // Use first pronunciation if multiple
                result.push_str(&pinyin[0]);
            } else {
                // Character not found in dictionary
                return None;
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}

impl Default for PinyinGenerator {
    fn default() -> Self {
        Self::new().expect("Failed to load pinyin resources")
    }
}

impl CodeGenerator for PinyinGenerator {
    fn generate_code(&self, word: &mut WordLibrary) -> Result<()> {
        // If already has pinyin, skip
        if word.code_type == CodeType::Pinyin && !word.codes.is_empty() {
            return Ok(());
        }

        let code = self.generate_code_for_string(&word.word)?;
        word.code_type = CodeType::Pinyin;
        word.codes = code;
        Ok(())
    }

    fn generate_code_for_string(&self, s: &str) -> Result<Code> {
        if let Some(pinyin) = self.get_word_pinyin(s) {
            // Remove apostrophes and join - pinyin is already in format like "ni'hao"
            // We want to store it as separate codes per character
            let codes: Vec<String> = pinyin.split('\'').map(|s| s.to_string()).collect();
            Ok(Code::from_char_list(codes))
        } else {
            Err(Error::CharacterNotFound(s.chars().next().unwrap_or('?')))
        }
    }

    fn get_codes_for_char(&self, c: char) -> Result<Vec<String>> {
        if c.is_ascii() {
            return Ok(vec![c.to_lowercase().to_string()]);
        }

        self.resources
            .get_char_codes(c, &CodeType::Pinyin)
            .ok_or(Error::CharacterNotFound(c))
    }

    fn is_multi_code_per_char(&self) -> bool {
        true // Pinyin can have multiple pronunciations (polyphonic characters)
    }

    fn is_one_code_per_char(&self) -> bool {
        true // Each character has its own code
    }

    fn code_type(&self) -> CodeType {
        CodeType::Pinyin
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinyin_generation() {
        let generator = PinyinGenerator::new().unwrap();

        let mut word = WordLibrary::new("你好".to_string());
        generator.generate_code(&mut word).unwrap();

        assert_eq!(word.code_type, CodeType::Pinyin);
        println!("你好 pinyin: {}", word.get_pinyin_string("'"));
    }

    #[test]
    fn test_polyphonic_character() {
        let generator = PinyinGenerator::new().unwrap();

        // Test if character has multiple pronunciations
        let codes = generator.get_codes_for_char('长');
        if let Ok(code_list) = codes {
            println!("长 has {} pronunciations: {:?}", code_list.len(), code_list);
            if code_list.len() > 1 {
                assert!(generator.is_polyphonic('长'));
            }
        }
    }

    #[test]
    fn test_ascii_handling() {
        let generator = PinyinGenerator::new().unwrap();

        let codes = generator.get_codes_for_char('a').unwrap();
        assert_eq!(codes, vec!["a".to_string()]);
    }
}
