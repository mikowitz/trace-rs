use std::fs;

use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::prelude::*;

use glam::Vec3;
use trace_rs::color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .collect::<Vec<(i32, i32)>>()
        .into_par_iter()
        .progress_count(image_width as u64 * image_height as u64)
        .map(|(y, x)| {
            let c = Vec3::new(
                0.0,
                y as f32 / (image_height - 1) as f32,
                x as f32 / (image_width - 1) as f32,
            );

            color::to_ppm(c)
        })
        .collect::<Vec<String>>()
        .join("\n");

    fs::write(
        "image.ppm",
        format!("P3\n{image_width} {image_height}\n255\n{pixels}\n"),
    )
    .unwrap();
}
