use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    ray::Ray,
};
use std::ops::Range;

pub struct BvhNode {
    pub left: Box<dyn Hittable + Sync>,
    pub right: Box<dyn Hittable + Sync>,
    pub bbox: Aabb,
}

impl BvhNode {
    pub fn new<T>(list: HittableList<T>) -> Self
    where
        T: Hittable + 'static + Clone + Sync,
    {
        let mut objects = list.objects.clone();

        let mut left = HittableList::<T>::new();
        let mut right = HittableList::<T>::new();

        let sort_axis = list.bounding_box().longest_axis();

        objects.sort_by(|a, b| {
            a.bounding_box()[sort_axis]
                .start
                .total_cmp(&b.bounding_box()[sort_axis].start)
        });

        let mid = objects.len() / 2;
        for o in &objects[..mid] {
            left.add(o.clone());
        }
        for o in &objects[mid..] {
            right.add(o.clone());
        }
        let bbox = Aabb::from_boxes(left.bounding_box().clone(), right.bounding_box());
        if objects.len() >= 32 {
            Self {
                left: Box::new(Self::new(left)),
                right: Box::new(Self::new(right)),
                bbox,
            }
        } else {
            Self {
                left: Box::new(left),
                right: Box::new(right),
                bbox,
            }
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, interval.clone());
        let right_interval = if let Some(hl) = hit_left.clone() {
            interval.start..hl.t
        } else {
            interval.clone()
        };
        let hit_right = self.right.hit(ray, right_interval);

        match (hit_left.clone(), hit_right.clone()) {
            (None, None) => None,
            (Some(_), None) => hit_left,
            (None, Some(_)) => hit_right,
            (Some(hl), Some(hr)) => {
                if hl.t <= hr.t {
                    hit_left
                } else {
                    hit_right
                }
            }
        }
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}
