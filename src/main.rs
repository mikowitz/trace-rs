use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .progress_count(image_width as u64 * image_height as u64)
        .map(|(y, x)| {
            let r = 0.0;
            let g = y as f32 / (image_height - 1) as f32;
            let b = x as f32 / (image_width - 1) as f32;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            format!("{ir} {ig} {ib}")
        })
        .join("\n");

    fs::write(
        "image.ppm",
        format!("P3\n{image_width} {image_height}\n255\n{pixels}\n"),
    )
    .expect("image.ppm written");
}
