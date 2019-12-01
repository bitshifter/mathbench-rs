#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main, Criterion};

// returns self to check overhead of benchmark
fn bench_transform3_ret_self(c: &mut Criterion) {
    use mathbench::BenchValue;
    let mut group = c.benchmark_group("transform3 return self");
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

fn bench_transform3_inverse(c: &mut Criterion) {
    let mut group = c.benchmark_group("transform3 inverse");
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

fn bench_transform3_mul_transform3(c: &mut Criterion) {
    use std::ops::Mul;
    let mut group = c.benchmark_group("transform3 mul transform3d");
    for size in [1, 100].iter() {
        group.throughput(criterion::Throughput::Elements(*size as u64));
        bench_nalgebra!(group, size, |b, size| {
            use nalgebra::Transform3;
            bench_binop!(b, size, op => mul, ty1 => Transform3<f32>, ty2 => Transform3<f32>, param => by_ref)
        });
        bench_euclid!(group, size, |b, size| {
            use euclid::{Transform3D, UnknownUnit};
            bench_binop!(b, size, op => post_transform, ty => Transform3D<f32, UnknownUnit, UnknownUnit>, param => by_ref)
        });
    }
    group.finish();
}

criterion_group!(
    transform3d_benches,
    bench_transform3_ret_self,
    bench_transform3_inverse,
    bench_transform3_mul_transform3,
);
criterion_main!(transform3d_benches);
