#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};
use macros::MIN_WIDE_BENCH_SIZE;
use mathbench::BenchValue;
use std::ops::Mul;

// returns self to check overhead of benchmark
fn bench_rotation3_nop(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 return self");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => ret_self, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => ret_self, ty => Rotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => ret_self, ty => UnitQuaternion<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::quaternion::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_unop!(b, op => ret_self, ty => Quaternion<f32>)
    });
    group.finish();
}

fn bench_rotation3_nop_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide rotation3 return self");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Quat;
        bench_unop_wide!(b, size, width => 1, op => ret_self, ty => Quat)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Rotor3x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Rotor3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => UnitQuaternion<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Rotor3x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Rotor3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => UnitQuaternion<f32x8>)
    });
    group.finish();
}

fn bench_rotation3_inverse(c: &mut Criterion) {
    // unit quaternion inverse is the conjugate
    let mut group = c.benchmark_group("scalar rotation3 inverse");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_unop!(b, op => conjugate, ty => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_unop!(b, op => conjugate, ty => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_unop!(b, op => reversed, ty => Rotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_unop!(b, op => conjugate, ty => UnitQuaternion<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::quaternion::Quaternion;
        bench_unop!(b, op => conj, ty => Quaternion<f32>)
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

fn bench_rotation3_inverse_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide rotation3 inverse");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Quat;
        bench_unop_wide!(b, size, width => 1, op => conjugate, ty => Quat)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Rotor3x4;
        bench_unop_wide!(b, size, width => 4, op => reversed, ty => Rotor3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => conjugate, ty => UnitQuaternion<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Rotor3x8;
        bench_unop_wide!(b, size, width => 8, op => reversed, ty => Rotor3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => conjugate, ty => UnitQuaternion<f32x8>)
    });
    group.finish();
}

fn bench_rotation3_mul_rotation3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 mul rotation3");
    bench_glam!(group, |b| {
        use glam::Quat;
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Rotor3;
        bench_binop!(b, op => mul, ty1 => Rotor3, ty2 => Rotor3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::UnitQuaternion;
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => UnitQuaternion<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::quaternion::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Rotation3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Rotation3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Quaternion;
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Quaternion<f32>);
    });
    group.finish();
}

fn bench_rotation3_mul_rotation3_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide rotation3 mul rotation3");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Quat;
        bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Quat, ty2 => Quat)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Rotor3x4;
        bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Rotor3x4, ty2 => Rotor3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x4;
        bench_binop_wide!(b, size, width => 4, op => mul, ty1 => UnitQuaternion<f32x4>, ty2 => UnitQuaternion<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Rotor3x8;
        bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Rotor3x8, ty2 => Rotor3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::UnitQuaternion;
        use simba::simd::f32x8;
        bench_binop_wide!(b, size, width => 8, op => mul, ty1 => UnitQuaternion<f32x8>, ty2 => UnitQuaternion<f32x8>)
    });
    group.finish();
}

fn bench_rotation3_mul_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar rotation3 mul vector3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Quat, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Quat, ty2 => Vec3)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Quaternion, Vector3};
            bench_binop!(b, size, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Rotor3, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Rotor3, ty2 => Vec3)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{UnitQuaternion, Vector3};
            bench_binop!(b, size, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
        });
        bench_static_math!(group, size, |b, size| {
            use static_math::quaternion::Quaternion;
            use static_math::vector3::V3;
            bench_binop!(b, size, op => mul, ty1 => Quaternion<f32>, ty2 => V3<f32>)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Point3D, Rotation3D, UnknownUnit};
            bench_binop!(b, size, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Quaternion, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Quaternion<f32>, ty2 => Vec3<f32>)
        });
    }
    group.finish();
}

fn bench_rotation3_mul_vector3_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide rotation3 mul vector3");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::{Quat, Vec3};
        bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Quat, ty2 => Vec3)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::{Rotor3x4, Vec3x4};
        bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Rotor3x4, ty2 => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::{UnitQuaternion, Vector3};
        use simba::simd::f32x4;
        bench_binop_wide!(b, size, width => 4, op => mul, ty1 => UnitQuaternion<f32x4>, ty2 => Vector3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::{Rotor3x8, Vec3x8};
        bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Rotor3x8, ty2 => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::{UnitQuaternion, Vector3};
        use simba::simd::f32x8;
        bench_binop_wide!(b, size, width => 8, op => mul, ty1 => UnitQuaternion<f32x8>, ty2 => Vector3<f32x8>)
    });
    group.finish();
}

criterion_group!(
    rotation3_benches,
    bench_rotation3_nop,
    bench_rotation3_nop_wide,
    bench_rotation3_inverse,
    bench_rotation3_inverse_wide,
    bench_rotation3_mul_rotation3,
    bench_rotation3_mul_rotation3_wide,
    bench_rotation3_mul_vector3,
    bench_rotation3_mul_vector3_wide,
);
criterion_main!(rotation3_benches);
