use crate::ray::Ray;
use glam::Vec3;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn with_front_face(p: Vec3, t: f32, outward_normal: Vec3, ray: &Ray) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        Self {
            p,
            normal,
            t,
            front_face,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord>;
}
