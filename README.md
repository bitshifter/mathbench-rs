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

| benchmark              |         glam   |       cgmath   |     nalgebra   |       euclid   |
|:-----------------------|---------------:|---------------:|---------------:|---------------:|
| euler 2d               |     8.465 us   |   __8.395 us__ |     20.84 us   |     13.35 us   |
| euler 3d               |   __13.78 us__ |     30.72 us   |     196.2 us   |     30.34 us   |
| mat2 determinant       |    1.3827 ns   |    1.0591 ns   |  __1.0557 ns__ |      N/A       |
| mat2 inverse           |  __2.0473 ns__ |    2.7092 ns   |    2.6357 ns   |      N/A       |
| mat2 mul mat2          |  __2.1362 ns__ |    3.0953 ns   |    4.1610 ns   |      N/A       |
| mat2 transform vector2 |    3.1926 ns   |  __2.3974 ns__ |    6.8836 ns   |      N/A       |
| mat2 transpose         |  __0.7124 ns__ |    1.3370 ns   |    1.8164 ns   |      N/A       |
| mat3 determinant       |    2.2119 ns   |    2.5514 ns   |    2.6440 ns   |  __1.1939 ns__ |
| mat3 inverse           |    9.3552 ns   |    7.9584 ns   |    9.8131 ns   |  __4.2979 ns__ |
| mat3 mul mat3          |    4.8614 ns   |    9.5457 ns   |    8.5263 ns   |  __4.6640 ns__ |
| mat3 transform point2  |  __2.6169 ns__ |    2.7039 ns   |    7.7552 ns   |    2.7560 ns   |
| mat3 transform vector2 |    2.5250 ns   |      N/A       |    3.3505 ns   |  __2.4924 ns__ |
| mat3 transform vector3 |  __2.3030 ns__ |    4.0836 ns   |    7.7676 ns   |      N/A       |
| mat3 transpose         |  __1.9804 ns__ |    3.5219 ns   |    7.4764 ns   |      N/A       |
| mat4 determinant       |  __7.8634 ns__ |   11.2128 ns   |   50.3752 ns   |   16.6337 ns   |
| mat4 inverse           | __20.8923 ns__ |   43.4538 ns   |   47.5601 ns   |   53.8143 ns   |
| mat4 mul mat4          |  __6.6313 ns__ |    9.3879 ns   |   15.7381 ns   |    9.1823 ns   |
| mat4 transform point3  |  __2.4195 ns__ |    5.8730 ns   |    9.1768 ns   |    6.2335 ns   |
| mat4 transform vector3 |  __2.2820 ns__ |    4.5448 ns   |    4.8356 ns   |    4.0491 ns   |
| mat4 transform vector4 |  __2.5198 ns__ |    3.5723 ns   |    4.7060 ns   |      N/A       |
| mat4 transpose         |  __2.7021 ns__ |    7.8229 ns   |   11.0251 ns   |      N/A       |
| quat conjugate         |  __0.8947 ns__ |    1.7631 ns   |    1.7695 ns   |    1.7840 ns   |
| quat mul quat          |  __2.6549 ns__ |    3.7420 ns   |    4.0232 ns   |    7.9351 ns   |
| quat transform vector3 |  __4.1744 ns__ |    6.4873 ns   |   25.4556 ns   |    6.7573 ns   |
| vec3 cross             |  __2.0804 ns__ |    2.8876 ns   |    2.8487 ns   |    2.8373 ns   |
| vec3 dot               |  __1.4839 ns__ |    1.8252 ns   |    1.6298 ns   |    1.6791 ns   |
| vec3 length            |    2.0967 ns   |  __2.0193 ns__ |    2.0208 ns   |    2.4133 ns   |
| vec3 normalize         |  __4.0307 ns__ |    4.3551 ns   |    8.1171 ns   |    8.0631 ns   |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.38 Rust compiler. Lower (better) numbers are
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
[mathbench report]: https://bitshifter.github.io/mathbench/0.1.9/report/index.html
