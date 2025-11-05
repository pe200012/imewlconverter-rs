//! Code generation for various encoding schemes

use crate::{Code, CodeType, Result, WordLibrary};

pub mod pinyin;

// Re-export common types
pub use pinyin::PinyinGenerator;

/// Trait for code generators
pub trait CodeGenerator {
    /// Generate code for a word library entry
    fn generate_code(&self, word: &mut WordLibrary) -> Result<()>;

    /// Generate code for a string
    fn generate_code_for_string(&self, text: &str) -> Result<Code>;

    /// Get all possible codes for a single character
    fn get_codes_for_char(&self, c: char) -> Result<Vec<String>>;

    /// Does this generator support multiple codes per character?
    fn is_multi_code_per_char(&self) -> bool;

    /// Is this one code per character (vs one code per word)?
    fn is_one_code_per_char(&self) -> bool;

    /// Get the code type this generator produces
    fn code_type(&self) -> CodeType;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generator_trait_exists() {
        // Just test that the trait compiles
    }
}
