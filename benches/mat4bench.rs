#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have a 4x4 matrix, it has Transform3D which is a
// stored as 4x4 matrix internally. It is included here as a 4x4 matrix is the
// closest point of comparison.

fn bench_mat4_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transpose");
    group.bench_function("glam", |b| {
        use glam::Mat4;
        bench_unop!(b, op => transpose, ty => Mat4);
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{prelude::*, Matrix4};
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    group.finish();
}

fn bench_mat4_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 determinant");
    group.bench_function("glam", |b| {
        use glam::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    group.finish();
}

fn bench_mat4_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 inverse");
    group.bench_function("glam", |b| {
        use glam::Mat4;
        bench_unop!(b, op => inverse, ty => Mat4)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => invert, ty => Matrix4<f32>)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    group.finish();
}

fn bench_mat4_mul_mat4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat4 mul mat4");
    group.bench_function("glam", |b| {
        use glam::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    group.bench_function("cgmath", |b| {
        use cgmath::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    group.bench_function("nalgebra", |b| {
        use nalgebra::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    group.bench_function("euclid", |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    group.finish();
}

criterion_group!(
    mat4_benches,
    bench_mat4_transpose,
    bench_mat4_determinant,
    bench_mat4_inverse,
    bench_mat4_mul_mat4,
);
criterion_main!(mat4_benches);
