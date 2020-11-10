#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};
use macros::MIN_WIDE_BENCH_SIZE;
use mathbench::BenchValue;
use std::ops::Mul;

// returns self to check overhead of benchmark
fn bench_matrix4_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 return self");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => ret_self, ty => Matrix4<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix4x4::M44;
        bench_unop!(b, op => ret_self, ty => M44<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => ret_self, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_ret_self_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix4 return self");
    group.throughput(criterion::Throughput::Elements(*size as u64));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat4;
        bench_unop_wide!(b, size, width => 1, op => ret_self, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Mat4x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Matrix4;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Matrix4<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Mat4x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Matrix4;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Matrix4<f32x8>)
    });
    group.finish();
}

fn bench_matrix4_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 transpose");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => transpose, ty => Mat4);
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix4};
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => transpose, ty => Matrix4<f32>);
    });
    bench_static_math!(group, |b| {
        use static_math::matrix4x4::M44;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => transpose, ty => M44<f32>);
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => transposed, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_transpose_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix4 transpose");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat4;
        bench_unop_wide!(b, size, width => 1, op => transpose, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, size, width => 4, op => transposed, ty => Mat4x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Matrix4;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => transpose, ty => Matrix4<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, size, width => 8, op => transposed, ty => Mat4x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Matrix4;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => transpose, ty => Matrix4<f32x8>)
    });
    group.finish();
}

fn bench_matrix4_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 determinant");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => determinant, ty => Matrix4<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix4x4::M44;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => det, ty => M44<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => determinant, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => determinant, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_determinant_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix4 determinant");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat4;
        bench_unop_wide!(b, size, width => 1, op => determinant, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, size, width => 4, op => determinant, ty => Mat4x4)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, size, width => 8, op => determinant, ty => Mat4x8)
    });
    group.finish();
}

fn bench_matrix4_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 inverse");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_unop!(b, op => inverse, ty => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, SquareMatrix};
        bench_unop!(b, op => invert, ty => Matrix4<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_unop!(b, op => inversed, ty => Mat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_unop!(b, op => try_inverse, ty => Matrix4<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix4x4::M44;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => inverse, ty => M44<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_unop!(b, op => inverted, ty => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_inverse_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 inverse");
    let size = &MIN_WIDE_BENCH_SIZE;
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat4;
        bench_unop_wide!(b, size, width => 1, op => inverse, ty => Mat4)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat4x4;
        bench_unop_wide!(b, size, width => 4, op => inversed, ty => Mat4x4)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat4x8;
        bench_unop_wide!(b, size, width => 8, op => inversed, ty => Mat4x8)
    });
    group.finish();
}

fn bench_matrix4_mul_matrix4(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 mul matrix4");
    bench_glam!(group, |b| {
        use glam::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Mat4)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix4;
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Matrix4<f32>, param => by_ref)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix4x4::M44;
        bench_binop!(b, op => mul, ty1 => M44<f32>, ty2 => M44<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_vek!(group, |b| {
        use vek::Mat4;
        bench_binop!(b, op => mul, ty1 => Mat4<f32>, ty2 => Mat4<f32>)
    });
    group.finish();
}

fn bench_matrix4_mul_matrix4_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 mul matrix4");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::Mat4;
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat4, ty2 => Mat4)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::Mat4x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat4x4, ty2 => Mat4x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::Matrix4;
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix4<f32x4>, ty2 => Matrix4<f32x4>)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::Mat4x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat4x8, ty2 => Mat4x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::Matrix4;
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix4<f32x8>, ty2 => Matrix4<f32x8>)
        });
    }
    group.finish();
}

fn bench_matrix4_mul_vector4(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix4 mul vector4");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4, ty2 => Vec4)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, size, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4, ty2 => Vec4)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            bench_binop!(b, size, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        });
        bench_static_math!(group, size, |b, size| {
            use static_math::matrix4x4::M44;
            use static_math::vector4::V4;
            bench_binop!(b, size, op => mul, ty1 => M44<f32>, ty2 => V4<f32>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat4, Vec4};
            bench_binop!(b, size, op => mul, ty1 => Mat4<f32>, ty2 => Vec4<f32>)
        });
    }
    group.finish();
}

fn bench_matrix4_mul_vector4_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix4 mul vector4");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Mat4, Vec4};
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat4, ty2 => Vec4)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat4x4, Vec4x4};
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat4x4, ty2 => Vec4x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix4<f32x4>, ty2 => Vector4<f32x4>)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat4x8, Vec4x8};
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat4x8, ty2 => Vec4x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{Matrix4, Vector4};
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix4<f32x8>, ty2 => Vector4<f32x8>)
        });
    }
    group.finish();
}

criterion_group!(
    matrix4_benches,
    bench_matrix4_ret_self,
    bench_matrix4_ret_self_wide,
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
