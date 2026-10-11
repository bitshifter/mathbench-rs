#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{Criterion, criterion_group, criterion_main};
use mathbench::BenchValue;
use std::ops::Mul;

// measures benchmark no-op baseline for this type
fn bench_rotation3_nop(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 no-op baseline");
    bench!("glam", group, |b| {
        use glam::Quat;
        bench_unop!(b, op => ret_self, ty => Quat)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => ret_self, ty => Rotor3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => ret_self, ty => UnitQuaternion<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_rotation3_inverse(c: &mut Criterion) {
    // unit quaternion inverse is the conjugate
    let mut group = c.benchmark_group("scalar rotation3 inverse");
    bench!("glam", group, |b| {
        use glam::Quat;
        bench_unop!(b, op => conjugate, ty => Quat)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => reversed, ty => Rotor3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => conjugate, ty => UnitQuaternion<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        // euclid inverse assumes normalized quaternion, so it's just a conjugate
        bench_unop!(b, op => inverse, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_rotation3_inverse_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide rotation3 inverse");
    bench!("glam_f32x1", group, |b| {
        use glam::Quat;
        bench_unop_wide!(b, width => 1, op => conjugate, ty => Quat)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Rotor3x4;
        bench_unop_wide!(b, width => 4, op => reversed, ty => Rotor3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => conjugate, ty => UnitQuaternion<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Rotor3x8;
        bench_unop_wide!(b, width => 8, op => reversed, ty => Rotor3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => conjugate, ty => UnitQuaternion<WideF32x8>)
    });
    group.finish();
}

fn bench_rotation3_mul_rotation3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 mul rotation3");
    bench!("glam", group, |b| {
        use glam::Quat;
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Rotor3;
        bench_binop!(b, op => mul, ty1 => Rotor3, ty2 => Rotor3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::UnitQuaternion;
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => UnitQuaternion<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench!("vek", group, |b| {
        use vek::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>);
    });
    group.finish();
}

fn bench_rotation3_mul_rotation3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide rotation3 mul rotation3");
    bench!("glam_f32x1", group, |b| {
        use glam::Quat;
        bench_binop_wide!(b, width => 1, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Rotor3x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Rotor3x4, ty2 => Rotor3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => UnitQuaternion<WideF32x4>, ty2 => UnitQuaternion<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Rotor3x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Rotor3x8, ty2 => Rotor3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::UnitQuaternion;
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => UnitQuaternion<WideF32x8>, ty2 => UnitQuaternion<WideF32x8>)
    });
    group.finish();
}

fn bench_rotation3_mul_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 mul vector3");
    bench!("glam", group, |b| {
        use glam::{Quat, Vec3A};
        bench_binop_broadcast!(b, op => mul, ty1 => Quat, ty2 => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Quaternion, Vector3};
        bench_binop_broadcast!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::{Rotor3, Vec3};
        bench_binop_broadcast!(b, op => mul, ty1 => Rotor3, ty2 => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        bench_binop_broadcast!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{Point3D, Rotation3D, UnknownUnit};
        bench_binop_broadcast!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::{Quaternion, Vec3};
        bench_binop_broadcast!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_rotation3_mul_vector3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide rotation3 mul vector3");
    bench!("glam_f32x1", group, |b| {
        use glam::{Quat, Vec3A};
        bench_binop_wide_broadcast!(b, width => 1, op => mul, ty1 => Quat, ty2 => Vec3A)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::{Rotor3x4, Vec3x4};
        bench_binop_wide_broadcast!(b, width => 4, op => mul, ty1 => Rotor3x4, ty2 => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        use simba::simd::WideF32x4;
        bench_binop_wide_broadcast!(b, width => 4, op => mul, ty1 => UnitQuaternion<WideF32x4>, ty2 => Vector3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::{Rotor3x8, Vec3x8};
        bench_binop_wide_broadcast!(b, width => 8, op => mul, ty1 => Rotor3x8, ty2 => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        use simba::simd::WideF32x8;
        bench_binop_wide_broadcast!(b, width => 8, op => mul, ty1 => UnitQuaternion<WideF32x8>, ty2 => Vector3<WideF32x8>)
    });
    group.finish();
}

criterion_group!(
    rotation3_benches,
    bench_rotation3_nop,
    bench_rotation3_inverse,
    bench_rotation3_inverse_wide,
    bench_rotation3_mul_rotation3,
    bench_rotation3_mul_rotation3_wide,
    bench_rotation3_mul_vector3,
    bench_rotation3_mul_vector3_wide,
);
criterion_main!(rotation3_benches);
