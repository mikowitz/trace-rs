use crate::{color, hittable::Hittable, ray::Ray};
use glam::Vec3;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;
use std::fs;

#[derive(Clone, Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,

    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn render<T>(&mut self, world: &T)
    where
        T: Hittable + 'static + Sync,
    {
        self.initialize();

        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(u32, u32)>>()
            .into_par_iter()
            .progress_count(self.image_width as u64 * self.image_height as u64)
            .map(|(y, x)| {
                let pixel_center = self.pixel00_loc
                    + (x as f32 * self.pixel_delta_u)
                    + (y as f32 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray {
                    origin: self.center,
                    direction: ray_direction,
                };
                let pixel_color = ray.color(world);
                color::to_ppm(pixel_color)
            })
            .collect::<Vec<String>>()
            .join("\n");

        fs::write(
            "image.ppm",
            format!(
                "P3\n{} {}\n255\n{pixels}\n",
                self.image_width, self.image_height,
            ),
        )
        .unwrap();
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f32 / self.aspect_ratio) as u32).max(1);

        self.center = Vec3::ZERO;

        let focal_length = 1.;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let viewport_u = Vec3::X * viewport_width;
        let viewport_v = Vec3::Y * -viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - Vec3::Z * focal_length - viewport_u / 2. - viewport_v / 2.;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
    }
}
