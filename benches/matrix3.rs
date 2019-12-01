#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_matrix3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("matrix3 return self");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => ret_self, ty => Matrix3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => ret_self, ty => Mat3<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => ret_self, ty => Transform2F)
    });
    group.finish();
}

fn bench_matrix3_transpose(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix3 transpose");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => transpose, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => transpose, ty => Matrix3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => transposed, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_determinant(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix3 determinant");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => determinant, ty => Matrix3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Mat3;
        bench_unop!(b, op => determinant, ty => Mat3<f32>)
    });
    group.finish();
}

fn bench_matrix3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("matrix3 inverse");
    bench_glam!(group, |b| {
        use glam::Mat3;
        bench_unop!(b, op => inverse, ty => Mat3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{prelude::*, Matrix3};
        bench_unop!(b, op => invert, ty => Matrix3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Matrix3;
        bench_unop!(b, op => try_inverse, ty => Matrix3<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => inverse, ty => Transform2F)
    });
    group.finish();
}

fn bench_matrix3_mul_matrix3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix3 mul matrix3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::Mat3;
            bench_binop!(b, size, op => mul, ty1 => Mat3, ty2 => Mat3)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::Matrix3;
            bench_binop!(b, size, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::Matrix3;
            bench_binop!(b, size, op => mul, ty1 => Matrix3<f32>, ty2 => Matrix3<f32>, param => by_ref)
        });
        bench_vek!(group, size, |b, size| {
            use vek::Mat3;
            bench_binop!(b, size, op => mul, ty1 => Mat3<f32>, ty2 => Mat3<f32>)
        });
        bench_pathfinder!(group, size, |b, size| {
            use pathfinder_geometry::transform2d::Transform2F;
            use std::ops::Mul;
            bench_binop!(b, size, op => mul, ty1 => Transform2F, ty2 => Transform2F)
        });
    }
    group.finish();
}

fn bench_matrix3_mul_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("matrix3 mul vector3");
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
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Matrix3, Vector3};
            bench_binop!(b, size, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat3, Vec3};
            bench_binop!(b, size, op => mul, ty1 => Mat3<f32>, ty2 => Vec3<f32>)
        });
    }
    group.finish();
}

criterion_group!(
    matrix3_benches,
    bench_matrix3_ret_self,
    bench_matrix3_transpose,
    bench_matrix3_determinant,
    bench_matrix3_inverse,
    bench_matrix3_mul_matrix3,
    bench_matrix3_mul_vector3,
);
criterion_main!(matrix3_benches);
