use clap::Parser;

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
    emoji_file: Option<std::path::PathBuf>,

    #[clap(long, parse(from_os_str))]
    output_file: std::path::PathBuf,
}

#[derive(clap::Args)]
#[clap(author, version, about, long_about = None)]
struct DetectDuplicates {
    #[clap(long, parse(from_os_str))]
    output_file: std::path::PathBuf,
}

fn main() {
    match Cargo::parse() {
        Cargo::BuildRomanTableWithEmoji(args) => {
            println!("{:?}", args.emoji_file);
            println!("{:?}", args.output_file);
        }
        Cargo::DetectDuplicates(args) => {
            println!("{:?}", args.output_file);
        }
    }
}
