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

| benchmark              |         glam   |       cgmath   |     nalgebra   |       euclid   |           vek   |
|:-----------------------|---------------:|---------------:|---------------:|---------------:|----------------:|
| euler 2d               |      8.52 us   |   __8.393 us__ |     21.32 us   |     13.22 us   |      8.394 us   |
| euler 3d               |    __13.6 us__ |     29.56 us   |     195.9 us   |     28.82 us   |      29.56 us   |
| mat2 determinant       |    1.3151 ns   |    1.0562 ns   |    1.1055 ns   |      N/A       |   __1.0558 ns__ |
| mat2 inverse           |  __2.0473 ns__ |    2.6241 ns   |    2.7428 ns   |      N/A       |       N/A       |
| mat2 mul mat2          |  __2.1270 ns__ |    3.0440 ns   |    4.0735 ns   |      N/A       |    18.7748 ns   |
| mat2 transform vector2 |  __2.1959 ns__ |    2.5543 ns   |    6.8928 ns   |      N/A       |     8.2650 ns   |
| mat2 transpose         |  __0.7521 ns__ |    1.3319 ns   |    1.8206 ns   |      N/A       |     1.3303 ns   |
| mat3 determinant       |    2.2126 ns   |    2.5986 ns   |    2.5845 ns   |  __1.1684 ns__ |     2.5459 ns   |
| mat3 inverse           |    8.7049 ns   |    7.9614 ns   |    9.3103 ns   |  __4.1184 ns__ |       N/A       |
| mat3 mul mat3          |    4.8569 ns   |    9.5129 ns   |    8.1683 ns   |  __4.6653 ns__ |   132.7678 ns   |
| mat3 transform point2  |    2.7996 ns   |  __2.7061 ns__ |    7.7523 ns   |    2.7637 ns   |    40.1648 ns   |
| mat3 transform vector2 |  __2.5432 ns__ |      N/A       |    3.3504 ns   |    2.5635 ns   |    40.3988 ns   |
| mat3 transform vector3 |  __2.2865 ns__ |    4.0590 ns   |    7.7440 ns   |      N/A       |    41.3515 ns   |
| mat3 transpose         |  __1.9804 ns__ |    3.5444 ns   |    7.5064 ns   |      N/A       |     3.5408 ns   |
| mat4 determinant       |  __8.1398 ns__ |   11.2489 ns   |   50.3802 ns   |   16.6821 ns   |    17.8935 ns   |
| mat4 inverse           | __20.8922 ns__ |   43.0659 ns   |   48.0782 ns   |   59.9804 ns   |   331.7690 ns   |
| mat4 mul mat4          |  __6.6243 ns__ |    9.8404 ns   |   15.7676 ns   |    9.2130 ns   |   200.9219 ns   |
| mat4 transform point3  |  __2.4146 ns__ |    5.9793 ns   |    9.1878 ns   |    6.1244 ns   |    45.2397 ns   |
| mat4 transform vector3 |  __2.2789 ns__ |    4.5462 ns   |    4.7495 ns   |    4.0544 ns   |    45.3536 ns   |
| mat4 transform vector4 |  __2.5089 ns__ |    3.5797 ns   |    4.7099 ns   |      N/A       |    46.8938 ns   |
| mat4 transpose         |  __2.6631 ns__ |    7.7573 ns   |   11.0105 ns   |      N/A       |     7.5961 ns   |
| quat conjugate         |  __0.8910 ns__ |    1.7659 ns   |    1.7684 ns   |    1.7989 ns   |     1.7712 ns   |
| quat mul quat          |  __2.5947 ns__ |    3.8442 ns   |    4.0468 ns   |    7.8900 ns   |     7.7072 ns   |
| quat transform vector3 |  __4.2280 ns__ |    6.5117 ns   |    6.8062 ns   |    6.9557 ns   |     9.9788 ns   |
| vec3 cross             |  __2.0338 ns__ |    2.8473 ns   |    2.8519 ns   |    2.8390 ns   |     2.8503 ns   |
| vec3 dot               |  __1.3957 ns__ |    1.7056 ns   |    1.7486 ns   |    1.7087 ns   |     1.7773 ns   |
| vec3 length            |    2.0941 ns   |    2.0217 ns   |  __2.0109 ns__ |    2.0263 ns   |     2.0123 ns   |
| vec3 normalize         |  __4.0403 ns__ |    4.1329 ns   |    8.2380 ns   |    8.0818 ns   |     8.1437 ns   |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.39 Rust compiler. Lower (better) numbers are
highlighted. The versions of the libraries tested were:

* `glam` - 0.8.2
* `cgmath` - 0.17.0
* `nalgebra` - 0.19.0
* `euclid` - 0.20.3
* `vek` - 0.9.10 (`repr_c` types)

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
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
[Intel i7-4710HQ]: https://ark.intel.com/content/www/us/en/ark/products/78930/intel-core-i7-4710hq-processor-6m-cache-up-to-3-50-ghz.html
[mathbench report]: https://bitshifter.github.io/mathbench/0.1.9/report/index.html
