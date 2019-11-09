mod support;

use cgmath::{self, InnerSpace};
use glam;
use mathbench::mint_support::{random_mint_vec2, random_mint_vec3, random_mint_vec4};
use nalgebra;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

const NUM_ITERS: usize = 1024;

fn vec2_normalize_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv = random_mint_vec2(&mut rng);

    let gv: glam::Vec2 = mv.into();
    let gvn = gv.normalize();

    let nv: nalgebra::Vector2<f32> = mv.into();
    let nvn = nv.normalize();

    let cv: cgmath::Vector2<f32> = mv.into();
    let cvn = cv.normalize();

    // use nalgebra as assumed correct answer
    let mvn: mint::Vector2<f32> = nvn.into();

    assert_ulps_eq!(cvn, mvn.into(), epsilon = 1e-6);
    assert_ulps_eq!(gvn, mvn.into(), epsilon = 1e-6);
}

fn vec3_dot_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv1 = random_mint_vec3(&mut rng);
    let mv2 = random_mint_vec3(&mut rng);

    let gv1: glam::Vec3 = mv1.into();
    let gv2: glam::Vec3 = mv2.into();
    let gd = gv1.dot(gv2);

    let nv1: nalgebra::Vector3<f32> = mv1.into();
    let nv2: nalgebra::Vector3<f32> = mv2.into();
    let nd = nv1.dot(&nv2);

    let cv1: cgmath::Vector3<f32> = mv1.into();
    let cv2: cgmath::Vector3<f32> = mv2.into();
    let cd = cv1.dot(cv2);

    // use nalgebra as assumed correct answer
    assert_ulps_eq!(cd, nd, epsilon = 1e-6);
    assert_ulps_eq!(gd, nd, epsilon = 1e-6);
}

fn vec3_cross_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv1 = random_mint_vec3(&mut rng);
    let mv2 = random_mint_vec3(&mut rng);

    let gv1: glam::Vec3 = mv1.into();
    let gv2: glam::Vec3 = mv2.into();
    let gc = gv1.cross(gv2);

    let nv1: nalgebra::Vector3<f32> = mv1.into();
    let nv2: nalgebra::Vector3<f32> = mv2.into();
    let nc = nv1.cross(&nv2);

    let cv1: cgmath::Vector3<f32> = mv1.into();
    let cv2: cgmath::Vector3<f32> = mv2.into();
    let cc = cv1.cross(cv2);

    // use nalgebra as assumed correct answer
    let mc: mint::Vector3<f32> = nc.into();

    assert_ulps_eq!(cc, mc.into(), epsilon = 1e-6);
    assert_ulps_eq!(gc, mc.into(), epsilon = 1e-6);
}

fn vec3_normalize_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv = random_mint_vec3(&mut rng);

    let gv: glam::Vec3 = mv.into();
    let gvn = gv.normalize();

    let nv: nalgebra::Vector3<f32> = mv.into();
    let nvn = nv.normalize();

    let cv: cgmath::Vector3<f32> = mv.into();
    let cvn = cv.normalize();

    // use nalgebra as assumed correct answer
    let mvn: mint::Vector3<f32> = nvn.into();

    assert_ulps_eq!(cvn, mvn.into(), epsilon = 1e-6);
    assert_ulps_eq!(gvn, mvn.into(), epsilon = 1e-6);
}

fn vec4_dot_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv1 = random_mint_vec4(&mut rng);
    let mv2 = random_mint_vec4(&mut rng);

    let gv1: glam::Vec4 = mv1.into();
    let gv2: glam::Vec4 = mv2.into();
    let gd = gv1.dot(gv2);

    let nv1: nalgebra::Vector4<f32> = mv1.into();
    let nv2: nalgebra::Vector4<f32> = mv2.into();
    let nd = nv1.dot(&nv2);

    let cv1: cgmath::Vector4<f32> = mv1.into();
    let cv2: cgmath::Vector4<f32> = mv2.into();
    let cd = cv1.dot(cv2);

    // use nalgebra as assumed correct answer
    assert_ulps_eq!(cd, nd, epsilon = 1e-6);
    assert_ulps_eq!(gd, nd, epsilon = 1e-6);
}

fn vec4_normalize_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mv = random_mint_vec4(&mut rng);

    let gv: glam::Vec4 = mv.into();
    let gvn = gv.normalize();

    let nv: nalgebra::Vector4<f32> = mv.into();
    let nvn = nv.normalize();

    let cv: cgmath::Vector4<f32> = mv.into();
    let cvn = cv.normalize();

    // use nalgebra as assumed correct answer
    let mvn: mint::Vector4<f32> = nvn.into();

    assert_ulps_eq!(cvn, mvn.into(), epsilon = 1e-6);
    assert_ulps_eq!(gvn, mvn.into(), epsilon = 1e-6);
}

#[test]
fn test_vec2_normalize() {
    for _ in 0..NUM_ITERS {
        vec2_normalize_compare();
    }
}

#[test]
fn test_vec3_dot() {
    for _ in 0..NUM_ITERS {
        vec3_dot_compare();
    }
}

#[test]
fn test_vec3_cross() {
    for _ in 0..NUM_ITERS {
        vec3_cross_compare();
    }
}

#[test]
fn test_vec3_normalize() {
    for _ in 0..NUM_ITERS {
        vec3_normalize_compare();
    }
}

#[test]
fn test_vec4_dot() {
    for _ in 0..NUM_ITERS {
        vec4_dot_compare();
    }
}

#[test]
fn test_vec4_normalize() {
    for _ in 0..NUM_ITERS {
        vec4_normalize_compare();
    }
}
