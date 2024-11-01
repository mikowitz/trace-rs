use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
};
use glam::Vec3;
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Ray,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center: Ray {
                origin: center,
                direction: Vec3::ZERO,
                time: 0.,
            },
            radius,
            material,
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center: Ray {
                origin: center1,
                direction: center2 - center1,
                time: 0.,
            },
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0. {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (h - sqrtd) / a;
        if !interval.contains(&root) {
            root = (h + sqrtd) / a;
            if !interval.contains(&root) {
                return None;
            }
        }

        let t = root;
        let p = ray.at(t);
        let normal = (p - current_center) / self.radius;

        Some(HitRecord::with_front_face(
            p,
            t,
            normal,
            self.material.clone(),
            ray,
        ))
    }
}
