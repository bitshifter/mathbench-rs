//! `glam_fearless` support: SoA kernels plus runtime dispatch wrappers.
//!
//! `glam_fearless` values carry their SIMD token, so they cannot be constructed without one
//! (and the width is chosen at runtime), which is why these entries take SoA component slices
//! and dispatch per call instead of using the fixed-width `bench_*_wide!` macros. The dispatch
//! match is negligible next to the kernel (measured: hoisting it changes the results by <=10%).

use glam_fearless::{Level, QuatxN, Vec2xN, Vec3xN, dispatch, prelude::*, simd};

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
pub fn normalize_soa_dispatch(a: [&[f32]; 3], ox: &mut [f32], oy: &mut [f32], oz: &mut [f32]) {
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
#[simd]
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
#[simd]
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

/// Euler integration step for 3D, at the best level this CPU supports.
pub fn euler3_soa_dispatch(acc: [&[f32]; 3], vel: &mut [Vec<f32>; 3], pos: &mut [Vec<f32>; 3]) {
    let level = Level::new();
    let [v0, v1, v2] = vel;
    let [p0, p1, p2] = pos;
    let v: [&mut [f32]; 3] = [v0.as_mut_slice(), v1.as_mut_slice(), v2.as_mut_slice()];
    let p: [&mut [f32]; 3] = [p0.as_mut_slice(), p1.as_mut_slice(), p2.as_mut_slice()];
    dispatch!(level, simd => euler3_soa(simd, acc, v, p))
}

/// Euler integration step for 2D, at the best level this CPU supports.
pub fn euler2_soa_dispatch(acc: [&[f32]; 2], vel: &mut [Vec<f32>; 2], pos: &mut [Vec<f32>; 2]) {
    let level = Level::new();
    let [v0, v1] = vel;
    let [p0, p1] = pos;
    let v: [&mut [f32]; 2] = [v0.as_mut_slice(), v1.as_mut_slice()];
    let p: [&mut [f32]; 2] = [p0.as_mut_slice(), p1.as_mut_slice()];
    dispatch!(level, simd => euler2_soa(simd, acc, v, p))
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
#[simd]
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
#[simd]
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
#[simd]
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

/// Quaternion conjugate, at the best level this CPU supports.
pub fn quat_conjugate_soa_dispatch(q: [&[f32]; 4], out: [&mut [f32]; 4]) {
    let level = Level::new();
    dispatch!(level, simd => quat_conjugate_soa(simd, q, out))
}

/// Quaternion multiply, at the best level this CPU supports.
pub fn quat_mul_soa_dispatch(a: [&[f32]; 4], b: [&[f32]; 4], out: [&mut [f32]; 4]) {
    let level = Level::new();
    dispatch!(level, simd => quat_mul_soa(simd, a, b, out))
}

/// Rotate a vector by a quaternion, at the best level this CPU supports.
pub fn quat_rotate_soa_dispatch(q: [&[f32]; 4], v: [&[f32]; 3], out: [&mut [f32]; 3]) {
    let level = Level::new();
    dispatch!(level, simd => quat_rotate_soa(simd, q, v, out))
}

/// Rotate every lane of `v` by one quaternion, given as four scalars.
///
/// This matches the `broadcast` benchmarks, where the rotation is held in registers while
/// the vectors stream past, which is also how one rotation is applied to many points.
#[simd]
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

/// Rotate every lane of `v` by one quaternion, at the best level this CPU supports.
pub fn quat_rotate_broadcast_soa_dispatch(q: [f32; 4], v: [&[f32]; 3], out: [&mut [f32]; 3]) {
    let level = Level::new();
    dispatch!(level, simd => quat_rotate_broadcast_soa(simd, q, v, out))
}
