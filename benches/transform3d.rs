#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_transform3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("scalar transform3 return self");
    bench_glam!(group, |b| {
        use glam::Affine3A;
        bench_unop!(b, op => ret_self, ty => Affine3A)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => ret_self, ty => Transform3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => ret_self, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_unop!(b, op => ret_self, ty => Transform4F)
    });
    group.finish();
}

fn bench_transform3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform3 inverse");
    bench_glam!(group, |b| {
        use glam::Affine3A;
        bench_unop!(b, op => inverse, ty => Affine3A)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => try_inverse, ty => Transform3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_unop!(b, op => inverse, ty => Transform4F)
    });
    group.finish();
}

fn bench_transform3_mul_transform3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("scalar transform3 mul transform3d");
    bench_glam!(group, |b| {
        use glam::Affine3A;
        bench_binop!(b, op => mul, ty1 => Affine3A, ty2 => Affine3A)
    });
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_binop!(b, op => mul, ty1 => Transform3<f32>, ty2 => Transform3<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => then, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform3d::Transform4F;
        bench_binop!(b, op => mul, ty1 => Transform4F, ty2 => Transform4F)
    });
    group.finish();
}

criterion_group!(
    transform3d_benches,
    bench_transform3_ret_self,
    bench_transform3_inverse,
    bench_transform3_mul_transform3,
);
criterion_main!(transform3d_benches);
