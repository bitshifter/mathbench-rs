#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

/// Homogeneous 4x4 matrix transform of a 3D vector
fn bench_mat4_transform_vector3(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 transform vector3",
        Benchmark::new("glam", |b| {
            use glam::{Mat4, Vec3};
            bench_binop!(b, op => transform_vector3, ty1 => Mat4, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Decomposed, Quaternion, Transform, Vector3};
            bench_binop!(b, op => transform_vector, ty1 => Decomposed<Vector3<f32>, Quaternion<f32>>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Transform3, Vector3};
            bench_binop!(b, op => transform_vector, ty1 => Transform3<f32>, ty2 => Vector3<f32>, param => by_ref)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform3D, Vector3D, UnknownUnit};
            bench_binop!(b, op => transform_vector3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
        }),
    );
}

/// Homogeneous 4x4 matrix transform of a 3D point
fn bench_mat4_transform_point3(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "mat4 transform point3",
        Benchmark::new("glam", |b| {
            use glam::{Mat4, Vec3};
            bench_binop!(b, op => transform_point3, ty1 => Mat4, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Decomposed, Quaternion, Point3, Transform, Vector3};
            bench_binop!(b, op => transform_point, ty1 => Decomposed<Vector3<f32>, Quaternion<f32>>, ty2 => Point3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Transform3, Point3};
            bench_binop!(b, op => transform_point, ty1 => Transform3<f32>, ty2 => Point3<f32>, param => by_ref)
        })
        .with_function("euclid", |b| {
            use euclid::{Transform3D, Point3D, UnknownUnit};
            bench_binop!(b, op => transform_point3d, ty1 => Transform3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
        }),
    );
}

/// 4x3 matrix multiply of a 4D vector
fn bench_mat4_transform_vector4(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat4 transform vector4",
        Benchmark::new("glam", |b| {
            use glam::{Mat4, Vec4};
            bench_binop!(b, op => mul, ty1 => Mat4, ty2 => Vec4)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix4, Vector4};
            bench_binop!(b, op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>)
        }),
    );
}

/// 3x3 matrix multiply of a 3D vector
fn bench_mat3_transform_vector3(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat3 transform vector3",
        Benchmark::new("glam", |b| {
            use glam::{Mat3, Vec3};
            bench_binop!(b, op => mul, ty1 => Mat3, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix3, Vector3};
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix3, Vector3};
            bench_binop!(b, op => mul, ty1 => Matrix3<f32>, ty2 => Vector3<f32>)
        }),
    );
}

/// 2x2 matrix multiply of a 2D vector
fn bench_mat2_transform_vector2(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "mat2 transform vector2",
        Benchmark::new("glam", |b| {
            use glam::{Mat2, Vec2};
            bench_binop!(b, op => mul, ty1 => Mat2, ty2 => Vec2)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Matrix2, Vector2};
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{Matrix2, Vector2};
            bench_binop!(b, op => mul, ty1 => Matrix2<f32>, ty2 => Vector2<f32>)
        })
        // TODO: this isn't the same at the others
        .with_function("euclid", |b| {
            use euclid::{Transform2D, Vector2D, UnknownUnit};
            bench_binop!(b, op => transform_vector, ty1 => Transform2D<f32, UnknownUnit, UnknownUnit>, ty2 => Vector2D<f32, UnknownUnit>)
        }),
    );
}

/// Quaternion rotation of a 3D vector
fn bench_quat_transform_vector3(c: &mut Criterion) {
    use criterion::Benchmark;
    use std::ops::Mul;
    c.bench(
        "quat transform vector3",
        Benchmark::new("glam", |b| {
            use glam::{Quat, Vec3};
            bench_binop!(b, op => mul, ty1 => Quat, ty2 => Vec3)
        })
        .with_function("cgmath", |b| {
            use cgmath::{Quaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => Quaternion<f32>, ty2 => Vector3<f32>)
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{UnitQuaternion, Vector3};
            bench_binop!(b, op => mul, ty1 => UnitQuaternion<f32>, ty2 => Vector3<f32>)
        })
        .with_function("euclid", |b| {
            use euclid::{Rotation3D, Point3D, UnknownUnit};
            bench_binop!(b, op => transform_point3d, ty1 => Rotation3D<f32, UnknownUnit, UnknownUnit>, ty2 => Point3D<f32, UnknownUnit>)
        })
        ,
    );
}

criterion_group!(
    transform_benches,
    bench_mat2_transform_vector2,
    bench_mat3_transform_vector3,
    bench_mat4_transform_point3,
    bench_mat4_transform_vector3,
    bench_mat4_transform_vector4,
    bench_quat_transform_vector3,
);
criterion_main!(transform_benches);
