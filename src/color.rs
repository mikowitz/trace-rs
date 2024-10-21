use crate::vec3::Vec3;
use std::ops::Range;

pub type Color = Vec3;

impl Color {
    pub fn black() -> Self {
        Color::new(0., 0., 0.)
    }

    pub fn white() -> Self {
        Color::new(1., 1., 1.)
    }

    pub fn to_ppm(&self) -> String {
        let r = self[0];
        let g = self[1];
        let b = self[2];

        let intensity = 0.000..0.999;
        let ir = (256. * clamp(&intensity, r)) as i32;
        let ig = (256. * clamp(&intensity, g)) as i32;
        let ib = (256. * clamp(&intensity, b)) as i32;

        format!("{ir} {ig} {ib}")
    }
}

fn clamp(interval: &Range<f32>, n: f32) -> f32 {
    if n < interval.start {
        return interval.start;
    }
    if n > interval.end {
        return interval.end;
    }
    n
}
