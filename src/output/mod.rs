//! Output module holds all the code that can print the result of
//! a linter pass in different forms. The most basic is simply printing
//! to the console (stdout).

use crate::checks::{Checker, LintItem};

pub(crate) mod console;
pub(crate) mod sarif;

/// Each output adapter needs to implement `LintOutput` trait.
/// The implementors not allowed to change the `LintItem` span,
/// but can mutate their own state.
pub trait LintOutput {
    fn print_result(&mut self, result: &[LintItem], active_checkers: &[Box<dyn Checker>]);
}

/// Returns the Checker that created this LintItem
fn lookup_checker<'a>(
    lint_item: &LintItem,
    active_checkers: &'a [Box<dyn Checker>],
) -> Option<&'a dyn Checker> {
    active_checkers
        .iter()
        .find(|&checker| checker.rule_id() == lint_item.rule_id)
        .map(|boxed| boxed.as_ref())
}
