use log::{debug, error};
use std::{env, fs, path::PathBuf};

// struct DuplicateChecker {
//     items: Vec<String>,
// }

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
