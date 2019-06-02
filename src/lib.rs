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
impl_random_vec!(cgmath::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(cgmath::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(cgmath::Matrix4<f32>, random_invertible_mat4);
impl_random_vec!(nalgebra::Matrix2<f32>, random_invertible_mat2);
impl_random_vec!(nalgebra::Matrix3<f32>, random_invertible_mat3);
impl_random_vec!(nalgebra::Matrix4<f32>, random_invertible_mat4);

impl_random_vec!(glam::Vec2);
impl_random_vec!(glam::Vec3);
impl_random_vec!(glam::Vec4);
impl_random_vec!(cgmath::Vector2<f32>);
impl_random_vec!(cgmath::Vector3<f32>);
impl_random_vec!(cgmath::Vector4<f32>);
impl_random_vec!(nalgebra::Vector2<f32>);
impl_random_vec!(nalgebra::Vector3<f32>);
impl_random_vec!(nalgebra::Vector4<f32>);

fn random_vec3<R>(rng: &mut R) -> glam::Vec3
where
    R: Rng,
{
    rng.gen()
}

fn random_nonzero_vec3<R>(rng: &mut R) -> glam::Vec3
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

fn random_quat<R>(rng: &mut R) -> glam::Quat
where
    R: Rng,
{
    let yaw = rng.gen();
    let pitch = rng.gen();
    let roll = rng.gen();
    glam::Quat::from_rotation_ypr(yaw, pitch, roll)
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

pub fn random_invertible_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
where
    R: Rng,
{
    loop {
        let m = glam::Mat4::from_scale_rotation_translation(
            random_nonzero_vec3(rng),
            random_quat(rng),
            random_vec3(rng),
        );
        if approx::relative_ne!(m.determinant(), 0.0) {
            return m.into();
        }
    }
}
