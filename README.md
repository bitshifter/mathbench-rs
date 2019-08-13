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

## The benchmarks

All benchmarks are performed using [Criterion.rs].

* `euler bench` - performs an Euler integration on arrays of 2D and 3D vectors
* `mat benches` - performs common matrix operations such as transpose, inverse,
  determinant and multiply
* `quat benches` - perform common quaternion operations
* `transform bench` - performs transformations on vectors

Note that `cgmath` and `nalgebra` matrix inverse methods return an `Option`
whereas `glam` does not, giving glam a performance advantage. If a
non-invertible matrix is inverted by `glam` the result will be invalid (it will
contain NaNs). The benchmarks are currently focused on `f32` types as that is
all `glam` supports.

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath` and `nalgebra` on `f32` data.

| benchmark           |         glam   |       cgmath   |     nalgebra   |
|:--------------------|---------------:|---------------:|---------------:|
| euler 2d            |   __9.045 us__ |     11.22 us   |     27.98 us   |
| euler 3d            |   __15.66 us__ |     29.54 us   |     195.2 us   |
| mat2 determinant    |    1.3129 ns   |  __1.0533 ns__ |    1.0838 ns   |
| mat2 inverse        |  __2.0472 ns__ |    2.6900 ns   |    2.6850 ns   |
| mat2 mul mat2       |  __2.1437 ns__ |    3.0665 ns   |    4.1104 ns   |
| mat2 transform vec2 |  __2.1776 ns__ |    2.3907 ns   |    6.9174 ns   |
| mat2 transpose      |  __0.7422 ns__ |    1.3226 ns   |    1.8211 ns   |
| mat3 determinant    |  __2.2018 ns__ |    2.5317 ns   |    2.5732 ns   |
| mat3 inverse        |  __7.7098 ns__ |    7.9422 ns   |    9.2800 ns   |
| mat3 mul mat3       |  __4.8232 ns__ |    9.4786 ns   |    9.4087 ns   |
| mat3 transform vec3 |  __2.4629 ns__ |    4.0456 ns   |    7.7514 ns   |
| mat3 transpose      |  __1.9799 ns__ |    3.5518 ns   |    8.9686 ns   |
| mat4 determinant    |  __7.6707 ns__ |   11.1863 ns   |   49.6599 ns   |
| mat4 inverse        | __22.1710 ns__ |   47.7588 ns   |   55.5281 ns   |
| mat4 mul mat4       |  __6.8146 ns__ |    9.7314 ns   |   19.0294 ns   |
| mat4 transform vec4 |  __2.5205 ns__ |    3.6180 ns   |    5.0138 ns   |
| mat4 transpose      |  __2.7045 ns__ |    7.9254 ns   |   10.7011 ns   |
| quat conjugate      |  __0.8886 ns__ |    1.7810 ns   |    1.7754 ns   |
| quat mul quat       |  __2.6494 ns__ |    5.3704 ns   |    5.2748 ns   |
| quat transform vec3 |  __4.1672 ns__ |    6.5700 ns   |    6.9521 ns   |
| vec3 cross          |  __2.0821 ns__ |    2.8592 ns   |    2.8456 ns   |
| vec3 dot            |  __1.3839 ns__ |    1.6771 ns   |    1.6275 ns   |
| vec3 length         |    2.0189 ns   |  __2.0176 ns__ |    2.1048 ns   |
| vec3 normalize      |  __4.0366 ns__ |    4.1326 ns   |    8.1539 ns   |

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the stable 1.36 Rust compiler. Lower (better) numbers are
highlighted. The versions of the libraries tested were:

* `glam` - 0.7.1
* `cgmath` - 0.17.0
* `nalgebra` - 0.18.0

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
[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html
[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html
[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
[mathbench report]: https://bitshifter.github.io/mathbench/criterion/report/index.html
