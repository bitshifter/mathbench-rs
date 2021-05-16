#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};
use macros::MIN_WIDE_BENCH_SIZE;
use mathbench::BenchValue;
use std::ops::Mul;

fn bench_matrix3_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 return self");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix3x3::M33;
        bench_unop!(b, op => ret_self, ty => M33<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_ret_self_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix3 return self");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat3;
        bench_unop_wide!(b, size, width => 1, op => ret_self, ty => Mat3)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Mat3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Matrix3;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => ret_self, ty => Matrix3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Mat3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Matrix3;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => ret_self, ty => Matrix3<f32x8>)
    });
    group.finish();
}

fn bench_matrix3_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 transpose");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => transpose, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix3x3::M33;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => transpose, ty => M33<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_transpose_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix3 transpose");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat3;
        bench_unop_wide!(b, size, width => 1, op => transpose, ty => Mat3)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, size, width => 4, op => transposed, ty => Mat3x4)
    });
    bench_nalgebra_f32x4!(group, size, |b, size| {
        use nalgebra::Matrix3;
        use simba::simd::f32x4;
        bench_unop_wide!(b, size, width => 4, op => transpose, ty => Matrix3<f32x4>)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, size, width => 8, op => transposed, ty => Mat3x8)
    });
    bench_nalgebra_f32x8!(group, size, |b, size| {
        use nalgebra::Matrix3;
        use simba::simd::f32x8;
        bench_unop_wide!(b, size, width => 8, op => transpose, ty => Matrix3<f32x8>)
    });
    group.finish();
}

fn bench_matrix3_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 determinant");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix3x3::M33;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => det, ty => M33<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_determinant_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix3 determinant");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat3;
        bench_unop_wide!(b, size, width => 1, op => determinant, ty => Mat3)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, size, width => 4, op => determinant, ty => Mat3x4)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, size, width => 8, op => determinant, ty => Mat3x8)
    });
    group.finish();
}

fn bench_matrix3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 inverse");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => inverse, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => invert, ty => Matrix3<f32>)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat3;
        bench_unop!(b, op => inversed, ty => Mat3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => try_inverse, ty => Matrix3<f32>)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix3x3::M33;
        use static_math::traits::LinearAlgebra;
        bench_unop!(b, op => inverse, ty => M33<f32>)
    });
    group.finish();
}

fn bench_matrix3_inverse_wide(c: &mut Criterion) {
    let size = &MIN_WIDE_BENCH_SIZE;
    let mut group = c.benchmark_group("wide matrix3 inverse");
    group.throughput(criterion::Throughput::Elements(*size));
    bench_glam_f32x1!(group, size, |b, size| {
        use glam::Mat3;
        bench_unop_wide!(b, size, width => 1, op => inverse, ty => Mat3)
    });
    bench_ultraviolet_f32x4!(group, size, |b, size| {
        use ultraviolet::Mat3x4;
        bench_unop_wide!(b, size, width => 4, op => inversed, ty => Mat3x4)
    });
    bench_ultraviolet_f32x8!(group, size, |b, size| {
        use ultraviolet::Mat3x8;
        bench_unop_wide!(b, size, width => 8, op => inversed, ty => Mat3x8)
    });
    group.finish();
}

fn bench_matrix3_mul_matrix3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 mul matrix3");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });
    bench_ultraviolet!(group, |b| {
        use ultraviolet::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Mat3)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
    });
    bench_static_math!(group, |b| {
        use static_math::matrix3x3::M33;
        bench_binop!(b, op => mul, ty1 => M33<f32>, ty2 => M33<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_binop!(b, op => mul, ty1 => Mat3<f32>, ty2 => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_mul_matrix3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 mul matrix3");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::Mat3;
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat3, ty2 => Mat3)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::Mat3x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat3x4, ty2 => Mat3x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::Matrix3;
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix3<f32x4>, ty2 => Matrix3<f32x4>)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::Mat3x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat3x8, ty2 => Mat3x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::Matrix3;
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix3<f32x8>, ty2 => Matrix3<f32x8>)
        });
    }
    group.finish();
}

fn bench_matrix3_mul_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar matrix3 mul vector3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Mat3, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Mat3, ty2 => Vec3)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix3, Vector3};
            bench_binop!(b, size, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat3, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Mat3, ty2 => Vec3)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix3, Vector3};
            bench_binop!(b, size, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        });
        bench_static_math!(group, size, |b, size| {
            use static_math::matrix3x3::M33;
            use static_math::vector3::V3;
            bench_binop!(b, size, op => mul, ty1 => M33<f32>, ty2 => V3<f32>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat3, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Mat3<f32>, ty2 => Vec3<f32>)
        });
    }
    group.finish();
}

fn bench_matrix3_mul_vector3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide matrix3 mul vector3");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Mat3, Vec3};
            bench_binop_wide!(b, size, width => 1, op => mul, ty1 => Mat3, ty2 => Vec3)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat3x4, Vec3x4};
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Mat3x4, ty2 => Vec3x4)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{Matrix3, Vector3};
            use simba::simd::f32x4;
            bench_binop_wide!(b, size, width => 4, op => mul, ty1 => Matrix3<f32x4>, ty2 => Vector3<f32x4>)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat3x8, Vec3x8};
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Mat3x8, ty2 => Vec3x8)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{Matrix3, Vector3};
            use simba::simd::f32x8;
            bench_binop_wide!(b, size, width => 8, op => mul, ty1 => Matrix3<f32x8>, ty2 => Vector3<f32x8>)
        });
    }
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
