use std::ops::Range;

use crate::{
    material::Material,
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new(p: Point3, t: f32, outward_normal: &Vec3, ray: &Ray, material: Material) -> Self {
        let front_face = ray.direction.dot(outward_normal) < 0.;
        let normal = if front_face {
            *outward_normal
        } else {
            -*outward_normal
        };
        Self {
            p,
            normal,
            t,
            front_face,
            material,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord>;
}

impl<T> Hittable for Vec<T>
where
    T: Hittable + 'static,
{
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        self.iter()
            .fold((None, interval.end), |acc, hittable| {
                if let Some(rec) = hittable.hit(ray, interval.start..acc.1) {
                    return (Some(rec), rec.t);
                }
                acc
            })
            .0
    }
}
