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
performance to `cgmath`, `nalgebra` and `euclid` on `f32` data.

| benchmark              |         glam   |       cgmath   |     nalgebra   |       euclid   |
|:-----------------------|---------------:|---------------:|---------------:|---------------:|
| euler 2d               |     9.073 us   |   __8.934 us__ |     26.43 us   |      13.0 us   |
| euler 3d               |   __15.24 us__ |     29.62 us   |     195.5 us   |     29.23 us   |
| mat2 determinant       |    1.3163 ns   |    1.0556 ns   |  __1.0545 ns__ |      N/A       |
| mat2 inverse           |  __2.0394 ns__ |    2.6781 ns   |    3.1784 ns   |      N/A       |
| mat2 mul mat2          |  __2.1452 ns__ |    3.0677 ns   |    4.1388 ns   |      N/A       |
| mat2 transform vector2 |  __2.1925 ns__ |    2.4002 ns   |    6.8965 ns   |      N/A       |
| mat2 transpose         |  __0.7142 ns__ |    1.3398 ns   |    1.8191 ns   |      N/A       |
| mat3 determinant       |    2.2111 ns   |    2.5348 ns   |    2.5907 ns   |  __1.1650 ns__ |
| mat3 inverse           |    7.8504 ns   |    7.9842 ns   |    9.2977 ns   |  __4.2679 ns__ |
| mat3 mul mat3          |    4.8757 ns   |    9.8963 ns   |    8.1304 ns   |  __4.6535 ns__ |
| mat3 transform point2  |  __2.5054 ns__ |    2.6898 ns   |    7.7799 ns   |    2.7398 ns   |
| mat3 transform vector2 |    2.5236 ns   |      N/A       |    3.0775 ns   |  __2.4915 ns__ |
| mat3 transform vector3 |  __2.2815 ns__ |    4.0497 ns   |    7.7459 ns   |      N/A       |
| mat3 transpose         |  __2.0142 ns__ |    3.5186 ns   |    8.9544 ns   |      N/A       |
| mat4 determinant       |  __7.8211 ns__ |   11.1859 ns   |   49.8123 ns   |   16.8770 ns   |
| mat4 inverse           | __20.9937 ns__ |   43.4030 ns   |   55.5264 ns   |   73.7835 ns   |
| mat4 mul mat4          |  __6.5843 ns__ |    9.2085 ns   |   15.9321 ns   |    9.8230 ns   |
| mat4 transform point3  |  __2.4372 ns__ |    6.0644 ns   |    9.1896 ns   |    6.8941 ns   |
| mat4 transform vector3 |  __2.2815 ns__ |    4.5304 ns   |    4.7440 ns   |    4.0788 ns   |
| mat4 transform vector4 |  __2.4980 ns__ |    3.6186 ns   |    4.7024 ns   |      N/A       |
| mat4 transpose         |  __2.7029 ns__ |    7.9501 ns   |   10.7095 ns   |      N/A       |
| quat conjugate         |  __0.8891 ns__ |    1.7593 ns   |    1.7678 ns   |    1.7626 ns   |
| quat mul quat          |  __2.5916 ns__ |    5.3766 ns   |    5.3322 ns   |    5.6205 ns   |
| quat transform vector3 |  __4.1817 ns__ |    6.5586 ns   |   26.4081 ns   |    6.7978 ns   |
| vec3 cross             |  __2.0762 ns__ |    2.8369 ns   |    2.8357 ns   |    2.8369 ns   |
| vec3 dot               |  __1.3473 ns__ |    1.6750 ns   |    1.6442 ns   |    1.7038 ns   |
| vec3 length            |    2.0183 ns   |    2.0183 ns   |  __2.0123 ns__ |    2.0228 ns   |
| vec3 normalize         |  __4.0360 ns__ |    4.1565 ns   |    8.1527 ns   |    8.0602 ns   |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.36 Rust compiler. Lower (better) numbers are
highlighted. The versions of the libraries tested were:

* `glam` - 0.7.1
* `cgmath` - 0.17.0
* `nalgebra` - 0.18.0
* `euclid` - 0.20.1

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

To add the new libary type to a benchmark, add another `.with_function` call to
the `Criterion` `bench`.

Increment the patch version number of `mathbench` in the `Cargo.toml`.

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
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
[mathbench report]: https://bitshifter.github.io/mathbench/0.1.8/report/index.html
