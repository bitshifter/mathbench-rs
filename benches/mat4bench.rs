#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have a 4x4 matrix, it has Transform3D which is a
// stored as 4x4 matrix internally. It is included here as a 4x4 matrix is the
// closest point of comparison.

fn bench_mat4_transpose(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 transpose",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_unop!(b, op => transpose, ty => Mat4);
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix4};
            bench_unop!(b, op => transpose, ty => Matrix4<f32>);
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix4;
            bench_unop!(b, op => transpose, ty => Matrix4<f32>);
        })
        .with_function("hektor", |b| {
            use hektor::Mat4;
            bench_unop!(b, op => transpose, ty => Mat4);
        }),
    );
}

fn bench_mat4_determinant(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 determinant",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_unop!(b, op => determinant, ty => Mat4)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, SquareMatrix};
            bench_unop!(b, op => determinant, ty => Matrix4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix4;
            bench_unop!(b, op => determinant, ty => Matrix4<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat4;
            bench_unop!(b, op => determinant, ty => Mat4)
        .with_function("euclid", |b| {
            use euclid::{Transform3D, UnknownUnit};
            bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
        }),
    );
}

fn bench_mat4_inverse(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 inverse",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_unop!(b, op => inverse, ty => Mat4)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, SquareMatrix};
            bench_unop!(b, op => invert, ty => Matrix4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix4;
            bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat4;
            bench_unop!(b, op => inverse, ty => Mat4)
        .with_function("euclid", |b| {
            use euclid::{Transform3D, UnknownUnit};
            bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
        }),
    );
}

fn bench_mat4_mul_mat4(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat4 mul mat4",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
        })
        .with_function("cgmath", |b| {
            use cgmath::Matrix4;
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix4;
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat4;
            bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
        }),
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform3D, UnknownUnit};
            bench_binop!(b, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
        })
    );
}

criterion_group!(
    mat4_benches,
    bench_mat4_transpose,
    bench_mat4_determinant,
    bench_mat4_inverse,
    bench_mat4_mul_mat4,
);
criterion_main!(mat4_benches);
