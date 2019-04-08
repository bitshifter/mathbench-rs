use criterion::{criterion_group, criterion_main};

macro_rules! bench_unop {
    ($name: ident, $desc: expr, $t: ty, $unop: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe {
                        criterion::black_box(elems.get_unchecked(i).$unop())
                    }
                })
            });
        }
    };
}

macro_rules! bench_func1 {
    ($name: ident, $desc: expr, $t: ty, $func1: ident) => {
        pub(crate) fn $name(c: &mut Criterion) {
            use rand::{Rng, SeedableRng};
            use rand_xoshiro::Xoshiro256Plus;
            const LEN: usize = 1 << 13;

            let mut rng = Xoshiro256Plus::seed_from_u64(0);

            let elems: Vec<$t> = (0..LEN).map(|_| rng.gen::<$t>()).collect();
            let mut i = 0;

            c.bench_function($desc, move |b| {
                b.iter(|| {
                    i = (i + 1) & (LEN - 1);

                    unsafe {
                        criterion::black_box($func1(elems.get_unchecked(i)))
                    }
                })
            });
        }
    };
}

mod bench_cgmath {
    use criterion::Criterion;
    use cgmath::{prelude::*, Matrix4};
    bench_unop!(mat4_transpose, "cgmath mat4 transpose",  Matrix4<f32>, transpose);
    bench_unop!(mat4_determinant, "cgmath mat4 determinant",  Matrix4<f32>, determinant);
    bench_unop!(mat4_inverse, "cgmath mat4 inverse",  Matrix4<f32>, invert);
}

mod bench_glam {
    use criterion::Criterion;
    use glam::f32::Mat4;
    bench_unop!(mat4_transpose, "glam mat4 transpose",  Mat4, transpose);
    bench_unop!(mat4_determinant, "glam mat4 determinant",  Mat4, determinant);
    bench_unop!(mat4_inverse, "glam mat4 inverse",  Mat4, inverse);
}

mod bench_nalgebra {
    use criterion::Criterion;
    use nalgebra_glm::{determinant, inverse, transpose, Mat4};
    bench_func1!(mat4_transpose, "nalgebra mat4 transpose",  Mat4, transpose);
    bench_func1!(mat4_determinant, "nalgebra mat4 determinant",  Mat4, determinant);
    bench_func1!(mat4_inverse, "nalgebra mat4 inverse",  Mat4, inverse);
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
