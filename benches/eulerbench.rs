use criterion::{criterion_group, criterion_main};

const UPDATE_RATE: f32 = 1.0 / 60.0;
const NUM_OBJECTS: usize = 10000;

#[macro_export]
macro_rules! euler {
    ($name: ident, $desc: expr, ty => $t: ty, zero => $zero: expr) => {
        pub(crate) fn $name(c: &mut Criterion) {
            use super::{NUM_OBJECTS, UPDATE_RATE};
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            struct TestData {
                acceleration: Vec<$t>,
                velocity: Vec<$t>,
                position: Vec<$t>,
            }
            let mut rng = Xoshiro256Plus::seed_from_u64(0);
            let mut data = TestData {
                acceleration: vec![rng.gen(); NUM_OBJECTS],
                velocity: vec![$zero; NUM_OBJECTS],
                position: vec![$zero; NUM_OBJECTS],
            };
            c.bench_function($desc, move |b| {
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
    };
}

mod cgmath_bench {
    use cgmath::{prelude::*, Vector2, Vector3};
    use criterion::Criterion;
    euler!(euler_3d, "cgmath euler 3d", ty => Vector3<f32>, zero => Vector3::zero());
    euler!(euler_2d, "cgmath euler 2d", ty => Vector2<f32>, zero => Vector2::zero());
}

mod glam_bench {
    use criterion::Criterion;
    use glam::{Vec2, Vec3};
    euler!(euler_3d_vec3, "glam euler 3d vec3", ty => Vec3, zero => Vec3::zero());
    euler!(euler_2d_vec2, "glam euler 2d vec2", ty => Vec2, zero => Vec2::zero());
}

mod nalgebra_bench {
    use criterion::Criterion;
    use nalgebra_glm::{zero, Vec2, Vec3};
    euler!(euler_3d, "nalgebra_glm euler 3d", ty => Vec3, zero => zero());
    euler!(euler_2d, "nalgebra_glm euler 2d", ty => Vec2, zero => zero());
}

criterion_group!(
    benches,
    cgmath_bench::euler_3d,
    cgmath_bench::euler_2d,
    glam_bench::euler_3d_vec3,
    glam_bench::euler_2d_vec2,
    nalgebra_bench::euler_3d,
    nalgebra_bench::euler_2d,
);
criterion_main!(benches);
