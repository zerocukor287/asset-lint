//! Checks the assets and lists which one exceeds the defined size

use crate::{asset_list::AssetItem, checks::Checker};

use super::LintItem;

pub(crate) struct MaxSizeCheck {
    max_size: u64,
}

impl MaxSizeCheck {
    pub fn new(max_size: u64) -> MaxSizeCheck {
        MaxSizeCheck { max_size }
    }
}

/// Implementation of the `Checker` trait for finding huge files.
/// Notifies if an asset is bigger than the defined margin.
impl Checker for MaxSizeCheck {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();
        for asset in assets {
            if asset.size > self.max_size {
                result.push(LintItem {
                    text: format!(
                        "Too big asset: {:?} size of {} bytes exceeds {} bytes",
                        asset.path, asset.size, self.max_size
                    ),
                });
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_list::AssetType;
    use std::path::PathBuf;

    #[test]
    fn test_small_files() {
        let mut checker = MaxSizeCheck::new(2048);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy.png"),
                asset_type: AssetType::Image,
                size: 1234,
                hash: [1; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_big_files() {
        let mut checker = MaxSizeCheck::new(2048);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 11024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy.png"),
                asset_type: AssetType::Image,
                size: 51234,
                hash: [1; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 2);
    }
}
