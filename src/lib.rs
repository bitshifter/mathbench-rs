use rand::Rng;

pub trait BenchValue {
    fn random_value<R: Rng>(rng: &mut R) -> Self;
    // Return self to test overhead of benches
    fn ret_self(&self) -> Self
    where
        Self: Copy + std::marker::Sized,
    {
        *self
    }
}

impl BenchValue for f32 {
    fn random_value<R: Rng>(rng: &mut R) -> Self {
        rng.gen()
    }
}

macro_rules! impl_bench_value {
    ($t:ty, $f:expr) => {
        impl BenchValue for $t {
            fn random_value<R: rand::Rng>(rng: &mut R) -> Self {
                $f(rng).into()
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
    use super::BenchValue;
    use glam;
    use rand::Rng;
    impl_bench_value!(glam::Mat2, random_mint_invertible_mat2);
    impl_bench_value!(glam::Mat3, random_mint_homogeneous_mat3);
    impl_bench_value!(glam::Mat4, random_mint_homogeneous_mat4);
    impl_bench_value!(glam::Quat, random_mint_quat);
    impl_bench_value!(glam::Vec2, random_mint_vec2);
    impl_bench_value!(glam::Vec3, random_mint_vec3);
    impl_bench_value!(glam::Vec4, random_mint_vec4);

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
    use super::BenchValue;
    use rand::Rng;
    impl_bench_value!(
        cgmath::Decomposed<cgmath::Vector3<f32>, cgmath::Quaternion<f32>>,
        random_cgmath_decomposed3
    );
    impl_bench_value!(cgmath::Matrix2<f32>, random_mint_invertible_mat2);
    impl_bench_value!(cgmath::Matrix3<f32>, random_mint_homogeneous_mat3);
    impl_bench_value!(cgmath::Matrix4<f32>, random_mint_homogeneous_mat4);
    impl_bench_value!(cgmath::Point2<f32>, random_cgmath_point2);
    impl_bench_value!(cgmath::Point3<f32>, random_cgmath_point3);
    impl_bench_value!(cgmath::Quaternion<f32>, random_mint_quat);
    impl_bench_value!(cgmath::Vector2<f32>, random_mint_vec2);
    impl_bench_value!(cgmath::Vector3<f32>, random_mint_vec3);
    impl_bench_value!(cgmath::Vector4<f32>, random_mint_vec4);

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
    use super::BenchValue;
    use rand::Rng;
    impl_bench_value!(nalgebra::Matrix2<f32>, random_mint_invertible_mat2);
    impl_bench_value!(nalgebra::Matrix3<f32>, random_mint_homogeneous_mat3);
    impl_bench_value!(nalgebra::Matrix4<f32>, random_mint_homogeneous_mat4);
    impl_bench_value!(nalgebra::Point2<f32>, random_na_point2);
    impl_bench_value!(nalgebra::Point3<f32>, random_na_point3);
    impl_bench_value!(nalgebra::Transform2<f32>, random_na_transform2);
    impl_bench_value!(nalgebra::Transform3<f32>, random_na_transform3);
    impl_bench_value!(nalgebra::UnitQuaternion<f32>, random_na_quat);
    impl_bench_value!(nalgebra::Vector2<f32>, random_na_vec2);
    impl_bench_value!(nalgebra::Vector3<f32>, random_na_vec3);
    impl_bench_value!(nalgebra::Vector4<f32>, random_na_vec4);

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

pub mod static_math_support {
    use super::BenchValue;
    use rand::Rng;
    use static_math;
    use static_math::matrix2x2::M22;
    use static_math::vector2::V2;
    use static_math::matrix3x3::M33;
    use static_math::vector3::V3;
    use static_math::matrix4x4::M44;
    use static_math::vector4::V4;

    fn random_nonzero_f32<R>(rng: &mut R) -> f32
    where
        R: Rng,
    {
        rng.gen_range(0.1, 1.0)
    }

    impl BenchValue for M22<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a = random_nonzero_f32(rng);
            let b = random_nonzero_f32(rng);
            let c = random_nonzero_f32(rng);
            let d = random_nonzero_f32(rng);
            M22::new([[a, b],
                     [c, d]])
        }
    }

    impl BenchValue for V2<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a = random_nonzero_f32(rng);
            let b = random_nonzero_f32(rng);
            V2::new([a, b])
        }
    }

    impl BenchValue for M33<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a_00 = random_nonzero_f32(rng);
            let a_01 = random_nonzero_f32(rng);
            let a_02 = random_nonzero_f32(rng);
            let a_10 = random_nonzero_f32(rng);
            let a_11 = random_nonzero_f32(rng);
            let a_12 = random_nonzero_f32(rng);
            let a_20 = random_nonzero_f32(rng);
            let a_21 = random_nonzero_f32(rng);
            let a_22 = random_nonzero_f32(rng);
            M33::new([[a_00, a_01, a_02],
                      [a_10, a_11, a_12],
                      [a_20, a_21, a_22]])
        }
    }
    impl BenchValue for V3<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a = random_nonzero_f32(rng);
            let b = random_nonzero_f32(rng);
            let c = random_nonzero_f32(rng);
            V3::new([a, b, c])
        }
    }
    impl BenchValue for M44<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a1 = random_nonzero_f32(rng);
            let a2 = random_nonzero_f32(rng);
            let a3 = random_nonzero_f32(rng);
            let a4 = random_nonzero_f32(rng);
            let a5 = random_nonzero_f32(rng);
            let a6 = random_nonzero_f32(rng);
            let a7 = random_nonzero_f32(rng);
            let a8 = random_nonzero_f32(rng);
            let a9 = random_nonzero_f32(rng);
            let a10 = random_nonzero_f32(rng);
            let a11 = random_nonzero_f32(rng);
            let a12 = random_nonzero_f32(rng);
            let a13 = random_nonzero_f32(rng);
            let a14 = random_nonzero_f32(rng);
            let a15 = random_nonzero_f32(rng);
            let a16 = random_nonzero_f32(rng);
            M44::new([[a1, a5,  a9, a13],
                      [a2, a6, a10, a14],
                      [a3, a7, a11, a15],
                      [a4, a8, a12, a16]])

        }
    }

    impl BenchValue for V4<f32> {
        fn random_value<R: Rng>(rng: &mut R) -> Self {
            let a = random_nonzero_f32(rng);
            let b = random_nonzero_f32(rng);
            let c = random_nonzero_f32(rng);
            let d = random_nonzero_f32(rng);
            V4::new([a, b, c, d])
        }
    }
}

#[cfg(feature = "euclid")]
pub mod euclid_support {
    use super::mint_support::*;
    use super::BenchValue;
    use rand::Rng;

    impl_bench_value!(euclid::Point2D<f32, euclid::UnknownUnit>, random_euclid_point2);
    impl_bench_value!(euclid::Point3D<f32, euclid::UnknownUnit>, random_euclid_point3);
    impl_bench_value!(euclid::Rotation3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_quat);
    impl_bench_value!(euclid::Transform2D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat3);
    impl_bench_value!(euclid::Transform3D<f32, euclid::UnknownUnit, euclid::UnknownUnit>, random_euclid_mat4);
    impl_bench_value!(euclid::Vector2D<f32, euclid::UnknownUnit>, random_euclid_vec2);
    impl_bench_value!(euclid::Vector3D<f32, euclid::UnknownUnit>, random_euclid_vec3);

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
    use super::BenchValue;
    use vek;
    impl_bench_value!(vek::Mat2<f32>, random_mint_invertible_mat2);
    impl_bench_value!(vek::Mat3<f32>, random_mint_homogeneous_mat3);
    impl_bench_value!(vek::Mat4<f32>, random_mint_homogeneous_mat4);
    impl_bench_value!(vek::Quaternion<f32>, random_mint_quat);
    impl_bench_value!(vek::Vec2<f32>, random_mint_vec2);
    impl_bench_value!(vek::Vec3<f32>, random_mint_vec3);
    impl_bench_value!(vek::Vec4<f32>, random_mint_vec4);

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

    pub fn vek_mat4_mul_mat4(m1: vek::Mat4<f32>, m2: vek::Mat4<f32>) -> vek::Mat4<f32> {
        m1 * m2
    }

    pub fn vek_mat4_mul_vec4(m: vek::Mat4<f32>, v: vek::Vec4<f32>) -> vek::Vec4<f32> {
        m * v
    }
}

#[cfg(feature = "pathfinder_geometry")]
pub mod pathfinder_support {
    use super::mint_support::*;
    use super::BenchValue;
    use rand::Rng;

    impl_bench_value!(pathfinder_geometry::vector::Vector2F, random_pf_vec2);
    impl_bench_value!(pathfinder_geometry::vector::Vector4F, random_pf_vec4);
    impl_bench_value!(pathfinder_geometry::transform2d::Matrix2x2F, random_pf_mat2);
    impl_bench_value!(
        pathfinder_geometry::transform2d::Transform2F,
        random_pf_mat3
    );
    impl_bench_value!(
        pathfinder_geometry::transform3d::Transform4F,
        random_pf_mat4
    );

    pub fn random_pf_vec2<R>(rng: &mut R) -> pathfinder_geometry::vector::Vector2F
    where
        R: Rng,
    {
        pathfinder_geometry::vector::Vector2F::new(rng.gen(), rng.gen())
    }

    pub fn random_pf_vec4<R>(rng: &mut R) -> pathfinder_geometry::vector::Vector4F
    where
        R: Rng,
    {
        pathfinder_geometry::vector::Vector4F::new(rng.gen(), rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_pf_mat2<R>(rng: &mut R) -> pathfinder_geometry::transform2d::Matrix2x2F
    where
        R: Rng,
    {
        let mat = random_mint_invertible_mat2(rng);
        pathfinder_geometry::transform2d::Matrix2x2F::row_major(mat.x.x, mat.y.x, mat.x.y, mat.y.y)
    }

    pub fn random_pf_mat3<R>(rng: &mut R) -> pathfinder_geometry::transform2d::Transform2F
    where
        R: Rng,
    {
        let mat = random_mint_homogeneous_mat3(rng);
        pathfinder_geometry::transform2d::Transform2F::row_major(
            mat.x.x, mat.y.x, mat.x.y, mat.y.y, mat.x.z, mat.y.z,
        )
    }

    pub fn random_pf_mat4<R>(rng: &mut R) -> pathfinder_geometry::transform3d::Transform4F
    where
        R: Rng,
    {
        let mat = random_mint_homogeneous_mat4(rng);
        pathfinder_geometry::transform3d::Transform4F::row_major(
            mat.x.x, mat.y.x, mat.z.x, mat.w.x, mat.x.y, mat.y.y, mat.z.y, mat.w.y, mat.x.z,
            mat.y.z, mat.z.z, mat.w.z, mat.x.w, mat.y.w, mat.z.w, mat.w.w,
        )
    }
}
