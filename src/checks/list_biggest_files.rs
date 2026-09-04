use crate::{
    asset_list::AssetItem,
    checks::{Checker, Severity},
};

use super::LintItem;

pub(crate) struct ListBiggestFiles {
    file_count: u64,
}

impl ListBiggestFiles {
    pub fn new(file_count: u64) -> ListBiggestFiles {
        println!("Checking for assets bigger than {} bytes", file_count);
        ListBiggestFiles { file_count }
    }
}

/// Implementation of the `Checker` trait for listing the biggest files.
/// Creates Information level notification for the biggest X files
impl Checker for ListBiggestFiles {
    fn rule_id(&self) -> i64 {
        1070
    }
    fn rule_name(&self) -> String {
        String::from("list-biggest-files")
    }
    fn severity(&self) -> Severity {
        Severity::Info
    }
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        // make a local copy of the assets
        let mut assets_copy = Vec::from(assets);

        // sort by size, biggest files first
        assets_copy.sort_by_key(|file| std::cmp::Reverse(file.size));

        // take the biggest X files
        for (index, asset_item) in assets_copy
            .iter()
            .take(self.file_count as usize)
            .enumerate()
        {
            result.push(LintItem {
                text: format!(
                    "The {} biggest asset is: {:?} with size of {} bytes",
                    index + 1,
                    asset_item.path,
                    asset_item.size
                ),
                locations: vec![
                    asset_item
                        .path
                        .clone()
                        .into_os_string()
                        .into_string()
                        .unwrap(),
                ],
                rule_id: self.rule_id(),
                releasable_size: 0, // Just an info message
            });
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
    fn test_normal_use_case() {
        let mut checker = ListBiggestFiles::new(2);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 128,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/main_character/icon.png"),
                asset_type: AssetType::Image,
                size: 1025,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/icon.png"),
                asset_type: AssetType::Image,
                size: 80,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 2);
        assert_eq!(
            results[0].locations[0],
            "assets/main_character/icon.png".to_string()
        ); // size of 1025
        assert_eq!(
            results[1].locations[0],
            "temp/temp_anim/hero_temp.png".to_string()
        ); // size of 128
    }

    #[test]
    fn test_underflow() {
        let mut checker = ListBiggestFiles::new(666);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/main_character/icon.png"),
                asset_type: AssetType::Image,
                size: 1025,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/icon.png"),
                asset_type: AssetType::Image,
                size: 80,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);

        // requested to list 666 items, but we have only 3 assets
        assert_eq!(results.len(), 3);
    }

    #[test]
    fn test_zero_items() {
        let mut checker = ListBiggestFiles::new(0);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/main_character/icon.png"),
                asset_type: AssetType::Image,
                size: 1025,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);

        // the only requirement is not to crash, and don't print anything I guess
        assert_eq!(results.len(), 0);
    }
}
