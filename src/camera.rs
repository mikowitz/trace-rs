use crate::{
    color::Color,
    hittable::Hittable,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use indicatif::{ParallelProgressIterator, ProgressStyle};
use itertools::Itertools;
use rand::*;
use rayon::prelude::*;
use std::fs;

#[derive(Clone, Debug)]
pub struct Camera {
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f32,
    pub lookfrom: Point3,
    pub lookto: Point3,
    pub vup: Vec3,
    pub defocus_angle: f32,
    pub focus_dist: f32,
    image_height: i32,
    pixel_samples_scale: f32,
    pixel00_loc: Point3,
    center: Point3,
    pixel_δ_u: Vec3,
    pixel_δ_v: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn render<T>(&mut self, world: &T)
    where
        T: Hittable + 'static + Sync,
    {
        self.initialize();

        let style = ProgressStyle::with_template(
            "[{elapsed_precise}/{eta_precise}] {bar:40.cyan/red} {pos:>8}/{len:8}",
        )
        .unwrap();

        let pixels = (0..self.image_height)
            .cartesian_product(0..self.image_width)
            .collect::<Vec<(i32, i32)>>()
            .into_par_iter()
            .progress_with_style(style)
            .map(|(y, x)| {
                let pixel_color = (0..self.samples_per_pixel).fold(Color::black(), |color, _| {
                    color + Camera::ray_color(&self.get_ray(x, y), world, self.max_depth)
                }) * self.pixel_samples_scale;
                pixel_color.to_ppm()
            })
            .collect::<Vec<String>>()
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

        self.pixel_samples_scale = (self.samples_per_pixel as f32).recip();

        self.center = self.lookfrom;

        let θ = self.vfov.to_radians();
        let h = (θ / 2.).tan();
        let viewport_height = 2. * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f32 / self.image_height as f32);

        let w = (self.lookfrom - self.lookto).unit_vector();
        let u = self.vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let viewport_u = u * viewport_width;
        let viewport_v = -v * viewport_height;

        self.pixel_δ_u = viewport_u / self.image_width as f32;
        self.pixel_δ_v = viewport_v / self.image_height as f32;

        let viewport_upper_left =
            self.center - (w * self.focus_dist) - viewport_u / 2. - viewport_v / 2.;
        self.pixel00_loc = viewport_upper_left + (self.pixel_δ_u + self.pixel_δ_v) * 0.5;

        let defocus_radius = self.focus_dist * (self.defocus_angle / 2.).to_radians().tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    fn get_ray(&self, x: i32, y: i32) -> Ray {
        let mut rng = rand::thread_rng();
        let x_offset = rng.gen::<f32>() - 0.5;
        let y_offset = rng.gen::<f32>() - 0.5;

        let pixel_sample = self.pixel00_loc
            + (self.pixel_δ_u * (x as f32 + x_offset))
            + (self.pixel_δ_v * (y as f32 + y_offset));
        let origin = if self.defocus_angle <= 0. {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let direction = pixel_sample - origin;
        Ray::new(origin, direction, rng.gen::<f32>())
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        self.center + (self.defocus_disk_u * p[0]) + (self.defocus_disk_v * p[1])
    }

    fn ray_color<T>(ray: &Ray, world: &T, depth: i32) -> Color
    where
        T: Hittable + 'static,
    {
        if depth <= 0 {
            return Color::black();
        }
        if let Some(hit_rec) = world.hit(ray, 0.001..f32::INFINITY) {
            if let Some(scatter) = hit_rec.material.scatter(ray, &hit_rec) {
                return Camera::ray_color(&scatter.scattered, world, depth - 1)
                    * scatter.attenuation;
            }
            return Color::black();
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
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0., 0., 0.),
            lookto: Point3::new(0., 0., -1.),
            vup: Vec3::new(0., 1., 0.),
            defocus_angle: 0.,
            focus_dist: 10.,
            image_height: 100,
            pixel_samples_scale: 0.1,
            pixel00_loc: Point3::new(0., 0., 0.),
            center: Point3::new(0., 0., 0.),
            pixel_δ_u: Vec3::new(1., 0., 0.),
            pixel_δ_v: Vec3::new(0., -1., 0.),
            defocus_disk_u: Vec3::new(0., 0., 0.),
            defocus_disk_v: Vec3::new(0., 0., 0.),
        }
    }
}
