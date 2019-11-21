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
| euler 2d               |      26.8 us   |     26.02 us   |     26.96 us   |     30.95 us   |      26.79 us   |   __19.07 us__ |
| euler 3d               |     37.06 us   |     37.37 us   |     95.09 us   |      35.7 us   |      38.12 us   |    __35.0 us__ |
| mat2 determinant       |    1.0438 ns   |    1.0343 ns   |    1.0393 ns   |      N/A       |   __1.0100 ns__ |    1.0934 ns   |
| mat2 inverse           |    2.4622 ns   |  __2.2760 ns__ |    2.3709 ns   |      N/A       |       N/A       |    2.3311 ns   |
| mat2 mul mat2          |    2.6205 ns   |    3.0582 ns   |    2.8025 ns   |      N/A       |    20.0352 ns   |  __2.2514 ns__ |
| mat2 transform vector2 |    2.2416 ns   |  __1.5772 ns__ |    2.3585 ns   |      N/A       |     9.4389 ns   |    1.8940 ns   |
| mat2 transpose         |    2.4281 ns   |    1.5202 ns   |    2.6319 ns   |      N/A       |   __1.4497 ns__ |      N/A       |
| mat3 determinant       |    2.4654 ns   |    2.6452 ns   |    2.6726 ns   |  __1.1404 ns__ |     2.9005 ns   |      N/A       |
| mat3 inverse           |   10.8626 ns   |   10.7703 ns   |   31.2619 ns   |    5.1197 ns   |       N/A       |  __3.6434 ns__ |
| mat3 mul mat3          |    6.3609 ns   |   21.1416 ns   |   10.8036 ns   |    4.5463 ns   |   147.7749 ns   |  __3.7432 ns__ |
| mat3 transform point2  |    3.0541 ns   |  __2.2774 ns__ |   24.1383 ns   |    2.5235 ns   |    26.8688 ns   |      N/A       |
| mat3 transform vector2 |    2.9641 ns   |      N/A       |   16.4682 ns   |    2.0954 ns   |    26.9119 ns   |  __2.0683 ns__ |
| mat3 transform vector3 |  __2.9362 ns__ |    3.3219 ns   |    3.6070 ns   |      N/A       |    32.1364 ns   |      N/A       |
| mat3 transpose         |    7.3356 ns   |  __4.2112 ns__ |    7.0528 ns   |      N/A       |     7.1327 ns   |      N/A       |
| mat4 determinant       | __10.8057 ns__ |   12.5214 ns   |   56.3375 ns   |   18.9185 ns   |    18.6105 ns   |      N/A       |
| mat4 inverse           | __24.6168 ns__ |   46.0135 ns   |   56.2455 ns   |   56.9040 ns   |   339.7553 ns   |   24.6807 ns   |
| mat4 mul mat4          |  __8.0256 ns__ |   12.4518 ns   |   13.9407 ns   |   12.1229 ns   |   203.4246 ns   |    8.1587 ns   |
| mat4 transform point3  |  __3.4690 ns__ |   11.1519 ns   |   29.5191 ns   |    6.9771 ns   |    56.0802 ns   |      N/A       |
| mat4 transform vector3 |  __3.5153 ns__ |    4.9797 ns   |   21.8940 ns   |    3.9720 ns   |    56.0841 ns   |      N/A       |
| mat4 transform vector4 |  __3.3714 ns__ |    3.5692 ns   |    3.7053 ns   |      N/A       |    60.0793 ns   |    3.5144 ns   |
| mat4 transpose         |  __8.4417 ns__ |   17.9832 ns   |   11.4669 ns   |      N/A       |    18.1702 ns   |      N/A       |
| quat conjugate         |    2.5137 ns   |    1.6490 ns   |    3.5830 ns   |    1.6779 ns   |   __1.6283 ns__ |      N/A       |
| quat mul quat          |  __2.6916 ns__ |    4.0596 ns   |    3.9678 ns   |    5.5147 ns   |     8.8757 ns   |      N/A       |
| quat transform vector3 |  __4.0963 ns__ |    4.3275 ns   |   39.5704 ns   |    6.4349 ns   |    24.5135 ns   |      N/A       |
| vec3 cross             |    2.3659 ns   |    2.1077 ns   |    2.0969 ns   |  __2.0624 ns__ |     2.0992 ns   |      N/A       |
| vec3 dot               |    1.6676 ns   |    1.4593 ns   |    9.9096 ns   |  __1.3430 ns__ |     1.4347 ns   |      N/A       |
| vec3 length            |    2.0672 ns   |    2.0527 ns   |   13.7811 ns   |    2.0620 ns   |   __1.0848 ns__ |      N/A       |
| vec3 normalize         |    4.1256 ns   |  __2.5767 ns__ |   22.5129 ns   |    4.1272 ns   |     4.1285 ns   |      N/A       |

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
[mathbench report]: https://bitshifter.github.io/mathbench/0.1.9/report/index.html
