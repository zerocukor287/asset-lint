use crate::{
    checks::{Checker, LintItem},
    output::{LintOutput, lookup_checker},
};
use human_bytes::human_bytes;

pub struct ConsoleOutput {}

/// Implement printing all the `LintItem`s to the console
impl LintOutput for ConsoleOutput {
    fn print_result(&mut self, results: &[LintItem], active_checkers: &[Box<dyn Checker>]) {
        let max_findings: usize = results.len();
        let mut releasable_size_total: u64 = 0;
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
            releasable_size_total += item.releasable_size;
        }

        if releasable_size_total > 0 {
            println!(
                "Reclaimable space after solving all findings: {} bytes",
                human_bytes(releasable_size_total as f64)
            );
        }

        if results.is_empty() {
            println!("\nHooray! Your assets are in perfect shape.");
        }
    }
}
