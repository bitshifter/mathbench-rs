#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main};

mod bench_cgmath {
    use criterion::Criterion;
    use cgmath::{Matrix4, Vector4};
    use std::ops::Mul;
    bench_binop!(matrix4_mul_vector4, "cgmath Matrix4 mul Vector4", op => mul, ty1 => Matrix4<f32>, ty2 => Vector4<f32>);
}

mod bench_glam {
    use criterion::Criterion;
    use glam::f32::{Mat4, Vec4};
    use std::ops::Mul;
    bench_binop!(vec4_mul_mat4, "glam Vec4 mul Mat4", op => mul, ty1 => Vec4, ty2 => Mat4);
}

mod bench_nalgebra {
    use criterion::Criterion;
    use nalgebra_glm::{Vec4, Mat4};
    use std::ops::Mul;
    bench_binop!(mat4_mul_vec4, "nalgebra Mat4 mul Vec4", op => mul, ty1 => Mat4, ty2 => Vec4);
}

criterion_group!(
    transform_benches,
    bench_cgmath::matrix4_mul_vector4,
    bench_glam::vec4_mul_mat4,
    bench_nalgebra::mat4_mul_vec4,
);
criterion_main!(transform_benches);
