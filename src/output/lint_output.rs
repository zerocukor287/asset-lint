use crate::checks::lint_item::LintItem;

pub trait LintOutput {
    fn print_result(&mut self, result: &[LintItem]);
}
