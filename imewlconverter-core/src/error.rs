//! Error types for the IME converter

use std::io;
use thiserror::Error;

/// Result type alias for converter operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types that can occur during conversion
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Encoding error: {0}")]
    Encoding(String),

    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    #[error("Unsupported operation: {0}")]
    Unsupported(String),

    #[error("Character not found in dictionary: {0}")]
    CharacterNotFound(char),

    #[error("Invalid code type")]
    InvalidCodeType,

    #[error("Binary parse error: {0}")]
    BinaryParse(String),

    #[error("UTF-16 decode error")]
    Utf16Decode,

    #[error("Invalid file format: expected {expected}, got {actual}")]
    FormatMismatch { expected: String, actual: String },
}
