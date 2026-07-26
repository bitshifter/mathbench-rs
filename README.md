# mathbench

[![Build Status]][github-ci]

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
* [`ultraviolet`][ultraviolet]
* [`vek`][vek]

[Build Status]: https://github.com/bitshifter/mathbench-rs/actions/workflows/ci.yml/badge.svg?branch=master
[github-ci]: https://github.com/bitshifter/mathbench-rs/actions/workflows/ci.yml
[cgmath]: https://crates.io/crates/cgmath
[euclid]: https://crates.io/crates/euclid
[glam]: https://github.com/bitshifter/glam-rs
[nalgebra]: https://nalgebra.org
[pathfinder_geometry]: https://crates.io/crates/pathfinder_geometry
[ultraviolet]: https://crates.io/crates/ultraviolet
[vek]: https://crates.io/crates/vek

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

[Criterion.rs]: https://bheisler.github.io/criterion.rs/book/index.html

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

## Crate differences

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

## Wide benchmarks

All benchmarks are gated as either "wide" or "scalar". This division allows us
to more fairly compare these different styles of libraries.

"scalar" benchmarks operate on standard scalar `f32` values, doing calculations
on one piece of data at a time (or in the case of a "horizontal" SIMD library
like `glam`, one `Vec3`/`Vec4` at a time).

"wide" benchmarks operate in a "vertical" AoSoA (Array-of-Struct-of-Arrays)
fashion, which is a programming model that allows the potential to more fully
use the advantages of SIMD operations. However, it has the cost of making
algorithm design harder, as scalar algorithms cannot be directly used by "wide"
architectures. Because of this difference in algorithms, we also can't really
*directly* compare the performance of "scalar" vs "wide" types because they
don't *quite* do the same thing (wide types operate on multiple pieces of data
at the same time).

The "wide" benchmarks still include `glam`, a scalar-only library, as a
comparison. Even though the comparison is somewhat apples-to-oranges, in each of
these cases, when running "wide" benchmark variants, `glam` is configured to do
the exact same *amount* of final work, producing the same outputs that the
"wide" versions would. The purpose is to give an idea of the possible throughput
benefits of "wide" types compared to writing the same algorithms with a scalar
type, at the cost of extra care being needed to write the algorithm.

To learn more about AoSoA architecture, see [this blog
post](https://www.rustsim.org/blog/2020/03/23/simd-aosoa-in-nalgebra/) by the
author of `nalgebra` which goes more in depth to how AoSoA works and its
possible benefits. Also take a look at the ["Examples"
section](https://github.com/termhn/ultraviolet#examples) of `ultraviolet`'s
README, which contains a discussion of how to port scalar algorithms to wide
ones, with the examples of the Euler integration and ray-sphere intersection
benchmarks from `mathbench`.

Note that the `nalgebra_f32x4` and `nalgebra_f32x8` benchmarks require a Rust

Additionally the `f32x8` benchmarks will require the `AVX2` instruction set, to
enable that you will need to build with `RUSTFLAGS='-C target-feature=+avx2`.

## Build settings

The default `profile.bench` settings are used, these are documented in the
[cargo reference].

Some math libraries are optimized to use specific instruction sets and may
benefit building with settings different to the defaults. Typically a game team
will need to decided on a minimum specification that they will target. Deciding
on a minimum specifiction dictates the potential audience size for a project.
This is an important decision for any game and it will be different for every
project. `mathbench` doesn't want to make assumptions about what build settings
any particular project may want to use which is why default settings are used.

I would encourage users who to use build settigs different to the defaults to
run the benchmarks themselves and consider publishing their results.

[cargo reference]: https://doc.rust-lang.org/cargo/reference/profiles.html#bench

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath`, `nalgebra`, `euclid`, `vek`, `pathfinder_geometry`,
and `ultraviolet` on `f32` data.

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux. They were
compiled with the `1.97.1 (8bab26f4f 2026-07-14)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `cgmath` - `0.18.0`
* `euclid` - `0.22.14`
* `glam` - `0.33.2`
* `nalgebra` - `0.35.0`
* `pathfinder_geometry` - `0.5.1`
* `ultraviolet` - `0.10.0`
* `vek` - `0.17.2`

See the full [mathbench report] for more detailed results.

### Scalar benchmarks

Run with the command:

```sh
cargo bench --features scalar scalar
```

| benchmark                      |          glam   |        cgmath   |      nalgebra   |        euclid   |           vek   |    pathfinder   |   ultraviolet   |
|--------------------------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|
| euler 2d x10000                |    __5.105 us__ |    __5.103 us__ |       7.04 us   |    __5.089 us__ |    __5.123 us__ |      5.936 us   |    __5.094 us__ |
| euler 3d x10000                |    __5.843 us__ |      16.66 us   |      16.38 us   |      16.66 us   |       16.5 us   |    __5.816 us__ |      16.42 us   |
| matrix2 determinant            |   __1.0264 ns__ |     1.1344 ns   |     1.1251 ns   |       N/A       |     1.1371 ns   |     1.1339 ns   |     1.1294 ns   |
| matrix2 inverse                |   __1.4480 ns__ |     2.2544 ns   |     2.1673 ns   |       N/A       |       N/A       |     1.6413 ns   |     1.7265 ns   |
| matrix2 mul matrix2            |   __1.2036 ns__ |     2.1007 ns   |     2.1017 ns   |       N/A       |     8.7975 ns   |     1.9278 ns   |     1.9789 ns   |
| matrix2 mul vector2 x1         |   __0.9248 ns__ |     1.1551 ns   |     1.1849 ns   |       N/A       |     4.4200 ns   |     1.1447 ns   |     1.1639 ns   |
| matrix2 mul vector2 x100       | __169.8841 ns__ |   178.2194 ns   |   178.7959 ns   |       N/A       |   440.0755 ns   | __169.8327 ns__ |   178.9964 ns   |
| matrix2 return self            |     1.6641 ns   |     1.5034 ns   |     1.3011 ns   |       N/A       |     1.2503 ns   |   __1.1120 ns__ |     1.2651 ns   |
| matrix2 transpose              |     1.2246 ns   |     2.0468 ns   |     1.2677 ns   |       N/A       |   __0.8364 ns__ |       N/A       |     1.0949 ns   |
| matrix3 determinant            |   __2.1669 ns__ |     2.2153 ns   |     2.2430 ns   |       N/A       |   __2.1571 ns__ |       N/A       |    10.6261 ns   |
| matrix3 inverse                |   __5.7174 ns__ |     9.4126 ns   |     5.9217 ns   |       N/A       |       N/A       |       N/A       |    12.2454 ns   |
| matrix3 mul matrix3            |     4.2349 ns   |     5.3985 ns   |   __4.0468 ns__ |       N/A       |    36.7534 ns   |       N/A       |     4.3751 ns   |
| matrix3 mul vector3 x1         |   __2.2035 ns__ |     3.4485 ns   |   __2.1695 ns__ |       N/A       |    12.4544 ns   |       N/A       |   __2.1676 ns__ |
| matrix3 mul vector3 x100       |   __0.3599 us__ |     0.4124 us   |     __0.36 us__ |       N/A       |       1.27 us   |       N/A       |   __0.3602 us__ |
| matrix3 return self            |     3.1043 ns   |     3.1475 ns   |   __2.9197 ns__ |       N/A       |     3.1150 ns   |       N/A       |   __2.9468 ns__ |
| matrix3 transpose              |   __2.0354 ns__ |     2.7774 ns   |     2.1004 ns   |       N/A       |     2.8037 ns   |       N/A       |     2.3741 ns   |
| matrix4 determinant            |   __4.3484 ns__ |     7.1268 ns   |    42.2470 ns   |    11.3741 ns   |    15.1950 ns   |       N/A       |     4.6184 ns   |
| matrix4 inverse                |  __12.2161 ns__ |    31.1015 ns   |    43.6364 ns   |    47.0161 ns   |    26.0357 ns   |       N/A       |    21.1654 ns   |
| matrix4 mul matrix4            |   __4.4729 ns__ |     6.3596 ns   |     4.7062 ns   |     6.3086 ns   |    99.9962 ns   |       N/A       |     4.8428 ns   |
| matrix4 mul vector4 x1         |   __1.6675 ns__ |     2.0058 ns   |     1.8523 ns   |       N/A       |    24.8905 ns   |       N/A       |     2.1029 ns   |
| matrix4 mul vector4 x100       |   __0.5005 us__ |     0.5214 us   |     0.5211 us   |       N/A       |      2.497 us   |       N/A       |      0.522 us   |
| matrix4 return self            |   __3.3184 ns__ |   __3.3582 ns__ |   __3.3468 ns__ |       N/A       |   __3.3241 ns__ |       N/A       |   __3.3679 ns__ |
| matrix4 transpose              |   __2.4832 ns__ |     7.6808 ns   |     7.7030 ns   |       N/A       |     7.7050 ns   |       N/A       |     7.6882 ns   |
| ray-sphere intersection x10000 |    __13.04 us__ |    __13.08 us__ |    __13.03 us__ |    __13.09 us__ |    __13.01 us__ |       N/A       |    __13.02 us__ |
| rotation3 inverse              |   __0.9278 ns__ |     1.6702 ns   |     1.8072 ns   |     1.6791 ns   |     1.6917 ns   |       N/A       |     1.6777 ns   |
| rotation3 mul rotation3        |   __1.8864 ns__ |     2.4742 ns   |     2.4316 ns   |     2.4884 ns   |     2.8098 ns   |       N/A       |     2.4154 ns   |
| rotation3 mul vector3 x1       |     3.2106 ns   |   __2.8860 ns__ |     3.1208 ns   |     3.0948 ns   |     4.7126 ns   |       N/A       |     5.0006 ns   |
| rotation3 mul vector3 x100     |   353.3250 ns   | __312.1237 ns__ |   338.1341 ns   |   337.7158 ns   |   496.4275 ns   |       N/A       |   519.0059 ns   |
| rotation3 return self          |     1.5232 ns   |   __1.1782 ns__ |   __1.1783 ns__ |       N/A       |   __1.1783 ns__ |       N/A       |     1.2085 ns   |
| transform point2 x1            |     1.4027 ns   |     2.3675 ns   |     2.8704 ns   |   __1.3384 ns__ |     4.4616 ns   |     1.4336 ns   |     2.1414 ns   |
| transform point2 x100          |   265.8952 ns   |   327.8923 ns   |   352.1796 ns   | __241.8527 ns__ |   501.8732 ns   |   266.4064 ns   |   327.5572 ns   |
| transform point3 x1            |   __2.0197 ns__ |    15.0797 ns   |     4.0689 ns   |     3.5944 ns   |    12.7437 ns   |   __2.0198 ns__ |     3.3336 ns   |
| transform point3 x100          |   __0.4963 us__ |      1.551 us   |     0.5856 us   |     0.5732 us   |      1.306 us   |   __0.5061 us__ |     0.5614 us   |
| transform vector2 x1           |   __1.7796 ns__ |       N/A       |     2.2092 ns   |   __1.7457 ns__ |     8.3798 ns   |       N/A       |     2.0981 ns   |
| transform vector2 x100         |   260.4659 ns   |       N/A       |   330.3821 ns   | __233.3422 ns__ |   888.4672 ns   |       N/A       |   301.9626 ns   |
| transform vector3 x1           |   __1.8805 ns__ |    11.7665 ns   |     2.9441 ns   |     2.3248 ns   |    18.5256 ns   |       N/A       |     2.6674 ns   |
| transform vector3 x100         |   __0.4895 us__ |      1.183 us   |      0.548 us   |      0.507 us   |      1.894 us   |       N/A       |     0.5154 us   |
| transform2 inverse             |   __2.2471 ns__ |       N/A       |     5.8765 ns   |     2.9375 ns   |       N/A       |     2.5209 ns   |       N/A       |
| transform2 mul transform2      |     2.1974 ns   |       N/A       |     4.0576 ns   |     2.3661 ns   |       N/A       |   __2.1055 ns__ |       N/A       |
| transform2 return self         |   __2.2791 ns__ |       N/A       |     2.8896 ns   |   __2.2685 ns__ |       N/A       |   __2.2697 ns__ |       N/A       |
| transform3 inverse             |   __6.2683 ns__ |       N/A       |    43.0645 ns   |    38.9633 ns   |       N/A       |    20.6553 ns   |       N/A       |
| transform3 mul transform3d     |   __3.5214 ns__ |       N/A       |     4.7709 ns   |     6.2820 ns   |       N/A       |     4.5047 ns   |       N/A       |
| transform3 return self         |   __3.3519 ns__ |       N/A       |   __3.3666 ns__ |   __3.3499 ns__ |       N/A       |   __3.3407 ns__ |       N/A       |
| vector3 cross                  |     1.6686 ns   |     1.8007 ns   |     1.9510 ns   |   __1.5773 ns__ |   __1.5469 ns__ |       N/A       |     1.8179 ns   |
| vector3 dot                    |     1.6312 ns   |     1.5790 ns   |   __1.2303 ns__ |     1.4930 ns   |     1.8033 ns   |       N/A       |   __1.2320 ns__ |
| vector3 length                 |   __1.3176 ns__ |   __1.3128 ns__ |   __1.3284 ns__ |   __1.3104 ns__ |   __1.3084 ns__ |       N/A       |   __1.3032 ns__ |
| vector3 normalize              |   __2.1981 ns__ |     2.5905 ns   |     3.0148 ns   |     3.0247 ns   |     3.0176 ns   |       N/A       |     2.6002 ns   |
| vector3 return self            |   __1.7745 ns__ |     3.1087 ns   |     2.9461 ns   |       N/A       |     2.9658 ns   |       N/A       |     2.9338 ns   |

### Wide benchmarks

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux. They were
compiled with the `1.98.0-nightly (8954863c8 2026-06-05)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `glam` - `0.33.2`
* `nalgebra` - `0.35.0` (not included, see note)
* `ultraviolet` - `0.10.0`

Run with the command:

```sh
RUSTFLAGS='-C target-feature=+avx2' cargo +nightly bench --features wide wide
```

| benchmark                      |    glam_f32x1   |   ultraviolet_f32x4   |   nalgebra_f32x4   |   ultraviolet_f32x8   |   nalgebra_f32x8   |
|--------------------------------|-----------------|-----------------------|--------------------|-----------------------|--------------------|
| euler 2d x80000                |       43.3 us   |            26.59 us   |          N/A       |          __18.66 us__ |          N/A       |
| euler 3d x80000                |      58.47 us   |             39.8 us   |          N/A       |          __28.68 us__ |          N/A       |
| matrix2 determinant x16        |    11.1725 ns   |           5.0338 ns   |          N/A       |         __3.6542 ns__ |          N/A       |
| matrix2 inverse x16            |    18.3731 ns   |          10.2998 ns   |          N/A       |         __8.1694 ns__ |          N/A       |
| matrix2 mul matrix2 x16        |    16.4601 ns   |          11.0250 ns   |          N/A       |         __9.2715 ns__ |          N/A       |
| matrix2 mul matrix2 x256       |   864.3274 ns   |         800.5418 ns   |          N/A       |       __694.9632 ns__ |          N/A       |
| matrix2 mul vector2 x16        |    12.9706 ns   |           7.1741 ns   |          N/A       |         __5.8479 ns__ |          N/A       |
| matrix2 mul vector2 x256       |   464.0427 ns   |       __419.5325 ns__ |          N/A       |         448.4969 ns   |          N/A       |
| matrix2 return self x16        |     9.9320 ns   |          15.4263 ns   |          N/A       |         __7.3024 ns__ |          N/A       |
| matrix2 transpose x16          |   __9.8450 ns__ |          49.7832 ns   |          N/A       |          11.4744 ns   |          N/A       |
| matrix3 determinant x16        |    28.6555 ns   |          10.9033 ns   |          N/A       |         __8.0215 ns__ |          N/A       |
| matrix3 inverse x16            |   103.2186 ns   |          38.3324 ns   |          N/A       |        __22.2609 ns__ |          N/A       |
| matrix3 mul matrix3 x16        |    81.6414 ns   |          41.1633 ns   |          N/A       |        __33.3126 ns__ |          N/A       |
| matrix3 mul matrix3 x256       |      1.582 us   |            1.594 us   |          N/A       |          __1.531 us__ |          N/A       |
| matrix3 mul vector3 x16        |    37.0081 ns   |          13.8279 ns   |          N/A       |         __9.9587 ns__ |          N/A       |
| matrix3 mul vector3 x256       |   926.2454 ns   |       __782.1606 ns__ |          N/A       |       __768.3449 ns__ |          N/A       |
| matrix3 return self x16        |    32.3532 ns   |          33.5448 ns   |          N/A       |        __27.9238 ns__ |          N/A       |
| matrix3 transpose x16          |  __24.8025 ns__ |          81.5843 ns   |          N/A       |          35.7089 ns   |          N/A       |
| matrix4 determinant x16        |    62.6694 ns   |          21.9621 ns   |          N/A       |        __15.2941 ns__ |          N/A       |
| matrix4 inverse x16            |   201.2134 ns   |       __103.0745 ns__ |          N/A       |         128.5983 ns   |          N/A       |
| matrix4 mul matrix4 x16        | __161.3080 ns__ |       __161.8795 ns__ |          N/A       |         176.2933 ns   |          N/A       |
| matrix4 mul matrix4 x256       |      3.348 us   |          __2.919 us__ |          N/A       |          __2.965 us__ |          N/A       |
| matrix4 mul vector4 x16        |    36.4614 ns   |          28.0231 ns   |          N/A       |        __22.8885 ns__ |          N/A       |
| matrix4 mul vector4 x256       |      1.292 us   |          __1.241 us__ |          N/A       |           __1.22 us__ |          N/A       |
| matrix4 return self x16        |  __72.3178 ns__ |          77.3502 ns   |          N/A       |          98.6233 ns   |          N/A       |
| matrix4 transpose x16          |  __75.8296 ns__ |         115.3968 ns   |          N/A       |          92.3958 ns   |          N/A       |
| ray-sphere intersection x80000 |      501.2 us   |            50.18 us   |          N/A       |          __25.37 us__ |          N/A       |
| rotation3 inverse x16          |     9.9220 ns   |           9.4615 ns   |          N/A       |         __7.1113 ns__ |          N/A       |
| rotation3 mul rotation3 x16    |    26.6809 ns   |          13.3517 ns   |          N/A       |         __9.4258 ns__ |          N/A       |
| rotation3 mul vector3 x16      |    53.0180 ns   |          17.6017 ns   |          N/A       |         __9.5417 ns__ |          N/A       |
| rotation3 return self x16      |     9.8377 ns   |          10.3090 ns   |          N/A       |         __7.4035 ns__ |          N/A       |
| transform point2 x16           |    15.6328 ns   |          12.3902 ns   |          N/A       |         __9.4146 ns__ |          N/A       |
| transform point2 x256          | __681.4482 ns__ |       __667.9056 ns__ |          N/A       |       __671.0573 ns__ |          N/A       |
| transform point3 x16           |    33.3548 ns   |          25.0054 ns   |          N/A       |        __19.3086 ns__ |          N/A       |
| transform point3 x256          |      1.262 us   |          __1.131 us__ |          N/A       |          __1.119 us__ |          N/A       |
| transform vector2 x16          |    13.2051 ns   |          10.2285 ns   |          N/A       |         __9.2823 ns__ |          N/A       |
| transform vector2 x256         |   658.2204 ns   |       __640.8898 ns__ |          N/A       |         681.3735 ns   |          N/A       |
| transform vector3 x16          |    30.4851 ns   |          21.6303 ns   |          N/A       |        __17.6012 ns__ |          N/A       |
| transform vector3 x256         |      1.258 us   |          __1.091 us__ |          N/A       |            1.144 us   |          N/A       |
| vector3 cross x16              |    16.4440 ns   |           9.2684 ns   |          N/A       |         __7.8525 ns__ |          N/A       |
| vector3 dot x16                |    13.7536 ns   |           6.7754 ns   |          N/A       |         __4.6961 ns__ |          N/A       |
| vector3 length x16             |    22.2836 ns   |           5.7117 ns   |          N/A       |         __3.7349 ns__ |          N/A       |
| vector3 normalize x16          |    35.9540 ns   |          10.6899 ns   |          N/A       |         __7.8765 ns__ |          N/A       |
| vector3 return self x16        |     9.7731 ns   |          23.5402 ns   |          N/A       |         __5.9862 ns__ |          N/A       |

[AMD Ryzen 7 3800X]: https://www.amd.com/en/products/cpu/amd-ryzen-7-3800x
[mathbench report]: https://bitshifter.github.io/mathbench/0.4.1/report/index.html

## Running the benchmarks

The benchmarks use the criterion crate which works on stable Rust, they can be
run with:

```sh
cargo bench
```

For the best results close other applications on the machine you are using to
benchmark!

When running "wide" benchmarks, be sure you compile with with the appropriate
`target-feature`s enabled, e.g. `+avx2`, for best results.

There is a script in `scripts/summary.py` to summarize the results in a nice
fashion. It requires Python 3 and the `prettytable` Python module, then can
be run to generate an ASCII output.

## Default and optional features

All libraries except for `glam` are optional for running benchmarks. The default
features include `cgmath`, `ultraviolet` and `nalgebra`. These can be disabled
with:

```sh
cargo bench --no-default-features
```

To selectively enable a specific default feature again use:

```sh
cargo bench --no-default-features --features nalgebra
```

Note that you can filter which benchmarks to run at runtime by using
Criterion's filtering feature. For example, to only run scalar benchmarks
and not wide ones, use:

```sh
cargo bench "scalar"
```

You can also get more granular. For example to only run wide matrix2 benchmarks,
use:

```sh
cargo bench --features wide "wide matrix2"
```

or to only run the scalar "vec3 length" benchmark for `glam`, use:

```sh
cargo bench "scalar vec3 length/glam"
```

### Crate features

There are a few extra features in addition to the direct features referring to
each benchmarked library.

* `ultraviolet_f32x4`, `ultraviolet_f32x8`, `nalgebra_f32x4`,
  `nalgebra_f32x8` - these each enable benchmarking specific wide types from
  each of `ultraviolet` or `nalgebra`.
* `ultraviolet_wide`, `nalgebra_wide` - these enable benchmarking all wide
  types from `ultraviolet` or `nalgebra` respectively.
* `wide` - enables all "wide" type benchmarks
* `all` - enables all supported libraries, including wide and scalar ones.
* `unstable` - see next section

#### `unstable` feature

The `unstable` feature requires a nightly compiler, and it allows us to tell
rustc not to inline certain functions within hot benchmark loops. This is used
in the ray-sphere intersection benchmark in order to simulate situations where
the autovectorizer would not be able to properly vectorize your code.

## Running the tests

The tests can be run using:

```sh
cargo test
```

## Publishing results

When publishing benchmark results it is important to document the details of how
the benchmarks were run, including:

* The version of `mathbench` used
* The versions of all libraries benched
* The Rust version
* The build settings used, especially when they differ from the defaults
* The specification of the hardware that was used
* The output of `scripts/summary.py`
* The full Criterion output from `target/criterion`

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

`mathbench` also includes a tool for comparing full build times in
`tools/buildbench`. Incremental build times are not measured as it would be non
trivial to create a meaningful test across different math crates.

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

| crate               | version | total (s) | self (s) | units |
|:--------------------|:--------|----------:|---------:|------:|
| cgmath              | 0.17.0  |       6.8 |      3.0 |    17 |
| euclid              | 0.22.1  |       3.4 |      1.0 |     4 |
| glam                | 0.9.4   |       1.1 |      0.6 |     2 |
| nalgebra            | 0.22.0  |      24.2 |     18.0 |    24 |
| pathfinder_geometry | 0.5.1   |       3.0 |      0.3 |     8 |
| ultraviolet         | 0.5.1   |       2.5 |      1.3 |     4 |
| vek                 | 0.12.0  |      34.4 |     10.1 |    16 |

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU with 32GB RAM on Linux.

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

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html

## Support

If you are interested in contributing or have a request or suggestion
[create an issue] on github.

[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
