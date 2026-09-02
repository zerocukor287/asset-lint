//! Asset-lint keeps your game assets neat and tidy.
//!
//! Command line tool to find and fix common problems with your game assets.
//!
//! This documentation meant to be read by developers, or enthusiast to understand the inner workings of the tool.  
//! Usage guide can be found in the [Repository](https://github.com/zerocukor287/asset-lint) or in the [wiki](https://github.com/zerocukor287/asset-lint/wiki)

use std::process::ExitCode;

use crate::asset_list::builder::read_or_build_asset_list;
use crate::asset_list::exporter::export_asset_list;
use crate::checks::Checker;
use crate::checks::LintItem;
use crate::checks::duplicates::DuplicateChecker;
use crate::checks::max_size::MaxSizeCheck;
use crate::checks::max_total_size::MaxTotalSizeCheck;
use crate::checks::placeholders::PlaceholderChecker;
use crate::output::LintOutput;
use crate::output::console::ConsoleOutput;
use crate::output::sarif::SarifOutput;
use crate::settings::config::Config;
use crate::settings::config::create_config;

mod asset_list;
mod checks;
mod output;
mod settings;

// minimum required `asset_lint_list.json` version
const MINIMUM_ASSET_LIST_VERSION: u32 = 1;

/// Entry point of the application.
/// Call it like `asset-lint --help` to see the possible usage
fn main() -> ExitCode {
    env_logger::init();
    let config = create_config();

    // instantiate checkers
    let mut checkers = create_checkers(&config);

    // validate that we have more than one check
    if checkers.is_empty() {
        println!("No rules to check");
    }

    // get the asset list
    let assets = read_or_build_asset_list(config.assets_path);

    // do the checks
    let mut lint_result: Vec<LintItem> = Vec::new();
    for checker in checkers.iter_mut() {
        lint_result.append(&mut checker.check(&assets));
    }

    // instantiate the outputs
    let mut printers: Vec<Box<dyn LintOutput>> = Vec::new();
    if !config.quiet {
        printers.push(Box::new(ConsoleOutput {}));
    }
    if config.sarif {
        printers.push(Box::new(SarifOutput {}));
    }

    // print the results
    for output in printers.iter_mut() {
        output.print_result(&lint_result, &checkers);
    }

    // export the naive asset list for future use
    if let Some(export_path) = config.export_asset_list {
        export_asset_list(export_path, &assets);
    }

    if lint_result.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

fn create_checkers(config: &Config) -> Vec<Box<dyn Checker>> {
    let mut checkers: Vec<Box<dyn Checker>> = Vec::new();
    if config.no_duplicates {
        checkers.push(Box::new(DuplicateChecker::new()));
        println!("Checking for duplicates");
    }
    if let Some(max_size) = config.max_size {
        checkers.push(Box::new(MaxSizeCheck::new(max_size)));
        println!("Checking for assets bigger than {} bytes", max_size);
    }
    if let Some(max_total_size) = config.max_total_size {
        checkers.push(Box::new(MaxTotalSizeCheck::new(max_total_size)));
        println!(
            "Checking if all the assets exceeds {} bytes",
            max_total_size
        );
    }
    if !config.no_placeholders.is_empty() {
        checkers.push(Box::new(PlaceholderChecker::new(
            config.no_placeholders.clone(),
        )));
        println!("Checking for placeholder assets");
    }

    checkers
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_creation_test_empty() {
        let config = Config {
            assets_path: None,
            no_duplicates: false,
            max_size: None,
            max_total_size: None,
            no_placeholders: Vec::new(),
            quiet: false,
            sarif: false,
            export_asset_list: None,
        };

        let checkers = create_checkers(&config);
        assert!(checkers.is_empty());
    }

    #[test]
    fn check_creation_test_half() {
        let config = Config {
            assets_path: None,
            no_duplicates: true,
            max_size: None,
            max_total_size: Some(4),
            no_placeholders: Vec::new(),
            quiet: false,
            sarif: false,
            export_asset_list: None,
        };

        let checkers = create_checkers(&config);
        assert_eq!(checkers.len(), 2);
    }

    #[test]
    fn check_creation_test_all() {
        let config = Config {
            assets_path: None,
            no_duplicates: true,
            max_size: Some(1),
            max_total_size: Some(4),
            no_placeholders: vec!["temp".to_string()],
            quiet: false,
            sarif: false,
            export_asset_list: None,
        };

        let checkers = create_checkers(&config);
        assert_eq!(checkers.len(), 4);
    }
}
