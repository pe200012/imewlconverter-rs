//! Filtering functionality for word libraries

use crate::{Result, WordLibrary, WordLibraryList};

pub mod length;
pub mod rank;

/// Trait for filters that process individual entries
pub trait SingleFilter {
    /// Check if a word should be kept
    fn is_keep(&self, word: &WordLibrary) -> bool;

    /// Filter a list of words
    fn filter(&self, words: &WordLibraryList) -> WordLibraryList {
        words.iter().filter(|w| self.is_keep(w)).cloned().collect()
    }
}

/// Trait for filters that process entire word lists
pub trait BatchFilter {
    /// Filter a word list
    fn filter(&self, words: WordLibraryList) -> Result<WordLibraryList>;
}

/// Filter configuration for special character handling
#[derive(Debug, Clone)]
pub struct FilterConfig {
    pub keep_number: bool,
    pub keep_english: bool,
    pub keep_space: bool,
    pub keep_punctuation: bool,
    pub full_width_to_half: bool,
    pub number_to_chinese: bool,
}

impl Default for FilterConfig {
    fn default() -> Self {
        FilterConfig {
            keep_number: true,
            keep_english: true,
            keep_space: true,
            keep_punctuation: true,
            full_width_to_half: false,
            number_to_chinese: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_config_default() {
        let config = FilterConfig::default();
        assert!(config.keep_number);
        assert!(config.keep_english);
    }
}
