# mathbench

[![Build Status]][travis-ci]

`mathbench` is a suite of unit tests and benchmarks comparing the output and
performance of a number of different Rust linear algebra libraries for common
game and graphics development tasks.

`mathbench` is written by the author of [`glam`][glam] and has been used to
compare the performance of `glam` with other similar 3D math libraries targeting
games and graphics development, including:

* [`cgmath`][cgmath]
* [`nalgebra`][nalgebra]
* [`euclid`][euclid]
* [`vek`][vek]
* [`pathfinder_geometry`][pathfinder_geometry]

## The benchmarks

All benchmarks are performed using [Criterion.rs].

* `euler bench` - performs an Euler integration on arrays of 2D and 3D vectors
* `mat benches` - performs common matrix operations such as transpose, inverse,
  determinant and multiply
* `quat benches` - perform common quaternion operations
* `transform bench` - performs transformations on vectors

The benchmarks are currently focused on `f32` types as that is all `glam`
currently supports.

Different libraries have different features and different ways of achieving the
same goal. For the purpose of trying to get a performance comparison sometimes
`mathbench` compares similar functionality, but sometimes it's not exactly the
same. Below is a list of differences between libraries that are notable for
performance comparisons.

### Matrices versus transforms

The `euclid` library does not support generic square matrix types like the other
libraries tested. Rather it has 2D and 3D transform types which can transform 2D
and 3D vector and point types. Each library has different types for supporting
transforms but `euclid` is unique amongst the libraries tested in that is
doesn't have generic square matrix types.

The `Transform2D` is stored as a 3x2 row major matrix that can be used to
transform 2D vectors and points. For the purposes of `mathbench` this is
compared against 3x3 homogeneous matrices, but the 3x2 matrix can skip some
computation so it has an advantage there.

Similarly `Transform3D` is used for transforming 3D vectors and points. This
is represented as a 4x4 matrix so it is more directly comparable to the other
libraries however it doesn't support some operations like transpose.

There is no equivalent to a 2x2 matrix type in `euclid`.


### Matrix inverse

Note that `cgmath` and `nalgebra` matrix inverse methods return an `Option`
whereas `glam` and `euclid` do not. If a
non-invertible matrix is inverted by `glam` or `euclid` the result will be invalid (it will
contain NaNs).

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath`, `nalgebra`, `euclid` and `vek` on `f32` data.

| benchmark              |         glam   |       cgmath   |     nalgebra   |       euclid   |           vek   |   pathfinder   |
|:-----------------------|---------------:|---------------:|---------------:|---------------:|----------------:|---------------:|
| euler 2d               |     8.767 us   |     8.713 us   |     20.63 us   |     13.34 us   |    __8.413 us__ |     11.25 us   |
| euler 3d               |   __15.49 us__ |     28.92 us   |     86.47 us   |     29.77 us   |       28.9 us   |     16.16 us   |
| mat2 determinant       |  __2.0364 ns__ |    2.0426 ns   |    2.0453 ns   |      N/A       |     2.0466 ns   |    2.0711 ns   |
| mat2 inverse           |  __2.5694 ns__ |    2.9510 ns   |    2.9472 ns   |      N/A       |       N/A       |    2.7693 ns   |
| mat2 mul mat2          |    2.5989 ns   |    2.7197 ns   |    4.0973 ns   |      N/A       |    20.3607 ns   |  __2.5301 ns__ |
| mat2 transform vector2 |    2.6697 ns   |    2.6576 ns   |    6.7453 ns   |      N/A       |     8.4590 ns   |  __2.1183 ns__ |
| mat2 transpose         |    2.0850 ns   |  __2.0816 ns__ |    2.6265 ns   |      N/A       |     2.0844 ns   |      N/A       |
| mat3 determinant       |    2.7516 ns   |    3.4176 ns   |    3.3756 ns   |  __2.0323 ns__ |     3.4497 ns   |      N/A       |
| mat3 inverse           |    9.7384 ns   |    8.8430 ns   |    9.7633 ns   |    4.8761 ns   |       N/A       |  __4.0245 ns__ |
| mat3 mul mat3          |    5.4994 ns   |    8.1237 ns   |    9.8879 ns   |    5.0860 ns   |   135.3526 ns   |  __3.8668 ns__ |
| mat3 transform point2  |  __2.7727 ns__ |    2.8822 ns   |    7.7018 ns   |    3.2579 ns   |    26.3885 ns   |      N/A       |
| mat3 transform vector2 |    3.0376 ns   |      N/A       |    3.7496 ns   |    2.8098 ns   |    26.6436 ns   |  __2.3009 ns__ |
| mat3 transform vector3 |  __2.9091 ns__ |    4.3690 ns   |    9.1395 ns   |      N/A       |    26.6590 ns   |      N/A       |
| mat3 transpose         |  __4.9576 ns__ |    5.7245 ns   |   10.5911 ns   |      N/A       |     5.7280 ns   |      N/A       |
| mat4 determinant       |  __8.3297 ns__ |   11.1496 ns   |   52.0317 ns   |   16.4971 ns   |    17.6341 ns   |      N/A       |
| mat4 inverse           | __21.9286 ns__ |   42.6754 ns   |   53.4406 ns   |   52.8403 ns   |   331.3933 ns   |   22.4474 ns   |
| mat4 mul mat4          |    7.6689 ns   |  __7.5832 ns__ |   14.7542 ns   |    9.1091 ns   |   205.1414 ns   |    7.6747 ns   |
| mat4 transform point3  |  __2.8806 ns__ |    6.2547 ns   |    6.6465 ns   |    6.0872 ns   |    43.7427 ns   |      N/A       |
| mat4 transform vector3 |  __2.7503 ns__ |    5.5468 ns   |    5.3642 ns   |    4.3981 ns   |    44.4737 ns   |      N/A       |
| mat4 transform vector4 |  __2.9953 ns__ |    3.0956 ns   |    4.0748 ns   |      N/A       |    41.4363 ns   |    3.0111 ns   |
| mat4 transpose         |  __5.5896 ns__ |    8.6730 ns   |   19.1668 ns   |      N/A       |     8.5626 ns   |      N/A       |
| quat conjugate         |  __2.2176 ns__ |    2.9146 ns   |    2.9095 ns   |    2.9100 ns   |     2.9046 ns   |      N/A       |
| quat mul quat          |  __4.2515 ns__ |    4.4174 ns   |    4.6679 ns   |    7.8819 ns   |     9.4261 ns   |      N/A       |
| quat transform vector3 |  __5.1249 ns__ |    6.7975 ns   |   26.6022 ns   |    6.9983 ns   |    11.9850 ns   |      N/A       |
| vec3 cross             |  __2.4443 ns__ |    3.5711 ns   |    3.2860 ns   |    3.5326 ns   |     3.6202 ns   |      N/A       |
| vec3 dot               |  __2.1148 ns__ |    2.3138 ns   |    2.3122 ns   |    2.3097 ns   |     2.2819 ns   |      N/A       |
| vec3 length            |    2.2045 ns   |    2.2068 ns   |    2.3512 ns   |  __2.1861 ns__ |     2.3559 ns   |      N/A       |
| vec3 normalize         |  __4.0705 ns__ |    4.3569 ns   |    8.2144 ns   |    8.0985 ns   |     8.0892 ns   |      N/A       |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.39 Rust compiler. Lower (better) numbers are
highlighted. The versions of the libraries tested were:

* `glam` - `0.8.2`
* `cgmath` - `0.17.0`
* `nalgebra` - `0.19.0`
* `euclid` - `0.20.3`
* `vek` - `0.9.10` (`repr_c` types)

See the full [mathbench report] for more detailed results.

## Running the benchmarks

The benchmarks use the criterion crate which works on stable Rust, they can be
run with:

```
cargo bench
```

For the best results close other applications on the machine you are using to
benchmark!

There is a script in `scripts/summary.py` to summarize the results in a nice
fashion. It requires Python 3 and the `prettytable` Python module, then can
be run to generate an ASCII output.

## Default and optional features

All libraries except for `glam` are optional for running benchmarks. The default
features include `cgmath`, `euclid` and `nalgebra`. These can be disabled with:

```
cargo bench --no-default-features
```

To selectively enable a specific default feature again use:

```
cargo bench --no-default-features --features nalgebra
```

Note that you can run individual benchmarks without needing to diable them at
compile time. For example to only run the "vec3 length" benchmark for `glam` use:

```
cargo bench "vec3 length/glam"
```

## Running the tests

The tests can be run using:

```
cargo test
```

## Adding a new library

There are different steps involved for adding a unit tests and benchmarks for a
new library.

Benchmarks require an implementation of the `mathbench::RandomVec` trait for the
types you want to benchmark. If the type implements the `rand` crate
`distribution::Distribution` trait for `Standard` then you can simply use the
`impl_random_vec!` macro in `src/lib.rs`. Otherwise you can provide a function
that generates a new random value of your type pass that to `impl_random_vec!`.

To add the new libary type to a benchmark, add another `bench_function` call to
the `Criterion` `BenchmarkGroup`.

Increment the patch version number of `mathbench` in the `Cargo.toml`.

Update `CHANGELOG.md`.

## Future work

* Calculate average error for each library against a high precision result
* Add more benchmarks
* Add `packed_simd` to benchmarks
* Add `ISPC` to benchmarks

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
  or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT)
  or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Contributions in any form (issues, pull requests, etc.) to this project must
adhere to Rust's [Code of Conduct].

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

## Support
If you are interested in contributing or have a request or suggestion
[create an issue] on github.

[Build Status]: https://travis-ci.org/bitshifter/mathbench-rs.svg?branch=master
[travis-ci]: https://travis-ci.org/bitshifter/mathbench-rs
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html
[glam]: https://github.com/bitshifter/glam-rs
[cgmath]: https://github.com/rustgd/cgmath
[nalgebra]: https://github.com/rustsim/nalgebra
[euclid]: https://github.com/servo/euclid
[vek]: https://github.com/yoanlcq/vek
[pathfinder_geometry]: https://crates.io/crates/pathfinder_geometry
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
[Intel i7-4710HQ]: https://ark.intel.com/content/www/us/en/ark/products/78930/intel-core-i7-4710hq-processor-6m-cache-up-to-3-50-ghz.html
[mathbench report]: https://bitshifter.github.io/mathbench/0.2.0/report/index.html
