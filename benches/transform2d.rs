#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// Note that euclid doesn't have 3x3 matrix, it has Transform2D which is a
// stored as 3x2 matrix internally. It is included here as a 3x3 matrix is the
// closest point of comparison, but euclid can shortcut some things compared to
// a 3x3 matrix, for example it's determinant and inverse are cheaper.

fn bench_transform2d_nop(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("transform2d return self");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform2;
        bench_unop!(b, op => nop_fn, ty => Transform2<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => nop_fn, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => nop_fn, ty => Transform2F)
    });
    group.finish();
}

fn bench_transform2d_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform2d inverse");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform2;
        bench_unop!(b, op => try_inverse, ty => Transform2<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform2D<f32, UnknownUnit, UnknownUnit>)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        bench_unop!(b, op => inverse, ty => Transform2F)
    });
    group.finish();
}

fn bench_transform2d_mul_transform2d(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("transform2d mul transform2d");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform2;
        bench_binop!(b, op => mul, ty1 => Transform2<f32>, ty2 => Transform2<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform2D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform2D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    bench_pathfinder!(group, |b| {
        use pathfinder_geometry::transform2d::Transform2F;
        use std::ops::Mul;
        bench_binop!(b, op => mul, ty1 => Transform2F, ty2 => Transform2F)
    });
    group.finish();
}

criterion_group!(
    transform2d_benches,
    bench_transform2d_nop,
    bench_transform2d_inverse,
    bench_transform2d_mul_transform2d,
);
criterion_main!(transform2d_benches);
