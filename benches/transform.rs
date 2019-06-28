#[path = "support/macros.rs"]
#[macro_use]
mod macros;
mod support;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mat4_transform_vec4(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat4 transform vec4",
        Benchmark::new("glam", |b| {
            use glam::{Mat4, Vec4};
            use support::glam_mat4_mul_vec4;
            bench_func!(b, op => glam_mat4_mul_vec4, ty1 => Mat4, ty2 => Vec4)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        })
        .with_function("vek", |b| {
            use vek::mat::repr_simd::column_major::Mat4;
            use vek::vec::repr_simd::Vec4;
            use support::vek_mat4_mul_vec4;
            bench_func!(b, op => vek_mat4_mul_vec4, ty1 => Mat4<f32>, ty2 => Vec4<f32>)
        }),
    );
}

fn bench_mat3_transform_vec3(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat3 transform vec3",
        Benchmark::new("glam", |b| {
            use glam::{Mat3, Vec3};
            use support::glam_mat3_mul_vec3;
            bench_func!(b, op => glam_mat3_mul_vec3, ty1 => Mat3, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix3, Vector3};
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix3, Vector3};
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        }),
    );
}

fn bench_mat2_transform_vec2(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat2 transform vec2",
        Benchmark::new("glam", |b| {
            use glam::{Mat2, Vec2};
            use support::glam_mat2_mul_vec2;
            bench_func!(b, op => glam_mat2_mul_vec2, ty1 => Mat2, ty2 => Vec2)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix2, Vector2};
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix2, Vector2};
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        }),
    );
}

fn bench_quat_transform_vec3(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "quat transform vec3",
        Benchmark::new("glam", |b| {
            use glam::{Quat, Vec3};
            use support::glam_quat_mul_vec3;
            bench_func!(b, op => glam_quat_mul_vec3, ty1 => Quat, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Quaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
        }),
    );
}

criterion_group!(
    transform_benches,
    bench_mat2_transform_vec2,
    bench_mat3_transform_vec3,
    bench_mat4_transform_vec4,
    bench_quat_transform_vec3,
);
criterion_main!(transform_benches);
