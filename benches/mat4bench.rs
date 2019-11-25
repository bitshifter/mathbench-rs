#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have a 4x4 matrix, it has Transform3D which is a
// stored as 4x4 matrix internally. It is included here as a 4x4 matrix is the
// closest point of comparison.

// returns self to check overhead of benchmark
fn bench_mat4_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("mat4 return self");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => nop_fn, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_unop!(b, op => nop_fn, ty => Matrix4<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => nop_fn, ty => Matrix4<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => nop_fn, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_mat4_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transpose");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => transpose, ty => Mat4);
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix4};
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_mat4_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 determinant");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_mat4_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 inverse");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => inverse, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => invert, ty => Matrix4<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => inverted, ty => Mat4<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_unop!(b, op => inverse, ty => Transform4F)
    });
    group.finish();
}

fn bench_mat4_mul_mat4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat4 mul mat4");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4<f32>, ty2 => Mat4<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_binop!(b, op => mul, ty1 => Transform4F, ty2 => Transform4F)
    });
    group.finish();
}

criterion_group!(
    mat4_benches,
    bench_mat4_nop,
    bench_mat4_transpose,
    bench_mat4_determinant,
    bench_mat4_inverse,
    bench_mat4_mul_mat4,
);
criterion_main!(mat4_benches);
