#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mat2_transpose(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat2 transpose",
        Benchmark::new("glam", |b| {
            use glam::Mat2;
            bench_unop!(b, op => transpose, ty => Mat2)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix2};
            bench_unop!(b, op => transpose, ty => Matrix2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix2;
            bench_unop!(b, op => transpose, ty => Matrix2<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat2;
            bench_unop!(b, op => transpose, ty => Mat2)
        }),
    );
}

fn bench_mat2_determinant(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat2 determinant",
        Benchmark::new("glam", |b| {
            use glam::Mat2;
            bench_unop!(b, op => determinant, ty => Mat2)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix2};
            bench_unop!(b, op => determinant, ty => Matrix2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix2;
            bench_unop!(b, op => determinant, ty => Matrix2<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat2;
            bench_unop!(b, op => determinant, ty => Mat2)
        }),
    );
}

fn bench_mat2_inverse(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat2 inverse",
        Benchmark::new("glam", |b| {
            use glam::Mat2;
            bench_unop!(b, op => inverse, ty => Mat2)
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix2};
            bench_unop!(b, op => invert, ty => Matrix2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix2;
            bench_unop!(b, op => try_inverse, ty => Matrix2<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat2;
            bench_unop!(b, op => inverse, ty => Mat2)
        }),
    );
}

fn bench_mat2_mul_mat2(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat2 mul mat2",
        Benchmark::new("glam", |b| {
            use glam::Mat2;
            bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
        })
        .with_function("cgmath", |b| {
            use cgmath::Matrix2;
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::Matrix2;
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
        })
        .with_function("hektor", |b| {
            use hektor::Mat2;
            bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
        }),
    );
}

criterion_group!(
    mat2_benches,
    bench_mat2_transpose,
    bench_mat2_determinant,
    bench_mat2_inverse,
    bench_mat2_mul_mat2,
);
criterion_main!(mat2_benches);
