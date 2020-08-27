#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_vector3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("vector3 return self");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
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

fn bench_vector3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 length");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => length, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
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

fn bench_vector3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 normalize");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => normalize, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    // bench_static_math!(group, |b| {
    //     use static_math::vector3::V3;
    //     bench_unop!(b, op => normalize, ty => V3<f32>)
    // });
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

fn bench_vector3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 dot");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
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

fn bench_vector3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("vector3 cross");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3, ty2 => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
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

criterion_group!(
    vector3_benches,
    bench_vector3_ret_self,
    bench_vector3_length,
    bench_vector3_normalize,
    bench_vector3_dot,
    bench_vector3_cross
);
criterion_main!(vector3_benches);
