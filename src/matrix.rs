use crate::tuple::Tuple4;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix([f32; 16]);
pub struct Matrix3x3([f32; 9]);
pub struct Matrix2x2([f32; 4]);

impl Matrix {
    pub fn new(contents: [f32; 16]) -> Self {
        Matrix(contents)
    }

    pub fn identity() -> Self {
        Self([
            1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
        ])
    }

    pub fn at(&self, row: usize, column: usize) -> f32 {
        self.0[row * 4 + column]
    }

    pub fn write(&mut self, row: usize, column: usize, value: f32) {
        self.0[row * 4 + column] = value;
    }
}

impl Mul<Matrix> for Matrix {
    type Output = Matrix;
    fn mul(self, rhs: Matrix) -> Self::Output {
        let mut m = Matrix([0.; 16]);
        for row in 0..4 {
            for col in 0..4 {
                let mut val = 0.;
                for i in 0..4 {
                    val += self.at(row, i) * rhs.at(i, col)
                }
                m.write(row, col, val);
            }
        }
        m
    }
}

impl Mul<Tuple4> for Matrix {
    type Output = Tuple4;
    fn mul(self, rhs: Tuple4) -> Self::Output {
        let mut t = [0.; 4];
        for row in 0..4 {
            t[row] = self.at(row, 0) * rhs.0
                + self.at(row, 1) * rhs.1
                + self.at(row, 2) * rhs.2
                + self.at(row, 3) * rhs.3;
        }
        Tuple4(t[0], t[1], t[2], t[3])
    }
}

impl Matrix2x2 {
    pub fn at(&self, row: usize, column: usize) -> f32 {
        self.0[row * 2 + column]
    }
}

impl Matrix3x3 {
    pub fn at(&self, row: usize, column: usize) -> f32 {
        self.0[row * 3 + column]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test::approx::Approx,
        tuple::{point, Tuple4},
    };

    impl Approx for Matrix {
        fn approximate(&self, rhs: Self) -> bool {
            let delta = 0.00001;
            for i in 0..16 {
                if (self.0[i] - rhs.0[i]).abs() >= delta {
                    return false;
                }
            }
            true
        }
    }

    #[test]
    fn test_matrix_access() {
        let m = Matrix::new([
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(m.at(0, 0), 1.);
        assert_eq!(m.at(0, 3), 4.);
        assert_eq!(m.at(1, 0), 5.5);
        assert_eq!(m.at(1, 2), 7.5);
        assert_eq!(m.at(2, 2), 11.);
        assert_eq!(m.at(3, 0), 13.5);
        assert_eq!(m.at(3, 2), 15.5);
    }

    #[test]
    fn test_matrix2x2_access() {
        let m = Matrix2x2([-3., 5., 1., -2.]);
        assert_eq!(m.at(0, 0), -3.);
        assert_eq!(m.at(0, 1), 5.);
        assert_eq!(m.at(1, 0), 1.);
        assert_eq!(m.at(1, 1), -2.);
    }

    #[test]
    fn test_matrix3x3_access() {
        let m = Matrix3x3([-3., 5., 0., 1., -2., -7., 0., 1., 1.]);
        assert_eq!(m.at(0, 0), -3.);
        assert_eq!(m.at(1, 1), -2.);
        assert_eq!(m.at(2, 2), 1.);
    }

    #[test]
    fn equality() {
        let a = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);

        assert!(a.approximate(b));
    }

    #[test]
    fn inequality() {
        let a = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix([
            1., 2., 3., 4.5, 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);

        assert!(!a.approximate(b));
    }

    #[test]
    fn multiplying_matrices() {
        let a = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix([
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        ]);

        let expected = Matrix([
            20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
        ]);

        let actual = a * b;
        assert!(actual.approximate(expected));
    }

    #[test]
    fn multiplying_a_matrix_by_a_tuple() {
        let a = Matrix([
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ]);
        let b = point(1., 2., 3.);

        assert_eq!(a * b, point(18., 24., 33.));
    }

    #[test]
    fn multiplying_by_the_identity_matrix() {
        let a = Matrix([
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ]);
        assert_eq!(a * Matrix::identity(), a);
    }
}
