use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::black(); width * height],
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Color {
        self.pixels[y * self.width + x]
    }

    pub fn write(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[y * self.width + x] = color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);

        for x in 0..10 {
            for y in 0..20 {
                assert_eq!(c.at(x, y), Color::black());
            }
        }
    }

    #[test]
    fn writing_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        c.write(2, 3, red);

        assert_eq!(c.at(2, 3), red);
    }
}
