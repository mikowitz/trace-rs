use std::ops::Range;

use crate::{ray::Ray, vec3::Point3};

#[derive(Clone, Debug, Default)]
pub struct Aabb {
    x: Range<f32>,
    y: Range<f32>,
    z: Range<f32>,
}

impl Aabb {
    pub fn new(x: Range<f32>, y: Range<f32>, z: Range<f32>) -> Self {
        Self { x, y, z }
    }

    pub fn from_extrema(a: Point3, b: Point3) -> Self {
        let x = if a[0] <= b[0] { a[0]..b[0] } else { b[0]..a[0] };
        let y = if a[1] <= b[1] { a[1]..b[1] } else { b[1]..a[1] };
        let z = if a[2] <= b[2] { a[2]..b[2] } else { b[2]..a[2] };

        Self { x, y, z }
    }

    pub fn from_boxes(box0: &Self, box1: &Self) -> Self {
        let x = Aabb::combine_ranges(box0.clone().x, box1.clone().x);
        let y = Aabb::combine_ranges(box0.clone().y, box1.clone().y);
        let z = Aabb::combine_ranges(box0.clone().z, box1.clone().z);

        Self { x, y, z }
    }

    fn combine_ranges(a: Range<f32>, b: Range<f32>) -> Range<f32> {
        let min = a.start.min(b.start);
        let max = a.end.max(b.end);

        min..max
    }

    pub fn axis_interval(&self, n: usize) -> Range<f32> {
        if n == 1 {
            return self.y.clone();
        }
        if n == 2 {
            return self.z.clone();
        }
        self.x.clone()
    }

    pub fn hit(&self, ray: &Ray, interval: Range<f32>) -> bool {
        let ray_orig = ray.origin;
        let ray_dir = ray.direction;

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = ray_dir[axis].recip();

            let mut min = interval.start;
            let mut max = interval.end;

            let t0 = (ax.start - ray_orig[axis]) * adinv;
            let t1 = (ax.end - ray_orig[axis]) * adinv;

            if t0 < t1 {
                if t0 > min {
                    min = t0;
                }
                if t1 < max {
                    max = t1;
                }
            } else {
                if t1 > min {
                    min = t1;
                }
                if t0 < max {
                    max = t0;
                }
            }

            if max <= min {
                return false;
            }
        }
        true
    }
}
