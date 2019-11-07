#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_quat_conjugate(c: &mut Criterion) {
    let mut group = c.benchmark_group("quat conjugate");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => conjugate, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => conjugate, ty => UnitQuaternion<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        // inverse assume normalized quaternion, so it's just a conjugate
        bench_unop!(b, op => inverse, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>)
    });
    group.finish();
}

fn bench_quat_mul_quat(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quat mul quat");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => UnitQuaternion<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        bench_binop!(b, op => pre_rotate, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    group.finish();
}

criterion_group!(quat_benches, bench_quat_conjugate, bench_quat_mul_quat,);
criterion_main!(quat_benches);
