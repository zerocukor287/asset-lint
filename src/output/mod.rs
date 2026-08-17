//! Output module holds all the code that can print the result of
//! a linter pass in different forms. The most basic is simply printing
//! to the console (stdout).

use crate::checks::LintItem;

pub(crate) mod console;
pub(crate) mod sarif;

/// Each output adapter needs to implement `LintOutput` trait.
/// The implementors not allowed to change the `LintItem` span,
/// but can mutate their own state.
pub trait LintOutput {
    fn print_result(&mut self, result: &[LintItem]);
}
