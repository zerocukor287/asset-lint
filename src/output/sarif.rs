// use https://crates.io/crates/serde-sarif and export a structured report

use std::fs::File;

use log::error;
use serde_sarif::sarif::{
    ArtifactLocation, Location, PhysicalLocation, Result, ResultLevel, Run, Sarif, ToolComponent,
    Version,
};

use crate::{
    checks::{Checker, Severity},
    output::{LintOutput, lookup_checker},
};
pub struct SarifOutput {}

/// Implement saving the `LintItem`s into a sarif file.
impl LintOutput for SarifOutput {
    fn print_result(
        &mut self,
        lint_items: &[crate::checks::LintItem],
        active_checkers: &[Box<dyn Checker>],
    ) {
        println!("Generating 'asset-lint.sarif' file");

        // create basic structure
        let mut sarif = Sarif::builder()
            .version(Version::V2_1_0.to_string())
            .schema(serde_sarif::sarif::SCHEMA_URL)
            .build();

        // register ourselves as a tool
        let mut run = Run::builder()
            .tool(ToolComponent::builder().name("asset-lint").build())
            .build();

        // iterate over the lint assets, and report as a results
        let mut results: Vec<Result> = Vec::new();
        for item in lint_items {
            // find corresponding rule
            let checker = lookup_checker(item, active_checkers);

            // build location array
            let mut locations: Vec<Location> = Vec::new();
            for path in &item.locations {
                locations.push(
                    Location::builder()
                        .physical_location(
                            PhysicalLocation::builder()
                                .artifact_location(ArtifactLocation::builder().uri(path).build())
                                .build(),
                        )
                        .build(),
                );
            }

            // serialize finding
            if let Some(rule) = checker {
                results.push(
                    Result::builder()
                        .rule_id(rule.rule_name())
                        .rule_index(item.rule_id)
                        .message(&item.text)
                        .level(rule.severity())
                        .locations(locations)
                        .build(),
                );
            } else {
                error!(
                    "Internal error: No matching rule for rule id {}",
                    item.rule_id
                );
            }
        }

        // add results if not empty
        if !results.is_empty() {
            run.results = Some(results);
        }

        // add everything above to the sarif structure
        sarif.runs.push(run);

        // create and open the sarif.json file and write the json
        if let Ok(file) = File::create("asset-lint.sarif") {
            if let Err(result) = serde_json::to_writer_pretty(file, &sarif) {
                error!("Couldn't serialize sarif file, with error: {result}");
            }
        } else {
            error!("Couldn't open sarif file");
        }
    }
}

/// Covert Severity to Sarif output
impl From<Severity> for ResultLevel {
    fn from(value: Severity) -> Self {
        match value {
            Severity::None => ResultLevel::None,
            Severity::Info => ResultLevel::Note,
            Severity::Warning => ResultLevel::Warning,
            Severity::Error => ResultLevel::Error,
        }
    }
}
