#![cfg_attr(feature = "unstable", feature(stmt_expr_attributes))]
#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{criterion_group, criterion_main, Criterion, Throughput};

#[cfg(any(feature = "ultraviolet_f32x4", feature = "ultraviolet_f32x8",))]
macro_rules! bench_intersection_wide_uv {
    ($b: ident, $size:expr, ty => $t: ty, wt => $wt: ident, zero_vec => $zero: expr, max => $max: expr) => {{
        use wide_mathbench::CmpGt;
        struct TestData {
            ray_d: Vec<$t>,
            result: Vec<$wt>,
        }

        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let ray_d = (0..*$size)
            .into_iter()
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng).normalized())
            .collect::<Vec<_>>();
        let mut data = TestData {
            ray_d,
            result: vec![$wt::splat(0.0); *$size],
        };

        let sphere_o = $zero;
        let ray_o = $zero;
        let sphere_r_sq = $wt::splat(100.0);
        let miss = $wt::splat($max);

        let do_inner = never_inline_closure!(|ray_d: &$t, result: &mut $wt| {
            let oc: $t = ray_o - sphere_o;
            let b = oc.dot(*ray_d);
            let c = oc.mag_sq() - sphere_r_sq;
            let descrim = b * b - c;

            let z = $wt::splat(0.0);
            let desc_pos = descrim.cmp_gt(z);

            let desc_sqrt = descrim.sqrt();

            let t1 = -b - desc_sqrt;
            let t1_valid = t1.cmp_gt(z) & desc_pos;

            let t2 = -b + desc_sqrt;
            let t2_valid = t2.cmp_gt(z) & desc_pos;

            let t = t2_valid.blend(t2, miss);
            let t = t1_valid.blend(t1, t);

            *result = t;
        });
        $b.iter(|| {
            for (ray_d, result) in data.ray_d.iter().zip(&mut data.result) {
                do_inner(ray_d, result);
            }
        })
    }};
}

#[cfg(any(feature = "nalgebra_f32x4", feature = "nalgebra_f32x8",))]
macro_rules! bench_intersection_wide_na {
    ($b: ident, $size:expr, ty => $t: ty, wt => $wt: ident, zero_vec => $zero: expr, max => $max: expr) => {{
        struct TestData {
            ray_d: Vec<$t>,
            result: Vec<$wt>,
        }

        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let ray_d = (0..*$size)
            .into_iter()
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng).normalize())
            .collect::<Vec<_>>();
        let mut data = TestData {
            ray_d,
            result: vec![$wt::splat(0.0); *$size],
        };

        let sphere_o: $t = $zero;
        let ray_o: $t = $zero;
        let sphere_r_sq = $wt::splat(100.0);
        let miss = $wt::splat($max);

        let do_inner = never_inline_closure!(|ray_d: &$t, result: &mut $wt| {
            let oc: $t = ray_o - sphere_o;
            let b = oc.dot(ray_d);
            let c = oc.norm_squared() - sphere_r_sq;
            let descrim = b * b - c;

            let z = $wt::splat(0.0);
            let desc_pos = descrim.0.gt(z.0);

            let desc_sqrt = simba::simd::Simd(descrim.0.sqrt());

            let t1 = -b - desc_sqrt;
            let t1_valid = t1.0.gt(z.0) & desc_pos;

            let t2 = -b + desc_sqrt;
            let t2_valid = t2.0.gt(z.0) & desc_pos;

            let t = t2_valid.select(t2.0, miss.0);
            let t = t1_valid.select(t1.0, t);

            *result = simba::simd::Simd(t);
        });
        $b.iter(|| {
            for (ray_d, result) in data.ray_d.iter().zip(&mut data.result) {
                do_inner(ray_d, result);
            }
        })
    }};
}

macro_rules! bench_intersection_scalar {
    ($b: ident, $size:expr, ty => $t: ty, zero => $zero: expr, norm => $norm: ident, mag_sq => $mag_sq: ident, param => $param: tt) => {{
        struct TestData {
            ray_d: Vec<$t>,
            result: Vec<f32>,
        }

        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let ray_d = (0..*$size)
            .into_iter()
            .map(|_| <$t as mathbench::BenchValue>::random_value(&mut rng).$norm())
            .collect::<Vec<_>>();
        let mut data = TestData {
            ray_d,
            result: vec![0.0; *$size],
        };

        let sphere_o: $t = $zero;
        let sphere_r_sq = 100.0;
        let ray_o = <$t>::new(0.0, 0.0, -11.0);

        let do_inner = never_inline_closure!(|ray_d: &$t, result: &mut f32| {
            let oc: $t = ray_o - sphere_o;
            let b = oc.dot($param!(ray_d));
            let c = oc.$mag_sq() - sphere_r_sq;
            let descrim = b * b - c;

            *result = if descrim > 0.0 {
                let desc_sqrt = descrim.sqrt();

                let t1 = -b - desc_sqrt;
                if t1 > 0.0 {
                    t1
                } else {
                    let t2 = -b + desc_sqrt;
                    if t2 > 0.0 {
                        t2
                    } else {
                        f32::MAX
                    }
                }
            } else {
                f32::MAX
            };
        });

        $b.iter(|| {
            for (ray_d, result) in data.ray_d.iter().zip(&mut data.result) {
                do_inner(ray_d, result);
            }
        })
    }};
}

fn bench_ray_sphere_intersect_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar ray-sphere intersection");
    for size in [10000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        bench_glam!(group, size, |b, size| {
            use glam::Vec3;
            bench_intersection_scalar!(b, size, ty => Vec3, zero => Vec3::ZERO, norm => normalize, mag_sq => length_squared, param => by_value);
        });
        bench_cgmath!(group, size, |b, size| {
            use cgmath::{prelude::*, Vector3};
            bench_intersection_scalar!(b, size, ty => Vector3<f32>, zero => Vector3::zero(), norm => normalize, mag_sq => magnitude2, param => by_value)
        });
        bench_ultraviolet!(group, size, |b, size| {
            use ultraviolet::Vec3;
            bench_intersection_scalar!(b, size, ty => Vec3, zero => Vec3::zero(), norm => normalized, mag_sq => mag_sq, param => by_value);
        });
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            bench_intersection_scalar!(b, size, ty => Vector3<f32>, zero => zero(), norm => normalize, mag_sq => norm_squared, param => by_ref);
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{UnknownUnit, Vector3D};
            bench_intersection_scalar!(b, size, ty => Vector3D<f32, UnknownUnit>, zero => Vector3D::zero(), norm => normalize, mag_sq => square_length, param => by_value);
        });
        bench_vek!(group, size, |b, size| {
            use vek::Vec3;
            bench_intersection_scalar!(b, size, ty => Vec3<f32>, zero => Vec3::zero(), norm => normalized, mag_sq => magnitude_squared, param => by_value)
        });
    }
    group.finish();
}

fn bench_ray_sphere_intersect_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide ray-sphere intersection");
    for size in [80000].iter() {
        group.throughput(Throughput::Elements(*size as u64));
        bench_glam_f32x1!(group, size, |b, size| {
            use glam::Vec3A;
            bench_intersection_scalar!(b, size, ty => Vec3A, zero => Vec3A::ZERO, norm => normalize, mag_sq => length_squared, param => by_value);
        });

        // sse
        bench_ultraviolet_f32x4!(group, size, |b, size| {
            use ultraviolet::{f32x4, Vec3x4};
            bench_intersection_wide_uv!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Vec3x4, wt => f32x4, zero_vec => Vec3x4::zero(), max => f32::MAX)
        });
        bench_nalgebra_f32x4!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            use simba::simd::{f32x4, SimdValue};
            bench_intersection_wide_na!(b, &((*size as f32 / 4.0).ceil() as usize), ty => Vector3<f32x4>, wt => f32x4, zero_vec => zero(), max => f32::MAX);
        });

        // avx
        bench_ultraviolet_f32x8!(group, size, |b, size| {
            use ultraviolet::{f32x8, Vec3x8};
            bench_intersection_wide_uv!(b, &((*size as f32 / 8.0).ceil() as usize), ty => Vec3x8, wt => f32x8, zero_vec => Vec3x8::zero(), max => f32::MAX)
        });
        bench_nalgebra_f32x8!(group, size, |b, size| {
            use nalgebra::{zero, Vector3};
            use simba::simd::{f32x8, SimdValue};
            bench_intersection_wide_na!(b, &((*size as f32 / 8.0).ceil() as usize), ty => Vector3<f32x8>, wt => f32x8, zero_vec => zero(), max => f32::MAX);
        });
    }
    group.finish();
}

criterion_group!(
    rsi_benches,
    bench_ray_sphere_intersect_scalar,
    bench_ray_sphere_intersect_wide,
);

criterion_main!(rsi_benches);
