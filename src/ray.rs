use glam::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, time: f32) -> Vec3 {
        self.origin + time * self.direction
    }

    pub fn color(&self) -> Vec3 {
        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);

        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
    }
}
