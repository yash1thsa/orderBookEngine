# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2026-08-24

### Changed
- Made output path configurable via command-line argument
- Updated CLI usage to accept `<input_file> <output_path>`
- Updated examples to use configurable output path
- Fixed borrow error in basic_parsing example

## [0.1.1] - 2026-08-24

### Added
- Open source release with MIT license
- Complete documentation (README, CONTRIBUTING, CODE_OF_CONDUCT, SECURITY, CHANGELOG)
- GitHub CI/CD workflow with multi-platform builds
- Issue and PR templates for community contributions
- Proper Cargo.toml metadata for crates.io publishing

### Fixed
- Resolved all clippy warnings with appropriate allowances
- Fixed integration test type mismatch
- Updated GitHub Actions to v4 (cache, upload-artifact)
- Added success condition to artifact uploads in CI
- Fixed code formatting with cargo fmt
- Removed unused imports in main.rs

### Changed
- Updated repository URLs to actual GitHub repository
- Added clippy allowances for packed structs and module structure
- Improved CI workflow error handling

## [0.1.0] - 2026-08-24

### Added
- Initial release
- Core parsing functionality
- Arrow/Parquet integration
- Basic CLI interface
