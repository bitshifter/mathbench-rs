#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{Criterion, criterion_group, criterion_main};
use mathbench::BenchValue;
use std::ops::Mul;

// measures benchmark no-op baseline for this type
fn bench_matrix4_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 no-op baseline");
    bench!("glam", group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 transpose");
    bench!("glam", group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => transpose, ty => Mat4);
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix4, prelude::*};
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });

    bench!("vek", group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_transpose_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 transpose");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat4;
        bench_unop_wide!(b, width => 1, op => transpose, ty => Mat4)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, width => 4, op => transposed, ty => Mat4x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => transpose, ty => Matrix4<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, width => 8, op => transposed, ty => Mat4x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => transpose, ty => Matrix4<WideF32x8>)
    });
    group.finish();
}

fn bench_matrix4_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 determinant");
    bench!("glam", group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_determinant_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 determinant");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat4;
        bench_unop_wide!(b, width => 1, op => determinant, ty => Mat4)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, width => 4, op => determinant, ty => Mat4x4)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, width => 8, op => determinant, ty => Mat4x8)
    });
    group.finish();
}

fn bench_matrix4_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 inverse");
    bench!("glam", group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => inverse, ty => Mat4)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => invert, ty => Matrix4<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => inversed, ty => Mat4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => inverted, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_inverse_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 inverse");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat4;
        bench_unop_wide!(b, width => 1, op => inverse, ty => Mat4)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, width => 4, op => inversed, ty => Mat4x4)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, width => 8, op => inversed, ty => Mat4x8)
    });
    group.finish();
}

fn bench_matrix4_mul_matrix4(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 mul matrix4");
    bench!("glam", group, |b| {
        use glam::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });

    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench!("vek", group, |b| {
        use vek::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4<f32>, ty2 => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_mul_matrix4_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 mul matrix4");
    bench!("glam_f32x1", group, |b| {
        use glam::Mat4;
        bench_binop_wide!(b, width => 1, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Mat4x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Mat4x4, ty2 => Mat4x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Matrix4<WideF32x4>, ty2 => Matrix4<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Mat4x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Mat4x8, ty2 => Mat4x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Matrix4;
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Matrix4<WideF32x8>, ty2 => Matrix4<WideF32x8>)
    });
    group.finish();
}

fn bench_matrix4_mul_vector4(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 mul vector4");
    bench!("glam", group, |b| {
        use glam::{Mat4, Vec4};
        bench_binop_broadcast!(b, op => mul, ty1 => Mat4, ty2 => Vec4)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{Matrix4, Vector4};
        bench_binop_broadcast!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::{Mat4, Vec4};
        bench_binop_broadcast!(b, op => mul, ty1 => Mat4, ty2 => Vec4)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::{Matrix4, Vector4};
        bench_binop_broadcast!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });

    bench!("vek", group, |b| {
        use vek::{Mat4, Vec4};
        bench_binop_broadcast!(b, op => mul, ty1 => Mat4<f32>, ty2 => Vec4<f32>)
    });
    group.finish();
}

fn bench_matrix4_mul_vector4_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 mul vector4");
    bench!("glam_f32x1", group, |b| {
        use glam::{Mat4, Vec4};
        bench_binop_wide!(b, width => 1, op => mul, ty1 => Mat4, ty2 => Vec4)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::{Mat4x4, Vec4x4};
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Mat4x4, ty2 => Vec4x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::{Matrix4, Vector4};
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => mul, ty1 => Matrix4<WideF32x4>, ty2 => Vector4<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::{Mat4x8, Vec4x8};
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Mat4x8, ty2 => Vec4x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::{Matrix4, Vector4};
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => mul, ty1 => Matrix4<WideF32x8>, ty2 => Vector4<WideF32x8>)
    });
    group.finish();
}

criterion_group!(
    matrix4_benches,
    bench_matrix4_ret_self,
    bench_matrix4_transpose,
    bench_matrix4_transpose_wide,
    bench_matrix4_determinant,
    bench_matrix4_determinant_wide,
    bench_matrix4_inverse,
    bench_matrix4_inverse_wide,
    bench_matrix4_mul_matrix4,
    bench_matrix4_mul_matrix4_wide,
    bench_matrix4_mul_vector4,
    bench_matrix4_mul_vector4_wide,
);
criterion_main!(matrix4_benches);
