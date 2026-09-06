//! Reads or builds the `asset_list`

use std::{
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use log::{debug, error};
use regex::Regex;
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

pub fn apply_ignore_list(all_assets: Vec<AssetItem>, ignore_patterns: &[String]) -> Vec<AssetItem> {
    let mut filtered_assets = Vec::<AssetItem>::new();
    let asset_count = all_assets.len();

    // create the patterns
    let patterns: Vec<Regex> = ignore_patterns
        .iter()
        .filter_map(|arg0: &String| Regex::new(arg0).ok())
        .collect();

    // filter out everything that is in the ignore list
    for asset in all_assets {
        if let Ok(path_part) = asset.path.clone().into_os_string().into_string() {
            debug!("Checking {}", path_part);
            if !patterns.iter().any(|p| p.is_match(&path_part)) {
                filtered_assets.push(asset);
            }
        }
    }

    println!(
        "Ignore filtered out {} assets",
        asset_count - filtered_assets.len()
    );
    filtered_assets
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
    use std::path::PathBuf;

    use super::*;
    use crate::asset_list::AssetType;

    #[test]
    fn test_type() {
        assert_eq!(guess_type(Path::new("./asset-lint.png")), AssetType::Image);
        assert_eq!(guess_type(Path::new("./asset-lint.mp3")), AssetType::Sound);
        assert_eq!(guess_type(Path::new("./asset-lint.toml")), AssetType::Text);
        assert_eq!(guess_type(Path::new("./asset/")), AssetType::Unknown);
    }

    #[test]
    fn test_ignore_list_filter_this_binary() {
        let assets = vec![AssetItem {
            path: PathBuf::from("./asset-lint.exe"),
            asset_type: AssetType::Unknown,
            size: 57845,
            hash: [3; 32],
        }];

        let ignore_list = vec![".*asset-lint.exe".to_string()];

        let filtered_assets = apply_ignore_list(assets, &ignore_list);

        assert!(filtered_assets.is_empty());
    }

    #[test]
    fn test_ignore_list_filter_all() {
        let assets = vec![AssetItem {
            path: PathBuf::from("./asset-lint.exe"),
            asset_type: AssetType::Unknown,
            size: 57845,
            hash: [3; 32],
        }];

        let ignore_list = vec![".*".to_string()];

        let filtered_assets = apply_ignore_list(assets, &ignore_list);

        assert!(filtered_assets.is_empty());
    }

    #[test]
    fn test_ignore_list_filter_some() {
        let assets = vec![
            AssetItem {
                path: PathBuf::from("./asset-lint.exe"),
                asset_type: AssetType::Unknown,
                size: 57845,
                hash: [3; 32],
            },
            AssetItem {
                path: PathBuf::from("./asset-lint.png"),
                asset_type: AssetType::Unknown,
                size: 57845,
                hash: [3; 32],
            },
            AssetItem {
                path: PathBuf::from("./asset-lint.ico"),
                asset_type: AssetType::Unknown,
                size: 57845,
                hash: [3; 32],
            },
            AssetItem {
                path: PathBuf::from("./asset-lint.sh"),
                asset_type: AssetType::Unknown,
                size: 57845,
                hash: [3; 32],
            },
        ];

        assert_eq!(assets.len(), 4);

        // filter all exe
        let ignore_list = vec![".*.exe".to_string()];

        let filtered_assets = apply_ignore_list(assets, &ignore_list);

        // previously we had 4 assets, but after the filter we have only 3
        assert_eq!(filtered_assets.len(), 3);
    }
}
