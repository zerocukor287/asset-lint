use clap::Parser;

use crate::settings::{args::Args, toml_config::TomlConfig};

#[derive(Default)]
pub struct Config {
    /// Path to check
    pub assets_path: Option<String>,

    /// Check for duplicate files
    pub no_duplicates: bool,

    /// Check for too big assets
    pub max_size: Option<u64>,

    /// Check for placeholder assets
    pub no_placeholders: Vec<String>,

    /// Minimal console output
    pub quiet: bool,

    /// SARIF output
    pub sarif: bool,

    /// Path to export naive `asset_lint_list.json`
    pub export_asset_list: Option<String>,
}

impl Config {
    pub fn resolve(args: Args, toml: TomlConfig) -> Config {
        // take the arguments first, and if they are missing,
        // read from the toml file
        Config {
            assets_path: args.assets_path.or(toml.assets_path),
            no_duplicates: args.no_duplicates || toml.no_duplicates.is_some_and(|val| val),
            max_size: args.max_size.or(toml.max_size),
            no_placeholders: args
                .no_placeholders
                .unwrap_or(toml.no_placeholders.unwrap_or_default()),
            quiet: args.quiet || toml.quiet.is_some_and(|val| val),
            sarif: args.sarif || toml.sarif.is_some_and(|val| val),
            export_asset_list: args.export_asset_list.or(toml.export_asset_list),
        }
    }
}

pub fn create_config() -> Config {
    let args = Args::parse();
    if let Ok(toml_content) = std::fs::read_to_string("./asset-lint.toml") {
        let toml: TomlConfig = toml::from_str(toml_content.as_str()).unwrap();

        return Config::resolve(args, toml);
    }
    Config {
        assets_path: None,
        no_duplicates: false,
        max_size: None,
        no_placeholders: Vec::new(),
        quiet: false,
        sarif: false,
        export_asset_list: None,
    }
}
