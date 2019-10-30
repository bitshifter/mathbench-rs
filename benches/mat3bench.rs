#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have 3x3 matrix, it has Transform2D which is a
// stored as 3x2 matrix internally. It is included here as a 3x3 matrix is the
// closest point of comparison, but euclid can shortcut some things compared to
// a 3x3 matrix, for example it's determinant and inverse are cheaper.

fn bench_mat3_transpose(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat3 transpose",
        Benchmark::new("glam", |b| {
            use glam::Mat3;
            bench_unop!(b, op => transpose, ty => Mat3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix3};
            bench_unop!(b, op => transpose, ty => Matrix3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix3;
            bench_unop!(b, op => transpose, ty => Matrix3<f32>)
        }),
    );
}

fn bench_mat3_determinant(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat3 determinant",
        Benchmark::new("glam", |b| {
            use glam::Mat3;
            bench_unop!(b, op => determinant, ty => Mat3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix3};
            bench_unop!(b, op => determinant, ty => Matrix3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix3;
            bench_unop!(b, op => determinant, ty => Matrix3<f32>)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform2D, UnknownUnit};
            bench_unop!(b, op => determinant, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
        }),
    );
}

fn bench_mat3_inverse(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat3 inverse",
        Benchmark::new("glam", |b| {
            use glam::Mat3;
            bench_unop!(b, op => inverse, ty => Mat3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix3};
            bench_unop!(b, op => invert, ty => Matrix3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix3;
            bench_unop!(b, op => try_inverse, ty => Matrix3<f32>)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform2D, UnknownUnit};
            bench_unop!(b, op => inverse, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
        }),
    );
}

fn bench_mat3_mul_mat3(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat3 mul mat3",
        Benchmark::new("glam", |b| {
            use glam::Mat3;
            bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
        })
        .with_function("cgmath", |b| {
            use cgmath::Matrix3;
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix3;
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform2D, UnknownUnit};
            bench_binop!(b, op => post_transform, ty => Transform2D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
        })
    );
}

criterion_group!(
    mat3_benches,
    bench_mat3_transpose,
    bench_mat3_determinant,
    bench_mat3_inverse,
    bench_mat3_mul_mat3,
);
criterion_main!(mat3_benches);
