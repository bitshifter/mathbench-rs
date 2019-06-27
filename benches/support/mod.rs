#![allow(dead_code)]

use glam;
use cgmath::{self, InnerSpace};

#[inline]
// work around missing cgmath Vector3::dot(self, &Vector3)
pub fn cgmath_vec3_dot(v1: &cgmath::Vector3<f32>, v2: &cgmath::Vector3<f32>) -> f32 {
    v1.dot(*v2)
}

#[inline]
// work around missing cgmath Vector3::cross(self, &Vector3)
pub fn cgmath_vec3_cross(v1: &cgmath::Vector3<f32>, v2: &cgmath::Vector3<f32>) -> cgmath::Vector3<f32> {
    v1.cross(*v2)
}
#[inline]
// work around missing Vec3::dot(self, &Vec3)
pub fn glam_vec3_dot(v1: &glam::Vec3, v2: &glam::Vec3) -> f32 {
    v1.dot(*v2)
}

#[inline]
// work around missing Vec3::cross(self, &Vec3)
pub fn glam_vec3_cross(v1: &glam::Vec3, v2: &glam::Vec3) -> glam::Vec3 {
    v1.cross(*v2)
}

#[inline]
// work around missing &Mat4 * &Vec4
pub fn glam_mat4_mul_vec4(m: &glam::Mat4, v: &glam::Vec4) -> glam::Vec4 {
    *m * *v
}

#[inline]
// work around missing &Mat4 * &Vec4
pub fn vek_mat4_mul_vec4(m: &vek::Mat4<f32>, v: &vek::Vec4<f32>) -> vek::Vec4<f32> {
    *m * *v
}

#[inline]
// work around missing &Quat * &Vec3
pub fn glam_quat_mul_vec3(q: &glam::Quat, v: &glam::Vec3) -> glam::Vec3 {
    *q * *v
}

#[inline]
// work around missing &Quat * &Quat
pub fn glam_quat_mul_quat(q1: &glam::Quat, q2: &glam::Quat) -> glam::Quat {
    *q1 * *q2
}

#[inline]
// work around missing &Mat2 * &Mat2
pub fn glam_mat2_mul_mat2(m1: &glam::Mat2, m2: &glam::Mat2) -> glam::Mat2 {
    *m1 * *m2
}

#[inline]
// work around missing &Mat3 * &Mat3
pub fn glam_mat3_mul_mat3(m1: &glam::Mat3, m2: &glam::Mat3) -> glam::Mat3 {
    *m1 * *m2
}

#[inline]
// work around missing &Mat4 * &Mat4
pub fn glam_mat4_mul_mat4(m1: &glam::Mat4, m2: &glam::Mat4) -> glam::Mat4 {
    *m1 * *m2
}

#[inline]
// work around missing &Mat4 * &Mat4
pub fn vek_mat4_mul_mat4(m1: &vek::Mat4<f32>, m2: &vek::Mat4<f32>) -> vek::Mat4<f32> {
    *m1 * *m2
}

