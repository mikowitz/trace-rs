use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    let mut i = Matrix::identity();
    i.0[3] = x;
    i.0[7] = y;
    i.0[11] = z;
    i
}

#[cfg(test)]
mod tests {
    use crate::tuple::{point, vector};

    use super::*;

    #[test]
    fn test_translation() {
        let transform = translation(5., -3., 2.);
        let p = point(-3., 4., 5.);

        assert_eq!(transform * p, point(2., 1., 7.));
    }

    #[test]
    fn test_translation_inverse() {
        let transform = translation(5., -3., 2.).inverse();
        let p = point(-3., 4., 5.);

        assert_eq!(transform * p, point(-8., 7., 3.));
    }

    #[test]
    fn test_translation_doesnt_affect_vectors() {
        let transform = translation(5., -3., 2.).inverse();
        let v = vector(-3., 4., 5.);

        assert_eq!(transform * v, v);
    }
}
