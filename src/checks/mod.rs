//! This module holds all the logic behind the different checks

pub(crate) mod duplicates;

use crate::asset_list::AssetItem;

/// `Checker` is the interface that each rule must implement
/// It receives a non-mutable span of `AssetItem`s, and should
/// implement the logic based on that list. It must build a new
/// `Vector` of `LintItem`s of the findings.  
/// An empty result list means, the rule found no issues.
pub trait Checker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem>;
}

pub struct LintItem {
    pub text: String,
}
