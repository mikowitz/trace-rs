use crate::tuple::Tuple4;
use std::ops::Mul;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix(pub [f32; 16]);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3x3([f32; 9]);
#[derive(Clone, Copy, Debug, PartialEq)]
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

    pub fn transpose(&self) -> Self {
        let mut m = Self([0.; 16]);

        for row in 0..4 {
            for col in 0..4 {
                m.write(col, row, self.at(row, col));
            }
        }
        m
    }

    pub fn inverse(&self) -> Self {
        if !self.is_invertible() {
            panic!("not invertible");
        }

        let mut m = Self([0.; 16]);
        for row in 0..4 {
            for col in 0..4 {
                m.write(col, row, self.cofactor(row, col) / self.determinant());
            }
        }
        m
    }

    pub fn submatrix(&self, xrow: usize, xcol: usize) -> Matrix3x3 {
        let mut m = Matrix3x3([0.; 9]);
        let mut mi: usize = 0;
        for row in 0..4 {
            for col in 0..4 {
                if row != xrow && col != xcol {
                    m.0[mi] = self.at(row, col);
                    mi += 1;
                }
            }
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            return m;
        }
        -m
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.;
        for col in 0..4 {
            det += self.at(0, col) * self.cofactor(0, col);
        }
        det
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.
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
        for (row, v) in t.iter_mut().enumerate() {
            *v = self.at(row, 0) * rhs.0
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

    pub fn determinant(&self) -> f32 {
        let m = self.0;
        m[0] * m[3] - m[1] * m[2]
    }
}

impl Matrix3x3 {
    pub fn at(&self, row: usize, column: usize) -> f32 {
        self.0[row * 3 + column]
    }

    pub fn submatrix(&self, xrow: usize, xcol: usize) -> Matrix2x2 {
        let mut m = Matrix2x2([0.; 4]);
        let mut mi: usize = 0;
        for row in 0..3 {
            for col in 0..3 {
                if row != xrow && col != xcol {
                    m.0[mi] = self.at(row, col);
                    mi += 1;
                }
            }
        }
        m
    }

    pub fn minor(&self, row: usize, col: usize) -> f32 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f32 {
        let m = self.minor(row, col);
        if (row + col) % 2 == 0 {
            return m;
        }
        -m
    }

    pub fn determinant(&self) -> f32 {
        let mut det = 0.;
        for col in 0..3 {
            det += self.at(0, col) * self.cofactor(0, col);
        }
        det
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test::approx::Approx, tuple::point};

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

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix([
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        ]);

        let b = Matrix([
            0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
        ]);

        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert_eq!(Matrix::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn determinant_2x2() {
        let a = Matrix2x2([1., 5., -3., 2.]);
        assert_eq!(a.determinant(), 17.);
    }

    #[test]
    fn determinant_3x3() {
        let a = Matrix3x3([1., 2., 6., -5., 8., -4., 2., 6., 4.]);
        assert_eq!(a.cofactor(0, 0), 56.);
        assert_eq!(a.cofactor(0, 1), 12.);
        assert_eq!(a.cofactor(0, 2), -46.);
        assert_eq!(a.determinant(), -196.);
    }

    #[test]
    fn determinant_4x4() {
        let a = Matrix([
            -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
        ]);
        assert_eq!(a.cofactor(0, 0), 690.);
        assert_eq!(a.cofactor(0, 1), 447.);
        assert_eq!(a.cofactor(0, 2), 210.);
        assert_eq!(a.cofactor(0, 3), 51.);
        assert_eq!(a.determinant(), -4071.);
    }

    #[test]
    fn submatrix_of_3x3_is_2x2() {
        let a = Matrix3x3([1., 5., 0., -3., 2., 7., 0., 6., -3.]);
        assert_eq!(a.submatrix(0, 2), Matrix2x2([-3., 2., 0., 6.]));
    }

    #[test]
    fn submatrix_of_4x4_is_3x3() {
        let a = Matrix([
            -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
        ]);
        assert_eq!(
            a.submatrix(2, 1),
            Matrix3x3([-6., 1., 6., -8., 8., 6., -7., -1., 1.,])
        );
    }

    #[test]
    fn minor_of_3x3() {
        let a = Matrix3x3([3., 5., 0., 2., -1., -7., 6., -1., 5.]);
        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.);
        assert_eq!(a.minor(1, 0), 25.);
    }

    #[test]
    fn cofactor_of_3x3() {
        let a = Matrix3x3([3., 5., 0., 2., -1., -7., 6., -1., 5.]);
        assert_eq!(a.minor(0, 0), -12.);
        assert_eq!(a.cofactor(0, 0), -12.);

        assert_eq!(a.minor(1, 0), 25.);
        assert_eq!(a.cofactor(1, 0), -25.);
    }

    #[test]
    fn is_invertible() {
        let a = Matrix([
            6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
        ]);
        let b = Matrix([
            -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
        ]);

        assert_eq!(a.determinant(), -2120.);
        assert!(a.is_invertible());

        assert_eq!(b.determinant(), 0.);
        assert!(!b.is_invertible());
    }

    #[test]
    fn test_inverse() {
        let a = Matrix([
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        ]);
        let b = a.inverse();

        assert_eq!(a.determinant(), 532.);

        assert_eq!(a.cofactor(2, 3), -160.);
        assert_eq!(b.at(3, 2), -160. / 532.);

        assert_eq!(a.cofactor(3, 2), 105.);
        assert_eq!(b.at(2, 3), 105. / 532.);

        let expected = Matrix([
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        ]);

        assert!(b.approximate(expected));
    }

    #[test]
    fn test_another_inverse() {
        let a = Matrix([
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        ]);
        let expected = Matrix([
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ]);

        assert!(a.inverse().approximate(expected));
    }

    #[test]
    fn test_yet_another_inverse() {
        let a = Matrix([
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        ]);
        let expected = Matrix([
            -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
            -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
        ]);

        assert!(a.inverse().approximate(expected));
    }
}
