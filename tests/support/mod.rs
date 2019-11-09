#[macro_use]
mod macros;

use glam::{Mat2, Mat3, Mat4, Quat, Vec2, Vec3, Vec4};
use nalgebra;

/// Trait used by the `assert_approx_eq` macro for floating point comparisons.
pub trait FloatCompare<Rhs: ?Sized = Self> {
    /// Return true if the absolute difference between `self` and `other` is
    /// less then or equal to `max_abs_diff`.
    fn approx_eq(&self, other: &Rhs, max_abs_diff: f32) -> bool;
    /// Returns the absolute difference of `self` and `other` which is printed
    /// if `assert_approx_eq` fails.
    fn abs_diff(&self, other: &Rhs) -> Rhs;
}

impl FloatCompare for f32 {
    #[inline]
    fn approx_eq(&self, other: &f32, max_abs_diff: f32) -> bool {
        self.abs_diff(other) <= max_abs_diff
    }
    #[inline]
    fn abs_diff(&self, other: &f32) -> f32 {
        (self - other).abs()
    }
}

impl FloatCompare for mint::Vector2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff) && self.y.approx_eq(&other.y, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
        }
    }
}

impl FloatCompare for mint::Vector3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
        }
    }
}

impl FloatCompare for mint::Vector4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
            && self.w.approx_eq(&other.w, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
            w: self.w.abs_diff(&other.w),
        }
    }
}

impl FloatCompare for mint::Quaternion<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.v.approx_eq(&other.v, max_abs_diff) && self.s.approx_eq(&other.s, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            v: self.v.abs_diff(&other.v),
            s: self.s.abs_diff(&other.s),
        }
    }
}

impl FloatCompare for mint::ColumnMatrix2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff) && self.y.approx_eq(&other.y, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
        }
    }
}

impl FloatCompare for mint::ColumnMatrix3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
        }
    }
}

impl FloatCompare for mint::ColumnMatrix4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
            && self.w.approx_eq(&other.w, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
            w: self.w.abs_diff(&other.w),
        }
    }
}

// glam -----------------------------------------------------------------------
impl FloatCompare for Mat2 {
    #[inline]
    fn approx_eq(&self, other: &Mat2, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Mat2) -> Mat2 {
        Mat2::from_cols(
            (self.x_axis() - other.x_axis()).abs(),
            (self.y_axis() - other.y_axis()).abs(),
        )
    }
}

impl FloatCompare for Mat3 {
    #[inline]
    fn approx_eq(&self, other: &Mat3, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Mat3) -> Mat3 {
        Mat3::from_cols(
            (self.x_axis() - other.x_axis()).abs(),
            (self.y_axis() - other.y_axis()).abs(),
            (self.z_axis() - other.z_axis()).abs(),
        )
    }
}

impl FloatCompare for Mat4 {
    #[inline]
    fn approx_eq(&self, other: &Mat4, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Mat4) -> Mat4 {
        Mat4::from_cols(
            (self.x_axis() - other.x_axis()).abs(),
            (self.y_axis() - other.y_axis()).abs(),
            (self.z_axis() - other.z_axis()).abs(),
            (self.w_axis() - other.w_axis()).abs(),
        )
    }
}

impl FloatCompare for Quat {
    #[inline]
    fn approx_eq(&self, other: &Quat, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Quat) -> Quat {
        let a: Vec4 = (*self).into();
        let b: Vec4 = (*other).into();
        (a - b).abs().into()
    }
}

impl FloatCompare for Vec2 {
    #[inline]
    fn approx_eq(&self, other: &Vec2, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Vec2) -> Vec2 {
        (*self - *other).abs()
    }
}

impl FloatCompare for Vec3 {
    #[inline]
    fn approx_eq(&self, other: &Vec3, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Vec3) -> Vec3 {
        (*self - *other).abs()
    }
}

impl FloatCompare for Vec4 {
    #[inline]
    fn approx_eq(&self, other: &Vec4, max_abs_diff: f32) -> bool {
        self.abs_diff_eq(*other, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Vec4) -> Vec4 {
        (*self - *other).abs()
    }
}

// cgmath ---------------------------------------------------------------------
impl FloatCompare for cgmath::Vector2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff) && self.y.approx_eq(&other.y, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
        }
    }
}

impl FloatCompare for cgmath::Vector3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
        }
    }
}

impl FloatCompare for cgmath::Vector4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
            && self.w.approx_eq(&other.w, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
            w: self.w.abs_diff(&other.w),
        }
    }
}

impl FloatCompare for cgmath::Quaternion<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.v.approx_eq(&other.v, max_abs_diff) && self.s.approx_eq(&other.s, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            s: self.s.abs_diff(&other.s),
            v: self.v.abs_diff(&other.v),
        }
    }
}

impl FloatCompare for cgmath::Matrix2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff) && self.y.approx_eq(&other.y, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
        }
    }
}

impl FloatCompare for cgmath::Matrix3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
        }
    }
}

impl FloatCompare for cgmath::Matrix4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
            && self.w.approx_eq(&other.w, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self {
            x: self.x.abs_diff(&other.x),
            y: self.y.abs_diff(&other.y),
            z: self.z.abs_diff(&other.z),
            w: self.w.abs_diff(&other.w),
        }
    }
}

// nalgebra -------------------------------------------------------------------
impl FloatCompare for nalgebra::Vector2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff) && self.y.approx_eq(&other.y, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::new(self.x.abs_diff(&other.x), self.y.abs_diff(&other.y))
    }
}

impl FloatCompare for nalgebra::Vector3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::new(
            self.x.abs_diff(&other.x),
            self.y.abs_diff(&other.y),
            self.z.abs_diff(&other.z),
        )
    }
}

impl FloatCompare for nalgebra::Vector4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.x.approx_eq(&other.x, max_abs_diff)
            && self.y.approx_eq(&other.y, max_abs_diff)
            && self.z.approx_eq(&other.z, max_abs_diff)
            && self.w.approx_eq(&other.w, max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::new(
            self.x.abs_diff(&other.x),
            self.y.abs_diff(&other.y),
            self.z.abs_diff(&other.z),
            self.w.abs_diff(&other.w),
        )
    }
}

impl FloatCompare for nalgebra::Quaternion<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.as_vector().approx_eq(&other.as_vector(), max_abs_diff)
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        Self::from(self.as_vector().abs_diff(other.as_vector()))
    }
}

impl FloatCompare for nalgebra::Matrix2<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        for (e1, e2) in self.as_slice().iter().zip(other.as_slice().iter()) {
            if !e1.approx_eq(e2, max_abs_diff) {
                return false;
            }
        }
        true
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (self - other).abs()
    }
}

impl FloatCompare for nalgebra::Matrix3<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        for (e1, e2) in self.as_slice().iter().zip(other.as_slice().iter()) {
            if !e1.approx_eq(e2, max_abs_diff) {
                return false;
            }
        }
        true
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (self - other).abs()
    }
}

impl FloatCompare for nalgebra::Matrix4<f32> {
    #[inline]
    fn approx_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        for (e1, e2) in self.as_slice().iter().zip(other.as_slice().iter()) {
            if !e1.approx_eq(e2, max_abs_diff) {
                return false;
            }
        }
        true
    }
    #[inline]
    fn abs_diff(&self, other: &Self) -> Self {
        (self - other).abs()
    }
}
