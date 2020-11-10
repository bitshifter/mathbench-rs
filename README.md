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
compiled with the `1.49.0-nightly (ffa2e7ae8 2020-10-24)` Rust compiler. Lower
(better) numbers are highlighted within a 2.5% range of the minimum for each
row.

The versions of the libraries tested were:

* `cgmath` - `0.17.0`
* `euclid` - `0.22.1`
* `glam` - `0.10.0`
* `nalgebra` - `0.23.0`
* `pathfinder_geometry` - `0.5.1`
* `static-math` - `0.1.7`
* `ultraviolet` - `0.5.1`
* `vek` - `0.12.0` (`repr_c` types)

See the full [mathbench report] for more detailed results.

### Scalar benchmarks

Run with the command:

```sh
cargo +nightly bench --features scalar scalar
```

| benchmark                      |          glam   |        cgmath   |      nalgebra   |       euclid   |           vek   |    pathfinder   |   static-math   |   ultraviolet   |
|:-------------------------------|----------------:|----------------:|----------------:|---------------:|----------------:|----------------:|----------------:|----------------:|
| euler 2d x10000                |      16.18 us   |       16.3 us   |      16.25 us   |     16.23 us   |      16.25 us   |    __10.42 us__ |      11.84 us   |      16.28 us   |
| euler 3d x10000                |    __16.13 us__ |      32.09 us   |      25.49 us   |      32.2 us   |      32.21 us   |    __16.23 us__ |      34.74 us   |      32.07 us   |
| matrix2 determinant            |   __2.0417 ns__ |     2.1235 ns   |     2.1118 ns   |      N/A       |     2.1132 ns   |     2.1182 ns   |     2.1173 ns   |     2.1161 ns   |
| matrix2 inverse                |   __2.8321 ns__ |     8.4686 ns   |     7.6035 ns   |      N/A       |       N/A       |     3.4420 ns   |     8.3189 ns   |     5.8985 ns   |
| matrix2 mul matrix2            |     6.1247 ns   |     4.8130 ns   |   __2.5461 ns__ |      N/A       |    11.6360 ns   |   __2.5541 ns__ |     4.7587 ns   |     4.7334 ns   |
| matrix2 mul vector2 x1         |     2.8408 ns   |     2.6186 ns   |     2.6343 ns   |      N/A       |     5.4199 ns   |   __2.1706 ns__ |     2.6969 ns   |     2.6153 ns   |
| matrix2 mul vector2 x100       |   276.1011 ns   |   237.2400 ns   |   243.3239 ns   |      N/A       |   545.8342 ns   | __220.7986 ns__ |   264.6844 ns   |   236.9462 ns   |
| matrix2 return self            |     2.8687 ns   |     2.8712 ns   |     2.8892 ns   |      N/A       |     2.8857 ns   |   __2.4157 ns__ |     2.8777 ns   |     2.9764 ns   |
| matrix2 transpose              |   __2.2713 ns__ |     3.0883 ns   |   __2.2310 ns__ |      N/A       |     3.0914 ns   |       N/A       |     3.0835 ns   |     3.0775 ns   |
| matrix3 determinant            |   __3.8307 ns__ |   __3.7721 ns__ |   __3.8148 ns__ |      N/A       |   __3.8240 ns__ |       N/A       |   __3.8223 ns__ |     8.9148 ns   |
| matrix3 inverse                |    15.2042 ns   |    18.2388 ns   |  __12.7075 ns__ |      N/A       |       N/A       |       N/A       |  __12.8133 ns__ |    22.1096 ns   |
| matrix3 mul matrix3            |    10.4010 ns   |    11.2899 ns   |  __10.3583 ns__ |      N/A       |    40.1530 ns   |       N/A       |  __10.1117 ns__ |    11.2713 ns   |
| matrix3 mul vector3 x1         |     4.7889 ns   |     4.4906 ns   |   __4.3330 ns__ |      N/A       |    13.2860 ns   |       N/A       |     4.7966 ns   |     4.4801 ns   |
| matrix3 mul vector3 x100       |     0.5121 us   |   __0.4669 us__ |   __0.4754 us__ |      N/A       |      1.348 us   |       N/A       |   __0.4767 us__ |   __0.4728 us__ |
| matrix3 return self            |   __5.4364 ns__ |   __5.4451 ns__ |   __5.4552 ns__ |      N/A       |   __5.4463 ns__ |       N/A       |   __5.4450 ns__ |   __5.4534 ns__ |
| matrix3 transpose              |    10.0869 ns   |    10.1385 ns   |  __10.0176 ns__ |      N/A       |    10.1395 ns   |       N/A       |    10.8063 ns   |   __9.7977 ns__ |
| matrix4 determinant            |   __6.1510 ns__ |    11.6457 ns   |    52.3414 ns   |   17.0240 ns   |    18.3800 ns   |       N/A       |    16.9031 ns   |     8.5125 ns   |
| matrix4 inverse                |  __16.5764 ns__ |    47.0562 ns   |    69.0789 ns   |   65.0189 ns   |   299.8796 ns   |       N/A       |    52.1599 ns   |    42.0630 ns   |
| matrix4 mul matrix4            |   __7.5811 ns__ |    26.6004 ns   |     8.2055 ns   |   11.5513 ns   |    91.5766 ns   |       N/A       |    21.0343 ns   |    26.5077 ns   |
| matrix4 mul vector4 x1         |   __3.1131 ns__ |     6.8211 ns   |     3.5017 ns   |      N/A       |    23.9593 ns   |       N/A       |     7.0599 ns   |     6.8278 ns   |
| matrix4 mul vector4 x100       |   __0.6175 us__ |      0.768 us   |   __0.6271 us__ |      N/A       |       2.26 us   |       N/A       |     0.8465 us   |     0.7875 us   |
| matrix4 return self            |     7.3269 ns   |   __7.1310 ns__ |     7.3162 ns   |      N/A       |     7.3160 ns   |       N/A       |   __7.2881 ns__ |   __7.1189 ns__ |
| matrix4 transpose              |   __7.4352 ns__ |    12.0065 ns   |    14.8833 ns   |      N/A       |    11.8665 ns   |       N/A       |    16.1124 ns   |    12.6715 ns   |
| ray-sphere intersection x10000 |    __16.09 us__ |    __16.12 us__ |      90.09 us   |   __16.06 us__ |      69.34 us   |       N/A       |       N/A       |    __16.12 us__ |
| rotation3 inverse              |   __2.2081 ns__ |     3.4053 ns   |     7.6562 ns   |    3.3040 ns   |     3.3085 ns   |       N/A       |     3.4015 ns   |     3.2964 ns   |
| rotation3 mul rotation3        |   __3.3522 ns__ |     6.9520 ns   |     7.0618 ns   |    7.1581 ns   |     7.0768 ns   |       N/A       |     7.5341 ns   |     7.0063 ns   |
| rotation3 mul vector3 x1       |   __6.5538 ns__ |     7.4976 ns   |     7.9157 ns   |    7.5374 ns   |    17.6267 ns   |       N/A       |     7.4405 ns   |     8.6141 ns   |
| rotation3 mul vector3 x100     |   __0.6592 us__ |     0.7402 us   |     0.7623 us   |    0.7663 us   |      1.786 us   |       N/A       |      0.742 us   |     0.8601 us   |
| rotation3 return self          |   __2.8756 ns__ |   __2.8689 ns__ |   __2.8714 ns__ |      N/A       |   __2.8778 ns__ |       N/A       |   __2.8630 ns__ |   __2.8725 ns__ |
| transform point2 x1            |     4.9832 ns   |     2.8866 ns   |     4.8093 ns   |    2.8645 ns   |    12.9289 ns   |   __2.3697 ns__ |       N/A       |     4.2484 ns   |
| transform point2 x100          |     0.5626 us   |     0.3667 us   |     0.4667 us   |  __0.2999 us__ |      1.318 us   |     0.3103 us   |       N/A       |      0.433 us   |
| transform point3 x1            |     4.8302 ns   |     8.5171 ns   |     8.3459 ns   |    7.4187 ns   |    23.8216 ns   |   __3.1912 ns__ |       N/A       |     7.5176 ns   |
| transform point3 x100          |     0.6283 us   |     0.9532 us   |     0.8853 us   |    0.7718 us   |      2.467 us   |   __0.6094 us__ |       N/A       |     0.8374 us   |
| transform vector2 x1           |     2.9159 ns   |       N/A       |     4.1823 ns   |  __2.6192 ns__ |    12.9047 ns   |       N/A       |       N/A       |     3.0131 ns   |
| transform vector2 x100         |     0.3701 us   |       N/A       |     0.4204 us   |  __0.2763 us__ |      1.306 us   |       N/A       |       N/A       |     0.3858 us   |
| transform vector3 x1           |   __3.8636 ns__ |     5.7279 ns   |     7.1869 ns   |    4.6699 ns   |    23.8999 ns   |       N/A       |       N/A       |     5.5564 ns   |
| transform vector3 x100         |   __0.5898 us__ |     0.6837 us   |     0.8102 us   |    0.6545 us   |      2.486 us   |       N/A       |       N/A       |     0.6623 us   |
| transform2 inverse             |       N/A       |       N/A       |    12.7549 ns   |    9.5978 ns   |       N/A       |   __8.5094 ns__ |       N/A       |       N/A       |
| transform2 mul transform2      |       N/A       |       N/A       |     9.8682 ns   |    6.1542 ns   |       N/A       |   __3.5796 ns__ |       N/A       |       N/A       |
| transform2 return self         |       N/A       |       N/A       |     5.4518 ns   |    4.2681 ns   |       N/A       |   __4.0640 ns__ |       N/A       |       N/A       |
| transform3 inverse             |       N/A       |       N/A       |    70.0825 ns   |   56.0094 ns   |       N/A       |  __23.1292 ns__ |       N/A       |       N/A       |
| transform3 mul transform3d     |       N/A       |       N/A       |    10.8279 ns   |   10.2269 ns   |       N/A       |   __7.3766 ns__ |       N/A       |       N/A       |
| transform3 return self         |       N/A       |       N/A       |   __7.1745 ns__ |  __7.0434 ns__ |       N/A       |   __7.0029 ns__ |       N/A       |       N/A       |
| vector3 cross                  |   __2.4517 ns__ |     3.7895 ns   |     3.6316 ns   |    3.7938 ns   |     3.8766 ns   |       N/A       |     3.8604 ns   |     3.5966 ns   |
| vector3 dot                    |   __2.0932 ns__ |     2.3043 ns   |     2.2996 ns   |    2.3064 ns   |     2.3114 ns   |       N/A       |     2.3114 ns   |     2.3363 ns   |
| vector3 length                 |   __2.5202 ns__ |   __2.5203 ns__ |     2.6035 ns   |  __2.5147 ns__ |   __2.5157 ns__ |       N/A       |   __2.5163 ns__ |   __2.5120 ns__ |
| vector3 normalize              |   __4.0407 ns__ |     5.8509 ns   |     8.2205 ns   |    8.0755 ns   |     8.0751 ns   |       N/A       |       N/A       |     5.8531 ns   |
| vector3 return self            |   __2.8560 ns__ |     3.1177 ns   |     3.0976 ns   |      N/A       |     3.1020 ns   |       N/A       |     3.0914 ns   |     3.0999 ns   |

### Wide benchmarks

Run with the command:

```sh
RUSTFLAGS='-C target-feature=+avx2' cargo +nightly bench --features wide wide
```

| benchmark                      |    glam_f32x1   |   ultraviolet_f32x4   |   nalgebra_f32x4   |   ultraviolet_f32x8   |   nalgebra_f32x8   |
|:-------------------------------|----------------:|----------------------:|-------------------:|----------------------:|-------------------:|
| euler 2d x80000                |      143.5 us   |           __63.9 us__ |         212.0 us   |             69.2 us   |         70.61 us   |
| euler 3d x80000                |      134.7 us   |          __95.25 us__ |         234.0 us   |            105.2 us   |         106.6 us   |
| matrix2 determinant x16        |    18.7044 ns   |          10.6443 ns   |          N/A       |        __10.0458 ns__ |          N/A       |
| matrix2 inverse x16            |    39.8025 ns   |          29.9882 ns   |          N/A       |        __22.9798 ns__ |          N/A       |
| matrix2 mul matrix2 x16        |   110.8825 ns   |          36.6508 ns   |       55.7935 ns   |        __31.7618 ns__ |       36.4354 ns   |
| matrix2 mul matrix2 x256       |      1.556 us   |           0.9449 us   |         1.102 us   |         __0.8686 us__ |        0.9062 us   |
| matrix2 mul vector2 x16        |    51.8334 ns   |          18.0458 ns   |       39.7479 ns   |        __17.5347 ns__ |       19.1624 ns   |
| matrix2 mul vector2 x256       |   810.7620 ns   |       __560.2045 ns__ |      743.5690 ns   |       __554.8579 ns__ |    __558.4294 ns__ |
| matrix2 return self x16        |    40.4023 ns   |          29.3419 ns   |       29.7301 ns   |        __21.2867 ns__ |     __21.5470 ns__ |
| matrix2 transpose x16          |    32.9701 ns   |          29.4639 ns   |       48.4303 ns   |        __19.7871 ns__ |       21.4183 ns   |
| matrix3 determinant x16        |    53.1394 ns   |          24.5118 ns   |          N/A       |        __21.1423 ns__ |          N/A       |
| matrix3 inverse x16            |   306.7033 ns   |          79.1990 ns   |          N/A       |        __70.2207 ns__ |          N/A       |
| matrix3 mul matrix3 x16        |   204.9730 ns   |       __116.6126 ns__ |      129.6155 ns   |       __116.3397 ns__ |      126.8384 ns   |
| matrix3 mul matrix3 x256       |      2.841 us   |           __1.97 us__ |         2.167 us   |          __1.957 us__ |         2.101 us   |
| matrix3 mul vector3 x16        |    89.4383 ns   |        __41.0862 ns__ |       56.6786 ns   |        __42.0856 ns__ |       42.2275 ns   |
| matrix3 mul vector3 x256       |      1.404 us   |          __0.985 us__ |         1.111 us   |            __1.0 us__ |         1.011 us   |
| matrix3 return self x16        |   117.4071 ns   |          74.9792 ns   |       75.9878 ns   |        __69.0985 ns__ |     __68.9782 ns__ |
| matrix3 transpose x16          |   116.2578 ns   |          69.1388 ns   |       83.7164 ns   |        __64.6128 ns__ |     __66.1932 ns__ |
| matrix4 determinant x16        |    98.5214 ns   |          62.2123 ns   |          N/A       |        __55.0411 ns__ |          N/A       |
| matrix4 inverse x16            |   285.0211 ns   |       __173.8923 ns__ |          N/A       |       __174.8764 ns__ |          N/A       |
| matrix4 mul matrix4 x16        |   241.3988 ns   |       __220.6856 ns__ |      254.1750 ns   |       __221.1143 ns__ |      242.1435 ns   |
| matrix4 mul matrix4 x256       |      3.776 us   |           __3.53 us__ |         4.068 us   |          __3.603 us__ |          3.88 us   |
| matrix4 mul vector4 x16        |    93.3108 ns   |        __90.8840 ns__ |       93.4190 ns   |          95.3603 ns   |     __91.5837 ns__ |
| matrix4 mul vector4 x256       |      1.595 us   |          __1.549 us__ |       __1.584 us__ |           __1.58 us__ |       __1.587 us__ |
| matrix4 return self x16        |   167.5312 ns   |       __159.6082 ns__ |      165.3852 ns   |         168.3952 ns   |      165.7410 ns   |
| matrix4 transpose x16          |   185.2348 ns   |       __155.0984 ns__ |    __153.7135 ns__ |         161.5637 ns   |    __155.1658 ns__ |
| ray-sphere intersection x80000 |      82.97 us   |            108.6 us   |         143.0 us   |          __70.35 us__ |         93.03 us   |
| rotation3 inverse x16          |    32.4653 ns   |          30.8475 ns   |       31.2066 ns   |          22.5417 ns   |     __20.1224 ns__ |
| rotation3 mul rotation3 x16    |    63.0726 ns   |          41.3994 ns   |       40.2430 ns   |        __33.6170 ns__ |     __33.7655 ns__ |
| rotation3 mul vector3 x16      |   115.3996 ns   |          36.8573 ns   |       37.9278 ns   |        __25.6498 ns__ |       26.7130 ns   |
| rotation3 return self x16      |    39.6630 ns   |          29.2236 ns   |       30.9480 ns   |        __21.1458 ns__ |     __20.8969 ns__ |
| transform point2 x16           |    89.9512 ns   |        __32.8298 ns__ |          N/A       |        __32.1583 ns__ |          N/A       |
| transform point2 x256          |      1.407 us   |         __0.8573 us__ |          N/A       |         __0.8538 us__ |          N/A       |
| transform point3 x16           |   103.5212 ns   |        __81.0851 ns__ |          N/A       |        __82.3375 ns__ |          N/A       |
| transform point3 x256          |      1.674 us   |          __1.409 us__ |          N/A       |           __1.44 us__ |          N/A       |
| transform vector2 x16          |    53.2552 ns   |        __32.3600 ns__ |          N/A       |          33.1964 ns   |          N/A       |
| transform vector2 x256         |   958.2088 ns   |       __826.2741 ns__ |          N/A       |         858.3724 ns   |          N/A       |
| transform vector3 x16          |    90.9893 ns   |        __75.6296 ns__ |          N/A       |          84.0528 ns   |          N/A       |
| transform vector3 x256         |      1.551 us   |            __1.4 us__ |          N/A       |            1.441 us   |          N/A       |
| vector3 cross x16              |    42.5796 ns   |          26.8007 ns   |       26.2831 ns   |        __21.8064 ns__ |     __21.8924 ns__ |
| vector3 dot x16                |    28.8746 ns   |          13.5288 ns   |       12.6610 ns   |        __12.3348 ns__ |     __12.4651 ns__ |
| vector3 length x16             |    32.5694 ns   |          10.0201 ns   |       10.0544 ns   |         __9.3971 ns__ |      __9.5219 ns__ |
| vector3 normalize x16          |    66.2781 ns   |          23.6915 ns   |       44.4884 ns   |        __20.0333 ns__ |       37.2278 ns   |
| vector3 return self x16        |    40.0744 ns   |          36.5467 ns   |       36.5094 ns   |        __17.1831 ns__ |     __17.2817 ns__ |

[Intel i7-4710HQ]: https://ark.intel.com/content/www/us/en/ark/products/78930/intel-core-i7-4710hq-processor-6m-cache-up-to-3-50-ghz.html
[mathbench report]: https://bitshifter.github.io/mathbench/0.4.0/report/index.html

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
