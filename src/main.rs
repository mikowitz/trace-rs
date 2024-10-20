#![allow(mixed_script_confusables)]
use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;
use trace_rs::{
    color::Color,
    ray::Ray,
    vec3::{Point3, Vec3},
};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    if image_height < 1 {
        image_height = 1;
    }

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);

    let camera_center = Point3::new(0., 0., 0.);

    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    let pixel_δ_u = viewport_u / image_width as f32;
    let pixel_δ_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + (pixel_δ_u + pixel_δ_v) * 0.5;

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .progress_count(image_width as u64 * image_height as u64)
        .map(|(y, x)| {
            let pixel_center = pixel00_loc + (pixel_δ_u * x as f32) + (pixel_δ_v * y as f32);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(&ray);
            color.to_ppm()
        })
        .join("\n");

    fs::write(
        "image.ppm",
        format!("P3\n{image_width} {image_height}\n255\n{pixels}\n"),
    )
    .expect("image.ppm written");
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(Point3::new(0., 0., 1.), 0.5, ray) {
        return Color::new(0.4, 0., 1.0);
    }
    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction[1] + 1.0);
    Color::white() * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> bool {
    let oc = center - ray.origin;
    let a = ray.direction.dot(&ray.direction);
    let b = -2.0 * ray.direction.dot(&oc);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = b * b - 4. * a * c;

    discriminant >= 0.
}
