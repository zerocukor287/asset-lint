//! This module builds the `asset_list` that holds all
//! the assets that are found in the specified path.

use std::path::PathBuf;

pub(crate) mod builder;

/// Types of supported assets
#[derive(Debug, PartialEq)]
pub enum AssetType {
    Image,
    Sound,
    Text,
    Unknown,
}

/// `AssetItem` is the structure that every rule use
pub struct AssetItem {
    /// Relative path to the asset
    pub path: PathBuf,
    /// Type of the asset
    pub asset_type: AssetType,
    /// Size in bytes
    pub size: u64,
    /// Hash of the content
    pub hash: [u8; 32],
}
