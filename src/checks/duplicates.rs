use log::{debug, error};
use std::{env, fs, path::PathBuf};

use crate::checks::{checker::Checker, lint_item::LintItem};

pub struct DuplicateChecker {
    items: Vec<PathBuf>,
}

impl DuplicateChecker {
    pub fn new() -> DuplicateChecker {
        DuplicateChecker { items: Vec::new() }
    }

    // maybe I should use `walkdir`
    fn build_asset_list(&mut self, root: &PathBuf) {
        if let Ok(paths) = fs::read_dir(&root) {
            for path in paths {
                if let Ok(path) = path {
                    if let Ok(file_type) = path.file_type() {
                        if file_type.is_file() {
                            self.items.push(path.path());
                        } else {

                        }
                    }
                    debug!("Found asset: {}", path.path().display())
                }
            }
        }
    }

    fn scan_dir(&mut self, _path: &PathBuf) {}
}

impl Checker for DuplicateChecker {
    fn check(&mut self, path: &PathBuf) -> Vec<LintItem> {
        // recursively read dirs
        self.build_asset_list(path);

        vec![]
    }
}

pub fn check_duplicates(path: String) -> u8 {
    let absolute_path = PathBuf::from(&path);
    if let Ok(paths) = fs::read_dir(&absolute_path) {
        for path in paths {
            debug!("Found asset: {}", path.unwrap().path().display())
        }

        0
    } else {
        if let Ok(work_dir) = env::current_dir() {
            error!("Cannot find \"{}\" in {:?}", path, work_dir);
        } else {
            error!("Working directory doesn't exist, or insufficent permission to enter it");
        }
        1
    }
}
