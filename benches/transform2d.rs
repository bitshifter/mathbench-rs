#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_transform2_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("scalar transform2 return self");
    bench!("glam", group, |b| {
        use glam::Affine2;
        bench_unop!(b, op => ret_self, ty => Affine2)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform2;
        bench_unop!(b, op => ret_self, ty => Transform2<f32>)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => ret_self, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => ret_self, ty => Transform2F)
    });
    group.finish();
}

fn bench_transform2_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("scalar transform2 inverse");
    bench!("glam", group, |b| {
        use glam::Affine2;
        bench_unop!(b, op => inverse, ty => Affine2)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform2;
        bench_unop!(b, op => try_inverse, ty => Transform2<f32>)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => inverse, ty => Transform2F)
    });
    group.finish();
}

fn bench_transform2_mul_transform2(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("scalar transform2 mul transform2");
    bench!("glam", group, |b| {
        use glam::Affine2;
        bench_binop!(b, op => mul, ty1 => Affine2, ty2 => Affine2)
    });
    bench!("nalgebra", group, |b| {
        use nalgebra::Transform2;
        bench_binop!(b, op => mul, ty1 => Transform2<f32>, ty2 => Transform2<f32>, param => by_ref)
    });
    bench!("euclid", group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_binop!(b, op => then, ty => Transform2D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench!("pathfinder_geometry" as "pathfinder", group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        use std::ops::Mul;
        bench_binop!(b, op => mul, ty1 => Transform2F, ty2 => Transform2F)
    });
    group.finish();
}

criterion_group!(
    transform2d_benches,
    bench_transform2_ret_self,
    bench_transform2_inverse,
    bench_transform2_mul_transform2,
);
criterion_main!(transform2d_benches);
