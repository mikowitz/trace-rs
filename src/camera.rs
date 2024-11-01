use crate::{color, hittable::Hittable, ray::Ray, vector::random_in_unit_disk};
use glam::Vec3;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rand::Rng;
use rayon::prelude::*;
use std::fs;

#[derive(Clone, Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: u32,
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub vfov: f32,
    pub lookfrom: Vec3,
    pub lookat: Vec3,
    pub vup: Vec3,

    pub defocus_angle: f32,
    pub focus_dist: f32,

    image_height: u32,
    center: Vec3,
    pixel00_loc: Vec3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel_samples_scale: f32,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
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
                let mut pixel_color = Vec3::ZERO;
                for _ in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += ray.color(world, self.max_depth);
                }
                pixel_color *= self.pixel_samples_scale;
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

    fn get_ray(&self, x: u32, y: u32) -> Ray {
        let mut rng = rand::thread_rng();
        let x_offset = rng.gen::<f32>();
        let y_offset = rng.gen::<f32>();

        let pixel_sample = self.pixel00_loc
            + (x as f32 + x_offset) * self.pixel_delta_u
            + (y as f32 + y_offset) * self.pixel_delta_v;

        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;

        Ray { origin, direction }
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f32 / self.aspect_ratio) as u32).max(1);

        self.center = self.lookfrom;

        self.pixel_samples_scale = (self.samples_per_pixel as f32).recip();

        let theta = self.vfov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = (self.lookfrom - self.lookat).normalize();
        let u = self.vup.cross(w).normalize();
        let v = w.cross(u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - w * self.focus_dist - viewport_u / 2. - viewport_v / 2.;

        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();

        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.center + p.x * self.defocus_disk_u + p.y * self.defocus_disk_v
    }
}
