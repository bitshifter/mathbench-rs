# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog].

## [Unreleased]

### Added
* Added this `CHANGELOG.md` file!

### Changed
* `cgmath`, `euclid` and `nalgebra` are now all optional features. They can be
  excluded from benchmarks using `cargo bench --no-default-features`.
* Updated `glam` to `0.8.2`
* Updated `nalgebra` to `0.19.0`
* Updated `euclid` to `0.20.3`
* Updated `criterion` to `0.3.0`
* Benchmarks now use `criterion`'s `BenchmarkGroup` struct. The old way of
  benchmarking functions has been silently deprecated in `0.3.0`.

[Keep a Changelog]: https://keepachangelog.com/
[Unreleased]: https://github.com/bitshifter/mathbench-rs/compare/0.1.8...HEAD
