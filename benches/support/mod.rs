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
// work around missing &Mat2 * &Vec2
pub fn glam_mat2_mul_vec2(m: &glam::Mat2, v: &glam::Vec2) -> glam::Vec2 {
    *m * *v
}

#[inline]
// work around missing &Mat3 * &Vec3
pub fn glam_mat3_mul_vec3(m: &glam::Mat3, v: &glam::Vec3) -> glam::Vec3 {
    *m * *v
}

#[inline]
// work around missing &Mat4 * &Vec4
pub fn glam_mat4_mul_vec4(m: &glam::Mat4, v: &glam::Vec4) -> glam::Vec4 {
    *m * *v
}

#[inline]
// work around missing &Quat * &Vec3
pub fn glam_quat_mul_vec3(q: &glam::Quat, v: &glam::Vec3) -> glam::Vec3 {
    *q * *v
}

#[inline]
// work around missing &Quat * &Vec3
pub fn glam_quat_mul_quat(q1: &glam::Quat, q2: &glam::Quat) -> glam::Quat {
    *q1 * *q2
}
