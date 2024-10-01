pub struct Matrix([f32; 16]);
pub struct Matrix3x3([f32; 9]);
pub struct Matrix2x2([f32; 4]);

impl Matrix {
    pub fn new(contents: [f32; 16]) -> Self {
        Matrix(contents)
    }

    pub fn at(&self, row: usize, column: usize) -> f32 {
        self.0[row * 4 + column]
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
    use crate::test::approx::Approx;

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
    fn matrix_equality() {
        let a = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);

        assert!(a.approximate(b));
    }

    #[test]
    fn matrix_inequality() {
        let a = Matrix([
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);
        let b = Matrix([
            1., 2., 3., 4.5, 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ]);

        assert!(!a.approximate(b));
    }
}
