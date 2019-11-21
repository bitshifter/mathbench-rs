#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

/// Homogeneous 4x4 matrix transform of a 3D vector
fn bench_mat4_transform_vector3(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transform vector3");
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

/// Homogeneous 4x4 matrix transform of a 3D point
fn bench_mat4_transform_point3(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat4 transform point3");
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

/// 4x4 matrix multiply of a 4D vector
fn bench_mat4_transform_vector4(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat4 transform vector4");
    bench_glam!(group, |b| {
        use glam::{Mat4, Vec4};
        bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Vec4)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix4, Vector4};
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Matrix4, Vector4};
        bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat4, Vec4};
        bench_binop!(b, op => mul, ty1 => Mat4<f32>, ty2 => Vec4<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::{transform3d::Transform4F, vector::Vector4F};
        bench_binop!(b, op => mul, ty1 => Transform4F, ty2 => Vector4F)
    });
    group.finish();
}

/// Homogeneous 3x3 matrix transform of a 2D point
fn bench_mat3_transform_point2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 transform point2");
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

/// Homogeneous 3x3 matrix transform of a 2D vector
fn bench_mat3_transform_vector2(c: &mut Criterion) {
    let mut group = c.benchmark_group("mat3 transform vector2");
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

/// 3x3 matrix multiply of a 3D vector
fn bench_mat3_transform_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat3 transform vector3");
    bench_glam!(group, |b| {
        use glam::{Mat3, Vec3};
        bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix3, Vector3};
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Matrix3, Vector3};
        bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat3, Vec3};
        bench_binop!(b, op => mul, ty1 => Mat3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

/// 2x2 matrix multiply of a 2D vector
fn bench_mat2_transform_vector2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("mat2 transform vector2");
    bench_glam!(group, |b| {
        use glam::{Mat2, Vec2};
        bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Vec2)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Matrix2, Vector2};
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{Matrix2, Vector2};
        bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
    });
    bench_vek!(group, |b| {
        use vek::{Mat2, Vec2};
        bench_binop!(b, op => mul, ty1 => Mat2<f32>, ty2 => Vec2<f32>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::{transform2d::Matrix2x2F, vector::Vector2F};
        bench_binop!(b, op => mul, ty1 => Matrix2x2F, ty2 => Vector2F)
    });
    group.finish();
}

/// Quaternion rotation of a 3D vector
fn bench_quat_transform_vector3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("quat transform vector3");
    bench_glam!(group, |b| {
        use glam::{Quat, Vec3};
        bench_binop!(b, op => mul, ty1 => Quat, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{Quaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::{UnitQuaternion, Vector3};
        bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Point3D, Rotation3D, UnknownUnit};
        bench_binop!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
    });
    bench_vek!(group, |b| {
        use vek::{Quaternion, Vec3};
        bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

criterion_group!(
    transform_benches,
    bench_mat2_transform_vector2,
    bench_mat3_transform_point2,
    bench_mat3_transform_vector2,
    bench_mat3_transform_vector3,
    bench_mat4_transform_point3,
    bench_mat4_transform_vector3,
    bench_mat4_transform_vector4,
    bench_quat_transform_vector3,
);
criterion_main!(transform_benches);
