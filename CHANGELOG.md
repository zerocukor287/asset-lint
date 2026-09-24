# Changelog

These are the changes that happened to `Asset-lint` between the versions.  
This format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/).

## 0.1.3
### Fixed
- Fix `--sarif` when no errors or warnings are found

## 0.1.2
### Fixed
- Starting the application without `.toml` config
- Fix the naming of for `--max-file-count`

## 0.1.1
### Added
- Implement checks: `--max-file-count`, `--max-filename-length`, `--max-total-size`
- Implement global ignore list
- Listing biggest files as info
- Implement `asset-lint.toml` setting file

### Fixed
- Readme on crates.io
- Calling parameters for `--no-placeholders`

## 0.1.0
### Added
- Implement checking for duplicates, big files, and placeholder images
- Add console and SARIF output
- Naively create `asset_lint_list.json` file
- Total reclaimable size printed to console