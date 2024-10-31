use std::fs;

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

use glam::Vec3;
use trace_rs::{color, ray::Ray};

fn main() {
    let aspect_ratio = 16. / 9.;
    let image_width = 400;

    let image_height = ((image_width as f32 / aspect_ratio) as i32).max(1);

    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Vec3::ZERO;

    let viewport_u = Vec3::X * viewport_width;
    let viewport_v = Vec3::Y * -viewport_height;

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left =
        camera_center - Vec3::Z * focal_length - viewport_u / 2. - viewport_v / 2.;

    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<(i32, i32)>>()
        .into_par_iter()
        .progress_count(image_width as u64 * image_height as u64)
        .map(|(y, x)| {
            let pixel_center =
                pixel00_loc + (x as f32 * pixel_delta_u) + (y as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray {
                origin: camera_center,
                direction: ray_direction,
            };
            let pixel_color = ray.color();
            color::to_ppm(pixel_color)
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(
        "image.ppm",
        format!("P3\n{image_width} {image_height}\n255\n{pixels}\n"),
    )
    .unwrap();
}
