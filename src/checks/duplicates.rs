//! Finds duplicates based on the hash of the content

use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem},
};

pub struct DuplicateChecker {}

impl DuplicateChecker {
    pub fn new() -> DuplicateChecker {
        DuplicateChecker {}
    }
}

/// Implementation of the `Checker` trait for finding duplicates
impl Checker for DuplicateChecker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        let size = assets.len();
        if size > 0 {
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
                        });
                    }
                }
            }
        }

        result
    }
}
