use crate::{matrix::Matrix, tuple::Point};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub transform: Matrix,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self {
            center,
            radius,
            transform: Matrix::identity(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix_transformations::translation, tuple::point};

    fn sphere() -> Sphere {
        Sphere::new(point(0., 0., 0.), 1.0)
    }

    #[test]
    fn default_transformation() {
        let s = sphere();

        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn changing_transformation() {
        let mut s = sphere();
        let t = translation(2., 3., 4.);

        s.set_transform(t);

        assert_eq!(s.transform, t);
    }
}
