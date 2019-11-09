#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mat2_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat2 transpose");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => transpose, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => transpose, ty => Matrix2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => transpose, ty => Matrix2<f32>)
    });
    group.finish();
}

fn bench_mat2_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat2 determinant");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => determinant, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => determinant, ty => Matrix2<f32>)
    });
    group.finish();
}

fn bench_mat2_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat2 inverse");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_unop!(b, op => inverse, ty => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix2};
        bench_unop!(b, op => invert, ty => Matrix2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_unop!(b, op => try_inverse, ty => Matrix2<f32>)
    });
    group.finish();
}

fn bench_mat2_mul_mat2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat2 mul mat2");
    bench_glam!(group, |b| {
        use glam::Mat2;
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Mat2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix2;
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Matrix2<f32>)
    });
    group.finish();
}

criterion_group!(
    mat2_benches,
    bench_mat2_transpose,
    bench_mat2_determinant,
    bench_mat2_inverse,
    bench_mat2_mul_mat2,
);
criterion_main!(mat2_benches);
