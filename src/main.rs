use clap::Parser;

mod checks;

#[derive(Parser, Debug)]
#[command(name = "asset-lint")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to check
    assets_path: String,

    /// Check for duplicate files
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_duplicates: bool,
}

fn main() {
    let args = Args::parse();

    if args.no_duplicates {
        println!("Checking {} for duplicates", args.assets_path);
    } else {
        println!("Checking only {}", args.assets_path);
    }
}
