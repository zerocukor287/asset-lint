use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem, Severity},
};
use human_bytes::human_bytes;

pub(crate) struct MaxTotalSizeCheck {
    max_size: u64,
}

impl MaxTotalSizeCheck {
    pub fn new(max_size: u64) -> MaxTotalSizeCheck {
        MaxTotalSizeCheck { max_size }
    }
}

/// Implementation of the `Checker` trait to check if the total size is below the margin.
/// Notifies if the size of all asset is bigger than the defined margin.
impl Checker for MaxTotalSizeCheck {
    fn rule_id(&self) -> i64 {
        1030
    }
    fn rule_name(&self) -> String {
        String::from("max-size-checker")
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let total_size: u64 = assets.iter().map(|item| item.size).sum();
        if total_size > self.max_size {
            vec![LintItem {
                text: format!(
                    "The total size of assets {} exceeding the limit of {}. Assets need to be reduced by {}",
                    total_size,
                    self.max_size,
                    human_bytes((total_size - self.max_size) as f64)
                ),
                rule_id: self.rule_id(),
                locations: Vec::new(),
                releasable_size: 0, // keep it 0 now.
            }]
        } else {
            Vec::new()
        }
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::asset_list::AssetType;

    use super::*;

    #[test]
    fn test_total_size_greater() {
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
            AssetItem {
                path: PathBuf::from("assets/game.exe"),
                asset_type: AssetType::Unknown,
                size: 12345678,
                hash: [2; 32],
            },
        ];

        let mut checker = MaxTotalSizeCheck::new(2048);

        let result = checker.check(&assets);

        // generated one result, because 1024 + 1234 > 2048
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_total_size_smaller() {
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
            AssetItem {
                path: PathBuf::from("assets/game.exe"),
                asset_type: AssetType::Unknown,
                size: 67108864, // 64 MB
                hash: [2; 32],
            },
        ];

        // checker of 500 MB
        let mut checker = MaxTotalSizeCheck::new(524288000);

        let result = checker.check(&assets);

        // No warnings
        assert!(result.is_empty());
    }
}
