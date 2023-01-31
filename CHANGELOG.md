# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Default variant for `SqwOutRate` and `SqwOutLevel`.

### Changed
- Updated internal use of `chrono`.
- Updated `embedded-hal-mock` dev-dependency.
- Updated MSRV to 1.62.0.

### Fixed
- Swapped implementations of `set_running` and `halt` and fixed output level of `running`.

## [0.4.0] - 2022-02-19

### Changed
- [breaking-change] Update `rtcc` to version `0.3`.
- [breaking-change] Remove `get_` from all public method names to comply with the Rust API guidelines.

## [0.3.0] - 2020-05-04

### Added
- Rust 1.31.0 is now the Minimum Supported Rust Version.

### Changed
- [breaking-change] Rename `DS1307` struct `Ds1307` for conformance with Rust
  naming conventions.
- [breaking-change] Renamed method `is_square_wave_output_enabled` ->
  `square_wave_output_enabled` for conformance with Rust naming conventions.
- [breaking-change] Renamed method `is_running` -> `running` for conformance
  with Rust naming conventions.
- [breaking-change] Method `set_square_wave_output_rate` sets the value with
  disregard of the previous configuration status.
- [breaking-change] Method `set_square_wave_output_rate` now takes a
  `SqwOutRate` parameter.
- [breaking-change] Method `set_square_wave_output_level` now takes a
  `SqwOutLevel` parameter.
- [breaking-change] Removed method `set_square_wave_output_level_high`.
- [breaking-change] Removed method `set_square_wave_output_level_low`.
- Use edition 2018.

### Fixed
- Setting date and time at once.
- Setting day of month.

## [0.2.1] - 2019-01-30

### Fixed
- Day of month validity check.

### Changed
- `Hours`, `DateTime` and `SQWOUTRateBits` are now `Copy` and `PartialEq`.

## [0.2.0] - 2018-09-07

This crate is now functionally complete.

### Added
- Enable/disable clock.
- Access to user RAM.
- Controlling square-wave output.

## 0.1.0 - 2018-08-15

This is the initial release to crates.io.

[Unreleased]: https://github.com/eldruin/ds1307-rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/eldruin/ds1307-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/eldruin/ds1307-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/eldruin/ds1307-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/ds1307-rs/compare/v0.1.0...v0.2.0
