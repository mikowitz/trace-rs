use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Color(f32, f32, f32);

impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self(r, g, b)
    }

    pub fn black() -> Self {
        Self(0., 0., 0.)
    }

    pub fn to_ppm(&self) -> String {
        let r = clamp(self.0 * 255.999).ceil() as u32;
        let g = clamp(self.1 * 255.999).ceil() as u32;
        let b = clamp(self.2 * 255.999).ceil() as u32;

        format!("{r} {g} {b}")
    }
}

fn clamp(n: f32) -> f32 {
    if n > 255. {
        return 255.;
    }
    if n < 0. {
        return 0.;
    }
    n
}

impl Add<Color> for Color {
    type Output = Color;
    fn add(self, rhs: Color) -> Self::Output {
        Color(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl Mul<f32> for Color {
    type Output = Color;
    fn mul(self, rhs: f32) -> Self::Output {
        Color(self.0 * rhs, self.1 * rhs, self.2 * rhs)
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::approx::Approx;

    impl Approx for Color {
        fn approximate(&self, rhs: Self) -> bool {
            let delta = 0.00001;
            (self.0 - rhs.0).abs() < delta
                && (self.1 - rhs.1).abs() < delta
                && (self.2 - rhs.2).abs() < delta
        }
    }

    #[test]
    fn adding_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert!((c1 + c2).approximate(Color(1.6, 0.7, 1.0)));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Color(0.9, 0.6, 0.75);
        let c2 = Color(0.7, 0.1, 0.25);
        assert!((c1 - c2).approximate(Color(0.2, 0.5, 0.5)));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Color(0.2, 0.3, 0.4);
        assert_eq!(c * 2., Color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Color(1., 0.2, 0.4);
        let c2 = Color(0.9, 1., 0.1);

        assert!((c1 * c2).approximate(Color(0.9, 0.2, 0.04)));
    }

    #[test]
    fn to_ppm() {
        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        assert_eq!(c1.to_ppm(), "255 0 0");
        assert_eq!(c2.to_ppm(), "0 128 0");
        assert_eq!(c3.to_ppm(), "0 0 255");
    }
}
