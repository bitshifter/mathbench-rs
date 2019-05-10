#[path = "support/macros.rs"]
#[macro_use]
mod macros;
use criterion::{criterion_group, criterion_main};

mod bench_cgmath {
    use criterion::Criterion;
    use cgmath::{prelude::*, Matrix4};
    bench_unop!(mat4_transpose, "cgmath mat4 transpose", op => transpose, ty => Matrix4<f32>);
    bench_unop!(mat4_determinant, "cgmath mat4 determinant", op => determinant, ty => Matrix4<f32>);
    bench_unop!(mat4_inverse, "cgmath mat4 inverse",  op => invert, ty => Matrix4<f32>);
}

mod bench_glam {
    use criterion::Criterion;
    use glam::f32::Mat4;
    bench_unop!(mat4_transpose, "glam mat4 transpose",  op => transpose, ty => Mat4);
    bench_unop!(mat4_determinant, "glam mat4 determinant",  op => determinant, ty => Mat4);
    bench_unop!(mat4_inverse, "glam mat4 inverse",  op => inverse, ty => Mat4);
}

mod bench_nalgebra {
    use criterion::Criterion;
    use nalgebra_glm::{determinant, inverse, transpose, Mat4};
    bench_func!(mat4_transpose, "nalgebra mat4 transpose",  op => transpose, ty => Mat4);
    bench_func!(mat4_determinant, "nalgebra mat4 determinant",  op => determinant, ty => Mat4);
    bench_func!(mat4_inverse, "nalgebra mat4 inverse",  op => inverse, ty => Mat4);
}

criterion_group!(
    mat4_benches,
    bench_cgmath::mat4_transpose,
    bench_cgmath::mat4_determinant,
    bench_cgmath::mat4_inverse,
    bench_glam::mat4_transpose,
    bench_glam::mat4_determinant,
    bench_glam::mat4_inverse,
    bench_nalgebra::mat4_transpose,
    bench_nalgebra::mat4_determinant,
    bench_nalgebra::mat4_inverse,
);
criterion_main!(mat4_benches);
