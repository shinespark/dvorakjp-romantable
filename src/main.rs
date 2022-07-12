use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use dvorakjp_romantable::build_roman_table_with_emoji::RomanTableWithEmojiBuilder;
use dvorakjp_romantable::detect_duplicates::DuplicateDetector;

const DEFAULT_EMOJI_FILE: &str = "./lib/emoji.txt";

#[derive(Parser)]
#[clap(name = "cargo")]
#[clap(bin_name = "cargo")]
enum Cargo {
    BuildRomanTableWithEmoji(BuildRomanTableWithEmoji),
    DetectDuplicates(DetectDuplicates),
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct BuildRomanTableWithEmoji {
    #[clap(long, parse(from_os_str))]
    emoji_file: Option<PathBuf>,

    #[clap(long, parse(from_os_str))]
    output_file: PathBuf,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct DetectDuplicates {
    #[clap(long, parse(from_os_str))]
    detect_file: PathBuf,
}

fn main() -> Result<()> {
    let _ = match Cargo::parse() {
        Cargo::BuildRomanTableWithEmoji(args) => RomanTableWithEmojiBuilder::exec(
            args.emoji_file
                .unwrap_or_else(|| PathBuf::from(DEFAULT_EMOJI_FILE)),
            args.output_file,
        ),
        Cargo::DetectDuplicates(args) => DuplicateDetector::exec(args.detect_file),
    };
    Ok(())
}
