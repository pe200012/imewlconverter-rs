/// Integration test using real files from 参考 directory
use imewlconverter_core::generate::CodeGenerator;
use imewlconverter_core::*;
use std::fs;
use std::path::PathBuf;

#[test]
fn test_pinyin_generation_from_wordlist() {
    // Load the Pinyin generator with real dictionary
    let generator = generate::PinyinGenerator::new().expect("Failed to load pinyin generator");

    // Test some common words from the 8万精准超小词库.txt
    let test_words = vec![
        "是的", "这个", "应该", "进行", "政府", "产品", "支持", "国家", "单位", "世界",
    ];

    for word in test_words {
        let mut wl = WordLibrary::new(word.to_string());
        match generator.generate_code(&mut wl) {
            Ok(_) => {
                println!("{} -> {}", word, wl.get_pinyin_string("'"),);
            }
            Err(e) => {
                eprintln!("Failed to generate pinyin for {}: {:?}", word, e);
            }
        }
    }
}

#[test]
fn test_convert_reference_file() {
    use encoding_rs::UTF_16LE;
    use std::io::Read;

    // Path to the reference file
    let mut file_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    file_path.pop(); // Go to rust/
    file_path.pop(); // Go to imewlconverter/
    file_path.push("参考");
    file_path.push("8万精准超小词库.txt");

    if !file_path.exists() {
        println!("Reference file not found, skipping test");
        return;
    }

    println!("Testing with file: {:?}", file_path);

    // Read file as bytes and decode as UTF-16LE
    let mut file = fs::File::open(&file_path).expect("Failed to open file");
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes).expect("Failed to read file");

    let (content, _, _) = UTF_16LE.decode(&bytes);
    let lines: Vec<&str> = content.lines().collect();

    let generator = generate::PinyinGenerator::new().expect("Failed to load pinyin generator");

    let mut success_count = 0;
    let mut fail_count = 0;
    let mut total_count = 0;

    for (i, line) in lines.iter().enumerate() {
        if i >= 100 {
            break; // Test first 100 lines
        }

        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        // Remove any existing pinyin annotations (like "是的de" -> "是的")
        let word: String = line
            .chars()
            .take_while(|c| !c.is_ascii_alphabetic())
            .collect();

        if word.is_empty() {
            continue;
        }

        total_count += 1;

        let mut wl = WordLibrary::new(word.clone());
        match generator.generate_code(&mut wl) {
            Ok(_) => {
                success_count += 1;
                if i < 20 {
                    // Print first 20 for inspection
                    println!("{:3}. {} -> {}", i + 1, word, wl.get_pinyin_string("'"));
                }
            }
            Err(e) => {
                fail_count += 1;
                if fail_count <= 10 {
                    // Print first 10 failures
                    eprintln!("Failed: {} - {:?}", word, e);
                }
            }
        }
    }

    println!("\n=== Summary ===");
    println!("Total processed: {}", total_count);
    println!("Success: {}", success_count);
    println!("Failed: {}", fail_count);
    println!(
        "Success rate: {:.2}%",
        (success_count as f64 / total_count as f64) * 100.0
    );

    // We expect at least 90% success rate
    assert!(
        success_count as f64 / total_count as f64 > 0.9,
        "Success rate too low"
    );
}
