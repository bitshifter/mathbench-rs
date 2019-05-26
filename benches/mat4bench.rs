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
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{transpose, Mat4};
            bench_func!(b, op => transpose, ty => Mat4);
        }),
    );
}

fn bench_mat4_determinant(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 determinant",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_unop!(b, op => determinant, ty => Mat4);
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix4};
            bench_unop!(b, op => determinant, ty => Matrix4<f32>);
        })
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{determinant, Mat4};
            bench_func!(b, op => determinant, ty => Mat4);
        }),
    );
}

fn bench_mat4_inverse(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 inverse",
        Benchmark::new("glam", |b| {
            use glam::Mat4;
            bench_unop!(b, op => inverse, ty => Mat4);
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Matrix4};
            bench_unop!(b, op => invert, ty => Matrix4<f32>);
        })
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{inverse, Mat4};
            bench_func!(b, op => inverse, ty => Mat4);
        }),
    );
}

criterion_group!(
    mat4_benches,
    bench_mat4_transpose,
    bench_mat4_determinant,
    bench_mat4_inverse,
);
criterion_main!(mat4_benches);
