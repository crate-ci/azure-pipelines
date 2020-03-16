# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4] - 2020-03-16
### Changed
- Ubuntu and macOS images updated to the latest available (18.04 and
  10.15 respectively).

## [0.3] - 2019-12-28
### Added
- `default.yml` opinionated default CI for "regular" Rust crates.
- `install-rust.yml` task for installing Rust

### Changed

### Removed
- Everything in `azure/`, pretty much all customization options.
  The intention is that each project now either uses `defaults.yml` or
  copy-pastes it into their own `azure-pipelines.yml` and modifies it as
  needed.
  See [#73](https://github.com/crate-ci/azure-pipelines/pull/73).

## [0.2] - 2019-10-18
### Added

### Changed
- Features changed from being a string to being a list ([#65](https://github.com/crate-ci/azure-pipelines/pull/65))

[0.4]: https://github.com/crate-ci/azure-pipelines/compare/v0.3...v0.4
[0.3]: https://github.com/crate-ci/azure-pipelines/compare/v0.2...v0.3
[0.2]: https://github.com/crate-ci/azure-pipelines/compare/v0.1...v0.2
