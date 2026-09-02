//! Warns if any placeholders are found among the assets

use log::debug;
use regex::Regex;

use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem, Severity},
};

pub struct PlaceholderChecker {
    patterns: Vec<Regex>,
}

/// Implementation of the `Checker` trait for finding placeholder assets.
/// It works based on the file path, and matches against regex
impl PlaceholderChecker {
    pub fn new(pattern: Vec<String>) -> PlaceholderChecker {
        println!("Checking for placeholder assets");
        PlaceholderChecker {
            patterns: pattern
                .into_iter()
                .filter_map(|arg0: String| Regex::new(&arg0).ok())
                .collect(),
        }
    }
}

impl Checker for PlaceholderChecker {
    fn rule_id(&self) -> i64 {
        1030
    }
    fn rule_name(&self) -> String {
        String::from("placeholder-checker")
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }

    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();
        debug!("Found {} regex patterns", self.patterns.len());
        for asset in assets {
            if let Ok(path_part) = asset.path.clone().into_os_string().into_string() {
                debug!("Checking {}", path_part);
                if self.patterns.iter().any(|p| p.is_match(&path_part)) {
                    result.push(LintItem {
                        text: format!("Found placeholder asset: {:?}", asset.path),
                        locations: vec![asset.path.clone().into_os_string().into_string().unwrap()],
                        rule_id: self.rule_id(),
                        releasable_size: 0, // don't know what to do with the placeholder
                    });
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
    fn test_placeholder_found() {
        let mut checker = PlaceholderChecker::new(vec!["placeholder_.*".to_string()]);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("assets/placeholder_hero.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/real_background.png"),
                asset_type: AssetType::Image,
                size: 2048,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 1);
        assert!(results[0].text.contains("placeholder_hero.png"));
    }

    #[test]
    fn test_no_placeholder_found() {
        let mut checker = PlaceholderChecker::new(vec!["placeholder_.*".to_string()]);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("assets/hero.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/background.png"),
                asset_type: AssetType::Image,
                size: 2048,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);
        assert_eq!(results.len(), 0);
    }

    #[test]
    fn test_multiple_placeholder_match_in_path() {
        let mut checker = PlaceholderChecker::new(vec!["temp.*".to_string()]);
        let assets = vec![AssetItem {
            path: PathBuf::from("temp/temp_anim/hero_temp.blend"),
            asset_type: AssetType::Image,
            size: 1024,
            hash: [0; 32],
        }];

        let results = checker.check(&assets);
        // we have only one asset, but each part of the path matches the "temp"
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_multiple_placeholder_matcher() {
        let mut checker = PlaceholderChecker::new(vec![
            ".*temp.*".to_string(),
            "placeholder.*".to_string(),
            "new_feature.*".to_string(),
            "unreleased.*".to_string(),
            ".*ReplaceMe.*".to_string(),
        ]);
        let assets = vec![
            AssetItem {
                path: PathBuf::from("assets/hero_final.gltf"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/hero_ReplaceMe.glb"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets_temp/hero_v1.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("unreleased-assets/hero_v3_good.kra"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("unreleased-placeholder-assets/hero_v8_saturated.psd"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/new_feature/hero_v8_saturated.jpeg"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
        ];

        let results = checker.check(&assets);
        // we have a match for all, except the first
        assert_eq!(results.len(), 5);
    }
}
