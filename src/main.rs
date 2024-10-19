use std::fs;

use indicatif::ProgressIterator;
use itertools::Itertools;
use trace_rs::color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let pixels = (0..image_height)
        .cartesian_product(0..image_width)
        .progress_count(image_width as u64 * image_height as u64)
        .map(|(y, x)| {
            let color = Color::new(
                0.0,
                y as f32 / (image_height - 1) as f32,
                x as f32 / (image_width - 1) as f32,
            );
            color.to_ppm()
        })
        .join("\n");

    fs::write(
        "image.ppm",
        format!("P3\n{image_width} {image_height}\n255\n{pixels}\n"),
    )
    .expect("image.ppm written");
}
