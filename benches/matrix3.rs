#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{Criterion, criterion_group, criterion_main};
use mathbench::BenchValue;
use std::ops::Mul;

fn bench_matrix3_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 harness overhead");
    bench!("glam", group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_ret_self_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 harness overhead");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat3;
        bench_unop_wide!(b, width => 1, op => ret_self, ty => Mat3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, width => 4, op => ret_self, ty => Mat3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => ret_self, ty => Matrix3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, width => 8, op => ret_self, ty => Mat3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => ret_self, ty => Matrix3<WideF32x8>)
    });
    group.finish();
}

fn bench_matrix3_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 transpose");
    bench!("glam", group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => transpose, ty => Mat3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix3, prelude::*};
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_transpose_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 transpose");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat3;
        bench_unop_wide!(b, width => 1, op => transpose, ty => Mat3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, width => 4, op => transposed, ty => Mat3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => transpose, ty => Matrix3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, width => 8, op => transposed, ty => Mat3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => transpose, ty => Matrix3<WideF32x8>)
    });
    group.finish();
}

fn bench_matrix3_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 determinant");
    bench!("glam", group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix3, prelude::*};
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_determinant_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 determinant");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat3;
        bench_unop_wide!(b, width => 1, op => determinant, ty => Mat3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, width => 4, op => determinant, ty => Mat3x4)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, width => 8, op => determinant, ty => Mat3x8)
    });
    group.finish();
}

fn bench_matrix3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 inverse");
    bench!("glam", group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => inverse, ty => Mat3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix3, prelude::*};
        bench_unop!(b, op => invert, ty => Matrix3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => inversed, ty => Mat3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => try_inverse, ty => Matrix3<f32>)
    });

    group.finish();
}

fn bench_matrix3_inverse_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 inverse");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat3;
        bench_unop_wide!(b, width => 1, op => inverse, ty => Mat3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, width => 4, op => inversed, ty => Mat3x4)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, width => 8, op => inversed, ty => Mat3x8)
    });
    group.finish();
}

fn bench_matrix3_mul_matrix3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 mul matrix3");
    bench!("glam", group, |b| {
        use glam::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });

    bench!("vek", group, |b| {
        use vek::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3<f32>, ty2 => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_mul_matrix3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 mul matrix3");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat3;
        bench_binop_wide!(b, width => 1, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat3x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Mat3x4, ty2 => Mat3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Matrix3<WideF32x4>, ty2 => Matrix3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat3x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Mat3x8, ty2 => Mat3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Matrix3;
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Matrix3<WideF32x8>, ty2 => Matrix3<WideF32x8>)
    });
    group.finish();
}

fn bench_matrix3_mul_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 mul vector3");
    bench!("glam", group, |b| {
        use glam::{Mat3, Vec3};
        bench_binop!(b, sizes => [1, 100], op => mul, ty1 => Mat3, ty2 => Vec3)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix3, Vector3};
        bench_binop!(b, sizes => [1, 100], op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::{Mat3, Vec3};
        bench_binop!(b, sizes => [1, 100], op => mul, ty1 => Mat3, ty2 => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::{Matrix3, Vector3};
        bench_binop!(b, sizes => [1, 100], op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });

    bench!("vek", group, |b| {
        use vek::{Mat3, Vec3};
        bench_binop!(b, sizes => [1, 100], op => mul, ty1 => Mat3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_matrix3_mul_vector3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 mul vector3");
    bench!("glam_f32x1", group, |b| {
        use glam::{Mat3, Vec3};
        bench_binop_wide!(b, width => 1, op => mul, ty1 => Mat3, ty2 => Vec3)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::{Mat3x4, Vec3x4};
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Mat3x4, ty2 => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::{Matrix3, Vector3};
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Matrix3<WideF32x4>, ty2 => Vector3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::{Mat3x8, Vec3x8};
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Mat3x8, ty2 => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::{Matrix3, Vector3};
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Matrix3<WideF32x8>, ty2 => Vector3<WideF32x8>)
    });
    group.finish();
}

criterion_group!(
    matrix3_benches,
    bench_matrix3_ret_self,
    bench_matrix3_ret_self_wide,
    bench_matrix3_transpose,
    bench_matrix3_transpose_wide,
    bench_matrix3_determinant,
    bench_matrix3_determinant_wide,
    bench_matrix3_inverse,
    bench_matrix3_inverse_wide,
    bench_matrix3_mul_matrix3,
    bench_matrix3_mul_matrix3_wide,
    bench_matrix3_mul_vector3,
    bench_matrix3_mul_vector3_wide,
);
criterion_main!(matrix3_benches);
