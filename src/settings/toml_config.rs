/// config structure read from asset-lint-config.toml
#[derive(serde::Deserialize, Debug, Default)]
pub struct TomlConfig {
    // make everything `Optional` here
    /// Path to check
    pub assets_path: Option<String>,

    /// Check for duplicate files
    pub no_duplicates: Option<bool>,

    /// Check for too big assets
    pub max_size: Option<u64>,

    /// Check for total size of all assets combined
    pub max_total_size: Option<u64>,

    /// Check for placeholder assets
    pub no_placeholders: Option<Vec<String>>,

    /// Minimal console output
    pub quiet: Option<bool>,

    /// SARIF output
    pub sarif: Option<bool>,

    /// Path to export naive `asset_lint_list.json`
    pub export_asset_list: Option<String>,
}
