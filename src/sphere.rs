use crate::{
    aabb::Aabb,
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

    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        let rvec = Vec3::splat(radius);
        Self {
            center: Ray {
                origin: center,
                direction: Vec3::ZERO,
                time: 0.,
            },
            radius,
            material,
            bbox: Aabb::from_points(center - rvec, center + rvec),
        }
    }

    pub fn moving(center1: Vec3, center2: Vec3, radius: f32, material: Material) -> Self {
        let rvec = Vec3::splat(radius);
        let center = Ray {
            origin: center1,
            direction: center2 - center1,
            time: 0.,
        };
        let box1 = Aabb::from_points(center.at(0.) - rvec, center.at(0.) + rvec);
        let box2 = Aabb::from_points(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius,
            material,
            bbox: Aabb::from_boxes(box1, &box2),
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

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
