//! Print logic to export the asset list

use crate::MINIMUM_ASSET_LIST_VERSION;
use crate::asset_list::{AssetItem, AssetListJson};

use log::error;
use std::fs::File;
use std::io::Write;

pub fn export_asset_list(path: String, asset_list: &[AssetItem]) {
    let current_version: u32 = match env!("CARGO_PKG_VERSION_MINOR").parse() {
        Ok(val) => val,
        Err(_) => {
            error!("Invalid minor version");
            0
        }
    };

    let asset_file = AssetListJson {
        minimum_version: MINIMUM_ASSET_LIST_VERSION,
        current_version,
        asset_list: asset_list.to_vec(),
    };

    if let Ok(json) = serde_json::to_string(&asset_file) {
        if let Ok(mut file) = File::create(&path) {
            if let Err(err) = file.write_all(json.as_bytes()) {
                error!("Failed to write export to {}: {}", path, err);
            }
        } else {
            error!("Failed to create export file at {}", path);
        }
    } else {
        error!("Couldn't serialize asset list.");
    }
}
