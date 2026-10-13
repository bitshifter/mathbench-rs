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

* no-op baseline - measures the per-type cost of running an identity operation
  through the benchmark loop (load + store plus amortized loop overhead).
  Useful as a floor and a control, not as a library performance comparison.
* single operations - measure the performance of common operations on types,
  e.g. a matrix inverse, vector normalization or multiplying two matrices.
* throughput operations - measure the performance of common operations on
  batches of data. These measure operations that would commonly be processing
  batches of input, for example transforming a number of vectors with the same
  matrix.
* workload operations - these attempt to recreate common workloads found in game
  development to try and demonstrate performance on real world tasks.

Operation benchmarks run a batch of operations per timed iteration and report
per-element throughput via Criterion's `Throughput::Elements`, at two sizes by
default: 16 (L1-resident) and 1024 (adds cache pressure). Batching amortizes the
fixed per-iteration harness cost, so results are directly comparable across
libraries and across scalar and wide types. The 16 point still carries a little
of that fixed overhead (more so for wider types, which need fewer operations per
batch); 1024 is close to pure throughput. Compare libraries at the same size. A
single input pool is generated per benchmark and reused across sizes. The
matrix/quaternion "multiply vector" and transform benches broadcast one
transform across a batch of vectors or points, matching the throughput workload
described above. For wide types "one transform" is a single packed value (e.g.
the four matrices in a `Mat4x4`) reused for the whole batch.

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

### Type representation and ABI

The scalar `glam` benchmarks use the SIMD-friendly padded types where they
exist (`Vec3A`, `Mat3A`, `Affine3A`), while other libraries use unpacked types
such as a 12-byte `Vec3` or a 36-byte `Mat3`. The padded types carry an extra
lane and 16-byte alignment, so a value is loaded, stored and operated on as a
single 128-bit SIMD register, whereas an unpacked value takes several narrower
moves. This shows up in the `no-op baseline`, which measures how a type is
moved through the benchmark loop, so differences there reflect type
representation rather than library efficiency. The same applies to `euclid`
transform types and to the `Option`-returning inverse methods noted above.

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
backed types (`WideF32x4` and `WideF32x8`), so they build on stable Rust.

Additionally the `f32x8` benchmarks will require the `AVX2` instruction set, to
enable that you will need to build with `RUSTFLAGS='-C target-feature=+avx2'`
(or `RUSTFLAGS='-C target-cpu=native'`).

## Build settings

The default `profile.bench` settings are used, these are documented in the
[cargo reference].

Some math libraries are optimized to use specific instruction sets and may
benefit building with settings different to the defaults. Typically a game team
will need to decided on a minimum specification that they will target. Deciding
on a minimum specifiction dictates the potential audience size for a project.
This is an important decision for any game and it will be different for every
project. `mathbench` doesn't want to make assumptions about what build settings
any particular project may want to use which is why the `profile.bench` settings
are left at their defaults. The results published here add
`-C target-cpu=native` on top of those defaults; the exact command used is shown
with each table.

I would encourage users who to use build settigs different to the defaults to
run the benchmarks themselves and consider publishing their results.

[cargo reference]: https://doc.rust-lang.org/cargo/reference/profiles.html#bench

## Benchmark results

The following is a table of benchmarks produced by `mathbench` comparing `glam`
performance to `cgmath`, `nalgebra`, `euclid`, `vek`, `pathfinder_geometry`,
and `ultraviolet` on `f32` data.

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux. They were
compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler with
`RUSTFLAGS='-C target-cpu=native'`. Lower
(better) numbers are highlighted when the library is within `-t` percent
(default 2.5) of the fastest in the row and the one-sided 95% interval for the
difference rules out a slowdown larger than that margin, i.e. when the library
is practically equivalent to the best.

The versions of the libraries tested were:

* `cgmath` - `0.18.0`
* `euclid` - `0.22.14`
* `glam` - `0.34.1`
* `nalgebra` - `0.35.0`
* `pathfinder_geometry` - `0.5.1`
* `ultraviolet` - `0.10.0`
* `vek` - `0.17.2`

See the full [mathbench report] for more detailed results.

### Scalar benchmarks

Run with the command:

```sh
RUSTFLAGS='-C target-cpu=native' cargo bench --features scalar scalar
```

| benchmark                        |         glam   |       cgmath   |     nalgebra   |       euclid   |          vek   |   pathfinder   |   ultraviolet   |
|----------------------------------|----------------|----------------|----------------|----------------|----------------|----------------|-----------------|
| euler 2d x10000                  |  __0.4993 ns__ |  __0.5047 ns__ |    0.5977 ns   |  __0.4994 ns__ |  __0.5008 ns__ |    0.5959 ns   |   __0.5009 ns__ |
| euler 3d x10000                  |  __0.5815 ns__ |    1.4042 ns   |    1.4056 ns   |    1.4009 ns   |    1.4034 ns   |  __0.5858 ns__ |     1.3889 ns   |
| matrix2 determinant x1024        |  __0.4799 ns__ |    0.7045 ns   |    0.7149 ns   |      N/A       |    0.7145 ns   |    0.7129 ns   |     0.7101 ns   |
| matrix2 determinant x16          |  __0.5018 ns__ |    0.7091 ns   |    0.7186 ns   |      N/A       |    0.7118 ns   |    0.7118 ns   |     0.7121 ns   |
| matrix2 inverse x1024            |  __0.9549 ns__ |    1.2330 ns   |    1.2120 ns   |      N/A       |      N/A       |    1.0188 ns   |     1.9215 ns   |
| matrix2 inverse x16              |  __0.9061 ns__ |    1.2145 ns   |    1.2080 ns   |      N/A       |      N/A       |    1.0010 ns   |     1.9143 ns   |
| matrix2 mul matrix2 x1024        |  __0.6627 ns__ |    2.2528 ns   |    2.2675 ns   |      N/A       |    2.1962 ns   |    0.7281 ns   |     2.2407 ns   |
| matrix2 mul matrix2 x16          |  __0.6547 ns__ |    2.2541 ns   |    2.2568 ns   |      N/A       |    2.1825 ns   |    0.7263 ns   |     2.2290 ns   |
| matrix2 mul vector2 x1024        |    0.4906 ns   |    0.2369 ns   |    0.2360 ns   |      N/A       |  __0.2196 ns__ |    0.4208 ns   |     0.2396 ns   |
| matrix2 mul vector2 x16          |    0.5076 ns   |    0.2302 ns   |    0.2681 ns   |      N/A       |  __0.2210 ns__ |    0.4172 ns   |     0.2290 ns   |
| matrix2 no-op baseline x1024     |    0.3672 ns   |    0.3697 ns   |  __0.2609 ns__ |      N/A       |  __0.2592 ns__ |  __0.2611 ns__ |     0.3600 ns   |
| matrix2 no-op baseline x16       |    0.2856 ns   |    0.2829 ns   |  __0.2790 ns__ |      N/A       |  __0.2752 ns__ |    0.2841 ns   |     0.2801 ns   |
| matrix2 transpose x1024          |    0.3712 ns   |  __0.1780 ns__ |  __0.1770 ns__ |      N/A       |  __0.1777 ns__ |      N/A       |   __0.1772 ns__ |
| matrix2 transpose x16            |    0.2749 ns   |  __0.1616 ns__ |  __0.1613 ns__ |      N/A       |  __0.1620 ns__ |      N/A       |     0.2604 ns   |
| matrix3 determinant x1024        |    1.1539 ns   |  __1.0718 ns__ |  __1.0894 ns__ |      N/A       |  __1.0846 ns__ |      N/A       |   __1.0755 ns__ |
| matrix3 determinant x16          |    1.1450 ns   |  __1.0644 ns__ |    1.0999 ns   |      N/A       |    1.1057 ns   |      N/A       |   __1.0620 ns__ |
| matrix3 inverse x1024            |    4.1165 ns   |    8.5685 ns   |    4.8496 ns   |      N/A       |      N/A       |      N/A       |   __3.1408 ns__ |
| matrix3 inverse x16              |    4.1170 ns   |    8.6799 ns   |    4.7974 ns   |      N/A       |      N/A       |      N/A       |   __3.1086 ns__ |
| matrix3 mul matrix3 x1024        |  __2.1312 ns__ |    5.4942 ns   |    3.1003 ns   |      N/A       |    3.3554 ns   |      N/A       |     3.3554 ns   |
| matrix3 mul matrix3 x16          |  __2.0814 ns__ |    5.7367 ns   |    3.1068 ns   |      N/A       |    3.2544 ns   |      N/A       |     3.3398 ns   |
| matrix3 mul vector3 x1024        |  __0.7485 ns__ |    2.6675 ns   |    0.8289 ns   |      N/A       |    0.7689 ns   |      N/A       |     0.8302 ns   |
| matrix3 mul vector3 x16          |  __0.7300 ns__ |    2.4659 ns   |    0.8215 ns   |      N/A       |    0.7713 ns   |      N/A       |     0.8329 ns   |
| matrix3 no-op baseline x1024     |    0.8868 ns   |    0.8212 ns   |  __0.7389 ns__ |      N/A       |  __0.7373 ns__ |      N/A       |   __0.7386 ns__ |
| matrix3 no-op baseline x16       |    0.7276 ns   |  __0.6947 ns__ |  __0.6945 ns__ |      N/A       |  __0.6946 ns__ |      N/A       |   __0.6943 ns__ |
| matrix3 transpose x1024          |    0.9584 ns   |  __0.8514 ns__ |    0.9389 ns   |      N/A       |    0.9467 ns   |      N/A       |     0.9410 ns   |
| matrix3 transpose x16            |    0.9451 ns   |  __0.7178 ns__ |    0.9404 ns   |      N/A       |    0.9388 ns   |      N/A       |     0.9382 ns   |
| matrix4 determinant x1024        |    3.4266 ns   |  __2.3661 ns__ |   40.7346 ns   |    9.2307 ns   |   10.2196 ns   |      N/A       |   __2.3333 ns__ |
| matrix4 determinant x16          |    3.4243 ns   |  __2.3779 ns__ |   38.6588 ns   |    8.9597 ns   |   10.2938 ns   |      N/A       |   __2.3292 ns__ |
| matrix4 inverse x1024            |   10.9287 ns   |   25.2419 ns   |   26.7860 ns   |   31.8244 ns   |   22.2220 ns   |      N/A       |   __6.1286 ns__ |
| matrix4 inverse x16              |   11.1835 ns   |   25.1730 ns   |   26.5568 ns   |   31.6973 ns   |   22.4290 ns   |      N/A       |   __6.0777 ns__ |
| matrix4 mul matrix4 x1024        |  __3.2445 ns__ |    6.2365 ns   |    6.3150 ns   |    6.1518 ns   |    4.9908 ns   |      N/A       |     6.6484 ns   |
| matrix4 mul matrix4 x16          |  __3.1375 ns__ |    6.1132 ns   |    6.2880 ns   |    6.1607 ns   |    4.9355 ns   |      N/A       |     6.6406 ns   |
| matrix4 mul vector4 x1024        |  __0.8177 ns__ |    1.7610 ns   |    1.7164 ns   |      N/A       |    1.6457 ns   |      N/A       |     1.7689 ns   |
| matrix4 mul vector4 x16          |  __0.8160 ns__ |    1.7681 ns   |    1.6911 ns   |      N/A       |    1.6540 ns   |      N/A       |     1.7641 ns   |
| matrix4 no-op baseline x1024     |    1.1265 ns   |  __1.0486 ns__ |  __1.0543 ns__ |      N/A       |  __1.0355 ns__ |      N/A       |   __1.0486 ns__ |
| matrix4 no-op baseline x16       |    0.9516 ns   |    0.9497 ns   |  __0.5030 ns__ |      N/A       |    0.9550 ns   |      N/A       |     0.9497 ns   |
| matrix4 transpose x1024          |  __1.4243 ns__ |    1.9234 ns   |    1.9390 ns   |      N/A       |    1.9388 ns   |      N/A       |     1.9410 ns   |
| matrix4 transpose x16            |  __1.4100 ns__ |    1.8846 ns   |    1.8967 ns   |      N/A       |    1.8995 ns   |      N/A       |     1.8857 ns   |
| ray-sphere intersection x10000   |    8.0682 ns   |   16.3118 ns   |   17.5249 ns   |   16.3678 ns   |   17.4790 ns   |      N/A       |   __5.7654 ns__ |
| rotation3 inverse x1024          |    0.3725 ns   |  __0.1663 ns__ |  __0.1665 ns__ |  __0.1663 ns__ |  __0.1668 ns__ |      N/A       |   __0.1654 ns__ |
| rotation3 inverse x16            |    0.2605 ns   |  __0.1694 ns__ |  __0.1691 ns__ |    0.2617 ns   |  __0.1678 ns__ |      N/A       |   __0.1692 ns__ |
| rotation3 mul rotation3 x1024    |  __1.2935 ns__ |    2.5332 ns   |    2.5375 ns   |    2.4943 ns   |    2.4460 ns   |      N/A       |     2.4295 ns   |
| rotation3 mul rotation3 x16      |  __1.1628 ns__ |    2.5452 ns   |    2.5393 ns   |    2.4759 ns   |    2.4740 ns   |      N/A       |     2.4166 ns   |
| rotation3 mul vector3 x1024      |    1.6042 ns   |    1.2165 ns   |    1.1947 ns   |  __1.1409 ns__ |    1.2763 ns   |      N/A       |     1.1699 ns   |
| rotation3 mul vector3 x16        |    1.5737 ns   |    1.2135 ns   |    1.1702 ns   |  __1.1390 ns__ |    1.2643 ns   |      N/A       |   __1.1539 ns__ |
| rotation3 no-op baseline x1024   |  __0.3687 ns__ |    0.4945 ns   |    0.4955 ns   |      N/A       |    0.4753 ns   |      N/A       |     0.4950 ns   |
| rotation3 no-op baseline x16     |  __0.2739 ns__ |    0.5263 ns   |    0.5284 ns   |      N/A       |    0.5255 ns   |      N/A       |     0.5297 ns   |
| transform point2 x1024           |    0.4793 ns   |    1.2990 ns   |    0.7113 ns   |    0.2574 ns   |  __0.2261 ns__ |    0.4789 ns   |     0.4079 ns   |
| transform point2 x16             |    0.5054 ns   |    1.3714 ns   |    0.7081 ns   |    0.2522 ns   |  __0.2236 ns__ |    0.5005 ns   |     0.4100 ns   |
| transform point3 x1024           |  __0.8019 ns__ |    3.0887 ns   |    1.0391 ns   |    2.3868 ns   |  __0.8077 ns__ |  __0.8171 ns__ |     1.0302 ns   |
| transform point3 x16             |  __0.7748 ns__ |    2.8913 ns   |    1.0344 ns   |    2.3855 ns   |    0.8045 ns   |    0.8165 ns   |     1.0375 ns   |
| transform vector2 x1024          |    0.4203 ns   |      N/A       |    0.8512 ns   |  __0.2288 ns__ |    0.2715 ns   |      N/A       |     0.2513 ns   |
| transform vector2 x16            |    0.4173 ns   |      N/A       |    0.8517 ns   |  __0.2317 ns__ |    0.2690 ns   |      N/A       |     0.2516 ns   |
| transform vector3 x1024          |  __0.7452 ns__ |    2.5915 ns   |    1.4770 ns   |    0.8279 ns   |    0.8165 ns   |      N/A       |     0.8785 ns   |
| transform vector3 x16            |  __0.7224 ns__ |    2.5385 ns   |    1.4683 ns   |    0.8301 ns   |    0.8238 ns   |      N/A       |     0.8854 ns   |
| transform2 inverse x1024         |  __1.8010 ns__ |      N/A       |    4.8340 ns   |    1.9033 ns   |      N/A       |    1.9623 ns   |       N/A       |
| transform2 inverse x16           |  __1.7954 ns__ |      N/A       |    4.7348 ns   |    1.8994 ns   |      N/A       |    1.9412 ns   |       N/A       |
| transform2 mul transform2 x1024  |  __1.1528 ns__ |      N/A       |    3.0817 ns   |    3.2475 ns   |      N/A       |  __1.1720 ns__ |       N/A       |
| transform2 mul transform2 x16    |  __1.1318 ns__ |      N/A       |    3.1102 ns   |    3.2381 ns   |      N/A       |  __1.1316 ns__ |       N/A       |
| transform2 no-op baseline x1024  |    0.9335 ns   |      N/A       |    0.7345 ns   |    0.5776 ns   |      N/A       |  __0.5565 ns__ |       N/A       |
| transform2 no-op baseline x16    |  __0.4958 ns__ |      N/A       |    0.6983 ns   |    0.5540 ns   |      N/A       |    0.5066 ns   |       N/A       |
| transform3 inverse x1024         |  __5.8707 ns__ |      N/A       |   26.5732 ns   |   36.8259 ns   |      N/A       |   15.3330 ns   |       N/A       |
| transform3 inverse x16           |  __5.8248 ns__ |      N/A       |   26.8705 ns   |   36.5833 ns   |      N/A       |   15.3115 ns   |       N/A       |
| transform3 mul transform3d x1024 |  __2.8067 ns__ |      N/A       |    6.3013 ns   |    6.2758 ns   |      N/A       |    3.1992 ns   |       N/A       |
| transform3 mul transform3d x16   |  __2.7414 ns__ |      N/A       |    6.2623 ns   |    6.1434 ns   |      N/A       |    3.1437 ns   |       N/A       |
| transform3 no-op baseline x1024  |    1.1351 ns   |      N/A       |    1.0115 ns   |    1.3669 ns   |      N/A       |  __0.9612 ns__ |       N/A       |
| transform3 no-op baseline x16    |    0.9554 ns   |      N/A       |  __0.5066 ns__ |  __0.5041 ns__ |      N/A       |  __0.4980 ns__ |       N/A       |
| vector3 cross x1024              |  __0.7430 ns__ |    0.9712 ns   |    0.9659 ns   |    0.9660 ns   |    0.9612 ns   |      N/A       |     0.9537 ns   |
| vector3 cross x16                |  __0.7205 ns__ |    0.9389 ns   |    0.9381 ns   |    0.9378 ns   |    0.9404 ns   |      N/A       |     0.9290 ns   |
| vector3 dot x1024                |    0.4997 ns   |  __0.4782 ns__ |  __0.4849 ns__ |  __0.4786 ns__ |  __0.4779 ns__ |      N/A       |   __0.4802 ns__ |
| vector3 dot x16                  |    0.5172 ns   |  __0.4850 ns__ |  __0.4877 ns__ |  __0.4842 ns__ |  __0.4887 ns__ |      N/A       |   __0.4890 ns__ |
| vector3 length x1024             |    1.2799 ns   |  __0.3100 ns__ |  __0.3107 ns__ |  __0.3133 ns__ |  __0.3096 ns__ |      N/A       |   __0.3135 ns__ |
| vector3 length x16               |    1.2801 ns   |  __0.3128 ns__ |  __0.3110 ns__ |  __0.3107 ns__ |  __0.3116 ns__ |      N/A       |     0.3200 ns   |
| vector3 no-op baseline x1024     |  __0.3560 ns__ |    0.5423 ns   |    0.5016 ns   |      N/A       |    0.5044 ns   |      N/A       |     0.5471 ns   |
| vector3 no-op baseline x16       |  __0.2864 ns__ |    0.5236 ns   |    0.5293 ns   |      N/A       |    0.5430 ns   |      N/A       |     0.5230 ns   |
| vector3 normalize x1024          |    2.1077 ns   |    1.0334 ns   |  __0.9517 ns__ |  __0.9462 ns__ |  __0.9626 ns__ |      N/A       |     1.0223 ns   |
| vector3 normalize x16            |    2.1279 ns   |    1.0323 ns   |  __0.9489 ns__ |  __0.9554 ns__ |  __0.9555 ns__ |      N/A       |     1.0332 ns   |

### Wide benchmarks

These benchmarks were performed on an [AMD Ryzen 7 3800X] CPU on Linux. They were
compiled with the `1.99.0 (b940084d7 2026-09-28)` Rust compiler with
`RUSTFLAGS='-C target-cpu=native'`. Lower
(better) numbers are highlighted when the library is within `-t` percent
(default 2.5) of the fastest in the row and the one-sided 95% interval for the
difference rules out a slowdown larger than that margin, i.e. when the library
is practically equivalent to the best.

The versions of the libraries tested were:

* `glam` - `0.34.1`
* `nalgebra` - `0.35.0`
* `ultraviolet` - `0.10.0`

Run with the command:

```sh
RUSTFLAGS='-C target-cpu=native' cargo bench --features wide wide
```

| benchmark                      |   glam_f32x1   |   ultraviolet_f32x4   |   nalgebra_f32x4   |   ultraviolet_f32x8   |   nalgebra_f32x8   |
|--------------------------------|----------------|-----------------------|--------------------|-----------------------|--------------------|
| euler 2d x80000                |    0.5061 ns   |           0.3225 ns   |        0.3215 ns   |         __0.2277 ns__ |        0.2371 ns   |
| euler 3d x80000                |    0.6870 ns   |           0.4902 ns   |        0.4904 ns   |         __0.3578 ns__ |      __0.3553 ns__ |
| matrix2 determinant x1024      |    0.4775 ns   |           0.1503 ns   |          N/A       |         __0.0784 ns__ |          N/A       |
| matrix2 determinant x16        |    0.5030 ns   |           0.1338 ns   |          N/A       |         __0.0750 ns__ |          N/A       |
| matrix2 inverse x1024          |    0.9681 ns   |           0.3466 ns   |          N/A       |         __0.2167 ns__ |          N/A       |
| matrix2 inverse x16            |    0.9065 ns   |           0.3281 ns   |          N/A       |         __0.1648 ns__ |          N/A       |
| matrix2 mul matrix2 x1024      |    0.6692 ns   |           0.4692 ns   |        0.5984 ns   |         __0.3992 ns__ |        0.4293 ns   |
| matrix2 mul matrix2 x16        |    0.6571 ns   |           0.3971 ns   |        0.6152 ns   |         __0.2073 ns__ |        0.2183 ns   |
| matrix2 mul vector2 x1024      |    0.4884 ns   |           0.1816 ns   |        0.1817 ns   |         __0.0955 ns__ |      __0.0943 ns__ |
| matrix2 mul vector2 x16        |    0.5095 ns   |           0.1921 ns   |        0.1915 ns   |         __0.1051 ns__ |      __0.1042 ns__ |
| matrix2 transpose x1024        |    0.3804 ns   |           1.9288 ns   |        1.7474 ns   |           0.4949 ns   |      __0.3598 ns__ |
| matrix2 transpose x16          |  __0.2762 ns__ |           2.0442 ns   |        1.8250 ns   |           0.4945 ns   |        0.3761 ns   |
| matrix3 determinant x1024      |    1.1506 ns   |           0.4810 ns   |          N/A       |         __0.3136 ns__ |          N/A       |
| matrix3 determinant x16        |    1.1441 ns   |           0.4300 ns   |          N/A       |         __0.2191 ns__ |          N/A       |
| matrix3 inverse x1024          |    4.1142 ns   |           1.5726 ns   |          N/A       |         __0.8339 ns__ |          N/A       |
| matrix3 inverse x16            |    4.1096 ns   |           1.5500 ns   |          N/A       |         __0.7919 ns__ |          N/A       |
| matrix3 mul matrix3 x1024      |    2.1455 ns   |           1.1577 ns   |        1.2924 ns   |         __0.8606 ns__ |        0.9531 ns   |
| matrix3 mul matrix3 x16        |    2.3720 ns   |           1.1672 ns   |        1.2841 ns   |         __0.5913 ns__ |        0.7141 ns   |
| matrix3 mul vector3 x1024      |    0.7485 ns   |           0.3405 ns   |        0.3420 ns   |         __0.1728 ns__ |      __0.1728 ns__ |
| matrix3 mul vector3 x16        |    0.7296 ns   |           0.3563 ns   |        0.3750 ns   |         __0.1894 ns__ |      __0.1881 ns__ |
| matrix3 transpose x1024        |  __1.0265 ns__ |           5.5623 ns   |        3.2017 ns   |           1.5549 ns   |        1.2423 ns   |
| matrix3 transpose x16          |  __0.9433 ns__ |           5.5360 ns   |        3.2043 ns   |           1.5342 ns   |        1.2288 ns   |
| matrix4 determinant x1024      |    3.4429 ns   |           1.0488 ns   |          N/A       |         __0.5266 ns__ |          N/A       |
| matrix4 determinant x16        |    3.4131 ns   |           0.9526 ns   |          N/A       |         __0.4872 ns__ |          N/A       |
| matrix4 inverse x1024          |   11.0112 ns   |           4.4601 ns   |          N/A       |         __2.8535 ns__ |          N/A       |
| matrix4 inverse x16            |   11.1364 ns   |           4.4673 ns   |          N/A       |         __2.8421 ns__ |          N/A       |
| matrix4 mul matrix4 x1024      |    3.1637 ns   |           3.4517 ns   |        7.4823 ns   |         __2.5313 ns__ |        4.5643 ns   |
| matrix4 mul matrix4 x16        |    3.1588 ns   |           3.4346 ns   |        7.8127 ns   |         __2.2894 ns__ |        4.1740 ns   |
| matrix4 mul vector4 x1024      |    0.7989 ns   |           0.7317 ns   |        0.5645 ns   |           0.3762 ns   |      __0.3084 ns__ |
| matrix4 mul vector4 x16        |    0.8231 ns   |           0.7497 ns   |        0.5898 ns   |           0.3842 ns   |      __0.3119 ns__ |
| matrix4 transpose x1024        |  __1.4185 ns__ |           5.6545 ns   |        3.7571 ns   |           2.4720 ns   |        1.9211 ns   |
| matrix4 transpose x16          |  __1.4212 ns__ |           5.1007 ns   |        3.6512 ns   |           2.3998 ns   |        1.8304 ns   |
| ray-sphere intersection x80000 |    9.2936 ns   |           1.2298 ns   |        1.2658 ns   |         __0.7473 ns__ |        0.7690 ns   |
| rotation3 inverse x1024        |    0.3745 ns   |           0.2421 ns   |        0.3098 ns   |         __0.1582 ns__ |      __0.1599 ns__ |
| rotation3 inverse x16          |    0.2603 ns   |           0.2617 ns   |        0.3214 ns   |         __0.1483 ns__ |        0.1764 ns   |
| rotation3 mul rotation3 x1024  |    1.3080 ns   |           0.7166 ns   |        0.7241 ns   |         __0.4513 ns__ |        0.4834 ns   |
| rotation3 mul rotation3 x16    |    1.1636 ns   |           0.6297 ns   |        0.6280 ns   |         __0.3250 ns__ |      __0.3278 ns__ |
| rotation3 mul vector3 x1024    |    1.5995 ns   |           0.8711 ns   |        0.7122 ns   |           0.4429 ns   |      __0.3611 ns__ |
| rotation3 mul vector3 x16      |    1.5737 ns   |           0.8632 ns   |        0.7171 ns   |           0.4328 ns   |      __0.3661 ns__ |
| transform point2 x1024         |    0.4908 ns   |           0.3759 ns   |          N/A       |         __0.1928 ns__ |          N/A       |
| transform point2 x16           |    0.5074 ns   |           0.3779 ns   |          N/A       |         __0.1904 ns__ |          N/A       |
| transform point3 x1024         |    0.8207 ns   |           0.7699 ns   |          N/A       |         __0.3933 ns__ |          N/A       |
| transform point3 x16           |    0.7732 ns   |           0.7876 ns   |          N/A       |         __0.4073 ns__ |          N/A       |
| transform vector2 x1024        |    0.4941 ns   |           0.2034 ns   |          N/A       |         __0.1049 ns__ |          N/A       |
| transform vector2 x16          |    0.5070 ns   |           0.2208 ns   |          N/A       |         __0.1187 ns__ |          N/A       |
| transform vector3 x1024        |    0.7454 ns   |           0.6061 ns   |          N/A       |         __0.3072 ns__ |          N/A       |
| transform vector3 x16          |    0.7229 ns   |           0.6176 ns   |          N/A       |         __0.3225 ns__ |          N/A       |
| vector3 cross x1024            |    0.7318 ns   |           0.4837 ns   |        0.4686 ns   |         __0.3525 ns__ |        0.3650 ns   |
| vector3 cross x16              |    0.7188 ns   |           0.3537 ns   |        0.3119 ns   |           0.1881 ns   |      __0.1628 ns__ |
| vector3 dot x1024              |    0.4906 ns   |           0.1834 ns   |        0.1815 ns   |         __0.0945 ns__ |      __0.0945 ns__ |
| vector3 dot x16                |    0.5069 ns   |           0.1923 ns   |        0.1922 ns   |           0.1076 ns   |      __0.1053 ns__ |
| vector3 length x1024           |    1.2698 ns   |           0.3208 ns   |        0.3200 ns   |         __0.1621 ns__ |      __0.1615 ns__ |
| vector3 length x16             |    1.2707 ns   |           0.3183 ns   |        0.3200 ns   |         __0.1629 ns__ |      __0.1629 ns__ |
| vector3 normalize x1024        |    2.1367 ns   |           0.5522 ns   |        0.9400 ns   |         __0.2775 ns__ |        0.4700 ns   |
| vector3 normalize x16          |    2.1252 ns   |           0.5527 ns   |        0.9365 ns   |         __0.2800 ns__ |        0.4691 ns   |

[AMD Ryzen 7 3800X]: https://www.amd.com/en/products/cpu/amd-ryzen-7-3800x
[mathbench report]: https://bitshifter.github.io/mathbench/0.5.0/report/index.html

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

There is a script in `scripts/summary.sh` to summarize the results in a nice
fashion. It prints the scalar benchmarks by default; pass `-w` for the wide
benchmarks, or `-w -s` / `-a` to include both. `-t` sets the equivalence margin
in percent used for highlighting (default 2.5). e.g. to include all libraries
and both row kinds and generate an ASCII table:

```sh
./scripts/summary.sh -a
```

The wrapper creates a `.venv` in the repository root and installs the
`prettytable` module into it on first use, so your system Python and its
packages are left alone. Pass `--help` for the other options. If you would
rather manage the environment yourself, `scripts/summary.py` is the script
being run and only needs Python 3 and `prettytable`.

Before running the benchmarks, clear any previous results with
`rm -rf target/criterion`.

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

The `ray-sphere intersection` benchmark calls the per-element inner operation
through a `#[inline(never)]` function. This keeps the autovectorizer from
inlining it into the benchmark loop, so the benchmark measures the scalar,
non-vectorized case, which is common in real code where the vectorizer cannot
prove that vectorizing is safe.

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
* The output of `scripts/summary.sh`
* The full Criterion output from `target/criterion`

## Adding a new library

There are different steps involved for adding unit tests and benchmarks for a
new library.

Benchmarks require an implementation of the `mathbench::BenchValue` trait for
the types you want to benchmark. If a function returning your type (or a type
convertible into it, such as a `mint` type) is available then you can use the
`impl_bench_value!` macro in `src/lib.rs`, e.g.
`impl_bench_value!(glam::Vec3, random_mint_vec3)`. Otherwise implement
`BenchValue::random_value` by hand.

To add the new library type to a benchmark, add a `bench!` arm for it in the
relevant `benches/*.rs` file and call one of the `bench_*` macros from
`benches/support/macros.rs` (for example `bench_unop!` for a single-operand
operation or `bench_binop!` for a two-operand one). The `bench!` macro gates the
benchmark on the library's Cargo feature.

Increment the version number of `mathbench` in `Cargo.toml`.

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
