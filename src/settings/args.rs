use clap_derive::Parser;

/// Structure to define the possible command line parameters
#[derive(Parser, Debug)]
#[command(name = "asset-lint")]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Path to check
    #[arg(long)]
    pub assets_path: Option<String>,

    /// Check for duplicate files
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub no_duplicates: bool,

    /// Check for too big assets
    #[arg(long)]
    pub max_size: Option<u64>,

    /// Check for placeholder assets
    #[arg(long)]
    pub no_placeholders: Option<Vec<String>>,

    /// Minimal console output
    #[arg(long, default_value_t = false)]
    pub quiet: bool,

    /// SARIF output
    #[arg(long, default_value_t = false)]
    pub sarif: bool,

    /// Path to export naive `asset_lint_list.json`
    #[arg(long)]
    pub export_asset_list: Option<String>,
}
