#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_transform3d_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("transform3d return self");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => ret_self, ty => Transform3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => ret_self, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    group.finish();
}

fn bench_transform3d_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform3d inverse");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_unop!(b, op => try_inverse, ty => Transform3<f32>)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_unop!(b, op => inverse, ty => Transform3D<f32, UnknownUnit, UnknownUnit>)
    });
    group.finish();
}

fn bench_transform3d_mul_transform3d(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("transform3d mul transform3d");
    bench_nalgebra!(group, |b| {
        use nalgebra::Transform3;
        bench_binop!(b, op => mul, ty1 => Transform3<f32>, ty2 => Transform3<f32>, param => by_ref)
    });
    bench_euclid!(group, |b| {
        use euclid::{Transform3D, UnknownUnit};
        bench_binop!(b, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
    });
    group.finish();
}

criterion_group!(
    transform3d_benches,
    bench_transform3d_ret_self,
    bench_transform3d_inverse,
    bench_transform3d_mul_transform3d,
);
criterion_main!(transform3d_benches);
