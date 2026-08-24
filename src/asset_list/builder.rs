//! Reads or builds the `asset_list`

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use log::{debug, error};
use walkdir::WalkDir;

use crate::asset_list::{AssetItem, AssetType, reader::read_asset_lint_list_file};

/// this function either reads a `asset_lint_list.json` or naively
/// builds the asset list from the given path.
/// The naive version decides the asset type based on the extension
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
                            asset_type: guess_type(path.path()),
                            size: path.metadata().unwrap().len(),
                            hash: calculate_hash(path.path()),
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
        println!("Reading `asset_lint_list.json`...");
        asset_list = read_asset_lint_list_file();
    }

    asset_list
}

// reads the file in 64 KB chunks, and calculates a hash for it
fn calculate_hash(path: &Path) -> [u8; 32] {
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

// categorize assets based on the extension
fn guess_type(path: &Path) -> AssetType {
    if let Some(raw_extension) = path.extension()
        && let Some(extension) = raw_extension.to_str()
    {
        // shadow the extension with lowercase version
        let extension = extension.to_lowercase();

        // here we can do the checks
        let image_extensions = ["png", "jpg", "jpeg", "tga"];
        if image_extensions.contains(&extension.as_str()) {
            return AssetType::Image;
        }

        let audio_extensions = ["mp3", "wav", "ogg"];
        if audio_extensions.contains(&extension.as_str()) {
            return AssetType::Sound;
        }

        let text_extensions = ["txt", "json", "toml", "xml", "html"];
        if text_extensions.contains(&extension.as_str()) {
            return AssetType::Text;
        }
    }

    // we have no idea what is it (or no extension at all!)
    AssetType::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asset_list::AssetType;

    #[test]
    fn test_type() {
        assert_eq!(guess_type(Path::new("./asset-lint.png")), AssetType::Image);
        assert_eq!(guess_type(Path::new("./asset/")), AssetType::Unknown);
    }
}
