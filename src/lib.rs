use approx;
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
impl_random_vec!(glam::Mat4, random_invertible_mat4);
impl_random_vec!(glam::Quat, random_quat);
impl_random_vec!(cgmath::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(cgmath::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(cgmath::Matrix4<f32>, random_invertible_mat4);
impl_random_vec!(cgmath::Quaternion<f32>, random_quat);
impl_random_vec!(nalgebra::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(nalgebra::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(nalgebra::Matrix4<f32>, random_invertible_mat4);
impl_random_vec!(nalgebra::UnitQuaternion<f32>, random_na_quat);

impl_random_vec!(glam::Vec2);
impl_random_vec!(glam::Vec3);
impl_random_vec!(glam::Vec4);
impl_random_vec!(cgmath::Vector2<f32>);
impl_random_vec!(cgmath::Vector3<f32>);
impl_random_vec!(cgmath::Vector4<f32>);
impl_random_vec!(nalgebra::Vector2<f32>);
impl_random_vec!(nalgebra::Vector3<f32>);
impl_random_vec!(nalgebra::Vector4<f32>);

pub fn random_quat<R>(rng: &mut R) -> mint::Quaternion<f32>
where
    R: Rng,
{
    rng.gen::<glam::Quat>().into()
}

fn random_na_quat<R>(rng: &mut R) -> nalgebra::UnitQuaternion<f32>
where
    R: Rng,
{
    nalgebra::UnitQuaternion::from_quaternion(random_quat(rng).into())
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

pub fn random_invertible_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
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

pub fn random_hektor_vec2(g: &mut impl Rng) -> hektor::Vec2 {
  hektor::Vec2::new(g.gen(), g.gen())
}
impl_random_vec!(hektor::Vec2, random_hektor_vec2);

pub fn random_hektor_vec3(g: &mut impl Rng) -> hektor::Vec3 {
  hektor::Vec3::new(g.gen(), g.gen(), g.gen())
}
impl_random_vec!(hektor::Vec3, random_hektor_vec3);
