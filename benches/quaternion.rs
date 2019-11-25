#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_quaternion_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("quaternion return self");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => nop_fn, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => nop_fn, ty => Quaternion<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => nop_fn, ty => UnitQuaternion<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => nop_fn, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_quaternion_conjugate(c: &mut Criterion) {
    let mut group = c.benchmark_group("quaternion conjugate");
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
        // euclid inverse assumes normalized quaternion, so it's just a conjugate
        bench_unop!(b, op => inverse, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_quaternion_mul_quaternion(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quaternion mul quaternion");
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
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>);
    });
    group.finish();
}

fn bench_quaternion_mul_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quaternion mul vector3");
    bench_glam!(group, |b| {
        use glam::{Quat, Vec3};
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Quaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Point3D, Rotation3D, UnknownUnit};
        bench_binop!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Quaternion, Vec3};
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

criterion_group!(
    quaternion_benches,
    bench_quaternion_nop,
    bench_quaternion_conjugate,
    bench_quaternion_mul_quaternion,
    bench_quaternion_mul_vector3,
);
criterion_main!(quaternion_benches);
