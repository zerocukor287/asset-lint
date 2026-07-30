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

impl Checker for MaxSizeCheck {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();
        for asset in assets {
            if asset.size > self.max_size {
                result.push(LintItem {
                    text: format!(
                        "Too big asset: {:?} size of {} exceeds {}",
                        asset.path, asset.size, self.max_size
                    ),
                });
            }
        }
        result
    }
}
