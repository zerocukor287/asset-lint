//! This module holds all the logic behind the different checks

pub(crate) mod duplicates;
pub(crate) mod max_size;
pub(crate) mod placeholders;

use crate::asset_list::AssetItem;
use strum_macros::AsRefStr;

/// types of severity
#[derive(AsRefStr)]
pub enum Severity {
    None,
    Info,
    Warning,
    Error,
}

/// `Checker` is the interface that each rule must implement
/// It receives a non-mutable span of `AssetItem`s, and should
/// implement the logic based on that list. It must build a new
/// `Vector` of `LintItem`s of the findings.  
/// An empty result list means, the rule found no issues.
pub trait Checker {
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem>;
    fn rule_name(&self) -> String;
    fn rule_id(&self) -> i64;
    fn severity(&self) -> Severity;
}

pub struct LintItem {
    pub text: String,
    pub rule_id: i64, // this is the unique ID of the rule, used for looking up severity, name etc.
    pub locations: Vec<String>,
    pub releasable_size: u64, // this is the space that could be freed up by solving this finding
}
