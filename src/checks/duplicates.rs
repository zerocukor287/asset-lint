use crate::{
    asset_list::AssetItem,
    checks::{Checker, lint_item::LintItem},
};

pub struct DuplicateChecker {}

impl DuplicateChecker {
    pub fn new() -> DuplicateChecker {
        DuplicateChecker {}
    }
}

impl Checker for DuplicateChecker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        let size = assets.len();
        if size > 0 {
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
