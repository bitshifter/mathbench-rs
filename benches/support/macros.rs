// Shared criterion benchmark macros.
//
// `bench!` binds a benchmark context, `(&mut BenchmarkGroup, lib_name)`, and
// forwards it to one of the `bench_*` macros below. Each `bench_*` macro
// registers one case per size and reports per-element throughput via
// `Throughput::Elements`. A timed iteration runs a batch of independent
// operations drawn from a pool that is generated once and reused across sizes,
// so the measurement is not dominated by per-iteration harness overhead.
//
// Default sizes are `[16, 1024]`: 16 is L1-resident, 1024 adds cache pressure.
// Pass `sizes => [..]` to override per benchmark.

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
/// bench!("glam", group, |b| { bench_unop!(b, op => length, ty => Vec3) });
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
    ($b:ident, sizes => [$($size:expr),+ $(,)?], op => $unop:ident, ty => $t:ty) => {
        bench_unop!($b, op => $unop, ty => $t, sizes => [$($size),+])
    };
    ($b:ident, op => $unop:ident, ty => $t:ty, sizes => [$($size:expr),+ $(,)?]) => {{
        let sizes: &[usize] = &[$($size),+];
        let max_size = *sizes.iter().max().unwrap();
        let mut rng = rand_pcg::Pcg64Mcg::new(0x853c49e6748fea9b_da3e39cb94b95bdb_u128);
        let inputs = (0..max_size)
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t>>();
        let mut outputs = (0..max_size)
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng).$unop())
            .collect::<Vec<_>>();
        let name = $b.1;
        for &size in sizes {
            let inputs = &inputs[..size];
            let outputs = &mut outputs[..size];
            $b.0.throughput(criterion::Throughput::Elements(size as u64));
            $b.0
                .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                    bencher.iter(|| {
                        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
                            *output = input.$unop();
                        }
                        std::hint::black_box(&outputs);
                    })
                });
        }
    }};
    ($b:ident, op => $unop:ident, ty => $t:ty) => {
        bench_unop!($b, op => $unop, ty => $t, sizes => [16, 1024])
    };
}

/// Run `$binop` on a batch of value pairs and report per-element throughput.
#[macro_export]
macro_rules! bench_binop {
    ($b:ident, sizes => [$($size:expr),+ $(,)?], op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [$($size),+])
    };
    ($b:ident, sizes => [$($size:expr),+ $(,)?], op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, sizes => [$($size),+])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt, sizes => [$($size:expr),+ $(,)?]) => {{
        let sizes: &[usize] = &[$($size),+];
        let max_size = *sizes.iter().max().unwrap();
        let mut rng = rand_pcg::Pcg64Mcg::new(0x853c49e6748fea9b_da3e39cb94b95bdb_u128);
        let inputs1 = (0..max_size)
            .map(|_| <$t1 as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t1>>();
        let inputs2 = (0..max_size)
            .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t2>>();
        let mut outputs = (0..max_size)
            .map(|_| {
                let a = <$t1 as mathbench::BenchValue>::random_value(&mut rng);
                let b = <$t2 as mathbench::BenchValue>::random_value(&mut rng);
                a.$binop($param!(&b))
            })
            .collect::<Vec<_>>();
        let name = $b.1;
        for &size in sizes {
            let inputs1 = &inputs1[..size];
            let inputs2 = &inputs2[..size];
            let outputs = &mut outputs[..size];
            $b.0.throughput(criterion::Throughput::Elements(size as u64));
            $b.0
                .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                    bencher.iter(|| {
                        for ((input1, input2), output) in
                            inputs1.iter().zip(inputs2).zip(outputs.iter_mut())
                        {
                            *output = input1.$binop($param!(input2));
                        }
                        std::hint::black_box(&outputs);
                    })
                });
        }
    }};
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, sizes => [$($size:expr),+ $(,)?]) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [$($size),+])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [16, 1024])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [16, 1024])
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty) => {
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

/// Run `$binop` with a single `ty1` operand reused across a batch of `ty2`
/// operands, e.g. transforming many vectors with the same matrix. `sizes` is
/// the number of `ty2` operands in the batch.
#[macro_export]
macro_rules! bench_binop_broadcast {
    ($b:ident, sizes => [$($size:expr),+ $(,)?], op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [$($size),+])
    };
    ($b:ident, sizes => [$($size:expr),+ $(,)?], op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, sizes => [$($size),+])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt, sizes => [$($size:expr),+ $(,)?]) => {{
        let sizes: &[usize] = &[$($size),+];
        let max_size = *sizes.iter().max().unwrap();
        let mut rng = rand_pcg::Pcg64Mcg::new(0x853c49e6748fea9b_da3e39cb94b95bdb_u128);
        let input1 = <$t1 as mathbench::BenchValue>::random_value(&mut rng);
        let inputs2 = (0..max_size)
            .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t2>>();
        let mut outputs = inputs2
            .iter()
            .map(|input2| input1.$binop($param!(input2)))
            .collect::<Vec<_>>();
        let name = $b.1;
        for &size in sizes {
            let inputs2 = &inputs2[..size];
            let outputs = &mut outputs[..size];
            $b.0.throughput(criterion::Throughput::Elements(size as u64));
            $b.0
                .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                    bencher.iter(|| {
                        for (input2, output) in inputs2.iter().zip(outputs.iter_mut()) {
                            *output = input1.$binop($param!(input2));
                        }
                        std::hint::black_box(&outputs);
                    })
                });
        }
    }};
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, sizes => [$($size:expr),+ $(,)?]) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [$($size),+])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [16, 1024])
    };
    ($b:ident, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [16, 1024])
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, op => $binop:ident, ty => $ty:ty) => {
        bench_binop_broadcast!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    };
}

/// Wide variant of [`bench_unop`]. `sizes` are scalar element counts; each
/// timed iteration runs `size / width` wide operations.
#[macro_export]
macro_rules! bench_unop_wide {
    ($b:ident, sizes => [$($size:expr),+ $(,)?], width => $width:expr, op => $unop:ident, ty => $t:ty) => {
        bench_unop_wide!($b, width => $width, op => $unop, ty => $t, sizes => [$($size),+])
    };
    ($b:ident, width => $width:expr, op => $unop:ident, ty => $t:ty, sizes => [$($size:expr),+ $(,)?]) => {{
        let sizes: &[usize] = &[$($size),+];
        let max_size = *sizes.iter().max().unwrap();
        let width: usize = $width;
        let max_count = max_size.div_ceil(width);
        let mut rng = rand_pcg::Pcg64Mcg::new(0x853c49e6748fea9b_da3e39cb94b95bdb_u128);
        let inputs = (0..max_count)
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t>>();
        let mut outputs = (0..max_count)
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng).$unop())
            .collect::<Vec<_>>();
        let name = $b.1;
        for &size in sizes {
            let count = size.div_ceil(width);
            let inputs = &inputs[..count];
            let outputs = &mut outputs[..count];
            $b.0.throughput(criterion::Throughput::Elements(size as u64));
            $b.0
                .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                    bencher.iter(|| {
                        for (input, output) in inputs.iter().zip(outputs.iter_mut()) {
                            *output = input.$unop();
                        }
                        std::hint::black_box(&outputs);
                    })
                });
        }
    }};
    ($b:ident, width => $width:expr, op => $unop:ident, ty => $t:ty) => {
        bench_unop_wide!($b, width => $width, op => $unop, ty => $t, sizes => [16, 1024])
    };
}

/// Wide variant of [`bench_binop`]. `sizes` are scalar element counts.
#[macro_export]
macro_rules! bench_binop_wide {
    ($b:ident, sizes => [$($size:expr),+ $(,)?], width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [$($size),+])
    };
    ($b:ident, sizes => [$($size:expr),+ $(,)?], width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, sizes => [$($size),+])
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt, sizes => [$($size:expr),+ $(,)?]) => {{
        let sizes: &[usize] = &[$($size),+];
        let max_size = *sizes.iter().max().unwrap();
        let width: usize = $width;
        let max_count = max_size.div_ceil(width);
        let mut rng = rand_pcg::Pcg64Mcg::new(0x853c49e6748fea9b_da3e39cb94b95bdb_u128);
        let inputs1 = (0..max_count)
            .map(|_| <$t1 as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t1>>();
        let inputs2 = (0..max_count)
            .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
            .collect::<Vec<$t2>>();
        let mut outputs = (0..max_count)
            .map(|_| {
                let a = <$t1 as mathbench::BenchValue>::random_value(&mut rng);
                let b = <$t2 as mathbench::BenchValue>::random_value(&mut rng);
                a.$binop($param!(&b))
            })
            .collect::<Vec<_>>();
        let name = $b.1;
        for &size in sizes {
            let count = size.div_ceil(width);
            let inputs1 = &inputs1[..count];
            let inputs2 = &inputs2[..count];
            let outputs = &mut outputs[..count];
            $b.0.throughput(criterion::Throughput::Elements(size as u64));
            $b.0
                .bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                    bencher.iter(|| {
                        for ((input1, input2), output) in
                            inputs1.iter().zip(inputs2).zip(outputs.iter_mut())
                        {
                            *output = input1.$binop($param!(input2));
                        }
                        std::hint::black_box(&outputs);
                    })
                });
        }
    }};
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, sizes => [$($size:expr),+ $(,)?]) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [$($size),+])
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => $param, sizes => [16, 1024])
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty1 => $t1:ty, ty2 => $t2:ty) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $t1, ty2 => $t2, param => by_value, sizes => [16, 1024])
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty, param => $param:tt) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    };
    ($b:ident, width => $width:expr, op => $binop:ident, ty => $ty:ty) => {
        bench_binop_wide!($b, width => $width, op => $binop, ty1 => $ty, ty2 => $ty)
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

// necessary because this macro isn't used in every benchmark
#[allow(unused_macros)]
macro_rules! never_inline_closure {
    (
        |$($arg:ident: $argt:ty),*| $body:tt
     ) => {{
        #[cfg(not(feature = "unstable"))]
        let closure = |$($arg: $argt),*| {
            $body
        };

        #[cfg(feature = "unstable")]
        let closure = #[inline(never)] |$($arg: $argt),*| {
            $body
        };

        closure
    }};
}
