use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    todo,
};

use log::{debug, error};
use walkdir::WalkDir;

// Types of supported assets
pub enum AssetType {
    Image,
    Sound,
    Binary,
    Text,
    Unknown,
}

// Types for supported hashing algorithms
pub enum HashType {
    Blake3,
    Unknown,
}

pub struct AssetItem {
    pub path: PathBuf,
    pub asset_type: AssetType,
    pub size: u64,
    pub hash: [u8; 32],
    pub hash_type: HashType,
}

// this function either reads a `asset_lint_list.json` or naively
// builds the asset list from the given path
pub fn read_or_build_asset_list(root: Option<String>) -> Vec<AssetItem> {
    let mut asset_list: Vec<AssetItem> = Vec::new();
    if let Some(path) = root {
        // create from folder
        println!("Building `asset_lint_list`...");
        let walk_dir = WalkDir::new(path.clone());
        for entry in walk_dir.into_iter() {
            match entry {
                Ok(path) => {
                    // add files to asset list, skip directories
                    if path.file_type().is_file() {
                        asset_list.push(AssetItem {
                            path: path.clone().into_path(),
                            asset_type: AssetType::Unknown,
                            size: path.metadata().unwrap().len(),
                            hash: claculate_hash(path.path()),
                            hash_type: HashType::Blake3,
                        });
                    }
                }
                Err(err) => {
                    error!("Path not found: {}", err);
                    break;
                }
            }
        }
        debug!("Found {} assets in {:?}", asset_list.len(), path);
    } else {
        // look for the `asset_lint_list.json` file in cwd
        todo!()
    }

    asset_list
}

// reads the file in 64 KB chunks, and calculates a hash for it
fn claculate_hash(path: &Path) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();

    if let Ok(file) = File::open(path) {
        let mut reader = BufReader::new(file);
        let mut buffer = [0; 1024 * 64];
        loop {
            if let Ok(bytes_read) = reader.read(&mut buffer) {
                if bytes_read == 0 {
                    debug!("Finished reading file {:?}", path);
                    break;
                }
                hasher.update(&buffer[..bytes_read]);
            } else {
                error!("Cannot read file content of {:?} to generate hash", path);
            }
        }
    } else {
        error!("Cannot open {:?} to generate hash", path)
    }

    hasher.finalize().into()
}
