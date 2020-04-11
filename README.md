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
performance to `cgmath`, `nalgebra`, `euclid`, `vek` and `pathfinder_geometry`
on `f32` data.

| benchmark                  |          glam   |        cgmath   |      nalgebra   |       euclid   |           vek   |    pathfinder   |
|:---------------------------|----------------:|----------------:|----------------:|---------------:|----------------:|----------------:|
| euler 2d x10000            |    __7.561 us__ |    __7.502 us__ |       16.4 us   |     11.85 us   |    __7.444 us__ |      9.781 us   |
| euler 3d x10000            |    __16.23 us__ |      25.01 us   |      106.2 us   |     25.01 us   |      25.01 us   |    __16.49 us__ |
| matrix2 determinant        |   __2.0274 ns__ |   __2.0462 ns__ |   __2.0419 ns__ |      N/A       |   __2.0379 ns__ |   __2.0342 ns__ |
| matrix2 inverse            |   __2.6395 ns__ |     3.0942 ns   |     3.0751 ns   |      N/A       |       N/A       |     2.7662 ns   |
| matrix2 mul matrix2        |     2.5950 ns   |     2.6691 ns   |     2.6982 ns   |      N/A       |    19.8359 ns   |   __2.5229 ns__ |
| matrix2 mul vector2 x1     |     2.6706 ns   |     2.6082 ns   |     2.6054 ns   |      N/A       |     8.7931 ns   |   __2.0691 ns__ |
| matrix2 mul vector2 x100   |   248.9718 ns   |   236.4801 ns   |   242.6354 ns   |      N/A       |   877.5105 ns   | __218.4014 ns__ |
| matrix2 return self        |   __2.4857 ns__ |     2.6060 ns   |     2.6121 ns   |      N/A       |     2.6103 ns   |   __2.4853 ns__ |
| matrix2 transpose          |   __2.1063 ns__ |   __2.0968 ns__ |     2.6066 ns   |      N/A       |   __2.0874 ns__ |       N/A       |
| matrix3 determinant        |   __2.7148 ns__ |     3.4121 ns   |     3.3747 ns   |      N/A       |     3.4487 ns   |       N/A       |
| matrix3 inverse            |     9.2056 ns   |   __8.9088 ns__ |     9.7705 ns   |      N/A       |       N/A       |       N/A       |
| matrix3 mul matrix3        |   __5.5154 ns__ |     7.3017 ns   |     9.5565 ns   |      N/A       |   126.4783 ns   |       N/A       |
| matrix3 mul vector3 x1     |   __2.6535 ns__ |     4.3995 ns   |     4.3562 ns   |      N/A       |    40.2655 ns   |       N/A       |
| matrix3 mul vector3 x100   |     0.5331 us   |   __0.4615 us__ |     0.4788 us   |      N/A       |      4.095 us   |       N/A       |
| matrix3 return self        |     5.6831 ns   |   __4.8593 ns__ |   __4.8362 ns__ |      N/A       |   __4.8495 ns__ |       N/A       |
| matrix3 transpose          |   __4.9529 ns__ |     5.6907 ns   |    10.5221 ns   |      N/A       |     5.6796 ns   |       N/A       |
| matrix4 determinant        |   __8.6441 ns__ |    11.3875 ns   |    51.9518 ns   |   16.3381 ns   |    17.6346 ns   |       N/A       |
| matrix4 inverse            |  __22.7544 ns__ |    41.7966 ns   |    51.5987 ns   |   52.8413 ns   |   330.2105 ns   |       N/A       |
| matrix4 mul matrix4        |   __7.3536 ns__ |     8.4243 ns   |    10.0277 ns   |    9.0933 ns   |   193.7167 ns   |       N/A       |
| matrix4 mul vector4 x1     |   __3.0563 ns__ |     3.3400 ns   |     3.3413 ns   |      N/A       |    46.8783 ns   |       N/A       |
| matrix4 mul vector4 x100   |   __0.6156 us__ |   __0.6242 us__ |   __0.6221 us__ |      N/A       |      4.753 us   |       N/A       |
| matrix4 return self        |   __6.9056 ns__ |   __6.9070 ns__ |     7.2379 ns   |      N/A       |     7.2512 ns   |       N/A       |
| matrix4 transpose          |   __6.1961 ns__ |     8.8517 ns   |    19.2261 ns   |      N/A       |     9.0352 ns   |       N/A       |
| quaternion conjugate       |   __2.1317 ns__ |     3.0002 ns   |     2.9109 ns   |    2.9336 ns   |     2.9182 ns   |       N/A       |
| quaternion mul quaternion  |   __3.4175 ns__ |     4.4150 ns   |     4.6578 ns   |    7.8328 ns   |     9.6710 ns   |       N/A       |
| quaternion mul vector3     |   __5.9589 ns__ |     6.7772 ns   |     7.6541 ns   |    6.9807 ns   |    11.9472 ns   |       N/A       |
| quaternion return self     |   __2.4756 ns__ |     2.6086 ns   |     2.6101 ns   |      N/A       |     2.6058 ns   |       N/A       |
| transform point2 x1        |     2.7722 ns   |     2.8795 ns   |     4.4155 ns   |    3.2566 ns   |    44.2422 ns   |   __2.3511 ns__ |
| transform point2 x100      |     0.4091 us   |     0.3605 us   |     0.4559 us   |  __0.3197 us__ |      4.462 us   |   __0.3129 us__ |
| transform point3 x1        |   __3.0101 ns__ |     6.0157 ns   |     6.0904 ns   |    6.1717 ns   |    53.2906 ns   |   __3.0708 ns__ |
| transform point3 x100      |   __0.6131 us__ |     0.7341 us   |     0.7386 us   |    0.7292 us   |      5.508 us   |   __0.6112 us__ |
| transform vector2 x1       |   __2.7438 ns__ |       N/A       |     3.7639 ns   |    2.8318 ns   |    43.9410 ns   |       N/A       |
| transform vector2 x100     |      0.415 us   |       N/A       |     0.4097 us   |  __0.2924 us__ |      4.477 us   |       N/A       |
| transform vector3 x1       |   __2.9709 ns__ |     5.1521 ns   |     8.8204 ns   |    4.2361 ns   |    53.2928 ns   |       N/A       |
| transform vector3 x100     |   __0.6128 us__ |     0.6593 us   |      0.927 us   |     0.629 us   |      5.537 us   |       N/A       |
| transform2 inverse         |       N/A       |       N/A       |     9.7428 ns   |    4.8843 ns   |       N/A       |   __4.0791 ns__ |
| transform2 mul transform2  |       N/A       |       N/A       |     9.5845 ns   |    5.1000 ns   |       N/A       |   __3.6468 ns__ |
| transform2 return self     |       N/A       |       N/A       |     4.8867 ns   |  __3.6033 ns__ |       N/A       |     4.2518 ns   |
| transform3 inverse         |       N/A       |       N/A       |    51.5970 ns   |   61.2819 ns   |       N/A       |  __24.0380 ns__ |
| transform3 mul transform3d |       N/A       |       N/A       |    10.0799 ns   |    9.0911 ns   |       N/A       |   __7.3294 ns__ |
| transform3 return self     |       N/A       |       N/A       |   __7.1534 ns__ |    7.4445 ns   |       N/A       |     7.4305 ns   |
| vector3 cross              |   __2.4388 ns__ |     3.5881 ns   |     3.2305 ns   |    3.4870 ns   |     3.5174 ns   |       N/A       |
| vector3 dot                |   __2.0990 ns__ |     2.3014 ns   |     2.2987 ns   |    2.3257 ns   |     2.2672 ns   |       N/A       |
| vector3 length             |   __2.2067 ns__ |     2.3783 ns   |     2.3537 ns   |  __2.1890 ns__ |     2.3540 ns   |       N/A       |
| vector3 normalize          |   __4.0619 ns__ |     4.2455 ns   |     8.1870 ns   |    8.0831 ns   |     8.0407 ns   |       N/A       |
| vector3 return self        |   __2.4950 ns__ |     2.9680 ns   |     2.9707 ns   |      N/A       |     2.9707 ns   |       N/A       |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.39 Rust compiler. Lower (better) numbers are
highlighted within a 2.5% range of the minimum for each row.

The versions of the libraries tested were:

* `glam` - `0.8.3`
* `cgmath` - `0.17.0`
* `nalgebra` - `0.19.0`
* `euclid` - `0.20.5`
* `vek` - `0.9.10` (`repr_c` types)
* `pathfinder_geometry` - `0.3.0`

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

## Build times

| crate               | total (s) | self (s) | units |
|:--------------------|----------:|---------:|------:|
| cgmath              |       7.0 |      2.9 |    17 |
| euclid              |       3.2 |      1.1 |     4 |
| glam                |       0.8 |      0.5 |     3 |
| nalgebra            |      22.9 |     16.5 |    22 |
| pathfinder_geometry |       2.7 |      0.3 |     8 |
| vek                 |      37.9 |     10.7 |    16 |

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
[mathbench report]: https://bitshifter.github.io/mathbench/0.3.0/report/index.html
