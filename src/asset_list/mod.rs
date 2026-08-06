//! This module builds the `asset_list` that holds all
//! the assets that are found in the specified path.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub(crate) mod builder;
pub(crate) mod exporter;
pub(crate) mod reader;

/// Types of supported assets
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum AssetType {
    Image,
    Sound,
    Text,
    Unknown,
}

/// `AssetItem` is the structure that every rule use
#[derive(Clone, Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct AssetListJson {
    minimum_version: u32,
    current_version: u32,
    asset_list: Vec<AssetItem>,
}
