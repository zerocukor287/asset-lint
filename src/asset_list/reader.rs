use std::fs::{self};

use log::error;

use crate::{
    MINIMUM_ASSET_LIST_VERSION,
    asset_list::{AssetItem, AssetListJson},
};

/// Reads the `asset_lint_list.json`, does the version check
/// and extracts the `asset_list` from the json file
pub fn read_asset_lint_list_file() -> Vec<AssetItem> {
    let mut asset_list = Vec::new();

    let current_version: u32 = match env!("CARGO_PKG_VERSION_MINOR").parse() {
        Ok(val) => val,
        Err(_) => {
            error!("Invalid minor version");
            0
        }
    };

    if let Ok(file_content) = fs::read_to_string("asset-lint-list.json") {
        let asset_result: Result<AssetListJson, _> = serde_json::from_str(&file_content);
        if let Ok(asset_file) = asset_result {
            // Check versions
            if asset_file.minimum_version < MINIMUM_ASSET_LIST_VERSION {
                error!("'asset_lint_list' file is too old. Regenerate with an updated application.")
            } else if current_version < asset_file.minimum_version {
                error!(
                    "Your 'asset-lint' is outdated! Update it to a newer version to use this 'asset_lint_list' file."
                );
            } else {
                // versions are good, get the asset list
                asset_list = asset_file.asset_list;
            }
        }
    } else {
        error!("Couldn't find 'asset-lint-list.json' in the working directory");
    }
    asset_list
}
