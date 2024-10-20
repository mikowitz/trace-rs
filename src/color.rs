use crate::vec3::Vec3;

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

        let ir = (255.999 * r) as i32;
        let ig = (255.999 * g) as i32;
        let ib = (255.999 * b) as i32;

        format!("{ir} {ig} {ib}")
    }
}
