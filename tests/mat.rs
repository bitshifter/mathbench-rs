mod support;
use cgmath;
use glam;
use mathbench::mint_support::*;
use nalgebra;
use rand_pcg::Pcg64Mcg;

const NUM_ITERS: usize = 1024;

#[macro_export]
macro_rules! semi_implicit_euler {
    ($delta_secs: expr, $accel: expr, $vel: expr, $pos: expr) => {{
        for ((position, acceleration), velocity) in
            $pos.iter_mut().zip($accel.iter()).zip($vel.iter_mut())
        {
            *velocity += *acceleration * $delta_secs;
            *position += *velocity * $delta_secs;
        }
    }};
}

fn mat2_mul_vec2_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm = random_mint_mat2(&mut rng);
    let mv = random_mint_vec2(&mut rng);

    let gm: glam::Mat2 = mm.into();
    let gv: glam::Vec2 = mv.into();
    let gmv = gm * gv;

    let nm: nalgebra::Matrix2<f32> = mm.into();
    let nv: nalgebra::Vector2<f32> = mv.into();
    let nmv = nm * nv;

    let cm: cgmath::Matrix2<f32> = mm.into();
    let cv: cgmath::Vector2<f32> = mv.into();
    let cmv = cm * cv;

    // use nalgebra as assumed correct answer
    let mmv: mint::Vector2<f32> = nmv.into();

    assert_ulps_eq!(cmv, mmv.into());
    assert_ulps_eq!(gmv, mmv.into());
    // assert_ulps_eq!(stmv, mmv.into());
}

fn mat3_mul_vec3_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm = random_mint_mat3(&mut rng);
    let mv = random_mint_vec3(&mut rng);

    let gm: glam::Mat3 = mm.into();
    let gv: glam::Vec3 = mv.into();
    let gmv = gm * gv;

    let nm: nalgebra::Matrix3<f32> = mm.into();
    let nv: nalgebra::Vector3<f32> = mv.into();
    let nmv = nm * nv;

    let cm: cgmath::Matrix3<f32> = mm.into();
    let cv: cgmath::Vector3<f32> = mv.into();
    let cmv = cm * cv;

    // use nalgebra as assumed correct answer
    let mmv: mint::Vector3<f32> = nmv.into();

    assert_ulps_eq!(cmv, mmv.into());
    assert_ulps_eq!(gmv, mmv.into());
}

fn mat4_mul_vec4_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm = random_mint_mat4(&mut rng);
    let mv = random_mint_vec4(&mut rng);

    let gm: glam::Mat4 = mm.into();
    let gv: glam::Vec4 = mv.into();
    let gmv = gm * gv;

    let nm: nalgebra::Matrix4<f32> = mm.into();
    let nv: nalgebra::Vector4<f32> = mv.into();
    let nmv = nm * nv;

    let cm: cgmath::Matrix4<f32> = mm.into();
    let cv: cgmath::Vector4<f32> = mv.into();
    let cmv = cm * cv;

    // use nalgebra as assumed correct answer
    let mmv: mint::Vector4<f32> = nmv.into();

    assert_ulps_eq!(cmv, mmv.into());
    assert_ulps_eq!(gmv, mmv.into());
}

fn mat2_mul_mat2_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_mat2(&mut rng);
    let mm2 = random_mint_mat2(&mut rng);

    let gm1: glam::Mat2 = mm1.into();
    let gm2: glam::Mat2 = mm2.into();
    let gm3 = gm1 * gm2;

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nm2: nalgebra::Matrix2<f32> = mm2.into();
    let nm3 = nm1 * nm2;

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cm2: cgmath::Matrix2<f32> = mm2.into();
    let cm3 = cm1 * cm2;

    // use nalgebra as assumed correct answer
    let mm3: mint::ColumnMatrix2<f32> = nm3.into();

    assert_ulps_eq!(cm3, mm3.into());
    assert_ulps_eq!(gm3, mm3.into());
}

fn mat3_mul_mat3_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_mat3(&mut rng);
    let mm2 = random_mint_mat3(&mut rng);

    let gm1: glam::Mat3 = mm1.into();
    let gm2: glam::Mat3 = mm2.into();
    let gm3 = gm1 * gm2;

    let nm1: nalgebra::Matrix3<f32> = mm1.into();
    let nm2: nalgebra::Matrix3<f32> = mm2.into();
    let nm3 = nm1 * nm2;

    let cm1: cgmath::Matrix3<f32> = mm1.into();
    let cm2: cgmath::Matrix3<f32> = mm2.into();
    let cm3 = cm1 * cm2;

    // use nalgebra as assumed correct answer
    let mm3: mint::ColumnMatrix3<f32> = nm3.into();

    assert_ulps_eq!(cm3, mm3.into());
    assert_ulps_eq!(gm3, mm3.into());
}

fn mat4_mul_mat4_compare() {
    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_mat4(&mut rng);
    let mm2 = random_mint_mat4(&mut rng);

    let gm1: glam::Mat4 = mm1.into();
    let gm2: glam::Mat4 = mm2.into();
    let gm3 = gm1 * gm2;

    let nm1: nalgebra::Matrix4<f32> = mm1.into();
    let nm2: nalgebra::Matrix4<f32> = mm2.into();
    let nm3 = nm1 * nm2;

    let cm1: cgmath::Matrix4<f32> = mm1.into();
    let cm2: cgmath::Matrix4<f32> = mm2.into();
    let cm3 = cm1 * cm2;

    // use nalgebra as assumed correct answer
    let mm3: mint::ColumnMatrix4<f32> = nm3.into();

    assert_ulps_eq!(cm3, mm3.into());
    assert_ulps_eq!(gm3, mm3.into());
}

fn mat2_det_compare() {
    use cgmath::prelude::*;

    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_mat2(&mut rng);

    let gm1: glam::Mat2 = mm1.into();
    let gmd = gm1.determinant();

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmd = nm1.determinant();

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmd = cm1.determinant();

    // use nalgebra as assumed correct answer
    assert_ulps_eq!(cmd, nmd);
    assert_ulps_eq!(gmd, nmd);
}

fn mat2_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_invertible_mat2(&mut rng);

    let gm1: glam::Mat2 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix2<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix2<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix2<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    // TODO: actually make a ulps test
    assert_ulps_eq!(gmi, mmi.into(), epsilon = 1e-2);
}

fn mat3_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Pcg64Mcg::new(0);
    let mm1 = random_mint_homogeneous_mat3(&mut rng);

    let gm1: glam::Mat3 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix3<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix3<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix3<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into());
    assert_ulps_eq!(gmi, mmi.into(), epsilon = 0.0001);
}

fn mat4_inv_compare() {
    use cgmath::prelude::*;

    let mut rng = Pcg64Mcg::new(rand::random());
    let mm1 = random_mint_homogeneous_mat4(&mut rng);

    let gm1: glam::Mat4 = mm1.into();
    let gmi = gm1.inverse();

    let nm1: nalgebra::Matrix4<f32> = mm1.into();
    let nmi = nm1.try_inverse();
    assert!(nmi.is_some());

    let cm1: cgmath::Matrix4<f32> = mm1.into();
    let cmi = cm1.invert();
    assert!(cmi.is_some());

    // use nalgebra as assumed correct answer
    let mmi: mint::ColumnMatrix4<f32> = nmi.unwrap().into();

    assert_ulps_eq!(cmi.unwrap(), mmi.into(), epsilon = 0.0001);
    assert_ulps_eq!(gmi, mmi.into(), epsilon = 0.0001);
}

#[test]
fn test_mat2_mul_vec2() {
    for _ in 0..NUM_ITERS {
        mat2_mul_vec2_compare();
    }
}

#[test]
fn test_mat3_mul_vec3() {
    for _ in 0..NUM_ITERS {
        mat3_mul_vec3_compare();
    }
}

#[test]
fn test_mat4_mul_vec4() {
    for _ in 0..NUM_ITERS {
        mat4_mul_vec4_compare();
    }
}

#[test]
fn test_mat2_mul_mat2() {
    for _ in 0..NUM_ITERS {
        mat2_mul_mat2_compare();
    }
}

#[test]
fn test_mat3_mul_mat3() {
    for _ in 0..NUM_ITERS {
        mat3_mul_mat3_compare();
    }
}

#[test]
fn test_mat4_mul_mat4() {
    for _ in 0..NUM_ITERS {
        mat4_mul_mat4_compare();
    }
}

#[test]
fn test_mat2_det() {
    for _ in 0..NUM_ITERS {
        mat2_det_compare();
    }
}

#[test]
fn test_mat2_inverse() {
    for _ in 0..NUM_ITERS {
        mat2_inv_compare();
    }
}

#[test]
fn test_mat3_inverse() {
    for _ in 0..NUM_ITERS {
        mat3_inv_compare();
    }
}

#[test]
fn test_mat4_inverse() {
    for _ in 0..NUM_ITERS {
        mat4_inv_compare();
    }
}
