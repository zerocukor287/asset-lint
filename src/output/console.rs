use crate::{checks::LintItem, output::LintOutput};

pub struct ConsoleOutput {}

/// Implement printing all the `LintItem`s to the console
impl LintOutput for ConsoleOutput {
    fn print_result(&mut self, results: &[LintItem]) {
        let max_findings = results.len();
        for (index, item) in results.iter().enumerate() {
            println!("\nFinding {} of {}\n\t{}", index, max_findings, item.text);
        }
        if results.is_empty() {
            println!("\nHooray! Your assets are in perfect shape.");
        }
    }
}
