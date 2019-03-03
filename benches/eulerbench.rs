use criterion::{criterion_group, criterion_main, Criterion};
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

const UPDATE_RATE: f32 = 1.0 / 60.0;
const NUM_OBJECTS: usize = 10000;

fn cgmath_benchmark(c: &mut Criterion) {
    use cgmath::{prelude::*, Vector3};
    struct TestData {
        acceleration: Vec<Vector3<f32>>,
        velocity: Vec<Vector3<f32>>,
        position: Vec<Vector3<f32>>,
    }
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    let mut data = TestData {
        acceleration: vec![rng.gen(); NUM_OBJECTS],
        velocity: vec![Vector3::zero(); NUM_OBJECTS],
        position: vec![Vector3::zero(); NUM_OBJECTS],
    };
    c.bench_function("cgmath_euler", move |b| {
        b.iter(|| {
            let dt = UPDATE_RATE;
            for ((position, acceleration), velocity) in data
                .position
                .iter_mut()
                .zip(&data.acceleration)
                .zip(&mut data.velocity)
            {
                *velocity += *acceleration * dt;
                *position += *velocity * dt;
            }
        })
    });
}

fn glam_benchmark(c: &mut Criterion) {
    use glam::Vec3;
    struct TestData {
        acceleration: Vec<Vec3>,
        velocity: Vec<Vec3>,
        position: Vec<Vec3>,
    }
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    let mut data = TestData {
        acceleration: vec![Vec3::new(rng.gen(), rng.gen(), rng.gen()); NUM_OBJECTS],
        velocity: vec![Vec3::zero(); NUM_OBJECTS],
        position: vec![Vec3::zero(); NUM_OBJECTS],
    };
    c.bench_function("glam_euler", move |b| {
        b.iter(|| {
            let dt = UPDATE_RATE;
            for ((position, acceleration), velocity) in data
                .position
                .iter_mut()
                .zip(&data.acceleration)
                .zip(&mut data.velocity)
            {
                *velocity += *acceleration * dt;
                *position += *velocity * dt;
            }
        })
    });
}

fn nalgebra_benchmark(c: &mut Criterion) {
    use nalgebra_glm::{zero, Vec3};
    struct TestData {
        acceleration: Vec<Vec3>,
        velocity: Vec<Vec3>,
        position: Vec<Vec3>,
    }
    let mut rng = Xoshiro256Plus::seed_from_u64(0);
    let mut data = TestData {
        acceleration: vec![rng.gen(); NUM_OBJECTS],
        velocity: vec![zero(); NUM_OBJECTS],
        position: vec![zero(); NUM_OBJECTS],
    };
    c.bench_function("nalgebra_euler", move |b| {
        b.iter(|| {
            let dt = UPDATE_RATE;
            for ((position, acceleration), velocity) in data
                .position
                .iter_mut()
                .zip(&data.acceleration)
                .zip(&mut data.velocity)
            {
                *velocity += *acceleration * dt;
                *position += *velocity * dt;
            }
        })
    });
}

criterion_group!(
    benches,
    cgmath_benchmark,
    glam_benchmark,
    nalgebra_benchmark
);
criterion_main!(benches);
