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
* `quaternion benches` - perform common quaternion operations.
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

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath`, `nalgebra`, `euclid` and `vek` on `f32` data.

| benchmark                  |          glam   |        cgmath   |      nalgebra   |       euclid   |           vek   |    pathfinder   |
|:---------------------------|-- -------------:|----------------:|----------------:|---------------:|----------------:|----------------:|
| euler 2d x10000            |    __7.792 us__ |    __7.648 us__ |      16.58 us   |     11.91 us   |    __7.623 us__ |       10.2 us   |
| euler 3d x10000            |    __15.73 us__ |      24.99 us   |      106.3 us   |     25.01 us   |      25.04 us   |      16.61 us   |
| matrix2 determinant        |   __2.0243 ns__ |   __2.0319 ns__ |   __2.0362 ns__ |      N/A       |   __2.0324 ns__ |   __2.0232 ns__ |
| matrix2 inverse            |   __2.5931 ns__ |     3.0771 ns   |     3.0963 ns   |      N/A       |       N/A       |     2.7660 ns   |
| matrix2 mul matrix2        |     2.6517 ns   |     2.7582 ns   |     2.7541 ns   |      N/A       |    19.9018 ns   |   __2.5568 ns__ |
| matrix2 mul vector2 x1     |     2.7149 ns   |     2.6294 ns   |     2.6264 ns   |      N/A       |     9.1836 ns   |   __2.1251 ns__ |
| matrix2 mul vector2 x100   |   249.2675 ns   |   239.2375 ns   |   243.6050 ns   |      N/A       |   887.8139 ns   | __221.0440 ns__ |
| matrix2 return self        |   __2.5002 ns__ |     2.6048 ns   |     2.6037 ns   |      N/A       |     2.6053 ns   |   __2.4642 ns__ |
| matrix2 transpose          |   __2.0986 ns__ |   __2.0800 ns__ |     2.6048 ns   |      N/A       |   __2.1175 ns__ |       N/A       |
| matrix3 determinant        |   __2.7236 ns__ |     3.4337 ns   |     3.3859 ns   |      N/A       |     3.4829 ns   |       N/A       |
| matrix3 inverse            |     9.0184 ns   |     8.4953 ns   |     9.7601 ns   |      N/A       |       N/A       |   __4.3284 ns__ |
| matrix3 mul matrix3        |     5.4612 ns   |     8.2805 ns   |    10.0504 ns   |      N/A       |   128.3130 ns   |   __3.8671 ns__ |
| matrix3 mul vector3 x1     |   __2.6286 ns__ |     4.3930 ns   |     4.3352 ns   |      N/A       |    40.8996 ns   |       N/A       |
| matrix3 mul vector3 x100   |     0.5308 us   |   __0.4601 us__ |     0.4776 us   |      N/A       |      4.131 us   |       N/A       |
| matrix3 return self        |     5.6968 ns   |     4.8419 ns   |     4.8427 ns   |      N/A       |     4.8328 ns   |   __4.0853 ns__ |
| matrix3 transpose          |   __4.9974 ns__ |     5.7100 ns   |    10.7090 ns   |      N/A       |     5.7240 ns   |       N/A       |
| matrix4 determinant        |   __8.3825 ns__ |    11.1800 ns   |    55.4632 ns   |   16.2066 ns   |    17.8108 ns   |       N/A       |
| matrix4 inverse            |  __21.9264 ns__ |    41.7964 ns   |    51.6510 ns   |   52.8500 ns   |   330.2365 ns   |    24.2825 ns   |
| matrix4 mul matrix4        |   __7.3645 ns__ |     7.8721 ns   |    10.0062 ns   |    9.0023 ns   |   189.4262 ns   |     7.6715 ns   |
| matrix4 mul vector4 x1     |   __3.0289 ns__ |     3.2053 ns   |     3.2392 ns   |      N/A       |    46.9531 ns   |   __3.0904 ns__ |
| matrix4 mul vector4 x100   |   __0.6255 us__ |   __0.6271 us__ |   __0.6258 us__ |      N/A       |      4.738 us   |   __0.6375 us__ |
| matrix4 return self        |     7.1649 ns   |     7.4951 ns   |     7.5701 ns   |      N/A       |     7.4928 ns   |   __6.9258 ns__ |
| matrix4 transpose          |   __7.1278 ns__ |    10.5022 ns   |    19.3988 ns   |      N/A       |    10.5206 ns   |       N/A       |
| quaternion conjugate       |   __2.1343 ns__ |     2.9152 ns   |     2.9108 ns   |    2.9205 ns   |     2.9131 ns   |       N/A       |
| quaternion mul quaternion  |   __3.5744 ns__ |     4.5548 ns   |     4.8202 ns   |    8.0146 ns   |    10.5981 ns   |       N/A       |
| quaternion mul vector3     |   __5.8698 ns__ |     6.7802 ns   |     7.0169 ns   |    6.9912 ns   |    11.9606 ns   |       N/A       |
| quaternion return self     |   __2.4632 ns__ |     2.5960 ns   |     2.5907 ns   |      N/A       |     2.6003 ns   |       N/A       |
| transform point2 x1        |   __2.7688 ns__ |     2.8938 ns   |     4.3806 ns   |    3.2530 ns   |    44.4697 ns   |       N/A       |
| transform point2 x100      |     0.4057 us   |     0.3586 us   |     0.4423 us   |  __0.3125 us__ |      4.385 us   |       N/A       |
| transform point3 x1        |   __2.9630 ns__ |     6.0267 ns   |     6.1170 ns   |    7.3728 ns   |    43.7141 ns   |       N/A       |
| transform point3 x100      |    __0.628 us__ |     0.7261 us   |     0.7432 us   |    0.7335 us   |      4.406 us   |       N/A       |
| transform vector2 x1       |     3.0185 ns   |       N/A       |     3.7697 ns   |    2.9227 ns   |    44.6045 ns   |   __2.3072 ns__ |
| transform vector2 x100     |     0.4115 us   |       N/A       |     0.3999 us   |  __0.2863 us__ |      4.383 us   |     0.3238 us   |
| transform vector3 x1       |   __2.9747 ns__ |     5.1365 ns   |     8.8130 ns   |    4.2195 ns   |    43.7552 ns   |       N/A       |
| transform vector3 x100     |   __0.6201 us__ |     0.6534 us   |     0.9004 us   |  __0.6217 us__ |       4.37 us   |       N/A       |
| transform2 inverse         |       N/A       |       N/A       |     9.9566 ns   |    4.9061 ns   |       N/A       |   __4.0351 ns__ |
| transform2 mul transform2  |       N/A       |       N/A       |     9.9321 ns   |    5.1075 ns   |       N/A       |   __3.6289 ns__ |
| transform2 return self     |       N/A       |       N/A       |     4.8614 ns   |  __3.5146 ns__ |       N/A       |     4.1484 ns   |
| transform3 inverse         |       N/A       |       N/A       |  __51.7319 ns__ |   62.0064 ns   |       N/A       |       N/A       |
| transform3 mul transform3d |       N/A       |       N/A       |  __10.3848 ns__ |   11.6121 ns   |       N/A       |       N/A       |
| transform3 return self     |       N/A       |       N/A       |     7.4340 ns   |  __6.8873 ns__ |       N/A       |       N/A       |
| vector3 cross              |   __2.4698 ns__ |     3.5068 ns   |     3.2558 ns   |    3.5117 ns   |     3.5321 ns   |       N/A       |
| vector3 dot                |   __2.1109 ns__ |     2.4331 ns   |     2.2965 ns   |    2.2988 ns   |     2.2628 ns   |       N/A       |
| vector3 length             |     2.2261 ns   |   __2.1732 ns__ |     2.3382 ns   |  __2.1707 ns__ |     2.3648 ns   |       N/A       |
| vector3 normalize          |   __4.0715 ns__ |     4.2994 ns   |     8.1961 ns   |    8.0856 ns   |     8.0477 ns   |       N/A       |
| vector3 return self        |   __2.4993 ns__ |     2.9701 ns   |     2.9681 ns   |      N/A       |     3.0001 ns   |       N/A       |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.39 Rust compiler. Lower (better) numbers are
highlighted. The versions of the libraries tested were:

* `glam` - `0.8.3`
* `cgmath` - `0.17.0`
* `nalgebra` - `0.19.0`
* `euclid` - `0.20.5`
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
