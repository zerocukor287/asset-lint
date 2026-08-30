//! Finds duplicates based on the hash of the content

use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem, Severity},
};

pub struct DuplicateChecker {}

impl DuplicateChecker {
    pub fn new() -> DuplicateChecker {
        DuplicateChecker {}
    }
}

/// Implementation of the `Checker` trait for finding duplicates.
/// It finds duplicates based on the hash of the file content.
impl Checker for DuplicateChecker {
    fn rule_id(&self) -> i64 {
        1001
    }
    fn rule_name(&self) -> String {
        String::from("duplicate-checker")
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        let size = assets.len();
        // We can only check duplicates if we have 2 or more assets
        if size >= 2 {
            // compare the hashes with the rest of the list
            // I tried with a slice and contains, but didn't end up well
            // so therefore the naive iteration...
            for i in 0..(size - 1) {
                for j in (i + 1)..size {
                    if assets[i].hash == assets[j].hash {
                        result.push(LintItem {
                            text: format!(
                                "Duplicated assets: {:?} and {:?}",
                                assets[i].path, assets[j].path
                            ),
                            locations: vec![
                                assets[i]
                                    .path
                                    .clone()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap(),
                                assets[j]
                                    .path
                                    .clone()
                                    .into_os_string()
                                    .into_string()
                                    .unwrap(),
                            ],
                            rule_id: self.rule_id(),
                            releasable_size: assets[j].size,
                        });
                    }
                }
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
    fn test_duplicates() {
        let mut checker = DuplicateChecker::new();
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
                size: 1024,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_no_duplicates() {
        let mut checker = DuplicateChecker::new();
        // create two assets, with typical copy, but different hash -> no dupe
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
                size: 1024,
                hash: [1; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 0);
    }

    // This is an interesting test case. What if we have a duplicate 3 times?
    // Sure, the first will be found as a duplicate of the 2nd and 3rd.
    // But the 2nd will be also found as a duplicate of the 3rd.
    // I mean it is not wrong, but it is a duplicated Finding.
    #[test]
    fn test_triple_duplicate() {
        let mut checker = DuplicateChecker::new();
        // Ctrl+C, Ctrl+V, Ctrl+V
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [8; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [8; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy(1).png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [8; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 3); // see the comment above this test
    }
}
