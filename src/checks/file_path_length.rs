use crate::{
    asset_list::AssetItem,
    checks::{Checker, LintItem, Severity},
};

pub(crate) struct FilePathLengthCheck {
    max_size: u64,
}

impl FilePathLengthCheck {
    pub fn new(max_size: u64) -> FilePathLengthCheck {
        println!(
            "Checking for asset paths longer than {} character",
            max_size
        );
        FilePathLengthCheck { max_size }
    }
}

/// Implementation of the `Checker` trait to check the length of the asset's path
/// Creates a notification for every asset whose path is longer than the margin.
impl Checker for FilePathLengthCheck {
    fn rule_id(&self) -> i64 {
        1050
    }
    fn rule_name(&self) -> String {
        String::from("file-path-length-checker")
    }
    fn severity(&self) -> Severity {
        Severity::Warning
    }
    fn check(&mut self, assets: &[AssetItem]) -> Vec<LintItem> {
        let mut result: Vec<LintItem> = Vec::new();

        for asset in assets {
            if let Ok(path_part) = asset.path.clone().into_os_string().into_string() {
                if path_part.len() > self.max_size as usize {
                    result.push(LintItem {
                        text: format!(
                            "Asset path is too long, {} exceeds the maximum of {} for asset:\n{:?}",
                            path_part.len(),
                            self.max_size,
                            asset.path
                        ),
                        locations: vec![asset.path.clone().into_os_string().into_string().unwrap()],
                        rule_id: self.rule_id(),
                        releasable_size: 0, // size stays, path shrinks
                    });
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test {
    use std::path::PathBuf;

    use crate::asset_list::AssetType;

    use super::*;

    #[test]
    fn test_path_length_greater() {
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy.png"),
                asset_type: AssetType::Image,
                size: 1234,
                hash: [1; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/game.exe"),
                asset_type: AssetType::Unknown,
                size: 12345678,
                hash: [2; 32],
            },
        ];

        let mut checker = FilePathLengthCheck::new(30);

        let result = checker.check(&assets);

        // generated one result
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_total_size_smaller() {
        let assets = vec![
            AssetItem {
                path: PathBuf::from("temp/temp_anim/hero_temp.png"),
                asset_type: AssetType::Image,
                size: 1024,
                hash: [0; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/temp_anim/hero_temp - Copy.png"),
                asset_type: AssetType::Image,
                size: 1234,
                hash: [1; 32],
            },
            AssetItem {
                path: PathBuf::from("assets/game.exe"),
                asset_type: AssetType::Unknown,
                size: 12345678,
                hash: [2; 32],
            },
        ];

        // checker for 240 char in path
        let mut checker = FilePathLengthCheck::new(240);

        let result = checker.check(&assets);

        // No warnings
        assert!(result.is_empty());
    }
}
