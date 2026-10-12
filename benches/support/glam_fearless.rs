//! `glam_fearless` support: SoA kernels plus one entry point per kernel.
//!
//! `glam_fearless` values carry their SIMD token, so they cannot be constructed without one (and
//! the width is chosen at runtime), which is why these entries take SoA component slices instead of
//! using the fixed-width `bench_*_wide!` macros.
//!
//! Each kernel is a plain generic body over `S: Simd`, wrapped by `dispatch_entry!`. The pool
//! builds the entry, which detects the level once, and the timed call goes straight to the kernel
//! for that level: no match, no closure, and when the build enables the level's features, no
//! indirect call either.
//!
//! Comparisons against the wide entries of other libraries are apples-for-apples only when the
//! build enables AVX2 for them: without `-C target-cpu=native` (or an explicit AVX2 target) the
//! `ultraviolet_f32x8` and `nalgebra_f32x8` entries are emulated as two `f32x4` halves and
//! understate what those libraries can do on this hardware. That is the problem `glam_fearless`
//! exists to solve rather than a caveat about it: it selects the width at run time, so one binary
//! covers whatever the CPU supports, instead of the caller writing an x4 and an x8 path by hand
//! or shipping a separate binary per SIMD level.

use glam_fearless::{Affine3xN, Level, Mat3xN, QuatxN, Vec2xN, Vec3xN, dispatch_entry, prelude::*};

/// Euler integration update rate, matching the other euler entries.
pub const UPDATE_RATE: f32 = 1.0 / 60.0;
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

/// Borrow the two component slices of a packed SoA batch for a kernel call.
pub fn refs2(a: &[Vec<f32>; 2]) -> [&[f32]; 2] {
    [&a[0], &a[1]]
}

/// Borrow the three component slices of a packed SoA batch for a kernel call.
pub fn refs3(a: &[Vec<f32>; 3]) -> [&[f32]; 3] {
    [&a[0], &a[1], &a[2]]
}

/// Borrow two component arrays as mutable slices.
pub fn refs2_mut(a: &mut [Vec<f32>; 2]) -> [&mut [f32]; 2] {
    let [a0, a1] = a;
    [a0.as_mut_slice(), a1.as_mut_slice()]
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
///
/// A plain generic body, so `dispatch_entry!` can wrap it as well as `dispatch!`.
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

/// Random 2-component SoA.
pub fn random_soa2(size: usize, rng: &mut Pcg64Mcg) -> [Vec<f32>; 2] {
    let mut out = [vec![0.0f32; size], vec![0.0f32; size]];
    for i in 0..size {
        let v = glam::Vec2::random_value(rng);
        out[0][i] = v.x;
        out[1][i] = v.y;
    }
    out
}

/// One Euler integration step for 3D vectors, in place.
#[inline(always)]
pub fn euler3_soa<S: Simd>(simd: S, acc: [&[f32]; 3], vel: [&mut [f32]; 3], pos: [&mut [f32]; 3]) {
    let n = Vec3xN::<S>::LANES;
    let dt = Vec3xN::splat(simd, UPDATE_RATE);

    let [ax, ay, az] = acc;
    let [vx, vy, vz] = vel;
    let [px, py, pz] = pos;
    let ins = ax
        .chunks_exact(n)
        .zip(ay.chunks_exact(n))
        .zip(az.chunks_exact(n));
    let vels = vx
        .chunks_exact_mut(n)
        .zip(vy.chunks_exact_mut(n))
        .zip(vz.chunks_exact_mut(n));
    let poss = px
        .chunks_exact_mut(n)
        .zip(py.chunks_exact_mut(n))
        .zip(pz.chunks_exact_mut(n));

    for ((((ax, ay), az), ((vx, vy), vz)), ((px, py), pz)) in ins.zip(vels).zip(poss) {
        let a = Vec3xN::from_slice(simd, ax, ay, az);
        let v = Vec3xN::from_slice(simd, vx, vy, vz);
        let p = Vec3xN::from_slice(simd, px, py, pz);

        let v = v + a * dt;
        let p = p + v * dt;

        v.x.store_slice(vx);
        v.y.store_slice(vy);
        v.z.store_slice(vz);
        p.x.store_slice(px);
        p.y.store_slice(py);
        p.z.store_slice(pz);
    }
}

/// One Euler integration step for 2D vectors, in place.
#[inline(always)]
pub fn euler2_soa<S: Simd>(simd: S, acc: [&[f32]; 2], vel: [&mut [f32]; 2], pos: [&mut [f32]; 2]) {
    let n = Vec2xN::<S>::LANES;
    let dt = Vec2xN::splat(simd, UPDATE_RATE);

    let [ax, ay] = acc;
    let [vx, vy] = vel;
    let [px, py] = pos;
    let ins = ax.chunks_exact(n).zip(ay.chunks_exact(n));
    let vels = vx.chunks_exact_mut(n).zip(vy.chunks_exact_mut(n));
    let poss = px.chunks_exact_mut(n).zip(py.chunks_exact_mut(n));

    for (((ax, ay), (vx, vy)), (px, py)) in ins.zip(vels).zip(poss) {
        let a = Vec2xN::from_slice(simd, ax, ay);
        let v = Vec2xN::from_slice(simd, vx, vy);
        let p = Vec2xN::from_slice(simd, px, py);

        let v = v + a * dt;
        let p = p + v * dt;

        v.x.store_slice(vx);
        v.y.store_slice(vy);
        p.x.store_slice(px);
        p.y.store_slice(py);
    }
}

/// Random unit quaternions as SoA components.
pub fn random_quat_soa(size: usize, rng: &mut Pcg64Mcg) -> [Vec<f32>; 4] {
    let mut out = [
        vec![0.0f32; size],
        vec![0.0f32; size],
        vec![0.0f32; size],
        vec![0.0f32; size],
    ];
    for i in 0..size {
        let q = glam::Quat::random_value(rng).normalize();
        out[0][i] = q.x;
        out[1][i] = q.y;
        out[2][i] = q.z;
        out[3][i] = q.w;
    }
    out
}

/// Four zeroed component arrays, for quaternion outputs.
pub fn zero_soa4(size: usize) -> [Vec<f32>; 4] {
    [
        vec![0.0f32; size],
        vec![0.0f32; size],
        vec![0.0f32; size],
        vec![0.0f32; size],
    ]
}

/// Borrow four component arrays as slices.
pub fn refs4(a: &[Vec<f32>; 4]) -> [&[f32]; 4] {
    [&a[0], &a[1], &a[2], &a[3]]
}

/// Borrow four component arrays as mutable slices.
pub fn refs4_mut(a: &mut [Vec<f32>; 4]) -> [&mut [f32]; 4] {
    let [a0, a1, a2, a3] = a;
    [
        a0.as_mut_slice(),
        a1.as_mut_slice(),
        a2.as_mut_slice(),
        a3.as_mut_slice(),
    ]
}

/// Borrow three component arrays as mutable slices.
pub fn refs3_mut(a: &mut [Vec<f32>; 3]) -> [&mut [f32]; 3] {
    let [a0, a1, a2] = a;
    [a0.as_mut_slice(), a1.as_mut_slice(), a2.as_mut_slice()]
}

/// Quaternion conjugate, per lane.
#[inline(always)]
pub fn quat_conjugate_soa<S: Simd>(simd: S, q: [&[f32]; 4], out: [&mut [f32]; 4]) {
    let n = S::f32s::LEN;
    let [x, y, z, w] = q;
    let [ox, oy, oz, ow] = out;
    let ins = x
        .chunks_exact(n)
        .zip(y.chunks_exact(n))
        .zip(z.chunks_exact(n))
        .zip(w.chunks_exact(n));
    let outs = ox
        .chunks_exact_mut(n)
        .zip(oy.chunks_exact_mut(n))
        .zip(oz.chunks_exact_mut(n))
        .zip(ow.chunks_exact_mut(n));
    for ((((i0, i1), i2), i3), (((o0, o1), o2), o3)) in ins.zip(outs) {
        let r = QuatxN::from_slice(simd, i0, i1, i2, i3).conjugate();
        o0.copy_from_slice(r.x.as_slice());
        o1.copy_from_slice(r.y.as_slice());
        o2.copy_from_slice(r.z.as_slice());
        o3.copy_from_slice(r.w.as_slice());
    }
}

/// Quaternion multiply, per lane.
#[inline(always)]
pub fn quat_mul_soa<S: Simd>(simd: S, a: [&[f32]; 4], b: [&[f32]; 4], out: [&mut [f32]; 4]) {
    let n = S::f32s::LEN;
    let left = a[0]
        .chunks_exact(n)
        .zip(a[1].chunks_exact(n))
        .zip(a[2].chunks_exact(n))
        .zip(a[3].chunks_exact(n));
    let right = b[0]
        .chunks_exact(n)
        .zip(b[1].chunks_exact(n))
        .zip(b[2].chunks_exact(n))
        .zip(b[3].chunks_exact(n));
    let [ox, oy, oz, ow] = out;
    let outs = ox
        .chunks_exact_mut(n)
        .zip(oy.chunks_exact_mut(n))
        .zip(oz.chunks_exact_mut(n))
        .zip(ow.chunks_exact_mut(n));
    for (((((l0, l1), l2), l3), (((r0, r1), r2), r3)), (((o0, o1), o2), o3)) in
        left.zip(right).zip(outs)
    {
        let qa = QuatxN::from_slice(simd, l0, l1, l2, l3);
        let qb = QuatxN::from_slice(simd, r0, r1, r2, r3);
        let r = qa * qb;
        o0.copy_from_slice(r.x.as_slice());
        o1.copy_from_slice(r.y.as_slice());
        o2.copy_from_slice(r.z.as_slice());
        o3.copy_from_slice(r.w.as_slice());
    }
}

/// Rotate a vector by a quaternion, per lane.
#[inline(always)]
pub fn quat_rotate_soa<S: Simd>(simd: S, q: [&[f32]; 4], v: [&[f32]; 3], out: [&mut [f32]; 3]) {
    let n = S::f32s::LEN;
    let quats = q[0]
        .chunks_exact(n)
        .zip(q[1].chunks_exact(n))
        .zip(q[2].chunks_exact(n))
        .zip(q[3].chunks_exact(n));
    let vecs = v[0]
        .chunks_exact(n)
        .zip(v[1].chunks_exact(n))
        .zip(v[2].chunks_exact(n));
    let [ox, oy, oz] = out;
    let outs = ox
        .chunks_exact_mut(n)
        .zip(oy.chunks_exact_mut(n))
        .zip(oz.chunks_exact_mut(n));
    for (((((q0, q1), q2), q3), ((v0, v1), v2)), ((o0, o1), o2)) in quats.zip(vecs).zip(outs) {
        let qa = QuatxN::from_slice(simd, q0, q1, q2, q3);
        let va = Vec3xN::from_slice(simd, v0, v1, v2);
        let r = qa * va;
        o0.copy_from_slice(r.x.as_slice());
        o1.copy_from_slice(r.y.as_slice());
        o2.copy_from_slice(r.z.as_slice());
    }
}

/// Rotate every lane of `v` by one quaternion, given as four scalars.
///
/// This matches the `broadcast` benchmarks, where the rotation is held in registers while
/// the vectors stream past, which is also how one rotation is applied to many points.
#[inline(always)]
pub fn quat_rotate_broadcast_soa<S: Simd>(
    simd: S,
    q: [f32; 4],
    v: [&[f32]; 3],
    out: [&mut [f32]; 3],
) {
    let n = S::f32s::LEN;
    let qa = QuatxN::splat_xyzw(simd, q[0], q[1], q[2], q[3]);
    let [ox, oy, oz] = out;
    let vecs = v[0]
        .chunks_exact(n)
        .zip(v[1].chunks_exact(n))
        .zip(v[2].chunks_exact(n));
    let outs = ox
        .chunks_exact_mut(n)
        .zip(oy.chunks_exact_mut(n))
        .zip(oz.chunks_exact_mut(n));
    for (((v0, v1), v2), ((o0, o1), o2)) in vecs.zip(outs) {
        let va = Vec3xN::from_slice(simd, v0, v1, v2);
        let r = qa * va;
        o0.copy_from_slice(r.x.as_slice());
        o1.copy_from_slice(r.y.as_slice());
        o2.copy_from_slice(r.z.as_slice());
    }
}

// One entry per kernel: a pool selects the level once, and the timed call goes straight to the
// kernel for that level.
dispatch_entry! {
    /// Ray/sphere test, selected once per pool.
    pub struct IntersectEntry(level: Level) -> fn(dirs: [&[f32]; 3], out: &mut [f32]) = intersect_soa;
}
dispatch_entry! {
    /// Per-lane length, selected once per pool.
    pub struct LengthEntry(level: Level) -> fn(a: [&[f32]; 3], out: &mut [f32]) = length_soa;
}
dispatch_entry! {
    /// Per-lane normalize, selected once per pool.
    pub struct NormalizeEntry(level: Level) -> fn(a: [&[f32]; 3], ox: &mut [f32], oy: &mut [f32], oz: &mut [f32]) = normalize_soa;
}
dispatch_entry! {
    /// Per-lane dot product, selected once per pool.
    pub struct DotEntry(level: Level) -> fn(a: [&[f32]; 3], b: [&[f32]; 3], out: &mut [f32]) = dot_soa;
}
dispatch_entry! {
    /// Per-lane cross product, selected once per pool.
    pub struct CrossEntry(level: Level) -> fn(a: [&[f32]; 3], b: [&[f32]; 3], ox: &mut [f32], oy: &mut [f32], oz: &mut [f32]) = cross_soa;
}
dispatch_entry! {
    /// Euler integration step for 3D, selected once per pool.
    pub struct Euler3Entry(level: Level) -> fn(acc: [&[f32]; 3], vel: [&mut [f32]; 3], pos: [&mut [f32]; 3]) = euler3_soa;
}
dispatch_entry! {
    /// Euler integration step for 2D, selected once per pool.
    pub struct Euler2Entry(level: Level) -> fn(acc: [&[f32]; 2], vel: [&mut [f32]; 2], pos: [&mut [f32]; 2]) = euler2_soa;
}
dispatch_entry! {
    /// Quaternion conjugate, selected once per pool.
    pub struct QuatConjugateEntry(level: Level) -> fn(q: [&[f32]; 4], out: [&mut [f32]; 4]) = quat_conjugate_soa;
}
dispatch_entry! {
    /// Quaternion multiply, selected once per pool.
    pub struct QuatMulEntry(level: Level) -> fn(a: [&[f32]; 4], b: [&[f32]; 4], out: [&mut [f32]; 4]) = quat_mul_soa;
}
dispatch_entry! {
    /// Rotate a vector by a quaternion, selected once per pool.
    pub struct QuatRotateEntry(level: Level) -> fn(q: [&[f32]; 4], v: [&[f32]; 3], out: [&mut [f32]; 3]) = quat_rotate_soa;
}
dispatch_entry! {
    /// Rotate every lane by one quaternion, selected once per pool.
    pub struct QuatRotateBroadcastEntry(level: Level) -> fn(q: [f32; 4], v: [&[f32]; 3], out: [&mut [f32]; 3]) = quat_rotate_broadcast_soa;
}

// --- Matrix and affine pools. ---

/// One array of conditioned random values.
pub fn random_array(size: usize, _rng: &mut Pcg64Mcg, seed: f32) -> Vec<f32> {
    (0..size)
        .map(|i| {
            let v = (i as f32 * 0.37 + seed).sin() * 2.0;
            if v.abs() < 0.6 { v + 1.5 } else { v }
        })
        .collect()
}

/// `count` zeroed arrays, for outputs.
pub fn zero_pool(count: usize, size: usize) -> Vec<Vec<f32>> {
    (0..count).map(|_| vec![0.0f32; size]).collect()
}

/// Nine component pools in column-major order.
pub fn random_mat3_pool(size: usize, rng: &mut Pcg64Mcg) -> Vec<Vec<f32>> {
    (0..9).map(|c| random_array(size, rng, 1.0 + c as f32)).collect()
}

/// Twelve component pools: the nine matrix elements, then the translation.
pub fn random_affine_pool(size: usize, rng: &mut Pcg64Mcg) -> Vec<Vec<f32>> {
    (0..12).map(|c| random_array(size, rng, 1.0 + c as f32)).collect()
}

/// One scalar affine transform, for the broadcast transform benches.
pub fn random_affine_scalar(rng: &mut Pcg64Mcg) -> glam::Affine3 {
    let mut v = |seed: f32| random_array(1, rng, seed)[0];
    glam::Affine3::from_mat3_translation(
        glam::Mat3::from_cols(
            glam::Vec3::new(v(1.0), v(2.0), v(3.0)),
            glam::Vec3::new(v(4.0), v(5.0), v(6.0)),
            glam::Vec3::new(v(7.0), v(8.0), v(9.0)),
        ),
        glam::Vec3::new(v(10.0), v(11.0), v(12.0)),
    )
}

#[inline(always)]
fn load_mat3<S: Simd>(simd: S, a: &[Vec<f32>], i: usize, n: usize) -> Mat3xN<S> {
    let sl = |c: usize| &a[c][i..i + n];
    Mat3xN::from_cols(
        simd,
        Vec3xN::from_slice(simd, sl(0), sl(1), sl(2)),
        Vec3xN::from_slice(simd, sl(3), sl(4), sl(5)),
        Vec3xN::from_slice(simd, sl(6), sl(7), sl(8)),
    )
}

#[inline(always)]
fn store_mat3<S: Simd>(m: Mat3xN<S>, out: &mut [Vec<f32>], i: usize, n: usize) {
    let cols = m.cols();
    let comps = [
        cols[0].x, cols[0].y, cols[0].z,
        cols[1].x, cols[1].y, cols[1].z,
        cols[2].x, cols[2].y, cols[2].z,
    ];
    for (c, comp) in comps.iter().enumerate() {
        out[c][i..i + n].copy_from_slice(comp.as_slice());
    }
}

#[inline(always)]
fn load_affine<S: Simd>(simd: S, a: &[Vec<f32>], i: usize, n: usize) -> Affine3xN<S> {
    let sl = |c: usize| &a[c][i..i + n];
    Affine3xN::from_mat3_translation(
        simd,
        Mat3xN::from_cols(
            simd,
            Vec3xN::from_slice(simd, sl(0), sl(1), sl(2)),
            Vec3xN::from_slice(simd, sl(3), sl(4), sl(5)),
            Vec3xN::from_slice(simd, sl(6), sl(7), sl(8)),
        ),
        Vec3xN::from_slice(simd, sl(9), sl(10), sl(11)),
    )
}

#[inline(always)]
fn store_affine<S: Simd>(r: Affine3xN<S>, out: &mut [Vec<f32>], i: usize, n: usize) {
    let cols = r.matrix3.cols();
    let comps = [
        cols[0].x, cols[0].y, cols[0].z,
        cols[1].x, cols[1].y, cols[1].z,
        cols[2].x, cols[2].y, cols[2].z,
        r.translation.x, r.translation.y, r.translation.z,
    ];
    for (c, comp) in comps.iter().enumerate() {
        out[c][i..i + n].copy_from_slice(comp.as_slice());
    }
}

/// Matrix3 ops. `op`: 0 transpose, 1 inverse, 2 determinant (into `det`).
#[inline(always)]
pub fn mat3_unop_soa<S: Simd>(
    simd: S,
    op: u32,
    a: &[Vec<f32>],
    out: &mut [Vec<f32>],
    det: &mut [f32],
) {
    let n = S::f32s::LEN;
    for i in (0..det.len()).step_by(n) {
        let m = load_mat3(simd, a, i, n);
        match op {
            0 => store_mat3(m.transpose(), out, i, n),
            1 => store_mat3(m.inverse(), out, i, n),
            _ => m.determinant().store_slice(&mut det[i..i + n]),
        }
    }
}

/// Matrix3 times matrix3.
#[inline(always)]
pub fn mat3_mul_mat3_soa<S: Simd>(simd: S, a: &[Vec<f32>], b: &[Vec<f32>], out: &mut [Vec<f32>]) {
    let n = S::f32s::LEN;
    for i in (0..a[0].len()).step_by(n) {
        let ma = load_mat3(simd, a, i, n);
        let mb = load_mat3(simd, b, i, n);
        store_mat3(ma * mb, out, i, n);
    }
}

/// Matrix3 times vector3.
#[inline(always)]
pub fn mat3_mul_vec3_soa<S: Simd>(simd: S, a: &[Vec<f32>], v: &[Vec<f32>], out: &mut [Vec<f32>]) {
    let n = S::f32s::LEN;
    for i in (0..a[0].len()).step_by(n) {
        let m = load_mat3(simd, a, i, n);
        let rhs = Vec3xN::from_slice(
            simd,
            &v[0][i..i + n],
            &v[1][i..i + n],
            &v[2][i..i + n],
        );
        let r = m * rhs;
        out[0][i..i + n].copy_from_slice(r.x.as_slice());
        out[1][i..i + n].copy_from_slice(r.y.as_slice());
        out[2][i..i + n].copy_from_slice(r.z.as_slice());
    }
}

/// Affine inverse (`op` 0) or composition (`op` 1 with a second pool).
#[inline(always)]
pub fn affine_unop_soa<S: Simd>(simd: S, a: &[Vec<f32>], out: &mut [Vec<f32>]) {
    let n = S::f32s::LEN;
    for i in (0..a[0].len()).step_by(n) {
        store_affine(load_affine(simd, a, i, n).inverse(), out, i, n);
    }
}

/// Affine times affine.
#[inline(always)]
pub fn affine_mul_soa<S: Simd>(simd: S, a: &[Vec<f32>], b: &[Vec<f32>], out: &mut [Vec<f32>]) {
    let n = S::f32s::LEN;
    for i in (0..a[0].len()).step_by(n) {
        let ma = load_affine(simd, a, i, n);
        let mb = load_affine(simd, b, i, n);
        store_affine(ma * mb, out, i, n);
    }
}

/// One transform applied to many points (`op` 0) or vectors (`op` 1).
#[inline(always)]
pub fn affine_transform_broadcast_soa<S: Simd>(
    simd: S,
    a: glam::Affine3,
    op: u32,
    v: &[Vec<f32>],
    out: &mut [Vec<f32>],
) {
    let n = S::f32s::LEN;
    let affine = Affine3xN::splat_affine3(simd, a);
    for i in (0..v[0].len()).step_by(n) {
        let p = Vec3xN::from_slice(
            simd,
            &v[0][i..i + n],
            &v[1][i..i + n],
            &v[2][i..i + n],
        );
        let r = if op == 0 {
            affine.transform_point3(p)
        } else {
            affine.transform_vector3(p)
        };
        out[0][i..i + n].copy_from_slice(r.x.as_slice());
        out[1][i..i + n].copy_from_slice(r.y.as_slice());
        out[2][i..i + n].copy_from_slice(r.z.as_slice());
    }
}

dispatch_entry! {
    /// Matrix3 transpose, inverse or determinant, selected once per pool.
    pub struct Mat3UnopEntry(level: Level) -> fn(op: u32, a: &[Vec<f32>], out: &mut [Vec<f32>], det: &mut [f32]) = mat3_unop_soa;
}
dispatch_entry! {
    /// Matrix3 times matrix3, selected once per pool.
    pub struct Mat3MulMat3Entry(level: Level) -> fn(a: &[Vec<f32>], b: &[Vec<f32>], out: &mut [Vec<f32>]) = mat3_mul_mat3_soa;
}
dispatch_entry! {
    /// Matrix3 times vector3, selected once per pool.
    pub struct Mat3MulVec3Entry(level: Level) -> fn(a: &[Vec<f32>], v: &[Vec<f32>], out: &mut [Vec<f32>]) = mat3_mul_vec3_soa;
}
dispatch_entry! {
    /// Affine inverse, selected once per pool.
    pub struct AffineUnopEntry(level: Level) -> fn(a: &[Vec<f32>], out: &mut [Vec<f32>]) = affine_unop_soa;
}
dispatch_entry! {
    /// Affine times affine, selected once per pool.
    pub struct AffineMulEntry(level: Level) -> fn(a: &[Vec<f32>], b: &[Vec<f32>], out: &mut [Vec<f32>]) = affine_mul_soa;
}
dispatch_entry! {
    /// One transform applied to many points or vectors, selected once per pool.
    pub struct AffineTransformEntry(level: Level) -> fn(a: glam::Affine3, op: u32, v: &[Vec<f32>], out: &mut [Vec<f32>]) = affine_transform_broadcast_soa;
}
