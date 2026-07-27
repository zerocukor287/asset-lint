use crate::{asset_list_builder::AssetItem, checks::lint_item::LintItem};

// `Checker` is the interface that each rule must implement
pub trait Checker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem>;
}
