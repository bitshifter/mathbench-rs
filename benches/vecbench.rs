#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_vec3_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("vec3 return self");
    bench_glam!(group, |b| {
        use glam::Vec3;
        bench_unop!(b, op => nop_fn, ty => Vec3)
    });
    bench_cgmath!(group, |b| {
        use cgmath::Vector3;
        bench_unop!(b, op => nop_fn, ty => Vector3<f32>)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => nop_fn, ty => Vector3<f32>)
    });
    bench_vek!(group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => nop_fn, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vec3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 length");
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

fn bench_vec3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 normalize");
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

fn bench_vec3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 dot");
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

fn bench_vec3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("vec3 cross");
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
    vec_benches,
    bench_vec3_nop,
    bench_vec3_length,
    bench_vec3_normalize,
    bench_vec3_dot,
    bench_vec3_cross
);
criterion_main!(vec_benches);
