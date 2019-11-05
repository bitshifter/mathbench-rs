#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_vec3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 length");
    group.bench_function("glam", |b| {
        use glam::Vec3;
        bench_unop!(b, op => length, ty => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    group.bench_function("euclid", |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => length, ty => Vector3D<f32, UnknownUnit>)
    });
    group.finish();
}

fn bench_vec3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 normalize");
    group.bench_function("glam", |b| {
        use glam::Vec3;
        bench_unop!(b, op => normalize, ty => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    group.bench_function("euclid", |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => normalize, ty => Vector3D<f32, UnknownUnit>)
    });
    group.finish();
}

fn bench_vec3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 dot");
    group.bench_function("glam", |b| {
        use glam::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Vector3D, UnknownUnit};
        bench_binop!(b, op => dot, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    group.finish();
}

fn bench_vec3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 cross");
    group.bench_function("glam", |b| {
        use glam::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3, ty2 => Vec3)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Vector3D, UnknownUnit};
        bench_binop!(b, op => cross, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    group.finish();
}

criterion_group!(
    vec_benches,
    bench_vec3_length,
    bench_vec3_normalize,
    bench_vec3_dot,
    bench_vec3_cross
);
criterion_main!(vec_benches);
