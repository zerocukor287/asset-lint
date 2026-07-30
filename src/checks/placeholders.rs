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
