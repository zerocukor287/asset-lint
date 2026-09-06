use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem, Severity},
};

pub(crate) struct FileCountCheck {
    max_size: u64,
}

impl FileCountCheck {
    pub fn new(max_size: u64) -> FileCountCheck {
        println!("Checking if the number of files is below {}", max_size);
        FileCountCheck { max_size }
    }
}

/// Implementation of the `Checker` trait to check the length of the asset's path
/// Creates a notification for every asset whose path is longer than the margin.
impl Checker for FileCountCheck {
    fn rule_id(&self) -> i64 {
        1060
    }
    fn rule_name(&self) -> String {
        String::from("file-path-length-checker")
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        if assets.len() > self.max_size as usize {
            vec![LintItem {
                text: format!(
                    "Found {} files, but the maximum amount is {}. Delete {} files to meet the criteria.",
                    assets.len(),
                    self.max_size,
                    assets.len() - self.max_size as usize
                ),
                rule_id: self.rule_id(),
                locations: Vec::new(),
                releasable_size: 0, // we are not sure how man bytes could be freed up
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
    fn test_total_file_count_greater() {
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

        let mut checker = FileCountCheck::new(5);

        let result = checker.check(&assets);

        // Yay, we are below X files
        assert!(result.is_empty());
    }

    #[test]
    fn test_total_file_count_smaller() {
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

        // checker for 240 char in path
        let mut checker = FileCountCheck::new(1);

        let result = checker.check(&assets);

        // Ohh no, we have more than X files
        assert_eq!(result.len(), 1);
    }
}
