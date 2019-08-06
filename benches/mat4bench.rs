#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

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
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix4;
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat4;
            bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
        }),
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
