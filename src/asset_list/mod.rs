use std::path::PathBuf;

pub(crate) mod builder;

// Types of supported assets
pub enum AssetType {
    Image,
    Sound,
    Binary,
    Text,
    Unknown,
}

pub struct AssetItem {
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: u64,
    pub hash: [u8; 32],
}
