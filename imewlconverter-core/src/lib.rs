//! IME Word List Converter Core Library
//!
//! This library provides functionality to convert between different IME (Input Method Editor)
//! dictionary formats. It supports 20+ different formats including Sogou, QQ Pinyin, Rime,
//! and many others.
//!
//! # Architecture
//!
//! The library follows a three-layer architecture:
//! - **Data Layer**: Core data structures (WordLibrary, Code, CodeType)
//! - **Processing Layer**: Import, Export, Code Generation, Filtering
//! - **Resource Layer**: Embedded dictionaries and encoding mappings
//!
//! # Example
//!
//! ```no_run
//! use imewlconverter_core::{WordLibrary, CodeType};
//!
//! // Create a word library entry
//! let mut word = WordLibrary::new("你好".to_string());
//! word.rank = 1000;
//! ```

pub mod data;
pub mod error;
pub mod export;
pub mod filter;
pub mod generate;
pub mod helpers;
pub mod import;
pub mod rank;
pub mod resource;
pub mod translate;

// Re-export commonly used types
pub use data::{Code, CodeType, WordLibrary, WordLibraryList};
pub use error::{Error, Result};

/// Version of the converter
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
