//! Warns if any placeholders are found among the assets

use regex::Regex;

use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem},
};

pub struct PlaceholderChecker {
    patterns: Vec<Regex>,
}

impl PlaceholderChecker {
    pub fn new(pattern: Vec<String>) -> PlaceholderChecker {
        PlaceholderChecker {
            patterns: pattern
                .into_iter()
                .filter_map(|arg0: String| Regex::new(&arg0).ok())
                .collect(),
        }
    }
}

impl Checker for PlaceholderChecker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        for asset in assets {
            for path_part in asset.path.iter() {
                if self
                    .patterns
                    .iter()
                    .any(|p| p.is_match(&path_part.to_string_lossy()))
                {
                    result.push(LintItem {
                        text: format!("Found placeholder asset: {:?}", asset.path),
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
}
