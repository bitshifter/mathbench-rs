#[path = "support/macros.rs"]
#[macro_use]
mod macros;
#[cfg(feature = "glam_fearless")]
#[allow(dead_code)] // shared with the other benches
#[path = "support/glam_fearless.rs"]
mod gf;

use criterion::{Criterion, criterion_group, criterion_main};

const UPDATE_RATE: f32 = 1.0 / 60.0;

macro_rules! bench_euler {
    ($b: ident, $size:expr, width => $width:expr, ty => $t: ty, zero => $zero: expr, dt => $dt: expr) => {{
        // `$size` is a scalar element count; a wide value covers `$width` of
        // them, so the pools are sized per lane. The benchmark id and the
        // declared throughput stay in elements so all widths are comparable.
        macros::bench_workload(
            $b,
            &[*$size],
            |rng, size| {
                let count = usize::div_ceil(size, $width);
                let acc = vec![<$t as mathbench::BenchValue>::random_value(rng); count];
                let vel: Vec<$t> = vec![$zero; count];
                let pos: Vec<$t> = vec![$zero; count];
                (acc, vel, pos, $dt)
            },
            |data| {
                let (acc, vel, pos, dt) = data;
                let dt = *dt;
                for ((position, acceleration), velocity) in
                    pos.iter_mut().zip(&*acc).zip(vel.iter_mut())
                {
                    *velocity += *acceleration * dt;
                    *position += *velocity * dt;
                }
                // Keep the writes live so the loop cannot be optimized away.
                std::hint::black_box(&pos[..]);
                std::hint::black_box(&vel[..]);
            },
        )
    }};
}

fn bench_euler_3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar euler 3d");
    {
        let size = &10000;
        bench!("glam", group, size, |b, size| {
            use glam::Vec3A;
            bench_euler!(b, size, width => 1, ty => Vec3A, zero => Vec3A::ZERO, dt => Vec3A::splat(UPDATE_RATE))
        });
        bench!("cgmath", group, size, |b, size| {
            use cgmath::{Vector3, prelude::*};
            bench_euler!(b, size, width => 1, ty => Vector3<f32>, zero => Vector3::zero(), dt => UPDATE_RATE)
        });
        bench!("ultraviolet", group, size, |b, size| {
            use ultraviolet::Vec3;
            bench_euler!(b, size, width => 1, ty => Vec3, zero => Vec3::zero(), dt => UPDATE_RATE);
        });
        bench!("nalgebra", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            bench_euler!(b, size, width => 1, ty => Vector3<f32>, zero => zero(), dt => UPDATE_RATE);
        });

        bench!("euclid", group, size, |b, size| {
            use euclid::{UnknownUnit, Vector3D};
            bench_euler!(b, size, width => 1, ty => Vector3D<f32, UnknownUnit>, zero => Vector3D::zero(), dt => UPDATE_RATE);
        });
        bench!("vek", group, size, |b, size| {
            use vek::Vec3;
            bench_euler!(b, size, width => 1, ty => Vec3<f32>, zero => Vec3::zero(), dt => Vec3::broadcast(UPDATE_RATE))
        });
        bench!("pathfinder_geometry" as "pathfinder", group, size, |b, size| {
            use pathfinder_geometry::vector::Vector4F;
            bench_euler!(b, size, width => 1, ty => Vector4F, zero => Vector4F::splat(0.0), dt => Vector4F::splat(UPDATE_RATE))
        });
    }
    group.finish();
}

fn bench_euler_3d_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide euler 3d");
    {
        let size = &80000;
        bench!("glam_f32x1", group, size, |b, size| {
            use glam::Vec3A;
            bench_euler!(b, size, width => 1, ty => Vec3A, zero => Vec3A::ZERO, dt => Vec3A::splat(UPDATE_RATE))
        });

        // sse
        bench!("ultraviolet_f32x4", group, size, |b, size| {
            use ultraviolet::{Vec3x4, f32x4};
            bench_euler!(b, size, width => 4, ty => Vec3x4, zero => Vec3x4::zero(), dt => f32x4::splat(UPDATE_RATE))
        });
        bench!("nalgebra_f32x4", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            use simba::simd::{SimdValue, WideF32x4};
            bench_euler!(b, size, width => 4, ty => Vector3<WideF32x4>, zero => zero(), dt => WideF32x4::splat(UPDATE_RATE));
        });

        // avx
        bench!("ultraviolet_f32x8", group, size, |b, size| {
            use ultraviolet::{Vec3x8, f32x8};
            bench_euler!(b, size, width => 8, ty => Vec3x8, zero => Vec3x8::zero(), dt => f32x8::splat(UPDATE_RATE))
        });
        bench!("nalgebra_f32x8", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            use simba::simd::{SimdValue, WideF32x8};
            bench_euler!(b, size, width => 8, ty => Vector3<WideF32x8>, zero => zero(), dt => WideF32x8::splat(UPDATE_RATE));
        });
        // Runtime-dispatched width: the same 80000 elements, SoA components, one kernel body.
        bench!("glam_fearless", group, |b| {
            macros::bench_workload(
                b,
                &[80000],
                |rng, size| {
                    (
                        gf::Euler3Entry::new(),
                        gf::random_soa(size, rng),
                        [vec![0.0f32; size], vec![0.0f32; size], vec![0.0f32; size]],
                        [vec![0.0f32; size], vec![0.0f32; size], vec![0.0f32; size]],
                    )
                },
                |s| {
                    let (euler3, acc, vel, pos) = s;
                    euler3.call(gf::refs3(acc), gf::refs3_mut(vel), gf::refs3_mut(pos));
                    std::hint::black_box((&*vel, &*pos));
                },
            )
        });
    }
    group.finish();
}

fn bench_euler_2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar euler 2d");
    {
        let size = &10000;
        bench!("glam", group, size, |b, size| {
            use glam::Vec2;
            bench_euler!(b, size, width => 1, ty => Vec2, zero => Vec2::ZERO, dt => Vec2::splat(UPDATE_RATE))
        });
        bench!("cgmath", group, size, |b, size| {
            use cgmath::{Vector2, prelude::*};
            bench_euler!(b, size, width => 1, ty => Vector2<f32>, zero => Vector2::zero(), dt => UPDATE_RATE)
        });
        bench!("ultraviolet", group, size, |b, size| {
            use ultraviolet::Vec2;
            bench_euler!(b, size, width => 1, ty => Vec2, zero => Vec2::zero(), dt => UPDATE_RATE);
        });
        bench!("nalgebra", group, size, |b, size| {
            use nalgebra::{Vector2, zero};
            bench_euler!(b, size, width => 1, ty => Vector2<f32>, zero => zero(), dt => UPDATE_RATE);
        });

        bench!("euclid", group, size, |b, size| {
            use euclid::{UnknownUnit, Vector2D};
            bench_euler!(b, size, width => 1, ty => Vector2D<f32, UnknownUnit>, zero => Vector2D::zero(), dt => UPDATE_RATE);
        });
        bench!("vek", group, size, |b, size| {
            use vek::Vec2;
            bench_euler!(b, size, width => 1, ty => Vec2<f32>, zero => Vec2::zero(), dt => Vec2::broadcast(UPDATE_RATE))
        });
        bench!("pathfinder_geometry" as "pathfinder", group, size, |b, size| {
            use pathfinder_geometry::vector::Vector2F;
            bench_euler!(b, size, width => 1, ty => Vector2F, zero => Vector2F::splat(0.0), dt => Vector2F::splat(UPDATE_RATE))
        });
    }
    group.finish();
}

fn bench_euler_2d_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide euler 2d");
    {
        let size = &80000;
        bench!("glam_f32x1", group, size, |b, size| {
            use glam::Vec2;
            bench_euler!(b, size, width => 1, ty => Vec2, zero => Vec2::ZERO, dt => Vec2::splat(UPDATE_RATE))
        });

        // sse
        bench!("ultraviolet_f32x4", group, size, |b, size| {
            use ultraviolet::{Vec2x4, f32x4};
            bench_euler!(b, size, width => 4, ty => Vec2x4, zero => Vec2x4::zero(), dt => f32x4::splat(UPDATE_RATE))
        });
        bench!("nalgebra_f32x4", group, size, |b, size| {
            use nalgebra::{Vector2, zero};
            use simba::simd::{SimdValue, WideF32x4};
            bench_euler!(b, size, width => 4, ty => Vector2<WideF32x4>, zero => zero(), dt => WideF32x4::splat(UPDATE_RATE));
        });
        // avx
        bench!("ultraviolet_f32x8", group, size, |b, size| {
            use ultraviolet::{Vec2x8, f32x8};
            bench_euler!(b, size, width => 8, ty => Vec2x8, zero => Vec2x8::zero(), dt => f32x8::splat(UPDATE_RATE))
        });
        bench!("nalgebra_f32x8", group, size, |b, size| {
            use nalgebra::{Vector2, zero};
            use simba::simd::{SimdValue, WideF32x8};
            bench_euler!(b, size, width => 8, ty => Vector2<WideF32x8>, zero => zero(), dt => WideF32x8::splat(UPDATE_RATE));
        });
        // Runtime-dispatched width: the same 80000 elements, SoA components, one kernel body.
        bench!("glam_fearless", group, |b| {
            macros::bench_workload(
                b,
                &[80000],
                |rng, size| {
                    (
                        gf::Euler2Entry::new(),
                        gf::random_soa2(size, rng),
                        [vec![0.0f32; size], vec![0.0f32; size]],
                        [vec![0.0f32; size], vec![0.0f32; size]],
                    )
                },
                |s| {
                    let (euler2, acc, vel, pos) = s;
                    euler2.call(gf::refs2(acc), gf::refs2_mut(vel), gf::refs2_mut(pos));
                    std::hint::black_box((&*vel, &*pos));
                },
            )
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_euler_2d,
    bench_euler_3d,
    bench_euler_3d_wide,
    bench_euler_2d_wide
);

criterion_main!(benches);
