//! `glam_fearless` support: SoA kernels plus runtime dispatch wrappers.
//!
//! `glam_fearless` values carry their SIMD token, so they cannot be constructed without one
//! (and the width is chosen at runtime), which is why these entries take SoA component slices
//! and dispatch per call instead of using the fixed-width `bench_*_wide!` macros. The dispatch
//! match is negligible next to the kernel (measured: hoisting it changes the results by <=10%).

use glam_fearless::{Level, Vec3xN, dispatch, prelude::*, simd};
use mathbench::BenchValue;
use rand_pcg::Pcg64Mcg;

/// Random SoA components, kept away from the origin so `normalize` stays conditioned.
pub fn random_soa(size: usize, rng: &mut Pcg64Mcg) -> [Vec<f32>; 3] {
    let mut out = [vec![0.0f32; size], vec![0.0f32; size], vec![0.0f32; size]];
    for i in 0..size {
        let mut v = glam::Vec3::random_value(rng);
        if v.length() < 0.5 {
            v += glam::Vec3::ONE;
        }
        out[0][i] = v.x;
        out[1][i] = v.y;
        out[2][i] = v.z;
    }
    out
}

/// Random unit SoA directions.
pub fn random_unit_soa(size: usize, rng: &mut Pcg64Mcg) -> [Vec<f32>; 3] {
    let mut out = [vec![0.0f32; size], vec![0.0f32; size], vec![0.0f32; size]];
    for i in 0..size {
        let v = glam::Vec3::random_value(rng).normalize();
        out[0][i] = v.x;
        out[1][i] = v.y;
        out[2][i] = v.z;
    }
    out
}

/// Ray/sphere test: sphere at the origin with radius 10, ray origin (0, 0, -11).
#[simd]
#[inline(always)]
pub fn intersect_soa<S: Simd>(simd: S, dirs: [&[f32]; 3], out: &mut [f32]) {
    let n = Vec3xN::<S>::LANES;
    let oc = Vec3xN::splat_xyz(simd, 0.0, 0.0, -11.0);
    let r_sq = S::f32s::splat(simd, 100.0);
    let miss = S::f32s::splat(simd, f32::MAX);

    let ins = dirs[0]
        .chunks_exact(n)
        .zip(dirs[1].chunks_exact(n))
        .zip(dirs[2].chunks_exact(n));
    for (o, ((x, y), z)) in out.chunks_mut(n).zip(ins) {
        let d = Vec3xN::from_slice(simd, x, y, z);
        let b = oc.dot(d);
        let c = oc.length_squared() - r_sq;
        let discr = b * b - c;

        let pos = discr.simd_gt(0.0);
        let discr_sqrt = discr.sqrt();

        let t1 = -b - discr_sqrt;
        let t2 = -b + discr_sqrt;

        let t = (t2.simd_gt(0.0) & pos).select(t2, miss);
        let t = (t1.simd_gt(0.0) & pos).select(t1, t);

        t.store_slice(o);
    }
}

/// Per-lane length.
#[simd]
#[inline(always)]
pub fn length_soa<S: Simd>(simd: S, a: [&[f32]; 3], out: &mut [f32]) {
    let n = Vec3xN::<S>::LANES;
    let ins = a[0]
        .chunks_exact(n)
        .zip(a[1].chunks_exact(n))
        .zip(a[2].chunks_exact(n));
    for (o, ((x, y), z)) in out.chunks_mut(n).zip(ins) {
        let v = Vec3xN::from_slice(simd, x, y, z);
        v.length().store_slice(o);
    }
}

/// Per-lane normalize.
#[simd]
#[inline(always)]
pub fn normalize_soa<S: Simd>(
    simd: S,
    a: [&[f32]; 3],
    ox: &mut [f32],
    oy: &mut [f32],
    oz: &mut [f32],
) {
    let n = Vec3xN::<S>::LANES;
    let ins = a[0]
        .chunks_exact(n)
        .zip(a[1].chunks_exact(n))
        .zip(a[2].chunks_exact(n));
    let outs = ox.chunks_mut(n).zip(oy.chunks_mut(n)).zip(oz.chunks_mut(n));
    for (((o0, o1), o2), ((x, y), z)) in outs.zip(ins) {
        let v = Vec3xN::from_slice(simd, x, y, z).normalize();
        v.x.store_slice(o0);
        v.y.store_slice(o1);
        v.z.store_slice(o2);
    }
}

/// Per-lane dot product.
#[simd]
#[inline(always)]
pub fn dot_soa<S: Simd>(simd: S, a: [&[f32]; 3], b: [&[f32]; 3], out: &mut [f32]) {
    let n = Vec3xN::<S>::LANES;
    let ia = a[0]
        .chunks_exact(n)
        .zip(a[1].chunks_exact(n))
        .zip(a[2].chunks_exact(n));
    let ib = b[0]
        .chunks_exact(n)
        .zip(b[1].chunks_exact(n))
        .zip(b[2].chunks_exact(n));
    for (o, (((ax, ay), az), ((bx, by), bz))) in out.chunks_mut(n).zip(ia.zip(ib)) {
        let va = Vec3xN::from_slice(simd, ax, ay, az);
        let vb = Vec3xN::from_slice(simd, bx, by, bz);
        va.dot(vb).store_slice(o);
    }
}

/// Per-lane cross product.
#[simd]
#[inline(always)]
pub fn cross_soa<S: Simd>(
    simd: S,
    a: [&[f32]; 3],
    b: [&[f32]; 3],
    ox: &mut [f32],
    oy: &mut [f32],
    oz: &mut [f32],
) {
    let n = Vec3xN::<S>::LANES;
    let ia = a[0]
        .chunks_exact(n)
        .zip(a[1].chunks_exact(n))
        .zip(a[2].chunks_exact(n));
    let ib = b[0]
        .chunks_exact(n)
        .zip(b[1].chunks_exact(n))
        .zip(b[2].chunks_exact(n));
    let outs = ox.chunks_mut(n).zip(oy.chunks_mut(n)).zip(oz.chunks_mut(n));
    for (((o0, o1), o2), (((ax, ay), az), ((bx, by), bz))) in outs.zip(ia.zip(ib)) {
        let va = Vec3xN::from_slice(simd, ax, ay, az);
        let vb = Vec3xN::from_slice(simd, bx, by, bz);
        let r = va.cross(vb);
        r.x.store_slice(o0);
        r.y.store_slice(o1);
        r.z.store_slice(o2);
    }
}

macro_rules! dispatch_wrapper {
    ($name:ident, $kernel:ident) => {
        /// Run the kernel at the best level this CPU supports.
        pub fn $name(dirs: [&[f32]; 3], out: &mut [f32]) {
            let level = Level::new();
            dispatch!(level, simd => $kernel(simd, dirs, out))
        }
    };
}

dispatch_wrapper!(intersect_soa_dispatch, intersect_soa);
dispatch_wrapper!(length_soa_dispatch, length_soa);

/// Run the kernel at the best level this CPU supports.
pub fn normalize_soa_dispatch(
    a: [&[f32]; 3],
    ox: &mut [f32],
    oy: &mut [f32],
    oz: &mut [f32],
) {
    let level = Level::new();
    dispatch!(level, simd => normalize_soa(simd, a, ox, oy, oz))
}

/// Run the kernel at the best level this CPU supports.
pub fn dot_soa_dispatch(a: [&[f32]; 3], b: [&[f32]; 3], out: &mut [f32]) {
    let level = Level::new();
    dispatch!(level, simd => dot_soa(simd, a, b, out))
}

/// Run the kernel at the best level this CPU supports.
pub fn cross_soa_dispatch(
    a: [&[f32]; 3],
    b: [&[f32]; 3],
    ox: &mut [f32],
    oy: &mut [f32],
    oz: &mut [f32],
) {
    let level = Level::new();
    dispatch!(level, simd => cross_soa(simd, a, b, ox, oy, oz))
}
