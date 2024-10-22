use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct Sphere {
    center: Ray,
    radius: f32,
    material: Material,
    bbox: Aabb,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Material) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(center, Vec3::new(0., 0., 0.), 0.),
            radius,
            material,
            bbox: Aabb::from_extrema(center - rvec, center + rvec),
        }
    }

    pub fn moving(center1: Point3, center2: Point3, radius: f32, material: Material) -> Self {
        let center = Ray::new(center1, center2 - center1, 0.);
        let rvec = Vec3::new(radius, radius, radius);
        let box0 = Aabb::from_extrema(center.at(0.) - rvec, center.at(0.) + rvec);
        let box1 = Aabb::from_extrema(center.at(1.) - rvec, center.at(1.) + rvec);
        Self {
            center,
            radius,
            material,
            bbox: Aabb::from_boxes(&box0, &box1),
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        let current_center = self.center.at(ray.time);
        let oc = current_center - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
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

        Some(HitRecord::new(p, t, &normal, ray, self.material))
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
