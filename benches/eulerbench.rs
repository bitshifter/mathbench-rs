use criterion::{criterion_group, criterion_main, Criterion};

const UPDATE_RATE: f32 = 1.0 / 60.0;
const NUM_OBJECTS: usize = 10000;

#[macro_export]
macro_rules! bench_euler {
    ($b: ident, ty => $t: ty, zero => $zero: expr) => {{
        let mut rng = Xoshiro256Plus::seed_from_u64(0);
        let accel_data: Vec<$t> = vec![rng.gen(); NUM_OBJECTS];
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
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
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
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{zero, Vec3};
            bench_euler!(b, ty => Vec3, zero => zero());
        }),
    );
}

fn bench_euler_2d(c: &mut Criterion) {
    use criterion::Benchmark;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
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
        .with_function("nalgebra-glm", |b| {
            use nalgebra_glm::{zero, Vec2};
            bench_euler!(b, ty => Vec2, zero => zero());
        }),
    );
}

criterion_group!(benches, bench_euler_2d, bench_euler_3d,);

criterion_main!(benches);
