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
            bench_binop!(b, op => mul, ty1 => Vec4, ty2 => Mat4);
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>);
        })
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{Mat4, Vec4};
            bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Vec4);
        }),
    );
}

criterion_group!(transform_benches, bench_mat4_transform_vec4,);
criterion_main!(transform_benches);
