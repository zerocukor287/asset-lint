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

    /// Check for placeholder assets
    pub no_placeholders: Vec<String>,

    /// List the biggest files
    pub list_biggest_files: Option<u64>,

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
            max_file_count: args.max_file_count.or(toml.max_file_count),
            max_filename_length: args.max_filename_length.or(toml.max_filename_length),
            max_size: args.max_size.or(toml.max_size),
            max_total_size: args.max_total_size.or(toml.max_total_size),
            list_biggest_files: args.list_biggest_files.or(toml.list_biggest_files),
            no_placeholders: args
                .no_placeholders
                .unwrap_or(toml.no_placeholders.unwrap_or_default()),
            quiet: args.quiet || toml.quiet.is_some_and(|val| val),
            sarif: args.sarif || toml.sarif.is_some_and(|val| val),
            export_asset_list: args.export_asset_list.or(toml.export_asset_list),
        }
    }
}

/// Creates config from the arguments, or the asset-lint.toml file
pub fn create_config() -> Config {
    let args = Args::parse();
    if let Ok(toml_content) = std::fs::read_to_string("./asset-lint.toml") {
        let toml: TomlConfig = toml::from_str(toml_content.as_str()).unwrap();

        return Config::resolve(args, toml);
    }
    Config {
        assets_path: None,
        no_duplicates: false,
        max_file_count: None,
        max_filename_length: None,
        max_size: None,
        max_total_size: None,
        list_biggest_files: None,
        no_placeholders: Vec::new(),
        quiet: false,
        sarif: false,
        export_asset_list: None,
    }
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
        assert!(config.quiet == false);
        assert!(config.sarif == false);
        assert!(config.export_asset_list.is_none());
    }

    #[test]
    fn test_resolve() {
        let args = Args {
            assets_path: Some("./assets/".to_string()),
            no_duplicates: true,
            max_file_count: Some(5),
            max_filename_length: None,
            max_size: None,
            max_total_size: Some(1234),
            list_biggest_files: None,
            no_placeholders: Some(vec![".*".to_string(), ".*.psd".to_string()]),
            quiet: false,
            sarif: true,
            export_asset_list: Some("./asset-lint-list.json".to_string()),
        };
        let toml = TomlConfig {
            assets_path: None,
            no_duplicates: Some(false),
            max_file_count: Some(0),
            max_filename_length: None,
            max_size: Some(14),
            max_total_size: None,
            list_biggest_files: None,
            no_placeholders: None,
            quiet: Some(true),
            sarif: Some(false),
            export_asset_list: None,
        };
        let config = Config::resolve(args, toml);

        assert!(config.assets_path.is_some()); // from arg
        assert!(config.no_duplicates); // from arg
        assert!(config.max_file_count.is_some()); // both set to value
        assert!(config.max_filename_length.is_none()); // both none
        assert!(config.max_size.is_some()); // from toml
        assert!(config.max_total_size.is_some()); // from arg
        assert!(config.list_biggest_files.is_none()); // both none
        assert!(config.no_placeholders.len() > 1); // from arg
        assert!(config.quiet); // from toml
        assert!(config.sarif); // from arg
        assert!(config.export_asset_list.is_some()); // from arg
    }
}
