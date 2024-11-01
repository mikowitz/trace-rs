use crate::{hittable::Hittable, vector};
use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, time: f32) -> Vec3 {
        self.origin + time * self.direction
    }

    pub fn color<T>(&self, world: &T, depth: usize) -> Vec3
    where
        T: Hittable + 'static + Sync,
    {
        if depth == 0 {
            return Vec3::ZERO;
        }

        if let Some(hit_rec) = world.hit(self, 0.001..f32::INFINITY) {
            let direction = hit_rec.normal + vector::random_unit_vector();
            let r = Self {
                origin: hit_rec.p,
                direction,
            };
            return 0.5 * r.color(world, depth - 1);
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
    }
}
