use crate::{checks::LintItem, output::LintOutput};

pub struct ConsoleOutput {}

/// Implement printing all the `LintItem`s to the console
impl LintOutput for ConsoleOutput {
    fn print_result(&mut self, results: &[LintItem]) {
        println!("Found problems:");
        for item in results {
            println!("{}", item.text);
        }
    }
}
