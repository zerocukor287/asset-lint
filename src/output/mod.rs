use crate::checks::lint_item::LintItem;

pub(crate) mod console;

pub trait LintOutput {
    fn print_result(&mut self, result: &[LintItem]);
}
