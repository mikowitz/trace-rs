use canvas::Canvas;
use color::Color;
use trace_rs::*;

fn main() {
    let mut canvas = Canvas::new(100, 100);

    for x in 0..100 {
        for y in 0..100 {
            let r = 0.0;
            let g = y as f32 / canvas.height as f32;
            let b = x as f32 / canvas.width as f32;

            canvas.write(x, y, Color::new(r, g, b));
        }
    }

    canvas.save("image.ppm");
}
