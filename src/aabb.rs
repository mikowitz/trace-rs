use std::ops::{Index, Range};

use glam::Vec3;

use crate::ray::Ray;

#[derive(Clone, Debug, Default)]
pub struct Aabb {
    pub x: Range<f32>,
    pub y: Range<f32>,
    pub z: Range<f32>,
}

impl Aabb {
    pub fn new(x: Range<f32>, y: Range<f32>, z: Range<f32>) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Vec3, b: Vec3) -> Self {
        let x = if a.x <= b.x { a.x..b.x } else { b.x..a.x };
        let y = if a.y <= b.y { a.y..b.y } else { b.y..a.y };
        let z = if a.z <= b.z { a.z..b.z } else { b.z..a.z };

        Self { x, y, z }
    }

    pub fn from_boxes(box0: Self, box1: &Self) -> Self {
        let x = Self::merge_intervals(box0.x, &box1.x);
        let y = Self::merge_intervals(box0.y, &box1.y);
        let z = Self::merge_intervals(box0.z, &box1.z);

        Self { x, y, z }
    }

    fn merge_intervals(a: Range<f32>, b: &Range<f32>) -> Range<f32> {
        a.start.min(b.start)..a.end.max(b.end)
    }

    pub fn longest_axis(&self) -> usize {
        let x = self.x.end - self.x.start;
        let y = self.y.end - self.y.start;
        let z = self.z.end - self.z.start;

        if x > y {
            if x > z {
                0
            } else {
                2
            }
        } else if y > z {
            1
        } else {
            2
        }
    }

    pub fn hit(&self, ray: &Ray, interval: Range<f32>) -> bool {
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;

        for axis in 0..3 {
            let ax = &self[axis];
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.start - ray_orig[axis]) * adinv;
            let t1 = (ax.end - ray_orig[axis]) * adinv;

            let mut min = interval.start;
            let mut max = interval.end;

            if t0 < t1 {
                if t0 > min {
                    min = t0
                };
                if t1 < max {
                    max = t1
                };
            } else {
                if t1 > min {
                    min = t1
                };
                if t0 < max {
                    max = t0
                };
            }

            if max <= min {
                return false;
            }
        }
        true
    }
}

impl Index<usize> for Aabb {
    type Output = Range<f32>;
    fn index(&self, index: usize) -> &Self::Output {
        match index {
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }
}
