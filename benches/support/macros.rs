// Shared criterion benchmark macros.
//
// `bench!` binds a benchmark context, `(&mut BenchmarkGroup, lib_name)`, and
// forwards it to one of the `bench_*` macros below. Each `bench_*` macro
// registers one case per size and reports per-element throughput via
// `Throughput::Elements`. All of them share [`bench_workload`], which owns the
// size loop, the per-size pool build, and the timed `iter` loop; a `bench_*`
// macro only supplies its pool shape and its per-iteration body.
//
// The sizes are [`SIZES`]: 16 is L1-resident, 1024 adds cache pressure.

/// The scalar element counts every `bench_*` entry registers.
#[allow(dead_code)]
pub const SIZES: &[usize] = &[16, 1024];

/// Deterministic seed used for all benchmark input generation.
pub const BENCH_RNG_SEED: u128 = 0x853c49e6748fea9b_da3e39cb94b95bdb;

/// Invoke a benchmark closure with its context.
///
/// A free function is used rather than an immediately-invoked closure because
/// rustc cannot infer the parameter type of a closure invoked in place.
#[allow(dead_code)]
pub fn bench_context<M, F>(
    group: &mut criterion::BenchmarkGroup<'_, M>,
    name: &'static str,
    closure: F,
) where
    M: criterion::measurement::Measurement,
    F: FnOnce(&mut (&mut criterion::BenchmarkGroup<'_, M>, &'static str)),
{
    let mut ctx = (group, name);
    closure(&mut ctx);
}

/// Variant of [`bench_context`] for workload benchmarks that own their sizing.
#[allow(dead_code)]
pub fn bench_context_sized<M, F, T>(
    group: &mut criterion::BenchmarkGroup<'_, M>,
    name: &'static str,
    size: T,
    closure: F,
) where
    M: criterion::measurement::Measurement,
    F: FnOnce(&mut (&mut criterion::BenchmarkGroup<'_, M>, &'static str), T),
{
    let mut ctx = (group, name);
    closure(&mut ctx, size);
}

/// Register one benchmark case per size for a workload that owns its pools.
///
/// `build` is called once per size with the shared RNG and the scalar element
/// count; `run` is the timed body and receives that state by `&mut`.
///
/// This is the skeleton every entry shares: the size list, per-size
/// [`Throughput::Elements`](criterion::Throughput), the `BenchmarkId`, and the
/// `iter` loop. The fixed-width `bench_*` macros pool per-lane values; a
/// runtime-dispatched library pools SoA component slices and calls its
/// `dispatch!` kernel here instead.
#[allow(dead_code)]
pub fn bench_workload<M, S, B, R>(
    ctx: &mut (&mut criterion::BenchmarkGroup<'_, M>, &'static str),
    sizes: &[usize],
    mut build: B,
    mut run: R,
) where
    M: criterion::measurement::Measurement,
    B: FnMut(&mut rand_pcg::Pcg64Mcg, usize) -> S,
    R: FnMut(&mut S),
{
    let name = ctx.1;
    for &size in sizes {
        let mut rng = rand_pcg::Pcg64Mcg::new(BENCH_RNG_SEED);
        let mut state = build(&mut rng, size);
        ctx.0
            .throughput(criterion::Throughput::Elements(size as u64));
        ctx.0
            .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                bencher.iter(|| run(&mut state))
            });
    }
}

/// Register a Criterion benchmark for a math library.
///
/// The closure is invoked with a context `(&mut BenchmarkGroup, lib_name)` and
/// is expected to call one of the `bench_*` macros. The two argument form is
/// for workload benchmarks that own their input sizing (euler, ray-sphere) and
/// receive the size as a second argument.
///
/// # Forms
///
/// **glam** (always compiled, no `#[cfg]` gate):
/// ```ignore
/// bench!("glam", group, |b| { bench_unop!(b, op => length, ty => Vec3A) });
/// ```
///
/// **Gated library** (feature name == display name):
/// ```ignore
/// bench!("cgmath", group, |b| { ... });
/// ```
///
/// **Display-name override** (feature name ≠ display name):
/// ```ignore
/// bench!("pathfinder_geometry" as "pathfinder", group, |b| { ... });
/// ```
#[macro_export]
macro_rules! bench {
    // glam (always compiled, no feature gate)
    ("glam", $group:ident, $closure:expr) => {
        macros::bench_context(&mut $group, "glam", $closure)
    };
    ("glam", $group:ident, $size:expr, $closure:expr) => {
        macros::bench_context_sized(&mut $group, "glam", $size, $closure)
    };

    // Gated libraries (feature name = display name).
    //
    // The disabled branch still borrows `group` mutably so that `let mut group`
    // is used even when every library in a group is compiled out (e.g. the wide
    // benchmarks under a scalar-only feature set).
    ($lib:literal, $group:ident, $closure:expr) => {{
        #[cfg(feature = $lib)]
        macros::bench_context(&mut $group, $lib, $closure);
        #[cfg(not(feature = $lib))]
        let _ = &mut $group;
    }};
    ($lib:literal, $group:ident, $size:expr, $closure:expr) => {{
        #[cfg(feature = $lib)]
        macros::bench_context_sized(&mut $group, $lib, $size, $closure);
        #[cfg(not(feature = $lib))]
        let _ = &mut $group;
    }};

    // Display-name override
    ($lib:literal as $display:literal, $group:ident, $closure:expr) => {{
        #[cfg(feature = $lib)]
        macros::bench_context(&mut $group, $display, $closure);
        #[cfg(not(feature = $lib))]
        let _ = &mut $group;
    }};
    ($lib:literal as $display:literal, $group:ident, $size:expr, $closure:expr) => {{
        #[cfg(feature = $lib)]
        macros::bench_context_sized(&mut $group, $display, $size, $closure);
        #[cfg(not(feature = $lib))]
        let _ = &mut $group;
    }};
}

/// Run `$unop` on a batch of values and report per-element throughput.
///
/// `$b` is the context bound by [`bench!`].
#[macro_export]
macro_rules! bench_unop {
    ($b:ident, op => $unop:ident, ty => $t:ty) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let inputs = (0..size)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t>>();
                let outputs = (0..size)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng).$unop())
                    .collect::<Vec<_>>();
                (inputs, outputs)
            },
            |state| {
                let (inputs, outputs) = state;
                for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
                    *output = input.$unop();
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
}

/// Run `$binop` on a batch of value pairs and report per-element throughput.
#[macro_export]
macro_rules! bench_binop {
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let inputs1 = (0..size)
                    .map(|_| <$t1 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t1>>();
                let inputs2 = (0..size)
                    .map(|_| <$t2 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t2>>();
                let outputs = (0..size)
                    .map(|_| {
                        let a = <$t1 as mathbench::BenchValue>::random_value(rng);
                        let b = <$t2 as mathbench::BenchValue>::random_value(rng);
                        a.$binop($param!(&b))
                    })
                    .collect::<Vec<_>>();
                (inputs1, inputs2, outputs)
            },
            |state| {
                let (inputs1, inputs2, outputs) = state;
                for ((input1, input2), output) in
                    inputs1.iter().zip(inputs2.iter()).zip(outputs.iter_mut())
                {
                    *output = input1.$binop($param!(input2));
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty) => {
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

/// Run `$binop` with a single `ty1` operand reused across a batch of `ty2`
/// operands, e.g. transforming many vectors with the same matrix.
#[macro_export]
macro_rules! bench_binop_broadcast {
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let input1 = <$t1 as mathbench::BenchValue>::random_value(rng);
                let inputs2 = (0..size)
                    .map(|_| <$t2 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t2>>();
                let outputs = inputs2
                    .iter()
                    .map(|input2| input1.$binop($param!(input2)))
                    .collect::<Vec<_>>();
                (input1, inputs2, outputs)
            },
            |state| {
                let (input1, inputs2, outputs) = state;
                for (input2, output) in inputs2.iter().zip(outputs.iter_mut()) {
                    *output = input1.$binop($param!(input2));
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

/// Wide variant of [`bench_unop`]. Sizes are scalar element counts; each timed
/// iteration runs `size / width` wide operations.
#[macro_export]
macro_rules! bench_unop_wide {
    ($b:ident, width => $width:expr, op => $unop:ident, ty => $t:ty) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let width: usize = $width;
                let count = size.div_ceil(width);
                let inputs = (0..count)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t>>();
                let outputs = (0..count)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng).$unop())
                    .collect::<Vec<_>>();
                (inputs, outputs)
            },
            |state| {
                let (inputs, outputs) = state;
                for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
                    *output = input.$unop();
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
}

/// Wide variant of [`bench_binop`]. Sizes are scalar element counts.
#[macro_export]
macro_rules! bench_binop_wide {
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let width: usize = $width;
                let count = size.div_ceil(width);
                let inputs1 = (0..count)
                    .map(|_| <$t1 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t1>>();
                let inputs2 = (0..count)
                    .map(|_| <$t2 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t2>>();
                let outputs = (0..count)
                    .map(|_| {
                        let a = <$t1 as mathbench::BenchValue>::random_value(rng);
                        let b = <$t2 as mathbench::BenchValue>::random_value(rng);
                        a.$binop($param!(&b))
                    })
                    .collect::<Vec<_>>();
                (inputs1, inputs2, outputs)
            },
            |state| {
                let (inputs1, inputs2, outputs) = state;
                for ((input1, input2), output) in
                    inputs1.iter().zip(inputs2.iter()).zip(outputs.iter_mut())
                {
                    *output = input1.$binop($param!(input2));
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value)
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

/// Wide variant of [`bench_binop_broadcast`]. Sizes are scalar element counts;
/// one `ty1` operand is reused across the batch.
#[macro_export]
macro_rules! bench_binop_wide_broadcast {
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        $crate::macros::bench_workload(
            $b,
            $crate::macros::SIZES,
            |rng, size| {
                let width: usize = $width;
                let count = size.div_ceil(width);
                let input1 = <$t1 as mathbench::BenchValue>::random_value(rng);
                let inputs2 = (0..count)
                    .map(|_| <$t2 as mathbench::BenchValue>::random_value(rng))
                    .collect::<Vec<$t2>>();
                let outputs = inputs2
                    .iter()
                    .map(|input2| input1.$binop($param!(input2)))
                    .collect::<Vec<_>>();
                (input1, inputs2, outputs)
            },
            |state| {
                let (input1, inputs2, outputs) = state;
                for (input2, output) in inputs2.iter().zip(outputs.iter_mut()) {
                    *output = input1.$binop($param!(input2));
                }
                std::hint::black_box(outputs.as_slice());
            },
        )
    }};
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_wide_broadcast!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value)
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop_wide_broadcast!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty) => {
        bench_binop_wide_broadcast!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

#[macro_export]
macro_rules! by_value {
    ($e:expr) => {
        *$e
    };
}

#[macro_export]
macro_rules! by_ref {
    ($e:expr) => {
        $e
    };
}
