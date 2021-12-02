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

[Build Status]: https://travis-ci.org/bitshifter/mathbench-rs.svg?branch=master
[travis-ci]: https://travis-ci.org/bitshifter/mathbench-rs
[cgmath]: https://crates.io/crates/cgmath
[euclid]: https://crates.io/crates/euclid
[glam]: https://github.com/bitshifter/glam-rs
[nalgebra]: https://nalgebra.org
[pathfinder_geometry]: https://crates.io/crates/pathfinder_geometry
[static-math]: https://crates.io/crates/static-math
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
`static-math` and `ultraviolet` on `f32` data.

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the `1.56.1 (59eed8a2a 2021-11-01)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `cgmath` - `0.18.0`
* `euclid` - `0.22.6`
* `glam` - `0.20.1`
* `nalgebra` - `0.29.0`
* `pathfinder_geometry` - `0.5.1`
* `static-math` - `0.2.3`
* `ultraviolet` - `0.8.1`
* `vek` - `0.15.3` (`repr_c` types)

See the full [mathbench report] for more detailed results.

### Scalar benchmarks

Run with the command:

```sh
cargo bench --features scalar scalar
```

| benchmark                      |          glam   |        cgmath   |      nalgebra   |       euclid   |           vek   |    pathfinder   |   static-math   |   ultraviolet   |
|--------------------------------|-----------------|-----------------|-----------------|----------------|-----------------|-----------------|-----------------|-----------------|
| euler 2d x10000                |      16.23 us   |      16.13 us   |    __9.954 us__ |     16.18 us   |       16.2 us   |      10.42 us   |     __9.97 us__ |      16.17 us   |
| euler 3d x10000                |    __15.95 us__ |      32.11 us   |      32.13 us   |     32.13 us   |      32.13 us   |    __16.27 us__ |      32.16 us   |      32.11 us   |
| matrix2 determinant            |   __2.0386 ns__ |     2.0999 ns   |     2.1018 ns   |      N/A       |     2.0997 ns   |     2.0987 ns   |     2.0962 ns   |     2.1080 ns   |
| matrix2 inverse                |   __2.8226 ns__ |     8.4418 ns   |     7.6303 ns   |      N/A       |       N/A       |     3.3459 ns   |     9.4636 ns   |     5.8796 ns   |
| matrix2 mul matrix2            |   __2.6036 ns__ |     5.0007 ns   |     4.8172 ns   |      N/A       |     9.3814 ns   |   __2.5516 ns__ |     4.7274 ns   |     4.9428 ns   |
| matrix2 mul vector2 x1         |     2.4904 ns   |     2.6144 ns   |     2.8714 ns   |      N/A       |     4.2139 ns   |   __2.0839 ns__ |     2.8873 ns   |     2.6250 ns   |
| matrix2 mul vector2 x100       |   227.5271 ns   |   243.3579 ns   |   265.1698 ns   |      N/A       |   400.6940 ns   | __219.7127 ns__ |   267.8780 ns   |   243.9880 ns   |
| matrix2 return self            |   __2.4235 ns__ |     2.8841 ns   |     2.8756 ns   |      N/A       |     2.8754 ns   |   __2.4147 ns__ |     2.8717 ns   |     2.8697 ns   |
| matrix2 transpose              |   __2.2887 ns__ |     3.0645 ns   |     7.9154 ns   |      N/A       |     2.9635 ns   |       N/A       |     3.0637 ns   |     3.0652 ns   |
| matrix3 determinant            |     3.9129 ns   |   __3.8107 ns__ |   __3.8191 ns__ |      N/A       |   __3.8180 ns__ |       N/A       |   __3.8151 ns__ |     8.9368 ns   |
| matrix3 inverse                |    17.5373 ns   |    18.6931 ns   |  __12.3183 ns__ |      N/A       |       N/A       |       N/A       |    12.8195 ns   |    21.9098 ns   |
| matrix3 mul matrix3            |     9.9578 ns   |    13.3648 ns   |     7.8154 ns   |      N/A       |    35.5802 ns   |       N/A       |   __6.4938 ns__ |    10.0527 ns   |
| matrix3 mul vector3 x1         |     4.8090 ns   |     4.9339 ns   |   __4.5046 ns__ |      N/A       |    12.5518 ns   |       N/A       |     4.8002 ns   |     4.8118 ns   |
| matrix3 mul vector3 x100       |   __0.4836 us__ |   __0.4808 us__ |   __0.4755 us__ |      N/A       |      1.247 us   |       N/A       |   __0.4816 us__ |   __0.4755 us__ |
| matrix3 return self            |   __5.4421 ns__ |   __5.4469 ns__ |   __5.4526 ns__ |      N/A       |   __5.4656 ns__ |       N/A       |   __5.4718 ns__ |   __5.4043 ns__ |
| matrix3 transpose              |   __9.9567 ns__ |  __10.0794 ns__ |    10.9704 ns   |      N/A       |   __9.9257 ns__ |       N/A       |    10.7350 ns   |    10.5334 ns   |
| matrix4 determinant            |   __6.2050 ns__ |    11.1041 ns   |    69.2549 ns   |   17.1809 ns   |    18.5233 ns   |       N/A       |    16.5331 ns   |     8.2704 ns   |
| matrix4 inverse                |  __16.4386 ns__ |    47.0674 ns   |    71.8174 ns   |   64.1356 ns   |   284.3703 ns   |       N/A       |    52.6993 ns   |    41.1780 ns   |
| matrix4 mul matrix4            |   __7.7715 ns__ |    26.7308 ns   |     8.6500 ns   |   10.4414 ns   |    86.1501 ns   |       N/A       |    21.7985 ns   |    26.8056 ns   |
| matrix4 mul vector4 x1         |   __3.0303 ns__ |     7.7400 ns   |     3.4091 ns   |      N/A       |    21.0968 ns   |       N/A       |     6.2971 ns   |     6.2537 ns   |
| matrix4 mul vector4 x100       |   __0.6136 us__ |     0.9676 us   |    __0.627 us__ |      N/A       |      2.167 us   |       N/A       |     0.7893 us   |     0.8013 us   |
| matrix4 return self            |     7.1741 ns   |   __6.8838 ns__ |     7.5030 ns   |      N/A       |     7.0410 ns   |       N/A       |   __6.7768 ns__ |     6.9508 ns   |
| matrix4 transpose              |   __6.6826 ns__ |    12.4966 ns   |    15.3265 ns   |      N/A       |    12.6386 ns   |       N/A       |    15.2657 ns   |    12.3396 ns   |
| ray-sphere intersection x10000 |       56.2 us   |       55.7 us   |    __15.32 us__ |     55.45 us   |      56.02 us   |       N/A       |       N/A       |      50.94 us   |
| rotation3 inverse              |   __2.3113 ns__ |     3.1752 ns   |     3.3292 ns   |    3.3311 ns   |     3.1808 ns   |       N/A       |     8.7109 ns   |     3.6535 ns   |
| rotation3 mul rotation3        |   __3.6584 ns__ |     7.5255 ns   |     7.4808 ns   |    8.1393 ns   |    14.1636 ns   |       N/A       |     6.8044 ns   |     7.6386 ns   |
| rotation3 mul vector3 x1       |   __6.4950 ns__ |     7.6808 ns   |     7.5784 ns   |    7.5746 ns   |    18.2547 ns   |       N/A       |     7.2727 ns   |     8.9732 ns   |
| rotation3 mul vector3 x100     |   __0.6465 us__ |     0.7844 us   |     0.7573 us   |    0.7533 us   |      1.769 us   |       N/A       |     0.7317 us   |     0.9416 us   |
| rotation3 return self          |   __2.4928 ns__ |     2.8740 ns   |     2.8687 ns   |      N/A       |     2.8724 ns   |       N/A       |     4.7868 ns   |     2.8722 ns   |
| transform point2 x1            |     2.7854 ns   |     2.8878 ns   |     4.4207 ns   |    2.8667 ns   |    11.9427 ns   |   __2.3601 ns__ |       N/A       |     4.1770 ns   |
| transform point2 x100          |     0.3316 us   |     0.3574 us   |     0.4445 us   |  __0.3008 us__ |      1.212 us   |     0.3184 us   |       N/A       |     0.4332 us   |
| transform point3 x1            |   __2.9619 ns__ |    10.6812 ns   |     6.1037 ns   |    7.7051 ns   |    13.2607 ns   |     3.0934 ns   |       N/A       |     6.8419 ns   |
| transform point3 x100          |   __0.6095 us__ |       1.27 us   |     0.8064 us   |    0.7674 us   |      1.446 us   |   __0.6189 us__ |       N/A       |     0.8899 us   |
| transform vector2 x1           |   __2.4944 ns__ |       N/A       |     3.7174 ns   |    2.6273 ns   |    11.9424 ns   |       N/A       |       N/A       |     3.0458 ns   |
| transform vector2 x100         |     0.3125 us   |       N/A       |     0.3871 us   |  __0.2817 us__ |      1.213 us   |       N/A       |       N/A       |     0.3649 us   |
| transform vector3 x1           |   __2.8091 ns__ |     7.7343 ns   |     5.5064 ns   |    4.4810 ns   |    15.4097 ns   |       N/A       |       N/A       |     4.8819 ns   |
| transform vector3 x100         |   __0.6035 us__ |     0.9439 us   |     0.7573 us   |    0.6327 us   |       1.63 us   |       N/A       |       N/A       |     0.6703 us   |
| transform2 inverse             |   __9.0256 ns__ |       N/A       |    12.2614 ns   |    9.4803 ns   |       N/A       |   __8.9047 ns__ |       N/A       |       N/A       |
| transform2 mul transform2      |     4.5111 ns   |       N/A       |     8.1434 ns   |    5.8677 ns   |       N/A       |   __3.8513 ns__ |       N/A       |       N/A       |
| transform2 return self         |   __4.1707 ns__ |       N/A       |     5.4356 ns   |    4.2775 ns   |       N/A       |   __4.1117 ns__ |       N/A       |       N/A       |
| transform3 inverse             |  __10.9869 ns__ |       N/A       |    71.4437 ns   |   56.0136 ns   |       N/A       |    23.0392 ns   |       N/A       |       N/A       |
| transform3 mul transform3d     |   __6.5903 ns__ |       N/A       |     8.5673 ns   |   10.1802 ns   |       N/A       |     7.6587 ns   |       N/A       |       N/A       |
| transform3 return self         |   __7.1828 ns__ |       N/A       |   __7.2619 ns__ |  __7.2407 ns__ |       N/A       |   __7.3214 ns__ |       N/A       |       N/A       |
| vector3 cross                  |   __2.4257 ns__ |     3.6842 ns   |     3.7945 ns   |    3.6821 ns   |     3.8323 ns   |       N/A       |     3.8622 ns   |     3.6927 ns   |
| vector3 dot                    |   __2.1055 ns__ |     2.3179 ns   |     2.3174 ns   |    2.3190 ns   |     2.3195 ns   |       N/A       |     2.3204 ns   |     2.3160 ns   |
| vector3 length                 |   __2.5020 ns__ |   __2.5002 ns__ |     2.5986 ns   |  __2.5013 ns__ |   __2.5021 ns__ |       N/A       |   __2.5036 ns__ |   __2.5017 ns__ |
| vector3 normalize              |   __4.0454 ns__ |     5.8411 ns   |     8.4069 ns   |    8.0679 ns   |     8.8137 ns   |       N/A       |       N/A       |     5.8440 ns   |
| vector3 return self            |   __2.4087 ns__ |     3.1021 ns   |     3.1061 ns   |      N/A       |     3.1052 ns   |       N/A       |     3.1136 ns   |     3.1071 ns   |

### Wide benchmarks

These benchmarks were performed on an [Intel i7-4710HQ] CPU on Linux. They were
compiled with the `1.59.0-nightly (207c80f10 2021-11-30)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `glam` - `0.20.1`
* `nalgebra` - `0.29.0`
* `ultraviolet` - `0.8.1`

Run with the command:

```sh
RUSTFLAGS='-C target-feature=+avx2' cargo +nightly bench --features wide wide
```

| benchmark                      |    glam_f32x1   |   ultraviolet_f32x4   |   nalgebra_f32x4   |   ultraviolet_f32x8   |   nalgebra_f32x8   |
|--------------------------------|-----------------|-----------------------|--------------------|-----------------------|--------------------|
| euler 2d x80000                |      142.7 us   |          __63.47 us__ |       __63.94 us__ |            69.27 us   |         69.25 us   |
| euler 3d x80000                |      141.2 us   |          __97.18 us__ |       __95.78 us__ |            103.7 us   |         105.7 us   |
| matrix2 determinant x16        |    18.6849 ns   |          11.4259 ns   |          N/A       |         __9.9982 ns__ |          N/A       |
| matrix2 inverse x16            |    39.1219 ns   |          29.8933 ns   |          N/A       |        __22.8757 ns__ |          N/A       |
| matrix2 mul matrix2 x16        |    42.7342 ns   |          36.4879 ns   |          N/A       |        __33.4814 ns__ |          N/A       |
| matrix2 mul matrix2 x256       |   959.1663 ns   |         935.4148 ns   |          N/A       |       __862.0910 ns__ |          N/A       |
| matrix2 mul vector2 x16        |    41.2464 ns   |          18.2382 ns   |          N/A       |        __17.2550 ns__ |          N/A       |
| matrix2 mul vector2 x256       |   698.1177 ns   |       __544.5315 ns__ |          N/A       |       __540.9743 ns__ |          N/A       |
| matrix2 return self x16        |    32.7553 ns   |          29.5064 ns   |          N/A       |        __21.4492 ns__ |          N/A       |
| matrix2 transpose x16          |    32.3247 ns   |          46.4836 ns   |          N/A       |        __20.0852 ns__ |          N/A       |
| matrix3 determinant x16        |    53.2366 ns   |          25.0158 ns   |          N/A       |        __22.1503 ns__ |          N/A       |
| matrix3 inverse x16            |   275.9330 ns   |          78.3532 ns   |          N/A       |        __69.2627 ns__ |          N/A       |
| matrix3 mul matrix3 x16        |   239.6124 ns   |       __115.2934 ns__ |          N/A       |       __116.6237 ns__ |          N/A       |
| matrix3 mul matrix3 x256       |       3.26 us   |          __1.959 us__ |          N/A       |          __1.963 us__ |          N/A       |
| matrix3 mul vector3 x16        |    78.4972 ns   |        __40.4734 ns__ |          N/A       |          47.0164 ns   |          N/A       |
| matrix3 mul vector3 x256       |      1.293 us   |            __1.0 us__ |          N/A       |          __1.007 us__ |          N/A       |
| matrix3 return self x16        |   112.4312 ns   |          78.4870 ns   |          N/A       |        __67.3272 ns__ |          N/A       |
| matrix3 transpose x16          |   116.9654 ns   |         100.1097 ns   |          N/A       |        __67.4544 ns__ |          N/A       |
| matrix4 determinant x16        |    98.8388 ns   |        __56.1177 ns__ |          N/A       |        __55.7623 ns__ |          N/A       |
| matrix4 inverse x16            |   276.2637 ns   |         191.7471 ns   |          N/A       |       __163.8408 ns__ |          N/A       |
| matrix4 mul matrix4 x16        |   230.9916 ns   |       __222.3948 ns__ |          N/A       |       __221.8563 ns__ |          N/A       |
| matrix4 mul matrix4 x256       |      3.793 us   |          __3.545 us__ |          N/A       |             3.67 us   |          N/A       |
| matrix4 mul vector4 x16        |    92.9485 ns   |        __87.7341 ns__ |          N/A       |          90.4404 ns   |          N/A       |
| matrix4 mul vector4 x256       |     __1.58 us__ |          __1.542 us__ |          N/A       |            1.596 us   |          N/A       |
| matrix4 return self x16        |   175.6153 ns   |       __158.7861 ns__ |          N/A       |         167.6639 ns   |          N/A       |
| matrix4 transpose x16          |   184.0498 ns   |         193.5497 ns   |          N/A       |       __147.1365 ns__ |          N/A       |
| ray-sphere intersection x80000 |      567.9 us   |            154.8 us   |          N/A       |          __61.49 us__ |          N/A       |
| rotation3 inverse x16          |    32.7517 ns   |          32.8107 ns   |          N/A       |        __22.3662 ns__ |          N/A       |
| rotation3 mul rotation3 x16    |    58.9408 ns   |          38.6848 ns   |          N/A       |        __34.3223 ns__ |          N/A       |
| rotation3 mul vector3 x16      |   130.6707 ns   |          36.7861 ns   |          N/A       |        __26.1154 ns__ |          N/A       |
| rotation3 return self x16      |    32.4345 ns   |          32.5213 ns   |          N/A       |        __21.8325 ns__ |          N/A       |
| transform point2 x16           |    52.6534 ns   |        __31.4527 ns__ |          N/A       |          32.7317 ns   |          N/A       |
| transform point2 x256          |   888.5654 ns   |       __831.9341 ns__ |          N/A       |       __848.0397 ns__ |          N/A       |
| transform point3 x16           |    96.9017 ns   |        __81.6828 ns__ |          N/A       |        __82.8904 ns__ |          N/A       |
| transform point3 x256          |      1.567 us   |          __1.398 us__ |          N/A       |           __1.43 us__ |          N/A       |
| transform vector2 x16          |    43.7679 ns   |        __29.9349 ns__ |          N/A       |          31.8630 ns   |          N/A       |
| transform vector2 x256         |   858.5660 ns   |       __825.0261 ns__ |          N/A       |         851.7501 ns   |          N/A       |
| transform vector3 x16          |    96.5535 ns   |        __80.1612 ns__ |          N/A       |          85.0659 ns   |          N/A       |
| transform vector3 x256         |      1.557 us   |          __1.394 us__ |          N/A       |            1.438 us   |          N/A       |
| vector3 cross x16              |    42.1941 ns   |          26.6677 ns   |          N/A       |        __22.0924 ns__ |          N/A       |
| vector3 dot x16                |    29.1805 ns   |          12.7972 ns   |          N/A       |        __12.2872 ns__ |          N/A       |
| vector3 length x16             |    32.6014 ns   |           9.7692 ns   |          N/A       |         __9.4271 ns__ |          N/A       |
| vector3 normalize x16          |    65.8815 ns   |          24.1661 ns   |          N/A       |        __20.3579 ns__ |          N/A       |
| vector3 return self x16        |    32.0051 ns   |          42.9462 ns   |          N/A       |        __16.7808 ns__ |          N/A       |

[Intel i7-4710HQ]: https://ark.intel.com/content/www/us/en/ark/products/78930/intel-core-i7-4710hq-processor-6m-cache-up-to-3-50-ghz.html
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
| static-math         | 0.1.6   |       6.9 |      1.7 |    10 |
| ultraviolet         | 0.5.1   |       2.5 |      1.3 |     4 |
| vek                 | 0.12.0  |      34.4 |     10.1 |    16 |

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

[Code of Conduct]: https://www.rust-lang.org/en-US/conduct.html

## Support

If you are interested in contributing or have a request or suggestion
[create an issue] on github.

[create an issue]: https://github.com/bitshifter/mathbench-rs/issues
