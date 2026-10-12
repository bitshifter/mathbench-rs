#[path = "support/macros.rs"]
#[macro_use]
mod macros;
#[cfg(feature = "glam_fearless")]
#[allow(dead_code)] // shared with the other benches
#[path = "support/glam_fearless.rs"]
mod gf;

use criterion::{Criterion, criterion_group, criterion_main};

// measures benchmark no-op baseline for this type
fn bench_transform3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("scalar transform3 no-op baseline");
    bench!("glam", group, |b| {
        use glam::Affine3A;
        bench_unop!(b, op => ret_self, ty => Affine3A)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => ret_self, ty => Transform3<f32>)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => ret_self, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_unop!(b, op => ret_self, ty => Transform4F)
    });
    group.finish();
}

fn bench_transform3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform3 inverse");
    bench!("glam", group, |b| {
        use glam::Affine3A;
        bench_unop!(b, op => inverse, ty => Affine3A)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => try_inverse, ty => Transform3<f32>)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_unop!(b, op => inverse, ty => Transform4F)
    });
    group.finish();
}

fn bench_transform3_mul_transform3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("scalar transform3 mul transform3d");
    bench!("glam", group, |b| {
        use glam::Affine3A;
        bench_binop!(b, op => mul, ty1 => Affine3A, ty2 => Affine3A)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform3;
        bench_binop!(b, op => mul, ty1 => Transform3<f32>, ty2 => Transform3<f32>, param => by_ref)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_binop!(b, op => mul, ty1 => Transform4F, ty2 => Transform4F)
    });
    group.finish();
}

fn bench_transform3_inverse_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform3 inverse");
    bench!("glam_fearless", group, |b| {
        macros::bench_workload(
            b,
            &[16, 1024],
            |rng, size| (
                gf::AffineUnopEntry::new(),
                gf::random_affine_pool(size, rng),
                gf::zero_pool(12, size),
            ),
            |s| {
                let (inv, a, out) = s;
                inv.call(a, out);
                std::hint::black_box(&*out);
            },
        )
    });
    group.finish();
}

fn bench_transform3_mul_transform3_wide(c: &mut Criterion) {
    let mut group = c.benchmark_group("wide transform3 mul transform3");
    bench!("glam_fearless", group, |b| {
        macros::bench_workload(
            b,
            &[16, 1024],
            |rng, size| (
                gf::AffineMulEntry::new(),
                gf::random_affine_pool(size, rng),
                gf::random_affine_pool(size, rng),
                gf::zero_pool(12, size),
            ),
            |s| {
                let (mul, a, c, out) = s;
                mul.call(a, c, out);
                std::hint::black_box(&*out);
            },
        )
    });
    group.finish();
}

criterion_group!(
    transform3d_benches,
    bench_transform3_ret_self,
    bench_transform3_inverse,
    bench_transform3_mul_transform3,
    bench_transform3_inverse_wide,
    bench_transform3_mul_transform3_wide,
);
criterion_main!(transform3d_benches);
