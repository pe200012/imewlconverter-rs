//! Helper utilities

pub mod pinyin;

use crate::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Write string to file with encoding
pub fn write_file(path: &Path, content: &str, encoding: &str) -> Result<()> {
    use encoding_rs::Encoding;

    let encoding = if encoding == "utf-8" {
        encoding_rs::UTF_8
    } else if encoding == "gbk" {
        encoding_rs::GBK
    } else if encoding == "utf-16le" {
        encoding_rs::UTF_16LE
    } else if encoding == "big5" {
        encoding_rs::BIG5
    } else {
        Encoding::for_label(encoding.as_bytes()).unwrap_or(encoding_rs::UTF_8)
    };

    let (encoded, _, _) = encoding.encode(content);
    let mut file = File::create(path)?;
    file.write_all(&encoded)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helper_module_exists() {}
}
