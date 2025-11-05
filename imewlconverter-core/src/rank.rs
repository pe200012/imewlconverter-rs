//! Word rank generation strategies

use crate::{Result, WordLibrary};

/// Trait for word rank generators
pub trait RankGenerator {
    /// Get rank/frequency for a word
    fn get_rank(&self, word: &str) -> Result<i32>;

    /// Whether to force use this generator even if word already has rank
    fn force_use(&self) -> bool {
        false
    }

    /// Generate rank for a word library entry
    fn generate_rank(&self, word: &mut WordLibrary) -> Result<()> {
        if word.rank == 0 || self.force_use() {
            word.rank = self.get_rank(&word.word)?;
        }
        Ok(())
    }
}

/// Default rank generator - returns a constant value
pub struct DefaultRankGenerator {
    pub default_rank: i32,
}

impl DefaultRankGenerator {
    pub fn new(default_rank: i32) -> Self {
        DefaultRankGenerator { default_rank }
    }
}

impl Default for DefaultRankGenerator {
    fn default() -> Self {
        DefaultRankGenerator { default_rank: 100 }
    }
}

impl RankGenerator for DefaultRankGenerator {
    fn get_rank(&self, _word: &str) -> Result<i32> {
        Ok(self.default_rank)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_rank_generator() {
        let generator = DefaultRankGenerator::new(500);
        assert_eq!(generator.get_rank("test").unwrap(), 500);
    }

    #[test]
    fn test_generate_rank() {
        let generator = DefaultRankGenerator::new(100);
        let mut word = WordLibrary::new("你好".to_string());

        generator.generate_rank(&mut word).unwrap();
        assert_eq!(word.rank, 100);
    }
}
