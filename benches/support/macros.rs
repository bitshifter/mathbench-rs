#[macro_export]
macro_rules! bench_glam {
    ($group:ident, $closure:expr) => {
        // #[cfg(feature = "glam")]
        $group.bench_function("glam", $closure)
    };
}

#[macro_export]
macro_rules! bench_cgmath {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "cgmath")]
        $group.bench_function("cgmath", $closure)
    };
}

#[macro_export]
macro_rules! bench_euclid {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "euclid")]
        $group.bench_function("euclid", $closure)
    };
}

#[macro_export]
macro_rules! bench_nalgebra {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "nalgebra")]
        $group.bench_function("nalgebra", $closure)
    };
}

#[macro_export]
macro_rules! bench_vek {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "vek")]
        $group.bench_function("vek", $closure)
    };
}

#[macro_export]
macro_rules! bench_pathfinder {
    ($group:ident, $closure:expr) => {
        #[cfg(feature = "pathfinder_geometry")]
        $group.bench_function("pathfinder", $closure)
    };
}

#[macro_export]
macro_rules! bench_unop {
    ($b: ident, op => $unop: ident, ty => $t:ty) => {{
        let mut rng = rand::thread_rng();
        $b.iter_batched(
            || <$t as mathbench::RandomValue>::random_value(&mut rng),
            |data| data.$unop(),
            criterion::BatchSize::SmallInput,
        )
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
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty, param => $param:tt) => {{
        let mut rng = rand::thread_rng();
        $b.iter_batched(
            || (<$ty1 as mathbench::RandomValue>::random_value(&mut rng), <$ty2 as mathbench::RandomValue>::random_value(&mut rng)),
            |data| (data.0).$binop($param!(&data.1)),
            criterion::BatchSize::SmallInput)
    }};
    ($b: ident, op => $binop: ident, ty1 => $ty1:ty, ty2 => $ty2:ty) => {{
        bench_binop!($b, op => $binop, ty1 => $ty1, ty2 => $ty2, param => by_value)
    }};
    ($b: ident, op => $binop: ident, ty => $ty:ty, param => $param:tt) => {{
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty, param => $param)
    }};
    ($b: ident, op => $binop: ident, ty => $ty:ty) => {{
        bench_binop!($b, op => $binop, ty1 => $ty, ty2 => $ty)
    }};
}
