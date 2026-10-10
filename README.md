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

* harness overhead - measures the per-type cost of the benchmarking harness
  itself. Useful as a control, not as a library performance comparison.
* single operations - measure the performance of common operations on types,
  e.g. a matrix inverse, vector normalization or multiplying two matrices.
* throughput operations - measure the performance of common operations on
  batches of data. These measure operations that would commonly be processing
  batches of input, for example transforming a number of vectors with the same
  matrix.
* workload operations - these attempt to recreate common workloads found in game
  development to try and demonstrate performance on real world tasks.

Operation benchmarks run a batch of independent operations per timed iteration
and report per-element throughput via Criterion's `Throughput::Elements`, at two
sizes by default: 16 (L1-resident) and 1024 (adds cache pressure). This keeps
the fixed per-iteration harness cost from dominating short operations, so
results are directly comparable across libraries and across scalar and wide
types. A single input pool is generated per benchmark and reused across sizes.

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

Note that the `nalgebra` wide benchmarks use `simba`'s stable, `wide` crate
backed types (`WideF32x4`, `WideF32x8` and `WideF64x4`), so they build on
stable Rust.

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
compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `cgmath` - `0.18.0`
* `euclid` - `0.22.14`
* `glam` - `0.34.1`
* `nalgebra` - `0.35.0`
* `pathfinder_geometry` - `0.5.1`
* `ultraviolet` - `0.10.0`
* `vek` - `0.17.2`

### Scalar benchmarks

Run with the command:

```sh
cargo bench --features scalar scalar
```

| benchmark                      |          glam   |        cgmath   |      nalgebra   |        euclid   |           vek   |    pathfinder   |   ultraviolet   |
|--------------------------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|-----------------|
| euler 2d x10000                |    __5.163 us__ |    __5.116 us__ |      7.093 us   |    __5.124 us__ |    __5.158 us__ |      5.894 us   |    __5.121 us__ |
| euler 3d x10000                |    __5.864 us__ |      16.56 us   |       16.5 us   |      16.48 us   |      16.46 us   |    __5.852 us__ |       16.6 us   |
| matrix2 determinant            |   __1.0262 ns__ |     1.1390 ns   |     1.1387 ns   |       N/A       |     1.1361 ns   |     1.1469 ns   |     1.1384 ns   |
| matrix2 inverse                |   __1.4776 ns__ |     2.0499 ns   |     2.0399 ns   |       N/A       |       N/A       |     1.6476 ns   |     1.7388 ns   |
| matrix2 mul matrix2            |     1.1531 ns   |   __1.1030 ns__ |     1.2088 ns   |       N/A       |     8.8631 ns   |     1.1403 ns   |   __1.1052 ns__ |
| matrix2 mul vector2 x1         |   __0.9283 ns__ |     1.1561 ns   |     1.1906 ns   |       N/A       |     4.6570 ns   |     0.9584 ns   |     1.1698 ns   |
| matrix2 mul vector2 x100       | __171.1991 ns__ |   179.7413 ns   |   178.3703 ns   |       N/A       |   463.2815 ns   | __171.7605 ns__ |   178.8623 ns   |
| matrix2 return self            |   __1.1184 ns__ |     1.6710 ns   |     1.6165 ns   |       N/A       |     1.7853 ns   |     1.6190 ns   |     1.7241 ns   |
| matrix2 transpose              |     1.2241 ns   |     1.3370 ns   |     1.3974 ns   |       N/A       |     0.9063 ns   |       N/A       |   __0.8504 ns__ |
| matrix3 determinant            |   __2.1892 ns__ |     2.2352 ns   |   __2.1973 ns__ |       N/A       |   __2.1650 ns__ |       N/A       |    10.4218 ns   |
| matrix3 inverse                |   __5.3608 ns__ |     9.0642 ns   |     5.6807 ns   |       N/A       |       N/A       |       N/A       |    12.2288 ns   |
| matrix3 mul matrix3            |     4.6602 ns   |     5.6509 ns   |   __4.4501 ns__ |       N/A       |    36.7220 ns   |       N/A       |   __4.5589 ns__ |
| matrix3 mul vector3 x1         |   __2.1895 ns__ |     3.4878 ns   |   __2.1900 ns__ |       N/A       |    12.7325 ns   |       N/A       |     2.3641 ns   |
| matrix3 mul vector3 x100       |   __0.3649 us__ |     0.4132 us   |   __0.3661 us__ |       N/A       |      1.276 us   |       N/A       |   __0.3659 us__ |
| matrix3 return self            |     2.9568 ns   |   __2.9042 ns__ |   __2.8369 ns__ |       N/A       |   __2.8971 ns__ |       N/A       |   __2.9001 ns__ |
| matrix3 transpose              |     2.1894 ns   |     2.1891 ns   |   __1.9596 ns__ |       N/A       |     2.1792 ns   |       N/A       |   __1.9453 ns__ |
| matrix4 determinant            |   __4.3934 ns__ |     7.1764 ns   |    40.9609 ns   |    11.2154 ns   |    12.2779 ns   |       N/A       |     4.6378 ns   |
| matrix4 inverse                |  __12.1455 ns__ |    30.8124 ns   |    40.8410 ns   |    54.4792 ns   |    27.3142 ns   |       N/A       |    20.0627 ns   |
| matrix4 mul matrix4            |   __4.4589 ns__ |     6.2970 ns   |     4.7309 ns   |     6.4629 ns   |   103.4461 ns   |       N/A       |     5.0440 ns   |
| matrix4 mul vector4 x1         |     2.1002 ns   |   __1.9625 ns__ |   __1.9185 ns__ |       N/A       |    24.5601 ns   |       N/A       |     2.2970 ns   |
| matrix4 mul vector4 x100       |   __0.5092 us__ |     0.5362 us   |     0.5379 us   |       N/A       |      2.491 us   |       N/A       |     0.5361 us   |
| matrix4 return self            |   __3.3286 ns__ |   __3.3602 ns__ |   __3.3612 ns__ |       N/A       |   __3.3757 ns__ |       N/A       |   __3.3850 ns__ |
| matrix4 transpose              |   __2.7248 ns__ |     7.6718 ns   |     7.7156 ns   |       N/A       |     7.6853 ns   |       N/A       |     7.6577 ns   |
| ray-sphere intersection x10000 |    __13.37 us__ |     __13.3 us__ |    __13.37 us__ |    __13.31 us__ |    __13.37 us__ |       N/A       |     __13.3 us__ |
| rotation3 inverse              |   __1.6893 ns__ |     2.5085 ns   |     2.5737 ns   |     2.5909 ns   |     2.4670 ns   |       N/A       |     2.5312 ns   |
| rotation3 mul rotation3        |   __1.8869 ns__ |     2.6284 ns   |     2.4297 ns   |     3.4803 ns   |     3.0026 ns   |       N/A       |     3.2824 ns   |
| rotation3 mul vector3 x1       |     3.1971 ns   |   __2.9104 ns__ |     3.3983 ns   |     3.1297 ns   |     5.1255 ns   |       N/A       |     4.3788 ns   |
| rotation3 mul vector3 x100     |   347.7257 ns   | __307.8286 ns__ |   349.4552 ns   |   346.2632 ns   |   506.3045 ns   |       N/A       |   431.1647 ns   |
| rotation3 return self          |   __1.4740 ns__ |     1.7942 ns   |     1.8652 ns   |       N/A       |     1.7913 ns   |       N/A       |     1.8121 ns   |
| transform point2 x1            |   __1.0489 ns__ |     2.0296 ns   |     2.4919 ns   |     1.3207 ns   |     4.4160 ns   |     1.0863 ns   |     1.8828 ns   |
| transform point2 x100          |   268.6793 ns   |   335.3824 ns   |   357.9078 ns   | __233.8814 ns__ |   501.7938 ns   |   268.9675 ns   |   331.0402 ns   |
| transform point3 x1            |   __1.9061 ns__ |    15.0678 ns   |     4.0615 ns   |     3.3012 ns   |    12.3817 ns   |     2.0046 ns   |     3.5482 ns   |
| transform point3 x100          |   __0.4933 us__ |      1.552 us   |     0.5895 us   |     0.5714 us   |      1.301 us   |   __0.5015 us__ |     0.5653 us   |
| transform vector2 x1           |   __0.9408 ns__ |       N/A       |     2.0241 ns   |     1.2064 ns   |     8.6210 ns   |       N/A       |     1.3573 ns   |
| transform vector2 x100         |   259.8743 ns   |       N/A       |   330.1703 ns   | __230.7005 ns__ |   862.0694 ns   |       N/A       |   306.3204 ns   |
| transform vector3 x1           |   __1.9841 ns__ |    11.6743 ns   |     2.7202 ns   |     2.2886 ns   |    18.1540 ns   |       N/A       |     2.7037 ns   |
| transform vector3 x100         |   __0.4938 us__ |       1.19 us   |     0.5509 us   |     0.5109 us   |      1.906 us   |       N/A       |      0.522 us   |
| transform2 inverse             |   __2.2689 ns__ |       N/A       |     5.6429 ns   |     2.9631 ns   |       N/A       |     2.5809 ns   |       N/A       |
| transform2 mul transform2      |     2.1811 ns   |       N/A       |     4.3640 ns   |   __1.9851 ns__ |       N/A       |     2.1966 ns   |       N/A       |
| transform2 return self         |   __2.2848 ns__ |       N/A       |     2.9137 ns   |     2.4864 ns   |       N/A       |   __2.2808 ns__ |       N/A       |
| transform3 inverse             |   __6.6932 ns__ |       N/A       |    40.6221 ns   |    38.0707 ns   |       N/A       |    20.7270 ns   |       N/A       |
| transform3 mul transform3d     |   __3.5545 ns__ |       N/A       |     4.8629 ns   |     6.3285 ns   |       N/A       |     4.4894 ns   |       N/A       |
| transform3 return self         |   __3.3643 ns__ |       N/A       |   __3.3640 ns__ |   __3.3515 ns__ |       N/A       |   __3.3487 ns__ |       N/A       |
| vector3 cross                  |   __1.0511 ns__ |     1.5374 ns   |     1.5364 ns   |     1.5369 ns   |     1.5453 ns   |       N/A       |     1.5370 ns   |
| vector3 dot                    |   __0.9757 ns__ |     1.2293 ns   |     1.2324 ns   |     1.2385 ns   |     1.2313 ns   |       N/A       |     1.2214 ns   |
| vector3 length                 |   __1.3147 ns__ |   __1.3128 ns__ |   __1.3205 ns__ |   __1.3079 ns__ |   __1.3108 ns__ |       N/A       |   __1.3091 ns__ |
| vector3 normalize              |   __2.1971 ns__ |     2.5868 ns   |     3.0146 ns   |     3.0183 ns   |     2.9928 ns   |       N/A       |     2.6405 ns   |
| vector3 return self            |   __1.0859 ns__ |     1.9663 ns   |     1.9419 ns   |       N/A       |     1.9943 ns   |       N/A       |     2.1768 ns   |

### Wide benchmarks

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux. They were
compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `glam` - `0.34.1`
* `nalgebra` - `0.35.0`
* `ultraviolet` - `0.10.0`

Run with the command:

```sh
RUSTFLAGS='-C target-feature=+avx2' cargo bench --features wide wide
```

| benchmark                      |    glam_f32x1   |   ultraviolet_f32x4   |   nalgebra_f32x4   |   ultraviolet_f32x8   |   nalgebra_f32x8   |
|--------------------------------|-----------------|-----------------------|--------------------|-----------------------|--------------------|
| euler 2d x80000                |      41.87 us   |            25.65 us   |         25.59 us   |          __18.94 us__ |       __18.57 us__ |
| euler 3d x80000                |      58.97 us   |            40.25 us   |         39.78 us   |          __28.54 us__ |       __28.73 us__ |
| matrix2 determinant x16        |    12.3683 ns   |           5.0255 ns   |          N/A       |         __3.6438 ns__ |          N/A       |
| matrix2 inverse x16            |    18.2726 ns   |          10.2942 ns   |          N/A       |         __7.4352 ns__ |          N/A       |
| matrix2 mul matrix2 x16        |    16.4108 ns   |          10.9767 ns   |       11.0702 ns   |         __9.2553 ns__ |      __9.3122 ns__ |
| matrix2 mul matrix2 x256       | __688.8713 ns__ |         844.2290 ns   |      851.0381 ns   |       __678.6878 ns__ |    __675.6790 ns__ |
| matrix2 mul vector2 x16        |    12.8488 ns   |           7.3729 ns   |        7.4172 ns   |         __5.9231 ns__ |        6.0974 ns   |
| matrix2 mul vector2 x256       |   482.0150 ns   |       __423.7576 ns__ |    __423.9817 ns__ |       __425.9341 ns__ |    __426.8713 ns__ |
| matrix2 return self x16        |     9.8083 ns   |           9.8692 ns   |        9.9349 ns   |         __7.2746 ns__ |      __7.3264 ns__ |
| matrix2 transpose x16          |     9.8868 ns   |          40.2972 ns   |       34.4561 ns   |          11.6302 ns   |      __9.0992 ns__ |
| matrix3 determinant x16        |    28.6072 ns   |          10.9073 ns   |          N/A       |         __7.8069 ns__ |          N/A       |
| matrix3 inverse x16            |    96.2555 ns   |          38.6667 ns   |          N/A       |        __22.0202 ns__ |          N/A       |
| matrix3 mul matrix3 x16        |    81.2391 ns   |          41.5798 ns   |       42.1062 ns   |        __33.2499 ns__ |       35.2442 ns   |
| matrix3 mul matrix3 x256       |      1.619 us   |            1.615 us   |         1.656 us   |          __1.551 us__ |       __1.582 us__ |
| matrix3 mul vector3 x16        |    34.1728 ns   |          13.8191 ns   |       13.7320 ns   |         __9.7462 ns__ |       10.6091 ns   |
| matrix3 mul vector3 x256       |   954.4842 ns   |       __789.3333 ns__ |    __792.1095 ns__ |       __779.5657 ns__ |    __776.0663 ns__ |
| matrix3 return self x16        |    43.1504 ns   |          35.2027 ns   |       35.3488 ns   |        __27.8165 ns__ |     __27.4926 ns__ |
| matrix3 transpose x16          |    52.7683 ns   |          83.7157 ns   |       62.4901 ns   |          35.9252 ns   |     __31.9769 ns__ |
| matrix4 determinant x16        |    62.0342 ns   |          20.9124 ns   |          N/A       |        __14.6545 ns__ |          N/A       |
| matrix4 inverse x16            |   198.0560 ns   |       __104.5414 ns__ |          N/A       |         120.0865 ns   |          N/A       |
| matrix4 mul matrix4 x16        | __163.5378 ns__ |         207.9730 ns   |      198.9647 ns   |       __167.0614 ns__ |      204.1703 ns   |
| matrix4 mul matrix4 x256       |      3.414 us   |            2.978 us   |         3.131 us   |          __2.892 us__ |         3.857 us   |
| matrix4 mul vector4 x16        |    34.5807 ns   |          29.2073 ns   |       29.1650 ns   |          22.6779 ns   |     __21.6758 ns__ |
| matrix4 mul vector4 x256       |      1.312 us   |          __1.257 us__ |       __1.261 us__ |          __1.242 us__ |       __1.234 us__ |
| matrix4 return self x16        |  __73.6516 ns__ |        __73.3864 ns__ |       76.9283 ns   |         153.5618 ns   |      149.6711 ns   |
| matrix4 transpose x16          |  __73.5089 ns__ |          93.7827 ns   |       89.3917 ns   |          96.3356 ns   |       87.0057 ns   |
| ray-sphere intersection x80000 |      497.6 us   |            49.65 us   |         50.09 us   |          __25.26 us__ |       __25.42 us__ |
| rotation3 inverse x16          |     9.8997 ns   |           9.4487 ns   |        9.4114 ns   |           7.3071 ns   |      __6.9988 ns__ |
| rotation3 mul rotation3 x16    |    26.5663 ns   |          13.1697 ns   |       13.1350 ns   |         __9.4821 ns__ |      __9.7145 ns__ |
| rotation3 mul vector3 x16      |    53.2221 ns   |          17.5765 ns   |       14.6217 ns   |           9.5778 ns   |      __8.5011 ns__ |
| rotation3 return self x16      |     9.7975 ns   |          10.6415 ns   |       10.5633 ns   |         __7.4200 ns__ |      __7.4645 ns__ |
| transform point2 x16           |    15.6934 ns   |          12.4223 ns   |          N/A       |         __9.3164 ns__ |          N/A       |
| transform point2 x256          | __691.7836 ns__ |       __676.6860 ns__ |          N/A       |       __683.5775 ns__ |          N/A       |
| transform point3 x16           |    31.7974 ns   |          24.2547 ns   |          N/A       |        __18.1546 ns__ |          N/A       |
| transform point3 x256          |      1.278 us   |          __1.144 us__ |          N/A       |          __1.131 us__ |          N/A       |
| transform vector2 x16          |    13.4952 ns   |          10.1578 ns   |          N/A       |         __9.0737 ns__ |          N/A       |
| transform vector2 x256         |   669.8862 ns   |       __649.1890 ns__ |          N/A       |         690.8455 ns   |          N/A       |
| transform vector3 x16          |    30.0967 ns   |          21.6747 ns   |          N/A       |        __18.1976 ns__ |          N/A       |
| transform vector3 x256         |      1.267 us   |          __1.107 us__ |          N/A       |            1.174 us   |          N/A       |
| vector3 cross x16              |    15.2375 ns   |           8.9911 ns   |        8.8311 ns   |         __7.6224 ns__ |      __7.4720 ns__ |
| vector3 dot x16                |    12.9763 ns   |           6.5773 ns   |        6.5842 ns   |           4.7245 ns   |      __4.5946 ns__ |
| vector3 length x16             |    20.5391 ns   |           5.5195 ns   |        5.5261 ns   |         __3.9270 ns__ |      __3.8417 ns__ |
| vector3 normalize x16          |    35.2002 ns   |          10.3503 ns   |       15.2167 ns   |         __7.3365 ns__ |        8.5396 ns   |
| vector3 return self x16        |     9.8028 ns   |          25.4662 ns   |       25.4119 ns   |         __5.6623 ns__ |      __5.6618 ns__ |

[AMD Ryzen 7 3800X]: https://www.amd.com/en/products/cpu/amd-ryzen-7-3800x

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
* `wide` - enables all "wide" type benchmarks, these build on stable Rust.
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
