use crate::{
    color::Color,
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use indicatif::ProgressIterator;
use itertools::Itertools;
use std::fs;

#[derive(Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    image_height: i32,
    pixel00_loc: Point3,
    center: Point3,
    pixel_δ_u: Vec3,
    pixel_δ_v: Vec3,
}

impl Camera {
    pub fn render<T>(&mut self, world: &T)
    where
        T: Hittable + 'static,
    {
        self.initialize();
        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .progress_count(self.image_width as u64 * self.image_height as u64)
            .map(|(y, x)| {
                let pixel_center =
                    self.pixel00_loc + (self.pixel_δ_u * x as f32) + (self.pixel_δ_v * y as f32);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction);
                let color = Camera::ray_color(&ray, world);
                color.to_ppm()
            })
            .join("\n");

        fs::write(
            "image.ppm",
            format!(
                "P3\n{} {}\n255\n{pixels}\n",
                self.image_width, self.image_height
            ),
        )
        .expect("image.ppm written");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        if self.image_height < 1 {
            self.image_height = 1;
        }

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        self.center = Point3::new(0., 0., 0.);

        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        self.pixel_δ_u = viewport_u / self.image_width as f32;
        self.pixel_δ_v = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + (self.pixel_δ_u + self.pixel_δ_v) * 0.5;
    }

    fn ray_color<T>(ray: &Ray, world: &T) -> Color
    where
        T: Hittable + 'static,
    {
        if let Some(hit_rec) = world.hit(ray, 0.0..f32::INFINITY) {
            return (hit_rec.normal + Color::white()) * 0.5;
        }
        let unit_direction = ray.direction.unit_vector();
        let a = 0.5 * (unit_direction[1] + 1.0);
        Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 100,
            pixel00_loc: Point3::new(0., 0., 0.),
            center: Point3::new(0., 0., 0.),
            pixel_δ_u: Vec3::new(1., 0., 0.),
            pixel_δ_v: Vec3::new(0., -1., 0.),
        }
    }
}
