use criterion::{criterion_group, criterion_main, Criterion};

const UPDATE_RATE: f32 = 1.0 / 60.0;
const NUM_OBJECTS: usize = 1 << 13;

#[macro_export]
macro_rules! bench_euler {
    ($b: ident, ty => $t: ty, zero => $zero: expr) => {{
        let accel_data = <$t as mathbench::RandomVec>::random_vec(0, NUM_OBJECTS);
        let mut vel_data: Vec<$t> = vec![$zero; NUM_OBJECTS];
        let mut pos_data: Vec<$t> = vec![$zero; NUM_OBJECTS];
        $b.iter(|| {
            let dt = UPDATE_RATE;
            for ((position, acceleration), velocity) in
                pos_data.iter_mut().zip(&accel_data).zip(&mut vel_data)
            {
                *velocity += *acceleration * dt;
                *position += *velocity * dt;
            }
        })
    }};
}

fn bench_euler_3d(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "euler 3d",
        Benchmark::new("glam", |b| {
            use glam::Vec3;
            bench_euler!(b, ty => Vec3, zero => Vec3::zero())
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Vector3};
            bench_euler!(b, ty => Vector3<f32>, zero => Vector3::zero())
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{zero, Vector3};
            bench_euler!(b, ty => Vector3<f32>, zero => zero());
        })
        .with_function("hektor", |b| {
            use hektor::*;
            bench_euler!(b, ty => Vec3, zero => Vec3::default());
        .with_function("euclid", |b| {
            use euclid::{UnknownUnit, Vector3D};
            bench_euler!(b, ty => Vector3D<f32, UnknownUnit>, zero => Vector3D::zero());
        }),
    );
}

fn bench_euler_2d(c: &mut Criterion) {
    use criterion::Benchmark;
    c.bench(
        "euler 2d",
        Benchmark::new("glam", |b| {
            use glam::Vec2;
            bench_euler!(b, ty => Vec2, zero => Vec2::zero())
        })
        .with_function("cgmath", |b| {
            use cgmath::{prelude::*, Vector2};
            bench_euler!(b, ty => Vector2<f32>, zero => Vector2::zero())
        })
        .with_function("nalgebra", |b| {
            use nalgebra::{zero, Vector2};
            bench_euler!(b, ty => Vector2<f32>, zero => zero());
        })
        .with_function("hektor", |b| {
            use hektor::*;
            bench_euler!(b, ty => Vec2, zero => Vec2::default());
        .with_function("euclid", |b| {
            use euclid::{UnknownUnit, Vector2D};
            bench_euler!(b, ty => Vector2D<f32, UnknownUnit>, zero => Vector2D::zero());
        }),
    );
}

criterion_group!(benches, bench_euler_2d, bench_euler_3d,);

criterion_main!(benches);
