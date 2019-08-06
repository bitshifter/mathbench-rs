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
* [hektor](https://docs.rs/hektor)

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
| euler 2d            |     9.063 us   |   __8.977 us__ |     27.94 us   |
| euler 3d            |   __15.78 us__ |     29.52 us   |     195.4 us   |
| mat2 determinant    |    1.3140 ns   |  __1.0528 ns__ |    1.0825 ns   |
| mat2 inverse        |  __2.0524 ns__ |    2.6565 ns   |    2.7056 ns   |
| mat2 mul mat2       |  __2.1091 ns__ |    3.0495 ns   |    3.6305 ns   |
| mat2 transform vec2 |  __2.1865 ns__ |    2.4035 ns   |    6.8877 ns   |
| mat2 transpose      |  __0.7117 ns__ |    1.3231 ns   |    1.8297 ns   |
| mat3 determinant    |  __2.2130 ns__ |    2.5348 ns   |    2.5766 ns   |
| mat3 inverse        |  __7.7296 ns__ |    7.9519 ns   |    9.3100 ns   |
| mat3 mul mat3       |  __4.9371 ns__ |    9.4619 ns   |    8.3076 ns   |
| mat3 transform vec3 |  __2.2893 ns__ |    4.0822 ns   |    8.0838 ns   |
| mat3 transpose      |  __2.0035 ns__ |    3.5175 ns   |    8.9548 ns   |
| mat4 determinant    |  __7.9524 ns__ |   12.2581 ns   |   54.2954 ns   |
| mat4 inverse        | __21.0455 ns__ |   43.6983 ns   |   55.8788 ns   |
| mat4 mul mat4       |  __6.7517 ns__ |    9.2621 ns   |   16.2498 ns   |
| mat4 transform vec4 |  __2.5298 ns__ |    3.5734 ns   |    4.1222 ns   |
| mat4 transpose      |  __2.7026 ns__ |    8.0926 ns   |   11.5411 ns   |
| quat conjugate      |  __0.8866 ns__ |    1.8263 ns   |    1.7650 ns   |
| quat mul quat       |  __2.8647 ns__ |    5.6678 ns   |    5.4807 ns   |
| quat transform vec3 |  __4.2107 ns__ |    6.5798 ns   |    7.0398 ns   |
| vec3 cross          |  __2.0677 ns__ |    2.8890 ns   |    2.8725 ns   |
| vec3 dot            |  __1.3789 ns__ |    1.6669 ns   |    1.6666 ns   |
| vec3 length         |  __2.0412 ns__ |    2.0508 ns   |   88.1277 ns   |
| vec3 normalize      |  __4.0433 ns__ |    4.1260 ns   |   87.2455 ns   |

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
