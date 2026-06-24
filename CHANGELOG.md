# Changelog

All notable changes to this project will be documented in this file.

## [0.4.0] - 2026-06-23

### Added

- Reader/Writer API for streaming serialization/deserialization:
  - `XCfg::load_from_reader`, `XCfg::save_to_writer`
  - `File::from_reader_with_fmt`, `File::save_to_writer`
  - `Format::deserialize_from_reader`, `Format::serialize_to_writer`
- `xcfg-derive` now supports generic structs and structs with lifetimes.
- `xcfg-derive` now validates that `#[derive(XCfg)]` is only used on structs.
- Added `rustfmt.toml` and workspace-wide lint configuration.
- Added GitHub Actions CI.
- Added trybuild UI tests for `xcfg-derive`.

### Changed

- **Breaking**: `Error::InvalidPath` now carries `{ path, reason }`.
- **Breaking**: `Error::UnknownFileFormat` now carries `{ path }`.
- `File::to_string` is deprecated in favor of `File::serialize_to_string`.
- `File::any_load` now uses deterministic format priority (TOML > YAML > JSON).
- `File::any_load` directory traversal is optimized to reduce stat calls and allocations.
- Internal module structure refactored: `format.rs` split into `file.rs`, `xcfg.rs`, and `format.rs`.

### Fixed

- Fixed `--all-features` compilation failure caused by missing `'static` bound on YAML deserialization.
- Fixed incorrect `#[cfg(feature = "toml")]` in `json_test.rs` and `load.rs`.
- Fixed `xcfg-derive` dependency lacking a local `path`, which caused local changes to be ignored.
- Fixed workspace `repository` URL pointing to the wrong project.
- Fixed README inconsistencies (crate name, example command, spelling).

## [0.3.4] - Previous release

- Initial public releases with TOML/YAML/JSON support and `#[derive(XCfg)]`.
