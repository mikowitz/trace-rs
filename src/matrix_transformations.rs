use crate::matrix::Matrix;

pub fn translation(x: f32, y: f32, z: f32) -> Matrix {
    set_transform(vec![(3, x), (7, y), (11, z)])
}

pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
    set_transform(vec![(0, x), (5, y), (10, z)])
}

pub fn rotation_x(radians: f32) -> Matrix {
    set_transform(vec![
        (5, radians.cos()),
        (6, -radians.sin()),
        (9, radians.sin()),
        (10, radians.cos()),
    ])
}

pub fn rotation_y(radians: f32) -> Matrix {
    set_transform(vec![
        (0, radians.cos()),
        (2, radians.sin()),
        (8, -radians.sin()),
        (10, radians.cos()),
    ])
}

pub fn rotation_z(radians: f32) -> Matrix {
    set_transform(vec![
        (0, radians.cos()),
        (1, -radians.sin()),
        (4, radians.sin()),
        (5, radians.cos()),
    ])
}

pub fn shearing(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Matrix {
    set_transform(vec![(1, xy), (2, xz), (4, yx), (6, yz), (8, zx), (9, zy)])
}

fn set_transform(transforms: Vec<(usize, f32)>) -> Matrix {
    let mut i = Matrix::identity();
    for (idx, v) in transforms.iter() {
        i.0[*idx] = *v;
    }
    i
}

#[cfg(test)]
mod translation_tests {
    use crate::tuple::{point, vector, Tuple4};

    use super::*;
    use crate::test::approx::Approx;

    impl Approx for Tuple4 {
        fn approximate(&self, rhs: Self) -> bool {
            let delta = 0.00001;
            (self.0 - rhs.0).abs() < delta
                && (self.1 - rhs.1).abs() < delta
                && (self.2 - rhs.2).abs() < delta
        }
    }

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

#[cfg(test)]
mod scaling_tests {
    use crate::tuple::{point, vector};

    use super::*;

    #[test]
    fn test_scaling() {
        let transform = scaling(2., 3., 4.);
        let p = point(-4., 6., 8.);

        assert_eq!(transform * p, point(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_a_vector() {
        let transform = scaling(2., 3., 4.);
        let v = vector(-4., 6., 8.);

        assert_eq!(transform * v, vector(-8., 18., 32.));
    }

    #[test]
    fn test_scaling_by_the_inverse() {
        let transform = scaling(2., 3., 4.).inverse();
        let v = vector(-4., 6., 8.);

        assert_eq!(transform * v, vector(-2., 2., 2.));
    }

    #[test]
    fn test_reflection_is_scaling_by_a_negative_value() {
        let transform = scaling(-1., 1., 1.);
        let p = point(2., 3., 4.);

        assert_eq!(transform * p, point(-2., 3., 4.));
    }
}

#[cfg(test)]
mod rotation_tests {
    use std::f32::consts::PI;

    use crate::{test::approx::Approx, tuple::point};

    use super::*;

    #[test]
    fn test_rotating_around_the_x_axis() {
        let p = point(0., 1., 0.);
        let half_q = rotation_x(PI / 4.);
        let full_q = rotation_x(PI / 2.);

        assert!((half_q * p).approximate(point(0., 2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2.)));
        assert!((full_q * p).approximate(point(0., 0., 1.)));
    }

    #[test]
    fn inverse_of_x_rotation_rotates_in_the_opposite_direction() {
        let p = point(0., 1., 0.);
        let half_q = rotation_x(PI / 4.).inverse();

        assert!((half_q * p).approximate(point(0., 2.0_f32.sqrt() / 2., -2.0_f32.sqrt() / 2.)));
    }

    #[test]
    fn test_rotating_around_the_y_axis() {
        let p = point(0., 0., 1.);
        let half_q = rotation_y(PI / 4.);
        let full_q = rotation_y(PI / 2.);

        assert!((half_q * p).approximate(point(2.0_f32.sqrt() / 2., 0., 2.0_f32.sqrt() / 2.)));
        assert!((full_q * p).approximate(point(1., 0., 0.)));
    }

    #[test]
    fn test_rotating_around_the_z_axis() {
        let p = point(0., 1., 0.);
        let half_q = rotation_z(PI / 4.);
        let full_q = rotation_z(PI / 2.);

        assert!((half_q * p).approximate(point(-2.0_f32.sqrt() / 2., 2.0_f32.sqrt() / 2., 0.)));
        assert!((full_q * p).approximate(point(-1., 0., 0.)));
    }
}

#[cfg(test)]
mod shearing_tests {
    use crate::tuple::point;

    use super::*;

    #[test]
    fn move_x_in_proportion_to_y() {
        let transform = shearing(1., 0., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(5., 3., 4.));
    }

    #[test]
    fn move_x_in_proportion_to_z() {
        let transform = shearing(0., 1., 0., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(6., 3., 4.));
    }

    #[test]
    fn move_y_in_proportion_to_x() {
        let transform = shearing(0., 0., 1., 0., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 5., 4.));
    }

    #[test]
    fn move_y_in_proportion_to_z() {
        let transform = shearing(0., 0., 0., 1., 0., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 7., 4.));
    }

    #[test]
    fn move_z_in_proportion_to_x() {
        let transform = shearing(0., 0., 0., 0., 1., 0.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 3., 6.));
    }

    #[test]
    fn move_z_in_proportion_to_y() {
        let transform = shearing(0., 0., 0., 0., 0., 1.);
        let p = point(2., 3., 4.);
        assert_eq!(transform * p, point(2., 3., 7.));
    }
}
