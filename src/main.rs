use clap::Parser;
use std::process::ExitCode;

use crate::asset_list::builder::read_or_build_asset_list;
use crate::checks::Checker;
use crate::checks::duplicates::DuplicateChecker;
use crate::checks::lint_item::LintItem;
use crate::output::LintOutput;
use crate::output::console::ConsoleOutput;

mod asset_list;
mod checks;
mod output;

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

    /// Minimal console output
    #[arg(long, action = clap::ArgAction::SetFalse)]
    quiet: bool,
}

fn main() -> ExitCode {
    env_logger::init();
    let args = Args::parse();

    // instantiate checkers
    let mut checkers: Vec<Box<dyn Checker>> = Vec::new();
    if args.no_duplicates {
        checkers.push(Box::new(DuplicateChecker::new()));
        println!("Checking for duplicates");
    } else {
        println!("No rules to check");
    }

    // get the asset list
    let assets = read_or_build_asset_list(args.assets_path);

    // do the checks
    let mut lint_result: Vec<LintItem> = Vec::new();
    for checker in checkers.iter_mut() {
        lint_result.append(&mut checker.check(&assets));
    }

    // print the results
    let mut printers: Vec<Box<dyn LintOutput>> = Vec::new();
    if !args.quiet {
        printers.push(Box::new(ConsoleOutput {}));
    }

    for output in printers.iter_mut() {
        output.print_result(&lint_result);
    }

    ExitCode::SUCCESS
}
