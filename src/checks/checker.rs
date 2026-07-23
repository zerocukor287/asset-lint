use crate::checks::lint_item::LintItem;
use std::path::PathBuf;

pub trait Checker {
    fn check(&mut self, path: &PathBuf) -> Vec<LintItem>;
}
