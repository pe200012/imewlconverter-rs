//! Rank filter - filters words based on frequency/rank

use crate::filter::SingleFilter;
use crate::WordLibrary;

/// Filter words by rank/frequency
pub struct RankFilter {
    pub min_rank: i32,
    pub max_rank: i32,
}

impl RankFilter {
    pub fn new(min_rank: i32, max_rank: i32) -> Self {
        RankFilter { min_rank, max_rank }
    }
}

impl Default for RankFilter {
    fn default() -> Self {
        RankFilter {
            min_rank: 0,
            max_rank: i32::MAX,
        }
    }
}

impl SingleFilter for RankFilter {
    fn is_keep(&self, word: &WordLibrary) -> bool {
        word.rank >= self.min_rank && word.rank <= self.max_rank
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_filter() {
        let filter = RankFilter::new(100, 1000);

        let word1 = WordLibrary::with_rank("你好".to_string(), 50);
        let word2 = WordLibrary::with_rank("世界".to_string(), 500);
        let word3 = WordLibrary::with_rank("测试".to_string(), 2000);

        assert!(!filter.is_keep(&word1)); // Too low
        assert!(filter.is_keep(&word2)); // OK
        assert!(!filter.is_keep(&word3)); // Too high
    }
}
