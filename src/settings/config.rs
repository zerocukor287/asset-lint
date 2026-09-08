use clap::Parser;

use crate::settings::{args::Args, toml_config::TomlConfig};

#[derive(Default)]
pub struct Config {
    /// Path to check
    pub assets_path: Option<String>,

    /// Check for duplicate files
    pub no_duplicates: bool,

    /// Check for having no more than X files
    pub max_file_count: Option<u64>,

    /// Check for long asset paths
    pub max_filename_length: Option<u64>,

    /// Check for too big assets
    pub max_size: Option<u64>,

    /// Check for total size of all assets combined
    pub max_total_size: Option<u64>,

    /// List the biggest files
    pub list_biggest_files: Option<u64>,

    /// Check for placeholder assets
    pub no_placeholders: Vec<String>,

    /// Ignore assets for all the checks that matches these patterns
    pub ignore: Vec<String>,

    /// Minimal console output
    pub quiet: bool,

    /// SARIF output
    pub sarif: bool,

    /// Path to export naive `asset_lint_list.json`
    pub export_asset_list: Option<String>,
}

impl Config {
    pub fn new() -> Config {
        Config {
            assets_path: None,
            no_duplicates: false,
            max_file_count: None,
            max_filename_length: None,
            max_size: None,
            max_total_size: None,
            list_biggest_files: None,
            no_placeholders: Vec::new(),
            ignore: Vec::new(),
            quiet: false,
            sarif: false,
            export_asset_list: None,
        }
    }

    pub fn apply_toml(config: Config, toml: TomlConfig) -> Config {
        // apply the config from the toml file
        Config {
            assets_path: toml.assets_path.or(config.assets_path),
            no_duplicates: toml.no_duplicates.unwrap_or(config.no_duplicates),
            max_file_count: toml.max_file_count.or(config.max_file_count),
            max_filename_length: toml.max_filename_length.or(config.max_filename_length),
            max_size: toml.max_size.or(config.max_size),
            max_total_size: toml.max_total_size.or(config.max_total_size),
            list_biggest_files: toml.list_biggest_files.or(config.list_biggest_files),
            no_placeholders: toml.no_placeholders.unwrap_or(config.no_placeholders),
            ignore: toml.ignore.unwrap_or(config.ignore),
            quiet: toml.quiet.unwrap_or(config.quiet),
            sarif: toml.sarif.unwrap_or(config.sarif),
            export_asset_list: toml.export_asset_list.or(config.export_asset_list),
        }
    }

    pub fn apply_args(config: Config, args: Args) -> Config {
        // apply the config from the calling arguments
        Config {
            assets_path: args.assets_path.or(config.assets_path),
            no_duplicates: args.no_duplicates.unwrap_or(config.no_duplicates),
            max_file_count: args.max_file_count.or(config.max_file_count),
            max_filename_length: args.max_filename_length.or(config.max_filename_length),
            max_size: args.max_size.or(config.max_size),
            max_total_size: args.max_total_size.or(config.max_total_size),
            list_biggest_files: args.list_biggest_files.or(config.list_biggest_files),
            no_placeholders: args.no_placeholders.unwrap_or(config.no_placeholders),
            ignore: args.ignore.unwrap_or(config.ignore),
            quiet: args.quiet.unwrap_or(config.quiet),
            sarif: args.sarif.unwrap_or(config.sarif),
            export_asset_list: args.export_asset_list.or(config.export_asset_list),
        }
    }
}

/// Creates config from the arguments, or the asset-lint.toml file
pub fn create_config() -> Config {
    // start with an empty config
    let mut config = Config::new();

    // apply the toml if present
    if let Ok(toml_content) = std::fs::read_to_string("./asset-lint.toml") {
        let toml: TomlConfig = toml::from_str(toml_content.as_str()).unwrap();

        config = Config::apply_toml(config, toml);
    }

    // apply any calling arguments
    let args = Args::parse();
    config = Config::apply_args(config, args);

    config
}

#[cfg(test)]
mod test {
    use super::*;

    // It might be not the best test, as we could accidentally put
    // an asset-lint.toml file in the folder
    #[test]
    fn test_empty_config_creation() {
        let config = create_config();

        assert!(config.assets_path.is_none());
        assert!(config.no_duplicates == false);
        assert!(config.max_file_count.is_none());
        assert!(config.max_size.is_none());
        assert!(config.max_total_size.is_none());
        assert!(config.list_biggest_files.is_none());
        assert!(config.no_placeholders.is_empty());
        assert!(config.ignore.is_empty());
        assert!(config.quiet == false);
        assert!(config.sarif == false);
        assert!(config.export_asset_list.is_none());
    }

    #[test]
    fn test_config_toml_layering() {
        let toml = TomlConfig {
            assets_path: None,
            no_duplicates: Some(false),
            max_file_count: Some(0),
            max_filename_length: None,
            max_size: Some(14),
            max_total_size: None,
            list_biggest_files: None,
            no_placeholders: None,
            ignore: None,
            quiet: Some(true),
            sarif: Some(false),
            export_asset_list: None,
        };
        let config = Config::apply_toml(Config::new(), toml);

        assert!(config.assets_path.is_none());
        assert!(config.no_duplicates == false);
        assert!(config.max_file_count.is_some());
        assert!(config.max_filename_length.is_none());
        assert!(config.max_size.is_some());
        assert!(config.max_total_size.is_none());
        assert!(config.list_biggest_files.is_none());
        assert!(config.no_placeholders.is_empty());
        assert!(config.ignore.is_empty());
        assert!(config.quiet == true);
        assert!(config.sarif == false);
        assert!(config.export_asset_list.is_none());
    }

    #[test]
    fn test_config_arg_layering() {
        let args = Args {
            assets_path: Some("./assets/".to_string()),
            no_duplicates: Some(true),
            max_file_count: Some(5),
            max_filename_length: None,
            max_size: None,
            max_total_size: Some(1234),
            list_biggest_files: None,
            no_placeholders: Some(vec![".*".to_string(), ".*.psd".to_string()]),
            ignore: Some(vec![".*asset-lint.exe".to_string()]),
            quiet: Some(false),
            sarif: Some(true),
            export_asset_list: Some("./asset-lint-list.json".to_string()),
        };

        let config = Config::apply_args(Config::new(), args);

        assert!(config.assets_path.is_some());
        assert!(config.no_duplicates == true);
        assert!(config.max_file_count.is_some());
        assert!(config.max_filename_length.is_none());
        assert!(config.max_size.is_none());
        assert!(config.max_total_size.is_some());
        assert!(config.list_biggest_files.is_none());
        assert!(config.no_placeholders.len() >= 1);
        assert!(config.ignore.len() >= 1);
        assert!(config.quiet == false);
        assert!(config.sarif == true);
        assert!(config.export_asset_list.is_some());
    }

    #[test]
    fn test_config_layering() {
        let toml = TomlConfig {
            assets_path: None,
            no_duplicates: Some(true),
            max_file_count: None,
            max_filename_length: None,
            max_size: Some(14),
            max_total_size: None,
            list_biggest_files: None,
            no_placeholders: None,
            ignore: None,
            quiet: Some(true),
            sarif: Some(false),
            export_asset_list: None,
        };
        let args = Args {
            assets_path: Some("./assets/".to_string()),
            no_duplicates: Some(false),
            max_file_count: Some(5),
            max_filename_length: None,
            max_size: None,
            max_total_size: Some(1234),
            list_biggest_files: None,
            no_placeholders: Some(vec![".*".to_string(), ".*.psd".to_string()]),
            ignore: Some(vec![".*asset-lint.exe".to_string()]),
            quiet: Some(false),
            sarif: Some(true),
            export_asset_list: Some("./asset-lint-list.json".to_string()),
        };

        let config = Config::apply_toml(Config::new(), toml);
        let config = Config::apply_args(config, args);

        assert!(config.assets_path.is_some());
        assert!(config.no_duplicates == false); // set by toml, removed by args
        assert!(config.max_file_count.is_some()); // from arg
        assert!(config.max_filename_length.is_none()); // both none
        assert!(config.max_size.is_some()); // from toml
        assert!(config.max_total_size.is_some()); // from arg
        assert!(config.list_biggest_files.is_none()); // both none
        assert!(config.no_placeholders.len() >= 1); // from arg
        assert!(config.ignore.len() >= 1); // from arg
        assert!(config.quiet == false); // set by toml, removed by args
        assert!(config.sarif == true); // from arg
        assert!(config.export_asset_list.is_some()); // from arg
    }
}
