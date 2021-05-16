#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_transform_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform vector3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Affine3A, Vec3A};
            bench_binop!(b, size, op => transform_vector3a, ty1 => Affine3A, ty2 => Vec3A)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat4, Vec3};
            bench_binop!(b, size, op => transform_vec3, ty1 => Mat4, ty2 => Vec3)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix4, Transform, Vector3};
            bench_binop!(b, size, op => transform_vector, ty1 => Matrix4<f32>, ty2 => Vector3<f32>)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Transform3, Vector3};
            bench_binop!(b, size, op => transform_vector, ty1 => Transform3<f32>, ty2 => Vector3<f32>, param => by_ref)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Transform3D, UnknownUnit, Vector3D};
            bench_binop!(b, size, op => transform_vector3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat4, Vec3};
            bench_binop!(b, size, op => mul_direction, ty1 => Mat4<f32>, ty2 => Vec3<f32>)
        });
    }
    group.finish();
}

fn bench_transform_vector3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform vector3");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Affine3A, Vec3A};
            bench_binop_wide!(b, size, width => 1, op => transform_vector3a, ty1 => Affine3A, ty2 => Vec3A)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat4x4, Vec3x4};
            bench_binop_wide!(b, size, width => 4, op => transform_vec3, ty1 => Mat4x4, ty2 => Vec3x4)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat4x8, Vec3x8};
            bench_binop_wide!(b, size, width => 8, op => transform_vec3, ty1 => Mat4x8, ty2 => Vec3x8)
        });
    }
    group.finish();
}

fn bench_transform_point3(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform point3");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Affine3A, Vec3A};
            bench_binop!(b, size, op => transform_point3a, ty1 => Affine3A, ty2 => Vec3A)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat4, Vec3};
            bench_binop!(b, size, op => transform_point3, ty1 => Mat4, ty2 => Vec3)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix4, Point3, Transform};
            bench_binop!(b, size, op => transform_point, ty1 => Matrix4<f32>, ty2 => Point3<f32>)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Point3, Transform3};
            bench_binop!(b, size, op => transform_point, ty1 => Transform3<f32>, ty2 => Point3<f32>, param => by_ref)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Point3D, Transform3D, UnknownUnit};
            bench_binop!(b, size, op => transform_point3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat4, Vec3};
            bench_binop!(b, size, op => mul_point, ty1 => Mat4<f32>, ty2 => Vec3<f32>)
        });
        bench_pathfinder!(group, size, |b, size| {
            // pathfinder doesn't have a point3 or vector3 type
            use pathfinder_geometry::{transform3d::Transform4F, vector::Vector4F};
            use std::ops::Mul;
            bench_binop!(b, size, op => mul, ty1 => Transform4F, ty2 => Vector4F)
        });
    }
    group.finish();
}

fn bench_transform_point3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform point3");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Affine3A, Vec3A};
            bench_binop_wide!(b, size, width => 1, op => transform_point3a, ty1 => Affine3A, ty2 => Vec3A)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat4x4, Vec3x4};
            bench_binop_wide!(b, size, width => 4, op => transform_point3, ty1 => Mat4x4, ty2 => Vec3x4)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat4x8, Vec3x8};
            bench_binop_wide!(b, size, width => 8, op => transform_point3, ty1 => Mat4x8, ty2 => Vec3x8)
        });
    }
    group.finish();
}

fn bench_transform_point2(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform point2");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Affine2, Vec2};
            bench_binop!(b, size, op => transform_point2, ty1 => Affine2, ty2 => Vec2)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat3, Vec2};
            bench_binop!(b, size, op => transform_point2, ty1 => Mat3, ty2 => Vec2)
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{Matrix3, Point2, Transform};
            bench_binop!(b, size, op => transform_point, ty1 => Matrix3<f32>, ty2 => Point2<f32>)
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Point2, Transform2};
            bench_binop!(b, size, op => transform_point, ty1 => Transform2<f32>, ty2 => Point2<f32>, param => by_ref)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Point2D, Transform2D, UnknownUnit};
            bench_binop!(b, size, op => transform_point, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Point2D<f32, UnknownUnit>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat3, Vec2};
            bench_binop!(b, size, op => mul_point_2d, ty1 => Mat3<f32>, ty2 => Vec2<f32>)
        });
        bench_pathfinder!(group, size, |b, size| {
            // pathfinder doesn't have a point type, this is an affine transformation.
            use pathfinder_geometry::{transform2d::Transform2F, vector::Vector2F};
            use std::ops::Mul;
            bench_binop!(b, size, op => mul, ty1 => Transform2F, ty2 => Vector2F);
        });
    }
    group.finish();
}

fn bench_transform_point2_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform point2");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Affine2, Vec2};
            bench_binop_wide!(b, size, width => 1, op => transform_point2, ty1 => Affine2, ty2 => Vec2)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat3x4, Vec2x4};
            bench_binop_wide!(b, size, width => 4, op => transform_point2, ty1 => Mat3x4, ty2 => Vec2x4)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat3x8, Vec2x8};
            bench_binop_wide!(b, size, width => 8, op => transform_point2, ty1 => Mat3x8, ty2 => Vec2x8)
        });
    }
    group.finish();
}

fn bench_transform_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform vector2");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::{Affine2, Vec2};
            bench_binop!(b, size, op => transform_vector2, ty1 => Affine2, ty2 => Vec2)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::{Mat3, Vec2};
            bench_binop!(b, size, op => transform_vec2, ty1 => Mat3, ty2 => Vec2)
        });
        // TODO: doesn't compile but might need a macro change to set return type
        // bench_cgmath!(group, |b| {
        //     use cgmath::{Matrix3, Transform, Vector2};
        //     bench_binop!(b, op => transform_vector, ty1 => Matrix3<f32>, ty2 => Vector2<f32>)
        // });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{Transform2, Vector2};
            bench_binop!(b, size, op => transform_vector, ty1 => Transform2<f32>, ty2 => Vector2<f32>, param => by_ref)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Transform2D, UnknownUnit, Vector2D};
            bench_binop!(b, size, op => transform_vector, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector2D<f32, UnknownUnit>)
        });
        bench_vek!(group, size, |b, size| {
            use vek::{Mat3, Vec2};
            bench_binop!(b, size, op => mul_direction_2d, ty1 => Mat3<f32>, ty2 => Vec2<f32>)
        });
    }
    group.finish();
}

fn bench_transform_vector2_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform vector2");
    for size in [16, 256].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::{Affine2, Vec2};
            bench_binop_wide!(b, size, width => 1, op => transform_vector2, ty1 => Affine2, ty2 => Vec2)
        });
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{Mat3x4, Vec2x4};
            bench_binop_wide!(b, size, width => 4, op => transform_vec2, ty1 => Mat3x4, ty2 => Vec2x4)
        });
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{Mat3x8, Vec2x8};
            bench_binop_wide!(b, size, width => 8, op => transform_vec2, ty1 => Mat3x8, ty2 => Vec2x8)
        });
    }
    group.finish();
}

criterion_group!(
    transformation_benches,
    bench_transform_vector2,
    bench_transform_vector2_wide,
    bench_transform_point2,
    bench_transform_point2_wide,
    bench_transform_vector3,
    bench_transform_vector3_wide,
    bench_transform_point3,
    bench_transform_point3_wide,
);
criterion_main!(transformation_benches);
