#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have 3x3 matrix, it has Transform2D which is a
// stored as 3x2 matrix internally. It is included here as a 3x3 matrix is the
// closest point of comparison, but euclid can shortcut some things compared to
// a 3x3 matrix, for example it's determinant and inverse are cheaper.

fn bench_mat3_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("mat3 return self");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => nop_fn, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix3;
        bench_unop!(b, op => nop_fn, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => nop_fn, ty => Matrix3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => nop_fn, ty => Mat3<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => nop_fn, ty => Transform2F)
    });
    group.finish();
}

fn bench_mat3_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 transpose");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => transpose, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_mat3_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 determinant");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_mat3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 inverse");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => inverse, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => invert, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => try_inverse, ty => Matrix3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => inverse, ty => Transform2F)
    });
    group.finish();
}

fn bench_mat3_mul_mat3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat3 mul mat3");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform2D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3<f32>, ty2 => Mat3<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        use std::ops::Mul;
        bench_binop!(b, op => mul, ty1 => Transform2F, ty2 => Transform2F)
    });
    group.finish();
}

criterion_group!(
    mat3_benches,
    bench_mat3_nop,
    bench_mat3_transpose,
    bench_mat3_determinant,
    bench_mat3_inverse,
    bench_mat3_mul_mat3,
);
criterion_main!(mat3_benches);
