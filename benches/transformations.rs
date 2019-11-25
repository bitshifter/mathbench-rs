#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_transform_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform vector3");
    bench_glam!(group, |b| {
        use glam::{Mat4, Vec3};
        bench_binop!(b, op => transform_vector3, ty1 => Mat4, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, Transform, Vector3};
        bench_binop!(b, op => transform_vector, ty1 => Matrix4<f32>, ty2 => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Transform3, Vector3};
        bench_binop!(b, op => transform_vector, ty1 => Transform3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit, Vector3D};
        bench_binop!(b, op => transform_vector3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat4, Vec3};
        bench_binop!(b, op => mul_direction, ty1 => Mat4<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_transform_point3(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform point3");
    bench_glam!(group, |b| {
        use glam::{Mat4, Vec3};
        bench_binop!(b, op => transform_point3, ty1 => Mat4, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, Point3, Transform};
        bench_binop!(b, op => transform_point, ty1 => Matrix4<f32>, ty2 => Point3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Point3, Transform3};
        bench_binop!(b, op => transform_point, ty1 => Transform3<f32>, ty2 => Point3<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Point3D, Transform3D, UnknownUnit};
        bench_binop!(b, op => transform_point3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat4, Vec3};
        bench_binop!(b, op => mul_point, ty1 => Mat4<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_transform_point2(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform point2");
    bench_glam!(group, |b| {
        use glam::{Mat3, Vec2};
        bench_binop!(b, op => transform_point2, ty1 => Mat3, ty2 => Vec2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix3, Point2, Transform};
        bench_binop!(b, op => transform_point, ty1 => Matrix3<f32>, ty2 => Point2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Point2, Transform2};
        bench_binop!(b, op => transform_point, ty1 => Transform2<f32>, ty2 => Point2<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Point2D, Transform2D, UnknownUnit};
        bench_binop!(b, op => transform_point, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Point2D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat3, Vec2};
        bench_binop!(b, op => mul_point_2d, ty1 => Mat3<f32>, ty2 => Vec2<f32>)
    });
    group.finish();
}

fn bench_transform_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform vector2");
    bench_glam!(group, |b| {
        use glam::{Mat3, Vec2};
        bench_binop!(b, op => transform_vector2, ty1 => Mat3, ty2 => Vec2)
    });
    // TODO: doesn't compile but might need a macro change to set return type
    // bench_cgmath!(group, |b| {
    //     use cgmath::{Matrix3, Transform, Vector2};
    //     bench_binop!(b, op => transform_vector, ty1 => Matrix3<f32>, ty2 => Vector2<f32>)
    // });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Transform2, Vector2};
        bench_binop!(b, op => transform_vector, ty1 => Transform2<f32>, ty2 => Vector2<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit, Vector2D};
        bench_binop!(b, op => transform_vector, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector2D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat3, Vec2};
        bench_binop!(b, op => mul_direction_2d, ty1 => Mat3<f32>, ty2 => Vec2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::{transform2d::Transform2F, vector::Vector2F};
        use std::ops::Mul;
        bench_binop!(b, op => mul, ty1 => Transform2F, ty2 => Vector2F);
    });
    group.finish();
}

criterion_group!(
    transformation_benches,
    bench_transform_vector2,
    bench_transform_point2,
    bench_transform_vector3,
    bench_transform_point3,
);
criterion_main!(transformation_benches);
