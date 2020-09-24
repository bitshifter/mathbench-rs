#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};
use macros::MIN_WIDE_BENCH_SIZE;
use mathbench::BenchValue;

// returns self to check overhead of benchmark
fn bench_vector3_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 return self");
    bench_glam!(group, |b| {
        use glam::Vec3A;
        bench_unop!(b, op => ret_self, ty => Vec3A)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::vector3::V3;
        bench_unop!(b, op => ret_self, ty => V3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_ret_self_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide vector3 return self");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Vec3A;
        bench_unop_wide!(b, size, width => 1, op => ret_self, ty => Vec3A)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Vec3x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Vector3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Vec3x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Vector3<f32x8>)
    });
    group.finish();
}

fn bench_vector3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 length");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => length, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => mag, ty => Vec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::vector3::V3;
        bench_unop!(b, op => norm2, ty => V3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => length, ty => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => magnitude, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_length_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide vector3 length");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Vec3A;
        bench_unop_wide!(b, size, width => 1, op => length, ty => Vec3A)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Vec3x4;
        bench_unop_wide!(b, size, width => 4, op => mag, ty => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => norm, ty => Vector3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Vec3x8;
        bench_unop_wide!(b, size, width => 8, op => mag, ty => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => norm, ty => Vector3<f32x8>)
    });
    group.finish();
}

fn bench_vector3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 normalize");
    bench_glam!(group, |b| {
        use glam::Vec3A;
        bench_unop!(b, op => normalize, ty => Vec3A)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => normalize, ty => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_normalize_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide vector3 normalize");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Vec3A;
        bench_unop_wide!(b, size, width => 1, op => normalize, ty => Vec3A)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Vec3x4;
        bench_unop_wide!(b, size, width => 4, op => normalized, ty => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => normalize, ty => Vector3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Vec3x8;
        bench_unop_wide!(b, size, width => 8, op => normalized, ty => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => normalize, ty => Vector3<f32x8>)
    });
    group.finish();
}

fn bench_vector3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 dot");
    bench_glam!(group, |b| {
        use glam::Vec3A;
        bench_binop!(b, op => dot, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    bench_static_math!(group, |b| {
        use static_math::vector3::V3;
        use std::ops::Mul;
        bench_binop!(b, op => mul, ty1 => V3<f32>, ty2 => V3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => dot, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_dot_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide vector3 dot");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Vec3A;
        bench_binop_wide!(b, size, width => 1, op => dot, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Vec3x4;
        bench_binop_wide!(b, size, width => 4, op => dot, ty1 => Vec3x4, ty2 => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_binop_wide!(b, size, width => 4, op => dot, ty1 => Vector3<f32x4>, ty2 => Vector3<f32x4>, param => by_ref)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Vec3x8;
        bench_binop_wide!(b, size, width => 8, op => dot, ty1 => Vec3x8, ty2 => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_binop_wide!(b, size, width => 8, op => dot, ty1 => Vector3<f32x8>, ty2 => Vector3<f32x8>, param => by_ref)
    });
    group.finish();
}

fn bench_vector3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 cross");
    bench_glam!(group, |b| {
        use glam::Vec3A;
        bench_binop!(b, op => cross, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3, ty2 => Vec3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    bench_static_math!(group, |b| {
        use static_math::vector3::V3;
        bench_binop!(b, op => cross, ty1 => V3<f32>, ty2 => V3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => cross, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_cross_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide vector3 cross");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Vec3A;
        bench_binop_wide!(b, size, width => 1, op => cross, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Vec3x4;
        bench_binop_wide!(b, size, width => 4, op => cross, ty1 => Vec3x4, ty2 => Vec3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x4;
        bench_binop_wide!(b, size, width => 4, op => cross, ty1 => Vector3<f32x4>, ty2 => Vector3<f32x4>, param => by_ref)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Vec3x8;
        bench_binop_wide!(b, size, width => 8, op => cross, ty1 => Vec3x8, ty2 => Vec3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Vector3;
        use simba::simd::f32x8;
        bench_binop_wide!(b, size, width => 8, op => cross, ty1 => Vector3<f32x8>, ty2 => Vector3<f32x8>, param => by_ref)
    });
    group.finish();
}

criterion_group!(
    vector3_benches,
    bench_vector3_ret_self,
    bench_vector3_ret_self_wide,
    bench_vector3_length,
    bench_vector3_length_wide,
    bench_vector3_normalize,
    bench_vector3_normalize_wide,
    bench_vector3_dot,
    bench_vector3_dot_wide,
    bench_vector3_cross,
    bench_vector3_cross_wide,
);
criterion_main!(vector3_benches);
