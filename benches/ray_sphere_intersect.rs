#[path = "support/macros.rs"]
#[macro_use]
mod macros;

use criterion::{Criterion, criterion_group, criterion_main};

#[cfg(any(feature = "ultraviolet_f32x4", feature = "ultraviolet_f32x8",))]
macro_rules! bench_intersection_wide_uv {
    ($b: ident, $size:expr, width => $width:expr, ty => $t: ty, wt => $wt: ident, zero_vec => $zero: expr, max => $max: expr) => {{
        use wide_v07::CmpGt;
        // `$size` is a scalar element count; the pools are sized per lane and
        // the benchmark id stays in elements so all widths are comparable.
        macros::bench_workload(
            $b,
            &[*$size],
            |rng, size| {
                let count = usize::div_ceil(size, $width);
                let ray_d = (0..count)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng).normalized())
                    .collect::<Vec<$t>>();
                let result: Vec<$wt> = vec![$wt::splat(0.0); count];
                let ray_o: $t = $zero;
                let sphere_o: $t = $zero;
                let sphere_r_sq = $wt::splat(100.0);
                let miss = $wt::splat($max);
                (ray_d, result, ray_o, sphere_o, sphere_r_sq, miss)
            },
            |data| {
                let (ray_d, result, ray_o, sphere_o, sphere_r_sq, miss) = data;
                let (ray_o, sphere_o, sphere_r_sq, miss) = (*ray_o, *sphere_o, *sphere_r_sq, *miss);

                // `#[inline(never)]` keeps the autovectorizer from inlining the inner
                // operation into the benchmark loop, so the loop stays scalar. This is
                // a stable equivalent of `#[inline(never)]` on a closure.
                #[inline(never)]
                fn do_inner(
                    ray_d: &$t,
                    result: &mut $wt,
                    ray_o: $t,
                    sphere_o: $t,
                    sphere_r_sq: $wt,
                    miss: $wt,
                ) {
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
                }

                for (ray_d, result) in ray_d.iter().zip(result.iter_mut()) {
                    do_inner(ray_d, result, ray_o, sphere_o, sphere_r_sq, miss);
                }
            },
        )
    }};
}

#[cfg(any(feature = "nalgebra_f32x4", feature = "nalgebra_f32x8",))]
macro_rules! bench_intersection_wide_na {
    ($b: ident, $size:expr, width => $width:expr, ty => $t: ty, wt => $wt: ident, zero_vec => $zero: expr, max => $max: expr) => {{
        use simba::simd::{SimdBool, SimdComplexField, SimdPartialOrd, SimdValue};
        // `$size` is a scalar element count; the pools are sized per lane and
        // the benchmark id stays in elements so all widths are comparable.
        macros::bench_workload(
            $b,
            &[*$size],
            |rng, size| {
                let count = usize::div_ceil(size, $width);
                let ray_d = (0..count)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng).normalize())
                    .collect::<Vec<$t>>();
                let result: Vec<$wt> = vec![$wt::splat(0.0); count];
                let ray_o: $t = $zero;
                let sphere_o: $t = $zero;
                let sphere_r_sq = $wt::splat(100.0);
                let miss = $wt::splat($max);
                (ray_d, result, ray_o, sphere_o, sphere_r_sq, miss)
            },
            |data| {
                let (ray_d, result, ray_o, sphere_o, sphere_r_sq, miss) = data;
                let (ray_o, sphere_o, sphere_r_sq, miss) = (*ray_o, *sphere_o, *sphere_r_sq, *miss);

                #[inline(never)]
                fn do_inner(
                    ray_d: &$t,
                    result: &mut $wt,
                    ray_o: $t,
                    sphere_o: $t,
                    sphere_r_sq: $wt,
                    miss: $wt,
                ) {
                    let oc: $t = ray_o - sphere_o;
                    let b = oc.dot(ray_d);
                    let c = oc.norm_squared() - sphere_r_sq;
                    let descrim = b * b - c;

                    let z = $wt::splat(0.0);
                    let desc_pos = descrim.simd_gt(z);

                    let desc_sqrt = descrim.simd_sqrt();

                    let t1 = -b - desc_sqrt;
                    let t1_valid = t1.simd_gt(z) & desc_pos;

                    let t2 = -b + desc_sqrt;
                    let t2_valid = t2.simd_gt(z) & desc_pos;

                    let t = t2_valid.if_else(|| t2, || miss);
                    let t = t1_valid.if_else(|| t1, || t);

                    *result = t;
                }

                for (ray_d, result) in ray_d.iter().zip(result.iter_mut()) {
                    do_inner(ray_d, result, ray_o, sphere_o, sphere_r_sq, miss);
                }
            },
        )
    }};
}

#[cfg(feature = "glam_fearless")]
#[allow(dead_code)] // shared by the ray-sphere and vector3 benches
#[path = "support/glam_fearless.rs"]
mod gf;

macro_rules! bench_intersection_scalar {
    ($b: ident, $size:expr, width => $width:expr, ty => $t: ty, zero => $zero: expr, norm => $norm: ident, mag_sq => $mag_sq: ident, param => $param: tt) => {{
        // `$size` is a scalar element count; the pools are sized per lane and
        // the benchmark id stays in elements so all widths are comparable.
        macros::bench_workload(
            $b,
            &[*$size],
            |rng, size| {
                let count = usize::div_ceil(size, $width);
                let ray_d = (0..count)
                    .map(|_| <$t as mathbench::BenchValue>::random_value(rng).$norm())
                    .collect::<Vec<$t>>();
                let result: Vec<f32> = vec![0.0f32; count];
                let ray_o: $t = <$t>::new(0.0, 0.0, -11.0);
                let sphere_o: $t = $zero;
                let sphere_r_sq: f32 = 100.0f32;
                (ray_d, result, ray_o, sphere_o, sphere_r_sq)
            },
            |data| {
                let (ray_d, result, ray_o, sphere_o, sphere_r_sq) = data;
                let (ray_o, sphere_o, sphere_r_sq) = (*ray_o, *sphere_o, *sphere_r_sq);

                #[inline(never)]
                fn do_inner(
                    ray_d: &$t,
                    result: &mut f32,
                    ray_o: $t,
                    sphere_o: $t,
                    sphere_r_sq: f32,
                ) {
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
                            if t2 > 0.0 { t2 } else { f32::MAX }
                        }
                    } else {
                        f32::MAX
                    };
                }

                for (ray_d, result) in ray_d.iter().zip(result.iter_mut()) {
                    do_inner(ray_d, result, ray_o, sphere_o, sphere_r_sq);
                }
            },
        )
    }};
}

fn bench_ray_sphere_intersect_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar ray-sphere intersection");
    {
        let size = &10000;
        bench!("glam", group, size, |b, size| {
            use glam::Vec3A;
            bench_intersection_scalar!(b, size, width => 1, ty => Vec3A, zero => Vec3A::ZERO, norm => normalize, mag_sq => length_squared, param => by_value);
        });
        bench!("cgmath", group, size, |b, size| {
            use cgmath::{Vector3, prelude::*};
            bench_intersection_scalar!(b, size, width => 1, ty => Vector3<f32>, zero => Vector3::zero(), norm => normalize, mag_sq => magnitude2, param => by_value)
        });
        bench!("ultraviolet", group, size, |b, size| {
            use ultraviolet::Vec3;
            bench_intersection_scalar!(b, size, width => 1, ty => Vec3, zero => Vec3::zero(), norm => normalized, mag_sq => mag_sq, param => by_value);
        });
        bench!("nalgebra", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            bench_intersection_scalar!(b, size, width => 1, ty => Vector3<f32>, zero => zero(), norm => normalize, mag_sq => norm_squared, param => by_ref);
        });
        bench!("euclid", group, size, |b, size| {
            use euclid::{UnknownUnit, Vector3D};
            bench_intersection_scalar!(b, size, width => 1, ty => Vector3D<f32, UnknownUnit>, zero => Vector3D::zero(), norm => normalize, mag_sq => square_length, param => by_value);
        });
        bench!("vek", group, size, |b, size| {
            use vek::Vec3;
            bench_intersection_scalar!(b, size, width => 1, ty => Vec3<f32>, zero => Vec3::zero(), norm => normalized, mag_sq => magnitude_squared, param => by_value)
        });
    }
    group.finish();
}

fn bench_ray_sphere_intersect_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide ray-sphere intersection");
    {
        let size = &80000;
        bench!("glam_f32x1", group, size, |b, size| {
            use glam::Vec3A;
            bench_intersection_scalar!(b, size, width => 1, ty => Vec3A, zero => Vec3A::ZERO, norm => normalize, mag_sq => length_squared, param => by_value);
        });

        // sse
        bench!("ultraviolet_f32x4", group, size, |b, size| {
            use ultraviolet::{Vec3x4, f32x4};
            bench_intersection_wide_uv!(b, size, width => 4, ty => Vec3x4, wt => f32x4, zero_vec => Vec3x4::zero(), max => f32::MAX)
        });
        bench!("nalgebra_f32x4", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            use simba::simd::WideF32x4;
            bench_intersection_wide_na!(b, size, width => 4, ty => Vector3<WideF32x4>, wt => WideF32x4, zero_vec => zero(), max => f32::MAX);
        });

        // avx
        bench!("ultraviolet_f32x8", group, size, |b, size| {
            use ultraviolet::{Vec3x8, f32x8};
            bench_intersection_wide_uv!(b, size, width => 8, ty => Vec3x8, wt => f32x8, zero_vec => Vec3x8::zero(), max => f32::MAX)
        });
        bench!("nalgebra_f32x8", group, size, |b, size| {
            use nalgebra::{Vector3, zero};
            use simba::simd::WideF32x8;
            bench_intersection_wide_na!(b, size, width => 8, ty => Vector3<WideF32x8>, wt => WideF32x8, zero_vec => zero(), max => f32::MAX);
        });

        // Runtime-dispatched width: the same `size` rays, SoA components, one kernel body.
        bench!("glam_fearless", group, size, |b, size| {
            let size = *size;
            let name = b.1;
            let mut rng = rand_pcg::Pcg64Mcg::new(macros::BENCH_RNG_SEED);
            let dirs = gf::random_unit_soa(size, &mut rng);
            let mut out = vec![0.0f32; size];
            let refs: [&[f32]; 3] = [&dirs[0], &dirs[1], &dirs[2]];
            b.0.bench_function(criterion::BenchmarkId::new(name, size), |bencher| {
                bencher.iter(|| {
                    gf::intersect_soa_dispatch(refs, &mut out);
                    std::hint::black_box(&out);
                })
            });
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
