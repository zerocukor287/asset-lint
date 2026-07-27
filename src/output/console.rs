use crate::{checks::lint_item::LintItem, output::lint_output::LintOutput};

pub struct ConsoleOutput {}

impl LintOutput for ConsoleOutput {
    fn print_result(&mut self, results: &[LintItem]) {
        println!("Found problems:");
        for item in results {
            println!("{}", item.text);
        }
    }
}
