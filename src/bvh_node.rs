use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    ray::Ray,
};
use std::ops::Range;

pub struct BvhNode {
    left: Box<dyn Hittable + Sync>,
    right: Box<dyn Hittable + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new<T>(mut objects: Vec<T>) -> Self
    where
        T: Hittable + Clone + 'static + Sync,
    {
        if objects.len() == 1 {
            let left: T = objects[0].clone();
            let right: T = objects[0].clone();
            let bbox = Aabb::from_boxes(&left.bounding_box(), &right.bounding_box());
            return Self {
                left: Box::new(left),
                right: Box::new(right),
                bbox,
            };
        } else if objects.len() == 2 {
            let left: T = objects[0].clone();
            let right: T = objects[1].clone();
            let bbox = Aabb::from_boxes(&left.bounding_box(), &right.bounding_box());
            return Self {
                left: Box::new(left),
                right: Box::new(right),
                bbox,
            };
        }

        let b = objects.bounding_box();
        let xr = b.axis_interval(0);
        let yr = b.axis_interval(1);
        let zr = b.axis_interval(2);
        let x = xr.end - xr.start;
        let y = yr.end - yr.start;
        let z = zr.end - zr.start;

        let index = if x > y {
            if x > z {
                0
            } else {
                2
            }
        } else if y > z {
            1
        } else {
            2
        };
        let mid = objects.len() / 2;
        objects.sort_by(|a, b| {
            (a.bounding_box().axis_interval(index).start)
                .total_cmp(&b.bounding_box().axis_interval(index).start)
        });
        let left: Vec<T> = objects[..mid].to_vec();
        let right: Vec<T> = objects[mid..].to_vec();
        let bbox = Aabb::from_boxes(&left.bounding_box(), &right.bounding_box());
        Self {
            left: Box::new(left),
            right: Box::new(right),
            bbox,
        }
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray: &Ray, interval: Range<f32>) -> Option<HitRecord> {
        if !self.bbox.hit(ray, interval.clone()) {
            return None;
        }

        let hit_left = self.left.hit(ray, interval.clone());
        let right_interval = if let Some(hl) = hit_left {
            interval.start..hl.t
        } else {
            interval.clone()
        };
        let hit_right = self.right.hit(ray, right_interval);

        match (hit_left, hit_right) {
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

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}
