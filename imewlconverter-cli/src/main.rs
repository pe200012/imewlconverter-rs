//! IME Word List Converter CLI
//!
//! Command-line interface for converting between different IME dictionary formats.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum};
use imewlconverter_core::*;
use std::path::PathBuf;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum InputFormat {
    /// QQ Pinyin text format
    QqPinyin,
    /// Rime format
    Rime,
    // TODO: Add more formats as they are implemented
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    /// QQ Pinyin text format
    QqPinyin,
    /// Rime format
    Rime,
    // TODO: Add more formats as they are implemented
}

#[derive(Parser, Debug)]
#[command(name = "imewlconverter")]
#[command(author = "studyzy <studyzy@163.com>")]
#[command(version = VERSION)]
#[command(about = "IME Word List Converter - Convert between different IME dictionary formats", long_about = None)]
struct Args {
    /// Input format
    #[arg(short = 'i', long, value_enum)]
    input_format: InputFormat,

    /// Input files
    #[arg(required = true)]
    input_files: Vec<PathBuf>,

    /// Output format
    #[arg(short = 'o', long, value_enum)]
    output_format: OutputFormat,

    /// Output file
    #[arg(required = true)]
    output: PathBuf,

    /// Minimum word length
    #[arg(long, default_value = "1")]
    min_length: usize,

    /// Maximum word length
    #[arg(long, default_value = "100")]
    max_length: usize,

    /// Minimum rank/frequency
    #[arg(long, default_value = "0")]
    min_rank: i32,

    /// Maximum rank/frequency
    #[arg(long, default_value = "2147483647")]
    max_rank: i32,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    if args.verbose {
        println!("IME Word List Converter v{}", VERSION);
        println!("Input format: {:?}", args.input_format);
        println!("Output format: {:?}", args.output_format);
        println!("Input files: {} file(s)", args.input_files.len());
    }

    // Create filters
    let length_filter = filter::length::LengthFilter::new(args.min_length, args.max_length);
    let rank_filter = filter::rank::RankFilter::new(args.min_rank, args.max_rank);

    // Import all files
    let mut all_words = Vec::new();

    for input_file in &args.input_files {
        if args.verbose {
            println!("Processing: {}", input_file.display());
        }

        let importer: Box<dyn import::WordLibraryImport> = match args.input_format {
            InputFormat::QqPinyin => Box::new(import::qq_pinyin::QQPinyinImport::new()),
            InputFormat::Rime => Box::new(import::rime::RimeImport::new()),
        };

        let input_path = input_file
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid file path"))?;

        let mut words = importer
            .import_from_file(input_path)
            .with_context(|| format!("Failed to import {}", input_file.display()))?;

        if args.verbose {
            println!("  Imported {} words", words.len());
        }

        // Apply filters
        use filter::SingleFilter;
        words.retain(|w| length_filter.is_keep(w));
        words.retain(|w| rank_filter.is_keep(w));

        if args.verbose {
            println!("  After filtering: {} words", words.len());
        }

        all_words.append(&mut words);
    }

    if args.verbose {
        println!("Total words: {}", all_words.len());
    }

    // Export
    let exporter: Box<dyn export::WordLibraryExport> = match args.output_format {
        OutputFormat::QqPinyin => Box::new(export::qq_pinyin::QQPinyinExport::new()),
        OutputFormat::Rime => Box::new(export::rime::RimeExport::new()),
    };

    let output_content = exporter.export(&all_words).context("Failed to export")?;

    // Write to file
    for (i, content) in output_content.iter().enumerate() {
        let output_path = if i == 0 {
            args.output.clone()
        } else {
            let mut path = args.output.clone();
            let stem = path.file_stem().unwrap().to_str().unwrap();
            let ext = path.extension().and_then(|s| s.to_str()).unwrap_or("txt");
            path.set_file_name(format!("{}{}.{}", stem, i, ext));
            path
        };

        helpers::write_file(&output_path, content, exporter.encoding())
            .with_context(|| format!("Failed to write {}", output_path.display()))?;

        if args.verbose {
            println!("Written to: {}", output_path.display());
        }
    }

    println!("Conversion completed successfully!");
    println!("Total words converted: {}", all_words.len());

    Ok(())
}
