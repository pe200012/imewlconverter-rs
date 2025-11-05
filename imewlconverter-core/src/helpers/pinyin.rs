//! Pinyin helper functions

use std::collections::HashMap;

/// Pinyin helper for character lookups
pub struct PinyinHelper {
    pinyin_dict: HashMap<char, Vec<String>>,
}

impl PinyinHelper {
    /// Create a new pinyin helper (will load embedded dictionary)
    pub fn new() -> Self {
        // TODO: Load from embedded resource
        PinyinHelper {
            pinyin_dict: HashMap::new(),
        }
    }

    /// Get default pinyin for a character
    pub fn get_default_pinyin(&self, c: char) -> Option<String> {
        self.pinyin_dict
            .get(&c)
            .and_then(|pinyins| pinyins.first())
            .cloned()
    }

    /// Get all pinyin for a character
    pub fn get_all_pinyin(&self, c: char) -> Option<&Vec<String>> {
        self.pinyin_dict.get(&c)
    }

    /// Check if character is polyphonic (has multiple pronunciations)
    pub fn is_polyphonic(&self, c: char) -> bool {
        self.pinyin_dict
            .get(&c)
            .map(|pinyins| pinyins.len() > 1)
            .unwrap_or(false)
    }

    /// Validate if pinyin matches word
    pub fn validate_pinyin(&self, word: &str, pinyins: &[String]) -> bool {
        let chars: Vec<char> = word.chars().collect();
        if chars.len() != pinyins.len() {
            return false;
        }

        for (c, py) in chars.iter().zip(pinyins.iter()) {
            if let Some(valid_pinyins) = self.pinyin_dict.get(c) {
                if !valid_pinyins.contains(py) {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }
}

impl Default for PinyinHelper {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinyin_helper_creation() {
        let helper = PinyinHelper::new();
        // This is a placeholder test since we haven't loaded the dictionary yet
    }
}
