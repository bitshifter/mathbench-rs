# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog].

## [0.3.2] - 2020-08-27

### Added

* Added the `static-math` library

### Changed

* Updated `glam` to `0.9.4` and changed benchmarks to use `Vec3A` instead of
  `Vec3`
* Updated `nalgebra` to `0.22.0`
* Updated `euclid` to `0.22.1`
* Updated `vek` to `0.12.0`
* Updated `pathfinder_geometry` to `0.5.1`
* Updated `ultraviolet` to `0.5.1` (currently build bench only)

## [0.3.1] - 2020-04-10

### Added

* Added `buildbench` tool for comparing build times of the math libraries
  supported by `mathbench`.

## [0.3.0] - 2019-12-03

### Added

* Added "return self" benchmarks to try establish the overhead of running a
  benchmark for each type.

### Changed

* Updated `glam` to `0.8.3` and `euclid` to `0.20.5`.
* Split 2D and 3D transform types out of 3x3 and 4x4 matrix benches into their
  own specialised transform benches.
* Run some benches over different input sizes, mostly 1 and 100 for binary
  operations, to get a better idea of a single operation versus operating over
  an array of data.
* Updated summary script to deal with input sizes and to highlight all benchmark
  times that are within a configurable threshold of the minimum time.

## [0.2.0] - 2019-11-25

### Added

* Added benchmarks for `pathfinder_geometry` `0.3.0`

### Changed

* Changed benchmarks to store output values instead of discarding them.
* Updated `euclid` to `0.20.4`

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
[Unreleased]: https://github.com/bitshifter/mathbench-rs/compare/0.3.1...HEAD
[0.3.1]: https://github.com/bitshifter/mathbench-rs/compare/0.3.0...0.3.1
[0.3.0]: https://github.com/bitshifter/mathbench-rs/compare/0.2.0...0.3.0
[0.2.0]: https://github.com/bitshifter/mathbench-rs/compare/0.1.9...0.2.0
[0.1.9]: https://github.com/bitshifter/mathbench-rs/compare/0.1.8...0.1.9
