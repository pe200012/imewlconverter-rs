//! Length filter - filters words based on character length

use crate::filter::SingleFilter;
use crate::WordLibrary;

/// Filter words by length (number of characters)
pub struct LengthFilter {
    pub min_length: usize,
    pub max_length: usize,
}

impl LengthFilter {
    pub fn new(min_length: usize, max_length: usize) -> Self {
        LengthFilter {
            min_length,
            max_length,
        }
    }
}

impl Default for LengthFilter {
    fn default() -> Self {
        LengthFilter {
            min_length: 1,
            max_length: 9999,
        }
    }
}

impl SingleFilter for LengthFilter {
    fn is_keep(&self, word: &WordLibrary) -> bool {
        let len = word.len();
        len >= self.min_length && len <= self.max_length
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_filter() {
        let filter = LengthFilter::new(2, 4);

        let word1 = WordLibrary::new("你".to_string());
        let word2 = WordLibrary::new("你好".to_string());
        let word3 = WordLibrary::new("你好世界啊".to_string());

        assert!(!filter.is_keep(&word1)); // Too short
        assert!(filter.is_keep(&word2)); // OK
        assert!(!filter.is_keep(&word3)); // Too long
    }
}
