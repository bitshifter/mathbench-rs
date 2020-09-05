# mathbench

[![Build Status]][travis-ci]

`mathbench` is a suite of unit tests and benchmarks comparing the output and
performance of a number of different Rust linear algebra libraries for common
game and graphics development tasks.

`mathbench` is written by the author of [`glam`][glam] and has been used to
compare the performance of `glam` with other similar 3D math libraries targeting
games and graphics development, including:

* [`cgmath`][cgmath]
* [`euclid`][euclid]
* [`nalgebra`][nalgebra]
* [`pathfinder_geometry`][pathfinder_geometry]
* [`static-math`][static-math]
* [`ultraviolet`][ultraviolet]
* [`vek`][vek]

## The benchmarks

All benchmarks are performed using [Criterion.rs]. Benchmarks are logically into
the following categories:

* return self - attempts to measure overhead of benchmarking each type.
* single operations - measure the performance of single common operations on
  types, e.g. a matrix inverse, vector normalization or multiplying two
  matrices.
* throughput operations - measure the performance of common operations on
  batches of data. These measure operations that would commonly be processing
  batches of input, for example transforming a number of vectors with the same
  matrix.
* workload operations - these attempt to recreate common workloads found in game
  development to try and demonstrate performance on real world tasks.

Despite best attempts, take the results of micro benchmarks with a pinch of
salt.

### Operation benchmarks

* `matrix benches` - performs common matrix operations such as transpose,
  inverse, determinant and multiply.
* `rotation 3d benches` - perform common 3D rotation operations.
* `transform 2d & 3d benches` - bench special purpose 2D and 3D transform types.
  These can be compared to 3x3 and 4x4 matrix benches to some extent.
* `transformations benches` - performs affine transformations on vectors - uses
  the best available type for the job, either matrix or transform types
  depending on the library.
* `vector benches` - perform common vector operations.

### Workload benchmarks

* `euler bench` - performs an Euler integration on arrays of 2D and 3D vectors

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
transform 2D vectors and points.

Similarly `Transform3D` is used for transforming 3D vectors and points. This
is represented as a 4x4 matrix so it is more directly comparable to the other
libraries however it doesn't support some operations like transpose.

There is no equivalent to a 2x2 matrix type in `euclid`.

### Matrix inverse

Note that `cgmath` and `nalgebra` matrix inverse methods return an `Option`
whereas `glam` and `euclid` do not. If a non-invertible matrix is inverted by
`glam` or `euclid` the result will be invalid (it will contain NaNs).

### Quaternions versus rotors

Most libraries provide quaternions for performing rotations except for
`ultraviolet` which provides rotors.

### Wide benchmarks

The mathbench benchmarks are currently only benchmarking f32 scalar operations.
The `ultraviolet` and `nalgebra` libraries both support wide types which are
able to better utililise SIMD instructions to get a higher throughput. The
caveat being that your program's data must be organised into an array of
structure of arrays format to take advantage of it. Until support for
benchmarking wide types is added to mathbench the author of `nalgebra` has his
own fork of mathbench and a [blog
post](https://www.rustsim.org/blog/2020/03/23/simd-aosoa-in-nalgebra/) with
benchmark results from `nalgebra` and `ultraviolet`.

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath`, `nalgebra`, `euclid`, `vek`, `pathfinder_geometry`,
`static-math` and `ultraviolet` on `f32` data.

| benchmark                  |          glam   |        cgmath   |      nalgebra   |       euclid   |           vek   |    pathfinder   |   static-math   |   ultraviolet   |
|:---------------------------|----------------:|----------------:|----------------:|---------------:|----------------:|----------------:|----------------:|----------------:|
| euler 2d x10000            |    __7.555 us__ |    __7.521 us__ |      16.38 us   |     11.86 us   |    __7.513 us__ |      9.806 us   |      11.83 us   |    __7.499 us__ |
| euler 3d x10000            |     __16.2 us__ |      25.04 us   |      106.3 us   |     25.05 us   |      25.16 us   |      16.76 us   |      25.03 us   |      25.05 us   |
| matrix2 determinant        |   __2.0332 ns__ |   __2.0311 ns__ |   __2.0250 ns__ |      N/A       |   __2.0209 ns__ |   __2.0323 ns__ |   __2.0254 ns__ |       N/A       |
| matrix2 inverse            |   __2.6114 ns__ |     3.0331 ns   |     2.9792 ns   |      N/A       |       N/A       |     2.7550 ns   |     3.0132 ns   |       N/A       |
| matrix2 mul matrix2        |     2.6047 ns   |   __2.5346 ns__ |   __2.5426 ns__ |      N/A       |     8.7573 ns   |   __2.5381 ns__ |     2.6028 ns   |     2.9668 ns   |
| matrix2 mul vector2 x1     |     2.6592 ns   |     2.6104 ns   |     2.6214 ns   |      N/A       |     4.2512 ns   |   __2.0663 ns__ |     2.8674 ns   |     2.6172 ns   |
| matrix2 mul vector2 x100   |   245.2897 ns   |   233.7149 ns   |   238.7395 ns   |      N/A       |   399.3148 ns   | __218.4107 ns__ |   260.6645 ns   |   234.7099 ns   |
| matrix2 return self        |   __2.4740 ns__ |     2.5994 ns   |     2.5968 ns   |      N/A       |     2.5969 ns   |   __2.4607 ns__ |     2.5928 ns   |     2.5974 ns   |
| matrix2 transpose          |   __2.0852 ns__ |   __2.0814 ns__ |     2.3426 ns   |      N/A       |   __2.1053 ns__ |       N/A       |   __2.0829 ns__ |       N/A       |
| matrix3 determinant        |   __3.3675 ns__ |   __3.4261 ns__ |   __3.3780 ns__ |      N/A       |   __3.4479 ns__ |       N/A       |   __3.4375 ns__ |       N/A       |
| matrix3 inverse            |    11.4209 ns   |   __8.3701 ns__ |     9.4315 ns   |      N/A       |       N/A       |       N/A       |     9.1710 ns   |    20.1731 ns   |
| matrix3 mul matrix3        |   __5.8501 ns__ |     6.5350 ns   |     9.8196 ns   |      N/A       |    47.9203 ns   |       N/A       |     9.5170 ns   |     6.5211 ns   |
| matrix3 mul vector3 x1     |   __3.9266 ns__ |     4.3876 ns   |     4.3333 ns   |      N/A       |    16.0858 ns   |       N/A       |     4.4220 ns   |     4.3304 ns   |
| matrix3 mul vector3 x100   |   __0.4372 us__ |   __0.4416 us__ |     0.4594 us   |      N/A       |       1.59 us   |       N/A       |      0.454 us   |   __0.4425 us__ |
| matrix3 return self        |   __4.8566 ns__ |   __4.8401 ns__ |   __4.8226 ns__ |      N/A       |   __4.8340 ns__ |       N/A       |   __4.8303 ns__ |   __4.8383 ns__ |
| matrix3 transpose          |   __5.7688 ns__ |   __5.6980 ns__ |     8.1508 ns   |      N/A       |   __5.6910 ns__ |       N/A       |   __5.6936 ns__ |   __5.6766 ns__ |
| matrix4 determinant        |   __8.3724 ns__ |    11.1604 ns   |    52.8697 ns   |   16.0723 ns   |    17.5301 ns   |       N/A       |    16.1402 ns   |       N/A       |
| matrix4 inverse            |  __21.3281 ns__ |    38.5833 ns   |    64.5172 ns   |   61.2347 ns   |   275.5253 ns   |       N/A       |    48.0641 ns   |    37.1436 ns   |
| matrix4 mul matrix4        |   __7.5043 ns__ |     8.3723 ns   |     9.4094 ns   |   10.1761 ns   |    90.7185 ns   |       N/A       |    20.6424 ns   |     8.4072 ns   |
| matrix4 mul vector4 x1     |   __3.3645 ns__ |     3.7273 ns   |     3.7251 ns   |      N/A       |    24.2185 ns   |       N/A       |     6.1311 ns   |     3.7524 ns   |
| matrix4 mul vector4 x100   |   __0.6105 us__ |   __0.6237 us__ |   __0.6202 us__ |      N/A       |      2.402 us   |       N/A       |     0.7044 us   |   __0.6202 us__ |
| matrix4 return self        |     6.8863 ns   |     7.1298 ns   |   __6.6961 ns__ |      N/A       |   __6.7079 ns__ |       N/A       |   __6.6772 ns__ |   __6.7079 ns__ |
| matrix4 transpose          |   __5.7312 ns__ |    10.1612 ns   |    14.9424 ns   |      N/A       |    10.2015 ns   |       N/A       |    10.1996 ns   |    10.2391 ns   |
| rotation3 inverse          |   __2.1867 ns__ |     2.9086 ns   |     2.8853 ns   |    2.9092 ns   |     2.8987 ns   |       N/A       |       N/A       |     2.9064 ns   |
| rotation3 mul rotation3    |   __3.3422 ns__ |     4.3602 ns   |     7.0680 ns   |    7.7111 ns   |     8.9616 ns   |       N/A       |       N/A       |    18.4088 ns   |
| rotation3 mul vector3      |   __6.6977 ns__ |   __6.7831 ns__ |     6.9924 ns   |    6.9801 ns   |    32.8778 ns   |       N/A       |       N/A       |    13.5267 ns   |
| rotation3 return self      |   __2.4622 ns__ |     2.5983 ns   |     2.6021 ns   |      N/A       |     2.5989 ns   |       N/A       |       N/A       |     2.5980 ns   |
| transform point2 x1        |     3.8946 ns   |     2.8843 ns   |     4.6543 ns   |    3.2271 ns   |    17.0089 ns   |   __2.3608 ns__ |       N/A       |       N/A       |
| transform point2 x100      |     0.4265 us   |     0.3677 us   |     0.4632 us   |   __0.322 us__ |      1.712 us   |   __0.3206 us__ |       N/A       |       N/A       |
| transform point3 x1        |     4.9958 ns   |     6.3712 ns   |     6.6426 ns   |    6.1114 ns   |    24.8255 ns   |   __3.1011 ns__ |       N/A       |       N/A       |
| transform point3 x100      |   __0.6261 us__ |     0.7418 us   |     0.7447 us   |    0.7296 us   |      2.507 us   |   __0.6295 us__ |       N/A       |       N/A       |
| transform vector2 x1       |   __2.7159 ns__ |       N/A       |     3.9917 ns   |    2.8070 ns   |    16.8257 ns   |       N/A       |       N/A       |       N/A       |
| transform vector2 x100     |     0.3463 us   |       N/A       |     0.4018 us   |  __0.2893 us__ |      1.709 us   |       N/A       |       N/A       |       N/A       |
| transform vector3 x1       |   __3.9868 ns__ |     5.5573 ns   |     8.4892 ns   |    4.4068 ns   |    25.0274 ns   |       N/A       |       N/A       |       N/A       |
| transform vector3 x100     |   __0.5905 us__ |     0.6584 us   |     0.8936 us   |    0.6365 us   |      2.513 us   |       N/A       |       N/A       |       N/A       |
| transform2 inverse         |       N/A       |       N/A       |     9.4094 ns   |    4.6388 ns   |       N/A       |   __3.9983 ns__ |       N/A       |       N/A       |
| transform2 mul transform2  |       N/A       |       N/A       |     9.8173 ns   |    6.2162 ns   |       N/A       |   __3.8699 ns__ |       N/A       |       N/A       |
| transform2 return self     |       N/A       |       N/A       |     4.8447 ns   |  __3.5091 ns__ |       N/A       |     4.1391 ns   |       N/A       |       N/A       |
| transform3 inverse         |       N/A       |       N/A       |    65.3982 ns   |   52.6160 ns   |       N/A       |  __32.0466 ns__ |       N/A       |       N/A       |
| transform3 mul transform3d |       N/A       |       N/A       |    10.9731 ns   |    9.9741 ns   |       N/A       |   __7.6754 ns__ |       N/A       |       N/A       |
| transform3 return self     |       N/A       |       N/A       |     7.1596 ns   |  __6.6096 ns__ |       N/A       |     7.0148 ns   |       N/A       |       N/A       |
| vector3 cross              |   __2.4542 ns__ |     3.5894 ns   |     3.2434 ns   |    3.4923 ns   |     3.5150 ns   |       N/A       |     3.2947 ns   |     7.1968 ns   |
| vector3 dot                |   __2.1001 ns__ |     2.3025 ns   |     2.2986 ns   |    2.3030 ns   |     2.3084 ns   |       N/A       |     2.3072 ns   |     3.7322 ns   |
| vector3 length             |   __2.1722 ns__ |   __2.1747 ns__ |     2.3414 ns   |  __2.1716 ns__ |   __2.2151 ns__ |       N/A       |   __2.2063 ns__ |     3.4787 ns   |
| vector3 normalize          |   __4.4248 ns__ |   __4.3266 ns__ |     8.1124 ns   |    8.0704 ns   |     8.0747 ns   |       N/A       |       N/A       |     8.0778 ns   |
| vector3 return self        |   __2.4642 ns__ |     2.9591 ns   |     2.9586 ns   |      N/A       |     2.9579 ns   |       N/A       |     2.9633 ns   |     2.9572 ns   |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.46 Rust compiler. Lower (better) numbers are
highlighted within a 2.5% range of the minimum for each row.

The versions of the libraries tested were:

* `cgmath` - `0.17.0`
* `euclid` - `0.22.1`
* `glam` - `0.9.4`
* `nalgebra` - `0.22.0`
* `pathfinder_geometry` - `0.5.1`
* `static-math` - `0.1.6`
* `ultraviolet` - `0.5.1`
* `vek` - `0.12.0` (`repr_c` types)

See the full [mathbench report] for more detailed results.

## Running the benchmarks

The benchmarks use the criterion crate which works on stable Rust, they can be
run with:

```sh
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

```sh
cargo bench --no-default-features
```

To selectively enable a specific default feature again use:

```sh
cargo bench --no-default-features --features nalgebra
```

Note that you can run individual benchmarks without needing to diable them at
compile time. For example to only run the "vec3 length" benchmark for `glam` use:

```sh
cargo bench "vec3 length/glam"
```

## Running the tests

The tests can be run using:

```sh
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

## Build times

`mathbench` also includes a tool for comparing build times in
`tools/buildbench`.

The `buildbench` tool uses the `-Z timings` feature of the nightly build of
`cargo`, thus you need a nightly build to run it.

`buildbench` generates a `Cargo.toml` and empty `src/lib.rs` in a temporary
directory for each library, recording some build time information which is
included in the summary table below. The temporary directory is created every
time the tool is run so this is a full build from a clean state.

Each library is only built once so you may wish to run `buildbench` multiple
times to ensure results are consistent.

By default crates are built using the `release` profile with default features
enabled. There are options for building the `dev` profile or without default
features, see `buildbench --help` for more information.

The columns outputted include the total build time, the self build time which is
the time it took to build the crate on it's own excluding dependencies, and the
number of units which is the number of dependencies (this will be 2 at minimum).

When comparing build times keep in mind that each library has different feature
sets and that naturally larger libraries will take longer to build. For many
crates tested the dependencies take longer than the math crate. Also keep in
mind if you are already building one of the dependencies in your project you
won't pay the build cost twice (unless it's a different version).

| crate               | version | total (s) | self (s) | units | full report link            |
|:--------------------|:--------|----------:|---------:|------:|:----------------------------|
| cgmath              | 0.17.0  |       6.5 |      3.0 |    17 | [cgmath build timings]      |
| euclid              | 0.20.5  |       3.2 |      1.1 |     4 | [euclid build timings]      |
| glam                | 0.8.6   |       0.8 |      0.5 |     3 | [glam build timings]        |
| nalgebra            | 0.21.0  |      32.1 |     17.8 |    29 | [nalgebra build timings]    |
| pathfinder_geometry | 0.5.0   |       5.6 |      0.3 |     8 | [pathfinder build timings]  |
| ultraviolet         | 0.4.5   |       2.4 |      1.2 |     4 | [ultraviolet build timings] |
| vek                 | 0.10.1  |      38.0 |     10.6 |    16 | [vek build timings]         |

These benchmarks were performed on an [Intel i7-4710HQ] CPU with 16GB RAM and a
Toshiba MQ01ABD100 HDD (SATA 3Gbps 5400RPM) on Linux.

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
[cgmath]: https://crates.io/crates/cgmath
[euclid]: https://crates.io/crates/euclid
[glam]: https://github.com/bitshifter/glam-rs
[nalgebra]: https://nalgebra.org
[pathfinder_geometry]: https://crates.io/crates/pathfinder_geometry
[static-math]: https://crates.io/crates/static-math
[ultraviolet]: https://crates.io/crates/ultraviolet
[vek]: https://crates.io/crates/vek
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
[Intel i7-4710HQ]: https://ark.intel.com/content/www/us/en/ark/products/78930/intel-core-i7-4710hq-processor-6m-cache-up-to-3-50-ghz.html
[mathbench report]: https://bitshifter.github.io/mathbench/0.3.2/report/index.html
[cgmath build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-cgmath-release-defaults.html
[euclid build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-euclid-release-defaults.html
[glam build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-glam-release-defaults.html
[nalgebra build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-nalgebra-release-defaults.html
[pathfinder build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-pathfinder_geometry-release-defaults.html
[ultraviolet build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-ultraviolet-release-defaults.html
[vek build timings]: https://bitshifter.github.io/buildbench/0.3.1/cargo-timing-vek-release-defaults.html
[issue #21]: https://github.com/bitshifter/mathbench-rs/issues/21
