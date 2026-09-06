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

    /// Check for having no more than X files
    #[arg(long)]
    pub max_file_count: Option<u64>,

    /// Check for long asset paths
    #[arg(long)]
    pub max_filename_length: Option<u64>,

    /// Check for too big assets
    #[arg(long)]
    pub max_size: Option<u64>,

    /// Check for total size of all assets combined
    #[arg(long)]
    pub max_total_size: Option<u64>,

    /// List the biggest files
    #[arg(long)]
    pub list_biggest_files: Option<u64>,

    /// Check for placeholder assets
    #[arg(long, num_args = 1..)]
    pub no_placeholders: Option<Vec<String>>,

    /// Ignore assets for all the checks that matches these patterns
    #[arg(long, num_args = 1..)]
    pub ignore: Option<Vec<String>>,

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
