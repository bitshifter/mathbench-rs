#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_mat4_transform_vec4(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat4 transform vec4",
        Benchmark::new("glam", |b| {
            use glam::{Mat4, Vec4};
            bench_binop!(b, op => transform_mat4, ty1 => Vec4, ty2 => Mat4)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
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
            bench_binop!(b, op => transform_mat3, ty1 => Vec3, ty2 => Mat3)
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
            bench_binop!(b, op => transform_mat2, ty1 => Vec2, ty2 => Mat2)
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

criterion_group!(
    transform_benches,
    bench_mat2_transform_vec2,
    bench_mat3_transform_vec3,
    bench_mat4_transform_vec4,
);
criterion_main!(transform_benches);
