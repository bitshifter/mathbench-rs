pub trait RandomVec {
    type Value;
    fn random_vec(seed: u64, len: usize) -> Vec<Self::Value>;
}

macro_rules! impl_random_vec {
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

pub mod mint_support {
    use super::glam_support::*;
    use mint;
    use rand::Rng;

    // mint random functions  -----------------------------------------------------
    pub fn random_mint_quat<R>(rng: &mut R) -> mint::Quaternion<f32>
    where
        R: Rng,
    {
        random_glam_quat(rng).into()
    }

    pub fn random_mint_vec2<R>(rng: &mut R) -> mint::Vector2<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 2]>().into()
    }

    pub fn random_mint_vec3<R>(rng: &mut R) -> mint::Vector3<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 3]>().into()
    }

    pub fn random_mint_vec4<R>(rng: &mut R) -> mint::Vector4<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 4]>().into()
    }

    pub fn random_mint_mat2<R>(rng: &mut R) -> mint::ColumnMatrix2<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 4]>().into()
    }

    pub fn random_mint_mat3<R>(rng: &mut R) -> mint::ColumnMatrix3<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 9]>().into()
    }

    pub fn random_mint_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 16]>().into()
    }

    pub fn random_mint_invertible_mat2<R>(rng: &mut R) -> mint::ColumnMatrix2<f32>
    where
        R: Rng,
    {
        loop {
            let m = glam::Mat2::from_cols_array(&rng.gen::<[f32; 4]>());
            if m.determinant().abs() > std::f32::EPSILON {
                return m.into();
            }
        }
    }

    pub fn random_mint_homogeneous_mat3<R>(rng: &mut R) -> mint::ColumnMatrix3<f32>
    where
        R: Rng,
    {
        loop {
            let m = glam::Mat3::from_scale_angle_translation(
                random_nonzero_glam_vec2(rng),
                random_angle_radians(rng),
                random_glam_vec2(rng),
            );
            if m.determinant().abs() > std::f32::EPSILON {
                return m.into();
            }
        }
    }

    pub fn random_mint_homogeneous_mat4<R>(rng: &mut R) -> mint::ColumnMatrix4<f32>
    where
        R: Rng,
    {
        loop {
            let m = glam::Mat4::from_scale_rotation_translation(
                random_glam_nonzero_vec3(rng),
                random_glam_quat(rng),
                random_glam_vec3(rng),
            );
            if m.determinant().abs() > std::f32::EPSILON {
                return m.into();
            }
        }
    }
}

pub mod glam_support {
    use super::mint_support::*;
    use super::RandomVec;
    use glam;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    impl_random_vec!(glam::Mat2, random_mint_invertible_mat2);
    impl_random_vec!(glam::Mat3, random_mint_homogeneous_mat3);
    impl_random_vec!(glam::Mat4, random_mint_homogeneous_mat4);
    impl_random_vec!(glam::Quat, random_mint_quat);
    impl_random_vec!(glam::Vec2, random_mint_vec2);
    impl_random_vec!(glam::Vec3, random_mint_vec3);
    impl_random_vec!(glam::Vec4, random_mint_vec4);

    // f32 random functions  ------------------------------------------------------
    fn random_nonzero_f32<R>(rng: &mut R) -> f32
    where
        R: Rng,
    {
        rng.gen_range(0.1, 1.0)
    }

    pub fn random_angle_radians<R>(rng: &mut R) -> f32
    where
        R: Rng,
    {
        rng.gen_range(-std::f32::consts::PI, std::f32::consts::PI)
    }

    // glam random functions ------------------------------------------------------
    pub fn random_glam_vec2<R>(rng: &mut R) -> glam::Vec2
    where
        R: Rng,
    {
        rng.gen::<[f32; 2]>().into()
    }

    pub fn random_glam_vec3<R>(rng: &mut R) -> glam::Vec3
    where
        R: Rng,
    {
        rng.gen::<[f32; 3]>().into()
    }

    pub fn random_nonzero_glam_vec2<R>(rng: &mut R) -> glam::Vec2
    where
        R: Rng,
    {
        glam::Vec2::new(random_nonzero_f32(rng), random_nonzero_f32(rng))
    }

    pub fn random_glam_nonzero_vec3<R>(rng: &mut R) -> glam::Vec3
    where
        R: Rng,
    {
        glam::Vec3::new(
            random_nonzero_f32(rng),
            random_nonzero_f32(rng),
            random_nonzero_f32(rng),
        )
    }

    pub fn random_glam_quat<R>(rng: &mut R) -> glam::Quat
    where
        R: Rng,
    {
        let yaw = random_angle_radians(rng);
        let pitch = random_angle_radians(rng);
        let roll = random_angle_radians(rng);
        glam::Quat::from_rotation_ypr(yaw, pitch, roll)
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
}

#[cfg(feature = "cgmath")]
pub mod cgmath_support {
    use super::mint_support::*;
    use super::RandomVec;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    impl_random_vec!(
        cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>,
        random_cgmath_decomposed3
    );
    impl_random_vec!(cgmath::Matrix2<f32>, random_mint_invertible_mat2);
    impl_random_vec!(cgmath::Matrix3<f32>, random_mint_homogeneous_mat3);
    impl_random_vec!(cgmath::Matrix4<f32>, random_mint_homogeneous_mat4);
    impl_random_vec!(cgmath::Point2<f32>, random_cgmath_point2);
    impl_random_vec!(cgmath::Point3<f32>, random_cgmath_point3);
    impl_random_vec!(cgmath::Quaternion<f32>, random_mint_quat);
    impl_random_vec!(cgmath::Vector2<f32>, random_mint_vec2);
    impl_random_vec!(cgmath::Vector3<f32>, random_mint_vec3);
    impl_random_vec!(cgmath::Vector4<f32>, random_mint_vec4);

    // cgmath random functions ----------------------------------------------------
    fn random_cgmath_decomposed3<R>(
        rng: &mut R,
    ) -> cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>
    where
        R: Rng,
    {
        cgmath::Decomposed {
            scale: rng.gen_range(0.1, 1.0),
            rot: random_mint_quat(rng).into(),
            disp: random_mint_vec3(rng).into(),
        }
    }

    fn random_cgmath_point2<R>(rng: &mut R) -> cgmath::Point2<f32>
    where
        R: Rng,
    {
        let v = random_mint_vec2(rng);
        cgmath::Point2::new(v.x, v.y)
    }

    fn random_cgmath_point3<R>(rng: &mut R) -> cgmath::Point3<f32>
    where
        R: Rng,
    {
        let v = random_mint_vec3(rng);
        cgmath::Point3::new(v.x, v.y, v.z)
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
}

#[cfg(feature = "nalgebra")]
pub mod nalgebra_support {
    use super::mint_support::*;
    use super::RandomVec;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;
    impl_random_vec!(nalgebra::Matrix2<f32>, random_mint_invertible_mat2);
    impl_random_vec!(nalgebra::Matrix3<f32>, random_mint_homogeneous_mat3);
    impl_random_vec!(nalgebra::Matrix4<f32>, random_mint_homogeneous_mat4);
    impl_random_vec!(nalgebra::Point2<f32>, random_na_point2);
    impl_random_vec!(nalgebra::Point3<f32>, random_na_point3);
    impl_random_vec!(nalgebra::Transform2<f32>, random_na_transform2);
    impl_random_vec!(nalgebra::Transform3<f32>, random_na_transform3);
    impl_random_vec!(nalgebra::UnitQuaternion<f32>, random_na_quat);
    impl_random_vec!(nalgebra::Vector2<f32>, random_na_vec2);
    impl_random_vec!(nalgebra::Vector3<f32>, random_na_vec3);
    impl_random_vec!(nalgebra::Vector4<f32>, random_na_vec4);

    // nalgebra random functions --------------------------------------------------
    fn random_na_quat<R>(rng: &mut R) -> nalgebra::UnitQuaternion<f32>
    where
        R: Rng,
    {
        nalgebra::UnitQuaternion::from_quaternion(random_mint_quat(rng).into())
    }

    fn random_na_transform2<R>(rng: &mut R) -> nalgebra::Transform2<f32>
    where
        R: Rng,
    {
        nalgebra::Transform2::from_matrix_unchecked(random_mint_homogeneous_mat3(rng).into())
    }

    fn random_na_transform3<R>(rng: &mut R) -> nalgebra::Transform3<f32>
    where
        R: Rng,
    {
        nalgebra::Transform3::from_matrix_unchecked(random_mint_homogeneous_mat4(rng).into())
    }

    fn random_na_point2<R>(rng: &mut R) -> nalgebra::Point2<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 2]>().into()
    }

    fn random_na_point3<R>(rng: &mut R) -> nalgebra::Point3<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 3]>().into()
    }

    fn random_na_vec2<R>(rng: &mut R) -> nalgebra::Vector2<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 2]>().into()
    }

    fn random_na_vec3<R>(rng: &mut R) -> nalgebra::Vector3<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 3]>().into()
    }

    fn random_na_vec4<R>(rng: &mut R) -> nalgebra::Vector4<f32>
    where
        R: Rng,
    {
        rng.gen::<[f32; 4]>().into()
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
}

#[cfg(feature = "euclid")]
pub mod euclid_support {
    use super::mint_support::*;
    use super::RandomVec;
    use euclid;
    use rand::{Rng, SeedableRng};
    use rand_xoshiro::Xoshiro256Plus;

    impl_random_vec!(euclid::Point2D<f32, euclid::UnknownUnit>, random_euclid_point2);
    impl_random_vec!(euclid::Point3D<f32, euclid::UnknownUnit>, random_euclid_point3);
    impl_random_vec!(euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_quat);
    impl_random_vec!(euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat3);
    impl_random_vec!(euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat4);
    impl_random_vec!(euclid::Vector2D<f32, euclid::UnknownUnit>, random_euclid_vec2);
    impl_random_vec!(euclid::Vector3D<f32, euclid::UnknownUnit>, random_euclid_vec3);

    // euclid random functions ----------------------------------------------------
    fn random_euclid_vec2<R>(rng: &mut R) -> euclid::Vector2D<f32, euclid::UnknownUnit>
    where
        R: Rng,
    {
        let (x, y) = rng.gen::<(f32, f32)>().into();
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
        let (x, y, z) = rng.gen::<(f32, f32, f32)>().into();
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
        let mq = random_mint_quat(rng);
        euclid::Rotation3D::quaternion(mq.v.x, mq.v.y, mq.v.z, mq.s)
    }

    fn random_euclid_mat3<R>(
        rng: &mut R,
    ) -> euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>
    where
        R: Rng,
    {
        let m = random_mint_homogeneous_mat3(rng);
        euclid::Transform2D::column_major(m.x.x, m.x.y, m.x.z, m.y.x, m.y.y, m.y.z)
    }

    fn random_euclid_mat4<R>(
        rng: &mut R,
    ) -> euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>
    where
        R: Rng,
    {
        let m = random_mint_homogeneous_mat4(rng);
        euclid::Transform3D::column_major(
            m.x.x, m.x.y, m.x.z, m.x.w, m.y.x, m.y.y, m.y.z, m.y.w, m.z.x, m.z.y, m.z.z, m.z.w,
            m.w.x, m.w.y, m.w.z, m.w.w,
        )
    }
}

#[cfg(feature = "vek")]
pub mod vek_support {
    use super::mint_support::*;
    use super::RandomVec;
    use rand::SeedableRng;
    use rand_xoshiro::Xoshiro256Plus;
    use vek;
    impl_random_vec!(
        vek::mat::repr_simd::column_major::Mat4<f32>,
        random_mint_homogeneous_mat4
    );
    impl_random_vec!(vek::vec::repr_simd::Vec3<f32>, random_mint_vec3);
    impl_random_vec!(vek::vec::repr_simd::Vec4<f32>, random_mint_vec4);

    // fn random_vek_invertible_mat4<R>(rng: &mut R) -> vek::mat::repr_simd::column_major::Mat4<f32>
    // where
    //     R: Rng,
    // {
    //     let mm = random_homogeneous_mat4(rng);
    //     vek::mat::repr_simd::column_major::Mat4::from_col_array(mm.into())
    // }

    // fn random_vek_vec3<R>(rng: &mut R) -> vek::vec::repr_simd::Vec3<f32>
    // where
    //     R: Rng,
    // {
    //     let v: [f32; 3] = rng.gen::<glam::Vec3>().into();
    //     v.into()
    // }

    // fn random_vek_vec4<R>(rng: &mut R) -> vek::vec::repr_simd::Vec4<f32>
    // where
    //     R: Rng,
    // {
    //     let v: [f32; 4] = rng.gen::<glam::Vec4>().into();
    //     v.into()
    // }

    pub fn vek_mat4_mul_mat4(
        m1: &vek::mat::repr_simd::column_major::Mat4<f32>,
        m2: &vek::mat::repr_simd::column_major::Mat4<f32>,
    ) -> vek::mat::repr_simd::column_major::Mat4<f32> {
        *m1 * *m2
    }

    pub fn vek_mat4_mul_vec4(
        m: &vek::mat::repr_simd::column_major::Mat4<f32>,
        v: &vek::vec::repr_simd::Vec4<f32>,
    ) -> vek::vec::repr_simd::Vec4<f32> {
        *m * *v
    }
}
