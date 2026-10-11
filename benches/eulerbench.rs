#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{Criterion, Throughput, criterion_group, criterion_main};

const UPDATE_RATE: f32 = 1.0 / 60.0;

macro_rules! bench_euler {
    ($b: ident, $size:expr, width => $width:expr, ty => $t: ty, zero => $zero: expr, dt => $dt: expr) => {{
        // `$size` is a scalar element count; a wide value covers `$width` of
        // them, so the pools are sized per lane. The benchmark id and the
        // declared throughput stay in elements so all widths are comparable.
        let count = usize::div_ceil(*$size, $width);

        struct TestData {
            acc: Vec<$t>,
            vel: Vec<$t>,
            pos: Vec<$t>,
        }

        let mut rng = rand_pcg::Pcg64Mcg::new(macros::BENCH_RNG_SEED);
        let mut data = TestData {
            acc: vec![<$t as mathbench::BenchValue>::random_value(&mut rng); count],
            vel: vec![$zero; count],
            pos: vec![$zero; count],
        };

        let dt = $dt;

        let name = $b.1;
        $b.0.bench_function(criterion::BenchmarkId::new(name, *$size), |bencher| {
            bencher.iter(|| {
                for ((position, acceleration), velocity) in
                    data.pos.iter_mut().zip(&data.acc).zip(&mut data.vel)
                {
                    *velocity += *acceleration * dt;
                    *position += *velocity * dt;
                }
                // Keep the writes live so the loop cannot be optimized away.
                std::hint::black_box(&data.pos[..]);
                std::hint::black_box(&data.vel[..]);
            })
        });
    }};
}

fn bench_euler_3d(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar euler 3d");
    {
        let size = &10000;
        group.throughput(Throughput::Elements(*size as u64));
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
        group.throughput(Throughput::Elements(*size as u64));
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
    }
    group.finish();
}

fn bench_euler_2d(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar euler 2d");
    {
        let size = &10000;
        group.throughput(Throughput::Elements(*size as u64));
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
        group.throughput(Throughput::Elements(*size as u64));
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
