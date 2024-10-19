use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn to_ppm(&self) -> String {
        let Vec3([r, g, b]) = self;

        let ir = (255.999 * r) as i32;
        let ig = (255.999 * g) as i32;
        let ib = (255.999 * b) as i32;

        format!("{ir} {ig} {ib}")
    }
}
