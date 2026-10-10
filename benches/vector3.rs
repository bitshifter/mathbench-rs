#[path = "support/macros.rs"]
#[macro_use]
mod macros;
#[cfg(feature = "glam_fearless")]
#[allow(dead_code)] // shared by the ray-sphere and vector3 benches
#[path = "support/glam_fearless.rs"]
mod gf;

use criterion::{Criterion, criterion_group, criterion_main};
use mathbench::BenchValue;

// measures benchmark no-op baseline for this type
fn bench_vector3_ret_self(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 no-op baseline");
    bench!("glam", group, |b| {
        use glam::Vec3A;
        bench_unop!(b, op => ret_self, ty => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => ret_self, ty => Vector3<f32>)
    });

    bench!("vek", group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => ret_self, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_length(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 length");
    bench!("glam", group, |b| {
        use glam::Vec3A;
        bench_unop!(b, op => length, ty => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => mag, ty => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => magnitude, ty => Vector3<f32>)
    });

    bench!("euclid", group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => length, ty => Vector3D<f32, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => magnitude, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_length_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide vector3 length");
    bench!("glam_f32x1", group, |b| {
        use glam::Vec3A;
        bench_unop_wide!(b, width => 1, op => length, ty => Vec3A)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Vec3x4;
        bench_unop_wide!(b, width => 4, op => mag, ty => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => norm, ty => Vector3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Vec3x8;
        bench_unop_wide!(b, width => 8, op => mag, ty => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => norm, ty => Vector3<WideF32x8>)
    });
    bench!("glam_fearless", group, size, |b, size| {
        let size = *size as usize;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let a = gf::random_soa(size, &mut rng);
        let mut out = vec![0.0f32; size];
        let refs: [&[f32]; 3] = [&a[0], &a[1], &a[2]];
        b.iter(|| gf::length_soa_dispatch(refs, &mut out));
        std::hint::black_box(&out);
    });
    group.finish();
}

fn bench_vector3_normalize(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 normalize");
    bench!("glam", group, |b| {
        use glam::Vec3A;
        bench_unop!(b, op => normalize, ty => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Vector3;
        bench_unop!(b, op => normalize, ty => Vector3<f32>)
    });
    bench!("euclid", group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_unop!(b, op => normalize, ty => Vector3D<f32, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Vec3;
        bench_unop!(b, op => normalized, ty => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_normalize_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide vector3 normalize");
    bench!("glam_f32x1", group, |b| {
        use glam::Vec3A;
        bench_unop_wide!(b, width => 1, op => normalize, ty => Vec3A)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Vec3x4;
        bench_unop_wide!(b, width => 4, op => normalized, ty => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x4;
        bench_unop_wide!(b, width => 4, op => normalize, ty => Vector3<WideF32x4>)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Vec3x8;
        bench_unop_wide!(b, width => 8, op => normalized, ty => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x8;
        bench_unop_wide!(b, width => 8, op => normalize, ty => Vector3<WideF32x8>)
    });
    bench!("glam_fearless", group, size, |b, size| {
        let size = *size as usize;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let a = gf::random_soa(size, &mut rng);
        let mut ox = vec![0.0f32; size];
        let mut oy = vec![0.0f32; size];
        let mut oz = vec![0.0f32; size];
        let refs: [&[f32]; 3] = [&a[0], &a[1], &a[2]];
        b.iter(|| gf::normalize_soa_dispatch(refs, &mut ox, &mut oy, &mut oz));
        std::hint::black_box((&ox, &oy, &oz));
    });
    group.finish();
}

fn bench_vector3_dot(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 dot");
    bench!("glam", group, |b| {
        use glam::Vec3A;
        bench_binop!(b, op => dot, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::{InnerSpace, Vector3};
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3, ty2 => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => dot, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });

    bench!("euclid", group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => dot, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => dot, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_dot_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide vector3 dot");
    bench!("glam_f32x1", group, |b| {
        use glam::Vec3A;
        bench_binop_wide!(b, width => 1, op => dot, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Vec3x4;
        bench_binop_wide!(b, width => 4, op => dot, ty1 => Vec3x4, ty2 => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => dot, ty1 => Vector3<WideF32x4>, ty2 => Vector3<WideF32x4>, param => by_ref)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Vec3x8;
        bench_binop_wide!(b, width => 8, op => dot, ty1 => Vec3x8, ty2 => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => dot, ty1 => Vector3<WideF32x8>, ty2 => Vector3<WideF32x8>, param => by_ref)
    });
    bench!("glam_fearless", group, size, |b, size| {
        let size = *size as usize;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let a = gf::random_soa(size, &mut rng);
        let c = gf::random_soa(size, &mut rng);
        let mut out = vec![0.0f32; size];
        let arefs: [&[f32]; 3] = [&a[0], &a[1], &a[2]];
        let crefs: [&[f32]; 3] = [&c[0], &c[1], &c[2]];
        b.iter(|| gf::dot_soa_dispatch(arefs, crefs, &mut out));
        std::hint::black_box(&out);
    });
    group.finish();
}

fn bench_vector3_cross(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar vector3 cross");
    bench!("glam", group, |b| {
        use glam::Vec3A;
        bench_binop!(b, op => cross, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench!("cgmath", group, |b| {
        use cgmath::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>)
    });
    bench!("ultraviolet", group, |b| {
        use ultraviolet::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3, ty2 => Vec3)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Vector3;
        bench_binop!(b, op => cross, ty1 => Vector3<f32>, ty2 => Vector3<f32>, param => by_ref)
    });

    bench!("euclid", group, |b| {
        use euclid::{UnknownUnit, Vector3D};
        bench_binop!(b, op => cross, ty1 => Vector3D<f32, UnknownUnit>, ty2 => Vector3D<f32, UnknownUnit>)
    });
    bench!("vek", group, |b| {
        use vek::Vec3;
        bench_binop!(b, op => cross, ty1 => Vec3<f32>, ty2 => Vec3<f32>)
    });
    group.finish();
}

fn bench_vector3_cross_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide vector3 cross");
    bench!("glam_f32x1", group, |b| {
        use glam::Vec3A;
        bench_binop_wide!(b, width => 1, op => cross, ty1 => Vec3A, ty2 => Vec3A)
    });
    bench!("ultraviolet_f32x4", group, |b| {
        use ultraviolet::Vec3x4;
        bench_binop_wide!(b, width => 4, op => cross, ty1 => Vec3x4, ty2 => Vec3x4)
    });
    bench!("nalgebra_f32x4", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x4;
        bench_binop_wide!(b, width => 4, op => cross, ty1 => Vector3<WideF32x4>, ty2 => Vector3<WideF32x4>, param => by_ref)
    });
    bench!("ultraviolet_f32x8", group, |b| {
        use ultraviolet::Vec3x8;
        bench_binop_wide!(b, width => 8, op => cross, ty1 => Vec3x8, ty2 => Vec3x8)
    });
    bench!("nalgebra_f32x8", group, |b| {
        use nalgebra::Vector3;
        use simba::simd::WideF32x8;
        bench_binop_wide!(b, width => 8, op => cross, ty1 => Vector3<WideF32x8>, ty2 => Vector3<WideF32x8>, param => by_ref)
    });
    bench!("glam_fearless", group, size, |b, size| {
        let size = *size as usize;
        let mut rng = rand_pcg::Pcg64Mcg::new(rand::random());
        let a = gf::random_soa(size, &mut rng);
        let c = gf::random_soa(size, &mut rng);
        let mut ox = vec![0.0f32; size];
        let mut oy = vec![0.0f32; size];
        let mut oz = vec![0.0f32; size];
        let arefs: [&[f32]; 3] = [&a[0], &a[1], &a[2]];
        let crefs: [&[f32]; 3] = [&c[0], &c[1], &c[2]];
        b.iter(|| gf::cross_soa_dispatch(arefs, crefs, &mut ox, &mut oy, &mut oz));
        std::hint::black_box((&ox, &oy, &oz));
    });
    group.finish();
}

criterion_group!(
    vector3_benches,
    bench_vector3_ret_self,
    bench_vector3_length,
    bench_vector3_length_wide,
    bench_vector3_normalize,
    bench_vector3_normalize_wide,
    bench_vector3_dot,
    bench_vector3_dot_wide,
    bench_vector3_cross,
    bench_vector3_cross_wide,
);
criterion_main!(vector3_benches);
