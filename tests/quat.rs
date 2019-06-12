use approx::assert_ulps_eq;
use cgmath;
use glam;
use mathbench::*;
use nalgebra;
use rand::SeedableRng;
use rand_xoshiro::Xoshiro256Plus;

const NUM_ITERS: usize = 1024;

fn quat_mul_vec3_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mq = random_quat(&mut rng);
    let mv = random_vec3(&mut rng);

    let gq: glam::Quat = mq.into();
    let gv: glam::Vec3 = mv.into();
    let gqv = gq * gv;

    let nq = nalgebra::UnitQuaternion::from_quaternion(mq.into());
    let nv: nalgebra::Vector3<f32> = mv.into();
    let nqv = nq * nv;

    let cq: cgmath::Quaternion<f32> = mq.into();
    let cv: cgmath::Vector3<f32> = mv.into();
    let cqv = cq * cv;

    // use nalgebra as assumed correct answer
    let mqv: mint::Vector3<f32> = nqv.into();

    assert_ulps_eq!(cqv, mqv.into(), epsilon = 1e-6);
    assert_ulps_eq!(gqv, mqv.into(), epsilon = 1e-6);
}

fn quat_mul_quat_compare() {
    let mut rng = Xoshiro256Plus::seed_from_u64(rand::random());
    let mq1 = random_quat(&mut rng);
    let mq2 = random_quat(&mut rng);

    let gq1: glam::Quat = mq1.into();
    let gq2: glam::Quat = mq2.into();
    let gq3 = gq1 * gq2;

    let nq1 = nalgebra::UnitQuaternion::from_quaternion(mq1.into());
    let nq2 = nalgebra::UnitQuaternion::from_quaternion(mq2.into());
    let nq3 = nq1 * nq2;

    let cq1: cgmath::Quaternion<f32> = mq1.into();
    let cq2: cgmath::Quaternion<f32> = mq2.into();
    let cq3 = cq1 * cq2;

    // use nalgebra as assumed correct answer
    let mq3: mint::Quaternion<f32> = nq3.into();

    assert_ulps_eq!(cq3, mq3.into(), epsilon = 1e-6);
    assert_ulps_eq!(gq3, mq3.into(), epsilon = 1e-6);
}

#[test]
fn test_quat_mul_vec3() {
    for _ in 0..NUM_ITERS {
        quat_mul_vec3_compare();
    }
}

#[test]
fn test_quat_mul_quat() {
    for _ in 0..NUM_ITERS {
        quat_mul_quat_compare();
    }
}
