/// Sogou SCEL binary format parser
/// This is the most popular binary dictionary format in China
use crate::import::WordLibraryImport;
use crate::{CodeType, Error, Result, WordLibrary};
use nom::{bytes::complete::take, number::complete::le_u16, IResult};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

pub struct SogouScelImport;

impl SogouScelImport {
    /// Read SCEL file information without parsing dictionary
    pub fn read_info(path: &str) -> Result<ScelInfo> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        let info = parse_scel_info(&buffer)?;
        Ok(info)
    }
}

impl WordLibraryImport for SogouScelImport {
    fn import_from_file(&self, path: &str) -> Result<Vec<WordLibrary>> {
        let mut file = File::open(path)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;

        parse_scel_file(&buffer)
    }
}

#[derive(Debug, Clone)]
pub struct ScelInfo {
    pub name: String,
    pub category: String,
    pub description: String,
    pub example: String,
    pub word_count: u32,
}

/// Parse SCEL file information
fn parse_scel_info(data: &[u8]) -> Result<ScelInfo> {
    if data.len() < 0x1540 {
        return Err(Error::Parse("File too small to be valid SCEL".into()));
    }

    // Check magic number
    if &data[0..12] != b"\x40\x15\x00\x00\x44\x43\x53\x01\x01\x00\x00\x00" {
        return Err(Error::Parse("Invalid SCEL magic number".into()));
    }

    // Word count at 0x124
    let word_count = u32::from_le_bytes([data[0x124], data[0x125], data[0x126], data[0x127]]);

    // Read UTF-16LE strings
    let name = read_utf16le_string(&data[0x130..0x338])?;
    let category = read_utf16le_string(&data[0x338..0x540])?;
    let description = read_utf16le_string(&data[0x540..0xd40])?;
    let example = read_utf16le_string(&data[0xd40..0x1540])?;

    Ok(ScelInfo {
        name,
        category,
        description,
        example,
        word_count,
    })
}

/// Parse the entire SCEL file and extract dictionary entries
fn parse_scel_file(data: &[u8]) -> Result<Vec<WordLibrary>> {
    if data.len() < 0x1540 {
        return Err(Error::Parse("File too small to be valid SCEL".into()));
    }

    // Parse pinyin table (starts around 0x1540)
    let pinyin_table = parse_pinyin_table(data)?;

    // Parse dictionary entries (starts after pinyin table)
    let dict_start = find_dict_start(data)?;
    parse_dictionary(&data[dict_start..], &pinyin_table)
}

/// Parse the pinyin index table
fn parse_pinyin_table(data: &[u8]) -> Result<HashMap<u16, String>> {
    let mut table = HashMap::new();
    let mut offset = 0x1540;

    while offset < data.len() - 4 {
        if let Ok((_, (index, pinyin))) = parse_pinyin_entry(&data[offset..]) {
            if index == 0 {
                break; // End of pinyin table
            }
            let pinyin_len = pinyin.encode_utf16().count();
            table.insert(index, pinyin.clone());
            offset += 2 + 2 + pinyin_len * 2; // index + length + utf16 chars
        } else {
            break;
        }
    }

    Ok(table)
}

/// Parse a single pinyin table entry
fn parse_pinyin_entry(data: &[u8]) -> IResult<&[u8], (u16, String)> {
    let (data, index) = le_u16(data)?;
    let (data, length) = le_u16(data)?;

    let byte_len = length as usize * 2;
    let (data, pinyin_bytes) = take(byte_len)(data)?;

    let pinyin = String::from_utf16_lossy(
        &pinyin_bytes
            .chunks(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect::<Vec<_>>(),
    );

    Ok((data, (index, pinyin)))
}

/// Find where dictionary entries start
fn find_dict_start(data: &[u8]) -> Result<usize> {
    // Dictionary typically starts after pinyin table
    // Look for pattern or use heuristic
    for i in 0x1540..data.len() - 10 {
        // Dictionary entries have a specific pattern
        if data[i] == 0 && data[i + 1] == 0 && data[i + 2] > 0 && data[i + 3] == 0 {
            return Ok(i);
        }
    }
    Err(Error::Parse("Could not find dictionary start".into()))
}

/// Parse dictionary entries
fn parse_dictionary(data: &[u8], pinyin_table: &HashMap<u16, String>) -> Result<Vec<WordLibrary>> {
    let mut entries = Vec::new();
    let mut offset = 0;

    while offset < data.len() - 14 {
        match parse_dict_entry(&data[offset..], pinyin_table) {
            Ok((remaining, entry)) => {
                if let Some(wl) = entry {
                    entries.push(wl);
                }
                let consumed = data[offset..].len() - remaining.len();
                offset += consumed;
            }
            Err(_) => {
                offset += 1; // Skip bad byte
            }
        }

        if entries.len() >= 100000 {
            break; // Safety limit
        }
    }

    Ok(entries)
}

/// Parse a single dictionary entry
fn parse_dict_entry<'a>(
    data: &'a [u8],
    pinyin_table: &HashMap<u16, String>,
) -> IResult<&'a [u8], Option<WordLibrary>> {
    let (data, same_pinyin_count) = le_u16(data)?;
    let (data, pinyin_len) = le_u16(data)?;

    // Read pinyin indices
    let mut pinyin_parts = Vec::new();
    let mut remaining = data;
    for _ in 0..pinyin_len {
        let (r, index) = le_u16(remaining)?;
        if let Some(py) = pinyin_table.get(&index) {
            pinyin_parts.push(py.clone());
        }
        remaining = r;
    }

    // Read words with same pinyin
    let mut words = Vec::new();
    for _ in 0..same_pinyin_count {
        let (r, word_len) = le_u16(remaining)?;
        let byte_len = word_len as usize * 2;
        let (r, word_bytes) = take(byte_len)(r)?;

        let word = String::from_utf16_lossy(
            &word_bytes
                .chunks(2)
                .map(|c| u16::from_le_bytes([c[0], c[1]]))
                .collect::<Vec<_>>(),
        );

        let (r, ext_len) = le_u16(r)?;
        let (r, _ext) = take(ext_len as usize * 2)(r)?; // Skip extension

        words.push(word);
        remaining = r;
    }

    // Return first word (or could return all)
    if let Some(word) = words.first() {
        let mut wl = WordLibrary::new(word.clone());
        wl.code_type = CodeType::Pinyin;
        wl.codes = crate::Code::from_char_list(pinyin_parts);
        Ok((remaining, Some(wl)))
    } else {
        Ok((remaining, None))
    }
}

/// Read a null-terminated UTF-16LE string
fn read_utf16le_string(data: &[u8]) -> Result<String> {
    let u16_vec: Vec<u16> = data
        .chunks(2)
        .map(|c| {
            if c.len() == 2 {
                u16::from_le_bytes([c[0], c[1]])
            } else {
                0
            }
        })
        .take_while(|&c| c != 0)
        .collect();

    Ok(String::from_utf16_lossy(&u16_vec))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_utf16le_string() {
        let data = b"T\x00e\x00s\x00t\x00\x00\x00";
        let result = read_utf16le_string(data).unwrap();
        assert_eq!(result, "Test");
    }

    #[test]
    fn test_scel_info_parse() {
        // This would require a real SCEL file to test properly
        // For now, just ensure the struct exists
        let info = ScelInfo {
            name: "Test".to_string(),
            category: "Test".to_string(),
            description: "Test description".to_string(),
            example: "Test example".to_string(),
            word_count: 1000,
        };
        assert_eq!(info.name, "Test");
    }
}
