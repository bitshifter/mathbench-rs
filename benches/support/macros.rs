#[allow(dead_code)]
pub const MIN_WIDE_BENCH_SIZE: u64 = 16;

#[macro_export]
macro_rules! bench_lib {
    ($libname:literal, $group:ident, $size:expr, $closure:expr) => {
        #[cfg(feature = $libname)]
        $group.bench_with_input(
            criterion::BenchmarkId::new($libname, $size),
            $size,
            $closure,
        )
    };
    ($libname:literal, $group:ident, $closure:expr) => {
        #[cfg(feature = $libname)]
        $group.bench_function($libname, $closure)
    };
}

#[macro_export]
macro_rules! bench_glam {
    ($group:ident, $closure:expr) => {
        // bench_lib!("glam", $group, $closure)
        $group.bench_function("glam", $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        // bench_lib!("glam", $group, $size, $closure)
        $group.bench_with_input(criterion::BenchmarkId::new("glam", $size), $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_glam_f32x1 {
    ($group:ident, $closure:expr) => {
        $group.bench_function("glam_f32x1", $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        $group.bench_with_input(criterion::BenchmarkId::new("glam_f32x1", $size), $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_cgmath {
    ($group:ident, $closure:expr) => {
        bench_lib!("cgmath", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("cgmath", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet_f32x4 {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet_f32x4", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet_f32x4", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet_f32x8 {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet_f32x8", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet_f32x8", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet_f64 {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet_f64", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet_f64", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet_f64x2 {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet_f64x2", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet_f64x2", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_ultraviolet_f64x4 {
    ($group:ident, $closure:expr) => {
        bench_lib!("ultraviolet_f64x4", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("ultraviolet_f64x4", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x4 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x4", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x4", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x8 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x8", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x8", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f32x16 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f32x16", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f32x16", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f64 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f64", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f64", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f64x2 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f64x2", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f64x2", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f64x4 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f64x4", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f64x4", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra_f64x8 {
    ($group:ident, $closure:expr) => {
        bench_lib!("nalgebra_f64x8", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("nalgebra_f64x8", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_euclid {
    ($group:ident, $closure:expr) => {
        bench_lib!("euclid", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("euclid", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_vek {
    ($group:ident, $closure:expr) => {
        bench_lib!("vek", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("vek", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_static_math {
    ($group:ident, $closure:expr) => {
        bench_lib!("static-math", $group, $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        bench_lib!("static-math", $group, $size, $closure)
    };
}

#[macro_export]
macro_rules! bench_pathfinder {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "pathfinder_geometry")]
        $group.bench_function("pathfinder", $closure)
    };
    ($group:ident, $size:expr, $closure:expr) => {
        #[cfg(feature = "pathfinder_geometry")]
        $group.bench_with_input(
            criterion::BenchmarkId::new("pathfinder", $size),
            $size,
            $closure,
        )
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        let size: usize = 1 << 13;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..size)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); size];
        let mut i = 0;
        $b.iter(|| {
            i = (i + 1) & (size - 1);
            let res = unsafe { inputs.get_unchecked(i).$unop() };
            unsafe { *outputs.get_unchecked_mut(i) = res }
            res
        });
        criterion::black_box(outputs);
    }};
}

#[macro_export]
macro_rules! bench_unop_wide {
    ($b: ident, $size: expr, width => $width: expr, op => $unop: ident, ty => $t:ty) => {{
        const SIZE: usize = 1 << 13;
        let size = *$size as f32;
        let batch_size = (size / $width as f32).ceil() as usize;
        let total_size = SIZE * batch_size;

        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let inputs = criterion::black_box(
            (0..total_size)
                .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t as mathbench::BenchValue>::random_value(&mut rng).$unop(); total_size];
        let mut i = 0;
        $b.iter(|| {
            // minimise overhead of accessing random data using get unchecked
            i = (i + 1) & (SIZE - 1);
            let start = i * batch_size;
            let end = start + batch_size;
            for j in start..end {
                let res = unsafe { inputs.get_unchecked(j).$unop() };
                criterion::black_box(res);
                unsafe { *outputs.get_unchecked_mut(j) = res; }
            }
        });
        criterion::black_box(outputs);
    }};
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

#[macro_export]
macro_rules! bench_binop {
    ($b: ident, $size:expr, op => $binop: ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        const SIZE: usize = 1 << 13;
        let batch_size = SIZE * $size;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        // generate input arrays
        let inputs1 = criterion::black_box(
            (0..batch_size)
                .map(|_| <$t1 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        let inputs2 = criterion::black_box(
            (0..batch_size)
                .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t1 as mathbench::BenchValue>::random_value(&mut rng).$binop($param!(&<$t2 as mathbench::BenchValue>::random_value(&mut rng))); batch_size];
        let mut i = 0;
        if *$size == 1usize {
            $b.iter(|| {
                // minimise overhead of accessing random data using get unchecked
                i = (i + 1) & (SIZE - 1);
                unsafe {
                    *outputs.get_unchecked_mut(i) = inputs1.get_unchecked(i).$binop($param!(inputs2.get_unchecked(i)))
                }
            })
        } else {
            $b.iter(|| {
                // minimise overhead of accessing random data using get unchecked
                i = (i + 1) & (SIZE - 1);
                let start = i * $size;
                let end = start + $size;
                for j in start..end {
                    unsafe {
                        *outputs.get_unchecked_mut(j) = inputs1.get_unchecked(j).$binop($param!(inputs2.get_unchecked(j)))
                    }
                }
            })
        }
    }};
    ($b: ident, op => $binop: ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        bench_binop!($b, &1, op => $binop, ty1 => $t1, ty2 => $t2, param => $param)
    }};
    ($b: ident, $size: expr, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, $size, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, $size:expr, op => $binop: ident, ty => $ty:ty, param => $param:tt) => {{
        bench_binop!($b, $size, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    }};
    ($b: ident, op => $binop: ident, ty => $ty:ty, param => $param:tt) => {{
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    }};
}

#[macro_export]
macro_rules! bench_binop_wide {
    ($b: ident, $size:expr, width => $width: expr, op => $binop: ident, ty1 => $t1:ty, ty2 => $t2:ty, param => $param:tt) => {{
        assert!(*$size >= 16);
        let size = *$size as usize;

        let batch_size = (size as f32 / $width as f32).ceil() as usize;
        const SIZE: usize = 1 << 13;
        let total_size = SIZE * size;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        // generate input arrays
        let inputs1 = criterion::black_box(
            (0..total_size)
                .map(|_| <$t1 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        let inputs2 = criterion::black_box(
            (0..total_size)
                .map(|_| <$t2 as mathbench::BenchValue>::random_value(&mut rng))
                .collect::<Vec<_>>(),
        );
        // pre-fill output vector with some random value
        let mut outputs = vec![<$t1 as mathbench::BenchValue>::random_value(&mut rng).$binop($param!(&<$t2 as mathbench::BenchValue>::random_value(&mut rng))); total_size];
        let mut i = 0;
        $b.iter(|| {
            // minimise overhead of accessing random data using get unchecked
            i = (i + 1) & (SIZE - 1);
            let start = i * batch_size;
            let end = start + batch_size;
            for j in start..end {
                let res = unsafe { inputs1.get_unchecked(j).$binop($param!(inputs2.get_unchecked(j))) };
                criterion::black_box(res);
                unsafe { *outputs.get_unchecked_mut(j) = res; }
            }
        })
    }};
    ($b: ident, $size:expr, width => $width: expr, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop_wide!($b, $size, width => $width, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
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
