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
        if hit_sphere(-Vec3::Z, 0.5, self) {
            return Vec3::X;
        }

        let unit_direction = self.direction.normalize();
        let a = 0.5 * (unit_direction.y + 1.0);
        Vec3::ONE.lerp(Vec3::new(0.5, 0.7, 1.0), a)
    }
}

fn hit_sphere(center: Vec3, radius: f32, ray: &Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.dot(ray.direction);
    let b = -2. * ray.direction.dot(oc);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;
    discriminant >= 0.
}
