//! Asset-lint keeps your game assets neat and tidy.
//!
//! Command line tool to find and fix common problems with your game assets.

use clap::Parser;
use std::process::ExitCode;

use crate::asset_list::builder::read_or_build_asset_list;
use crate::asset_list::exporter::export_asset_list;
use crate::checks::Checker;
use crate::checks::LintItem;
use crate::checks::duplicates::DuplicateChecker;
use crate::checks::max_size::MaxSizeCheck;
use crate::checks::placeholders::PlaceholderChecker;
use crate::output::LintOutput;
use crate::output::console::ConsoleOutput;
use crate::output::sarif::SarifOutput;

mod asset_list;
mod checks;
mod output;

// minimum required `asset_lint_list.json` version
const MINIMUM_ASSET_LIST_VERSION: u32 = 1;

/// Structure to define the possible command line parameters
#[derive(Parser, Debug)]
#[command(name = "asset-lint")]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to check
    #[arg(long)]
    assets_path: Option<String>,

    /// Check for duplicate files
    #[arg(long, action = clap::ArgAction::SetTrue)]
    no_duplicates: bool,

    /// Check for too big assets
    #[arg(long)]
    max_size: Option<u64>,

    /// Check for placeholder assets
    #[arg(long)]
    no_placeholders: Vec<String>,

    /// Minimal console output
    #[arg(long, default_value_t = false)]
    quiet: bool,

    /// SARIF output
    #[arg(long, default_value_t = false)]
    sarif: bool,

    /// Path to export naive `asset_lint_list.json`
    #[arg(long)]
    export_asset_list: Option<String>,
}

/// Entry point of the application.
/// Call it like `asset-lint --help` to see the possible usage
fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    // instantiate checkers
    let mut checkers: Vec<Box<dyn Checker>> = Vec::new();
    if args.no_duplicates {
        checkers.push(Box::new(DuplicateChecker::new()));
        println!("Checking for duplicates");
    }
    if let Some(max_size) = args.max_size {
        checkers.push(Box::new(MaxSizeCheck::new(max_size)));
        println!("Checking for assets bigger than {} bytes", max_size);
    }
    if !args.no_placeholders.is_empty() {
        checkers.push(Box::new(PlaceholderChecker::new(args.no_placeholders)));
        println!("Checking for placeholder assets");
    }

    if checkers.is_empty() {
        println!("No rules to check");
    }

    // get the asset list
    let assets = read_or_build_asset_list(args.assets_path);

    // do the checks
    let mut lint_result: Vec<LintItem> = Vec::new();
    for checker in checkers.iter_mut() {
        lint_result.append(&mut checker.check(&assets));
    }

    // instantiate the outputs
    let mut printers: Vec<Box<dyn LintOutput>> = Vec::new();
    if !args.quiet {
        printers.push(Box::new(ConsoleOutput {}));
    }
    if args.sarif {
        printers.push(Box::new(SarifOutput {}));
    }

    // print the results
    for output in printers.iter_mut() {
        output.print_result(&lint_result);
    }

    // export the naive asset list for future use
    if let Some(export_path) = args.export_asset_list {
        export_asset_list(export_path, &assets);
    }

    ExitCode::SUCCESS
}
