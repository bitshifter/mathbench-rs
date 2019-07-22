use approx;
use euclid;
use glam;
use mint;
use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256Plus;

pub trait RandomVec {
    type Value;
    fn random_vec(seed: u64, len: usize) -> Vec<Self::Value>;
}

macro_rules! impl_random_vec {
    ($t:ty) => {
        impl RandomVec for $t {
            type Value = Self;
            fn random_vec(seed: u64, len: usize) -> Vec<Self::Value> {
                let mut rng = Xoshiro256Plus::seed_from_u64(seed);
                (0..len).map(|_| rng.gen::<Self::Value>().into()).collect()
            }
        }
    };
    ($t:ty, $f:expr) => {
        impl RandomVec for $t {
            type Value = Self;
            fn random_vec(seed: u64, len: usize) -> Vec<Self::Value> {
                let mut rng = Xoshiro256Plus::seed_from_u64(seed);
                (0..len).map(|_| $f(&mut rng).into()).collect()
            }
        }
    };
}

impl_random_vec!(glam::Mat2, random_invertible_mat2);
impl_random_vec!(glam::Mat3, random_invertible_mat3);
impl_random_vec!(glam::Mat4, random_homogeneous_mat4);
impl_random_vec!(glam::Quat, random_quat);
impl_random_vec!(glam::Vec2);
impl_random_vec!(glam::Vec3);
impl_random_vec!(glam::Vec4);

impl_random_vec!(
    cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>,
    random_cgmath_decomposed3
);
impl_random_vec!(cgmath::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(cgmath::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(cgmath::Matrix4<f32>, random_homogeneous_mat4);
impl_random_vec!(cgmath::Point3<f32>, random_cgmath_point3);
impl_random_vec!(cgmath::Quaternion<f32>, random_quat);
impl_random_vec!(cgmath::Vector2<f32>);
impl_random_vec!(cgmath::Vector3<f32>);
impl_random_vec!(cgmath::Vector4<f32>);

impl_random_vec!(nalgebra::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(nalgebra::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(nalgebra::Matrix4<f32>, random_homogeneous_mat4);
impl_random_vec!(nalgebra::Point3<f32>);
impl_random_vec!(nalgebra::Transform3<f32>, random_na_transform3);
impl_random_vec!(nalgebra::UnitQuaternion<f32>, random_na_quat);
impl_random_vec!(nalgebra::Vector2<f32>);
impl_random_vec!(nalgebra::Vector3<f32>);
impl_random_vec!(nalgebra::Vector4<f32>);

impl_random_vec!(euclid::Point2D<f32, euclid::UnknownUnit>, random_euclid_point2);
impl_random_vec!(euclid::Point3D<f32, euclid::UnknownUnit>, random_euclid_point3);
impl_random_vec!(euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_quat);
impl_random_vec!(euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat3);
impl_random_vec!(euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat4);
impl_random_vec!(euclid::Vector2D<f32, euclid::UnknownUnit>, random_euclid_vec2);
impl_random_vec!(euclid::Vector3D<f32, euclid::UnknownUnit>, random_euclid_vec3);

// glam random functions ------------------------------------------------------
fn random_glam_vec3<R>(rng: &mut R) -> glam::Vec3
where
    R: Rng,
{
    rng.gen()
}

fn random_nonzero_glam_vec3<R>(rng: &mut R) -> glam::Vec3
where
    R: Rng,
{
    loop {
        let v: glam::Vec3 = rng.gen();
        if v.length_squared() > 0.1 {
            return v;
        }
    }
}

fn random_glam_quat<R>(rng: &mut R) -> glam::Quat
where
    R: Rng,
{
    let yaw = rng.gen();
    let pitch = rng.gen();
    let roll = rng.gen();
    glam::Quat::from_rotation_ypr(yaw, pitch, roll)
}

// mint random functions  -----------------------------------------------------
pub fn random_quat<R>(rng: &mut R) -> mint::Quaternion<f32>
where
    R: Rng,
{
    rng.gen::<glam::Quat>().into()
}

pub fn random_vec2<R>(rng: &mut R) -> mint::Vector2<f32>
where
    R: Rng,
{
    rng.gen::<glam::Vec2>().into()
}

pub fn random_vec3<R>(rng: &mut R) -> mint::Vector3<f32>
where
    R: Rng,
{
    rng.gen::<glam::Vec3>().into()
}

pub fn random_vec4<R>(rng: &mut R) -> mint::Vector4<f32>
where
    R: Rng,
{
    rng.gen::<glam::Vec4>().into()
}

pub fn random_mat2<R>(rng: &mut R) -> mint::ColumnMatrix2<f32>
where
    R: Rng,
{
    rng.gen::<glam::Mat2>().into()
}

pub fn random_mat3<R>(rng: &mut R) -> mint::ColumnMatrix3<f32>
where
    R: Rng,
{
    rng.gen::<glam::Mat3>().into()
}

pub fn random_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
where
    R: Rng,
{
    rng.gen::<glam::Mat4>().into()
}

pub fn random_invertible_mat2<R>(rng: &mut R) -> mint::ColumnMatrix2<f32>
where
    R: Rng,
{
    loop {
        let m = rng.gen::<glam::Mat2>();
        if approx::relative_ne!(m.determinant(), 0.0) {
            return m.into();
        }
    }
}

pub fn random_invertible_mat3<R>(rng: &mut R) -> mint::ColumnMatrix3<f32>
where
    R: Rng,
{
    loop {
        let m = rng.gen::<glam::Mat3>();
        if approx::relative_ne!(m.determinant(), 0.0) {
            return m.into();
        }
    }
}

pub fn random_homogeneous_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
where
    R: Rng,
{
    loop {
        let m = glam::Mat4::from_scale_rotation_translation(
            random_nonzero_glam_vec3(rng),
            random_glam_quat(rng),
            random_glam_vec3(rng),
        );
        if approx::relative_ne!(m.determinant(), 0.0) {
            return m.into();
        }
    }
}

// cgmath random functions ----------------------------------------------------
fn random_cgmath_decomposed3<R>(
    rng: &mut R,
) -> cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>
where
    R: Rng,
{
    cgmath::Decomposed {
        scale: rng.gen_range(0.1, 1.0),
        rot: random_quat(rng).into(),
        disp: random_vec3(rng).into(),
    }
}

fn random_cgmath_point3<R>(rng: &mut R) -> cgmath::Point3<f32>
where
    R: Rng,
{
    let v = random_vec3(rng);
    cgmath::Point3::new(v.x, v.y, v.z)
}

// nalgebra random functions --------------------------------------------------
fn random_na_quat<R>(rng: &mut R) -> nalgebra::UnitQuaternion<f32>
where
    R: Rng,
{
    nalgebra::UnitQuaternion::from_quaternion(random_quat(rng).into())
}

fn random_na_transform3<R>(rng: &mut R) -> nalgebra::Transform3<f32>
where
    R: Rng,
{
    nalgebra::Transform3::from_matrix_unchecked(random_homogeneous_mat4(rng).into())
}

// euclid random functions ----------------------------------------------------
fn random_euclid_vec2<R>(rng: &mut R) -> euclid::Vector2D<f32, euclid::UnknownUnit>
where
    R: Rng,
{
    let (x, y) = rng.gen::<glam::Vec2>().into();
    euclid::vec2(x, y)
}

fn random_euclid_point2<R>(rng: &mut R) -> euclid::Point2D<f32, euclid::UnknownUnit>
where
    R: Rng,
{
    random_euclid_vec2(rng).to_point()
}

fn random_euclid_vec3<R>(rng: &mut R) -> euclid::Vector3D<f32, euclid::UnknownUnit>
where
    R: Rng,
{
    let (x, y, z) = rng.gen::<glam::Vec3>().into();
    euclid::vec3(x, y, z)
}

fn random_euclid_point3<R>(rng: &mut R) -> euclid::Point3D<f32, euclid::UnknownUnit>
where
    R: Rng,
{
    random_euclid_vec3(rng).to_point()
}

fn random_euclid_quat<R>(
    rng: &mut R,
) -> euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>
where
    R: Rng,
{
    let (x, y, z, w) = rng.gen::<glam::Quat>().into();
    euclid::Rotation3D::quaternion(x, y, z, w)
}

fn random_euclid_mat3<R>(
    rng: &mut R,
) -> euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>
where
    R: Rng,
{
    let m = random_invertible_mat3(rng);
    euclid::Transform2D::column_major(m.x.x, m.x.y, m.x.z, m.y.x, m.y.y, m.y.z)
}

fn random_euclid_mat4<R>(
    rng: &mut R,
) -> euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>
where
    R: Rng,
{
    let m = random_homogeneous_mat4(rng);
    euclid::Transform3D::column_major(
        m.x.x, m.x.y, m.x.z, m.x.w, m.y.x, m.y.y, m.y.z, m.y.w, m.z.x, m.z.y, m.z.z, m.z.w, m.w.x,
        m.w.y, m.w.z, m.w.w,
    )
}

// public non-inlined functions for cargo asm
pub fn glam_mat4_det(m: &glam::Mat4) -> f32 {
    m.determinant()
}

pub fn glam_mat4_inv(m: &glam::Mat4) -> glam::Mat4 {
    m.inverse()
}

pub fn glam_mat4_try_inv(m: &glam::Mat4) -> Option<glam::Mat4> {
    // glam doesn't support this and it's really slow presumably due to alignment
    Some(m.inverse())
}

pub fn glam_mat4_mul(lhs: &glam::Mat4, rhs: &glam::Mat4) -> glam::Mat4 {
    lhs.mul_mat4(rhs)
}

pub fn glam_mat4_mul_vec4(lhs: &glam::Mat4, rhs: &glam::Vec4) -> glam::Vec4 {
    *lhs * *rhs
}

pub fn cgmath_mat4_det(m: &cgmath::Matrix4<f32>) -> f32 {
    use cgmath::SquareMatrix;
    m.determinant()
}

pub fn cgmath_mat4_inv(m: &cgmath::Matrix4<f32>) -> cgmath::Matrix4<f32> {
    use cgmath::SquareMatrix;
    // cgmath always returns an Option
    m.invert().unwrap_or(*m)
}

pub fn cgmath_mat4_try_inv(m: &cgmath::Matrix4<f32>) -> Option<cgmath::Matrix4<f32>> {
    use cgmath::SquareMatrix;
    m.invert()
}

pub fn cgmath_mat4_mul(
    lhs: &cgmath::Matrix4<f32>,
    rhs: &cgmath::Matrix4<f32>,
) -> cgmath::Matrix4<f32> {
    lhs * rhs
}

pub fn nalgebra_mat4_det(m: &nalgebra::Matrix4<f32>) -> f32 {
    m.determinant()
}

pub fn nalgebra_mat4_inv(m: &nalgebra::Matrix4<f32>) -> nalgebra::Matrix4<f32> {
    m.try_inverse().unwrap_or(*m)
}

pub fn nalgebra_mat4_try_inv(m: &nalgebra::Matrix4<f32>) -> Option<nalgebra::Matrix4<f32>> {
    m.try_inverse()
}

pub fn nalgebra_mat4_mul(
    lhs: &nalgebra::Matrix4<f32>,
    rhs: &nalgebra::Matrix4<f32>,
) -> nalgebra::Matrix4<f32> {
    lhs * rhs
}
