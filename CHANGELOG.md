# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog].

## [Unreleased]
### Added
* Added benchmarks for `pathfinder_geometry` `0.3.0`
* Updated `euclid` to `0.20.4`

### Changed
* Use Criterion `Bencher` `iter_batched` instead of `iter` for benchmarks. This
  changes a lot of the results but I think they are more accurate as output is
  being stored.

## [0.1.9] - 2019-11-13
### Added
* Added this `CHANGELOG.md` file!
* Added benchmarks for `vek` `0.9.10`

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
[Unreleased]: https://github.com/bitshifter/mathbench-rs/compare/0.1.9...HEAD
[0.1.9]: https://github.com/bitshifter/mathbench-rs/compare/0.1.8...0.1.9
