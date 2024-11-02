use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::ops::Range;

#[derive(Clone, Debug)]
pub struct HittableList<T>
where
    T: Hittable + 'static,
{
    pub objects: Vec<T>,
    bbox: Aabb,
}

impl<T> HittableList<T>
where
    T: Hittable + 'static + Clone,
{
    pub fn new() -> Self {
        Self {
            objects: vec![],
            bbox: Aabb::default(),
        }
    }

    pub fn add(&mut self, object: T) {
        self.objects.push(object.clone());
        self.bbox = Aabb::from_boxes(self.bbox.clone(), object.bounding_box());
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable + 'static,
{
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        self.objects
            .iter()
            .fold((None, interval.end), |(acc, max), object| {
                if let Some(hit_rec) = object.hit(ray, interval.start..max) {
                    (Some(hit_rec.clone()), hit_rec.t)
                } else {
                    (acc, max)
                }
            })
            .0
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl<T> Default for HittableList<T>
where
    T: Hittable + 'static + Clone,
{
    fn default() -> Self {
        Self::new()
    }
}
