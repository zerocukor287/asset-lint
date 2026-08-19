use crate::{
    checks::{Checker, LintItem},
    output::{LintOutput, lookup_checker},
};

pub struct ConsoleOutput {}

/// Implement printing all the `LintItem`s to the console
impl LintOutput for ConsoleOutput {
    fn print_result(&mut self, results: &[LintItem], active_checkers: &[Box<dyn Checker>]) {
        let max_findings = results.len();
        for (index, item) in results.iter().enumerate() {
            // find corresponding rule
            if let Some(checker) = lookup_checker(item, active_checkers) {
                println!(
                    "\nFinding by {}\n{} {} of {}\n\t{}",
                    checker.rule_name(),
                    checker.severity().as_ref(),
                    index + 1, // human readable
                    max_findings,
                    item.text
                );
            } else {
                // print info to console without matching rule
                println!("\nFinding {} of {}\n\t{}", index, max_findings, item.text);
            }
        }
        if results.is_empty() {
            println!("\nHooray! Your assets are in perfect shape.");
        }
    }
}
