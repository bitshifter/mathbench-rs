#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_quat_conjugate(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "quat conjugate",
        Benchmark::new("glam", |b| {
            use glam::Quat;
            bench_unop!(b, op => conjugate, ty => Quat)
        })
        .with_function("cgmath", |b| {
            use cgmath::Quaternion;
            bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::UnitQuaternion;
            bench_unop!(b, op => conjugate, ty => UnitQuaternion<f32>)
        }),
    );
}

fn bench_quat_mul_quat(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "quat mul quat",
        Benchmark::new("glam", |b| {
            use glam::Quat;
            bench_binop!(b, op => mul, ty1 => Quat, ty2 => Quat)
        })
        .with_function("cgmath", |b| {
            use cgmath::Quaternion;
            bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::UnitQuaternion;
            bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => UnitQuaternion<f32>)
        }),
    );
}

criterion_group!(quat_benches, bench_quat_conjugate, bench_quat_mul_quat,);
criterion_main!(quat_benches);
