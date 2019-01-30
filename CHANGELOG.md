# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

...

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

[Unreleased]: https://github.com/eldruin/ds1307-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/eldruin/ds1307-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/ds1307-rs/compare/v0.1.0...v0.2.0
