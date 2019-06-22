#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_vec3_length(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "vec3 length",
        Benchmark::new("glam", |b| {
            use glam::Vec3;
            bench_unop!(b, op => length, ty => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{InnerSpace, Vector3};
            bench_unop!(b, op => magnitude, ty => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Vector3;
            bench_unop!(b, op => magnitude, ty => Vector3<f32>)
        }),
    );
}

fn bench_vec3_normalize(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "vec3 normalize",
        Benchmark::new("glam", |b| {
            use glam::Vec3;
            bench_unop!(b, op => normalize, ty => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{InnerSpace, Vector3};
            bench_unop!(b, op => normalize, ty => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Vector3;
            bench_unop!(b, op => normalize, ty => Vector3<f32>)
        }),
    );
}

fn bench_vec3_dot(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "vec3 dot",
        Benchmark::new("glam", |b| {
            use glam::Vec3;
            use support::glam_vec3_dot;
            bench_func!(b, op => glam_vec3_dot, ty1 => Vec3, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::Vector3;
            use support::cgmath_vec3_dot;
            bench_func!(b, op => cgmath_vec3_dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Vector3;
            bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
        }),
    );
}

fn bench_vec3_cross(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "vec3 cross",
        Benchmark::new("glam", |b| {
            use glam::Vec3;
            use support::glam_vec3_cross;
            bench_func!(b, op => glam_vec3_cross, ty1 => Vec3, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::Vector3;
            use support::cgmath_vec3_cross;
            bench_func!(b, op => cgmath_vec3_cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Vector3;
            bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
        }),
    );
}

criterion_group!(
    vec_benches,
    bench_vec3_length,
    bench_vec3_normalize,
    bench_vec3_dot,
    bench_vec3_cross
);
criterion_main!(vec_benches);
