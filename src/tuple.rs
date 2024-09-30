use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple4(f32, f32, f32, f32);

pub type Vector = Tuple4;
pub type Point = Tuple4;

pub fn point(x: f32, y: f32, z: f32) -> Point {
    Tuple4(x, y, z, 1.0)
}

pub fn vector(x: f32, y: f32, z: f32) -> Vector {
    Tuple4(x, y, z, 0.0)
}

impl Tuple4 {
    pub fn magnitude(&self) -> f32 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2 + self.3 * self.3).sqrt()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn dot(&self, rhs: Self) -> f32 {
        self.0 * rhs.0 + self.1 * rhs.1 + self.2 * rhs.2 + self.3 * rhs.3
    }

    pub fn cross(&self, rhs: Self) -> Self {
        vector(
            self.1 * rhs.2 - self.2 * rhs.1,
            self.2 * rhs.0 - self.0 * rhs.2,
            self.0 * rhs.1 - self.1 * rhs.0,
        )
    }

    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
}

impl Add<Tuple4> for Tuple4 {
    type Output = Tuple4;
    fn add(self, rhs: Tuple4) -> Self::Output {
        Tuple4(
            self.0 + rhs.0,
            self.1 + rhs.1,
            self.2 + rhs.2,
            self.3 + rhs.3,
        )
    }
}

impl Sub<Tuple4> for Tuple4 {
    type Output = Tuple4;
    fn sub(self, rhs: Tuple4) -> Self::Output {
        Tuple4(
            self.0 - rhs.0,
            self.1 - rhs.1,
            self.2 - rhs.2,
            self.3 - rhs.3,
        )
    }
}

impl Neg for Tuple4 {
    type Output = Tuple4;
    fn neg(self) -> Self::Output {
        Tuple4(-self.0, -self.1, -self.2, -self.3)
    }
}

impl Mul<f32> for Tuple4 {
    type Output = Tuple4;
    fn mul(self, rhs: f32) -> Self::Output {
        Tuple4(self.0 * rhs, self.1 * rhs, self.2 * rhs, self.3 * rhs)
    }
}

impl Div<f32> for Tuple4 {
    type Output = Tuple4;
    fn div(self, rhs: f32) -> Self::Output {
        Tuple4(self.0 / rhs, self.1 / rhs, self.2 / rhs, self.3 / rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_is_a_point() {
        let p = Tuple4(4.3, -4.2, 3.1, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vector());
    }

    #[test]
    fn a_vector_is_a_vector() {
        let p = Tuple4(4.3, -4.2, 3.1, 0.0);
        assert!(p.is_vector());
        assert!(!p.is_point());
    }

    #[test]
    fn point_constructor() {
        let p = point(4.3, -4.2, 3.1);
        assert!(p.is_point());
    }

    #[test]
    fn vector_constructor() {
        let v = vector(4.3, -4.2, 3.1);
        assert!(v.is_vector());
    }

    #[test]
    fn adding_tuples() {
        let a1 = Tuple4(3., -2., 5., 1.);
        let a2 = Tuple4(-2., 3., 1., 0.);

        assert_eq!(a1 + a2, Tuple4(1., 1., 6., 1.));
        assert_eq!(a2 + a1, Tuple4(1., 1., 6., 1.));
    }

    #[test]
    fn subtracting_two_points() {
        let p1 = point(3., 2., 1.);
        let p2 = point(5., 6., 7.);

        assert_eq!(p1 - p2, vector(-2., -4., -6.));
        assert_eq!(p2 - p1, vector(2., 4., 6.));
    }

    #[test]
    fn subtracting_a_vector_from_a_point() {
        let p = point(3., 2., 1.);
        let v = vector(5., 6., 7.);

        assert_eq!(p - v, point(-2., -4., -6.));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = vector(3., 2., 1.);
        let v2 = vector(5., 6., 7.);

        assert_eq!(v1 - v2, vector(-2., -4., -6.));
    }

    #[test]
    fn negating_a_tuple() {
        let t = Tuple4(1., -2., 3., -4.);
        assert_eq!(-t, Tuple4(-1., 2., -3., 4.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let t = Tuple4(1., -2., 3., -4.);
        assert_eq!(t * 3.5, Tuple4(3.5, -7., 10.5, -14.));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let t = Tuple4(1., -2., 3., -4.);
        assert_eq!(t * 0.5, Tuple4(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let t = Tuple4(1., -2., 3., -4.);
        assert_eq!(t / 2., Tuple4(0.5, -1., 1.5, -2.));
    }

    #[test]
    fn dot_product() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);

        assert_eq!(a.dot(b), 20.);
        assert_eq!(b.dot(a), 20.);
    }

    #[test]
    fn cross_product() {
        let a = vector(1., 2., 3.);
        let b = vector(2., 3., 4.);

        assert_eq!(a.cross(b), vector(-1., 2., -1.));
        assert_eq!(b.cross(a), vector(1., -2., 1.));
    }
}

#[cfg(test)]
mod magnitude_tests {
    use super::*;

    #[test]
    fn simple_vector_in_x() {
        let v = vector(1., 0., 0.);
        assert_eq!(v.magnitude(), 1.)
    }

    #[test]
    fn simple_vector_in_y() {
        let v = vector(0., 1., 0.);
        assert_eq!(v.magnitude(), 1.)
    }

    #[test]
    fn simple_vector_in_z() {
        let v = vector(0., 0., 1.);
        assert_eq!(v.magnitude(), 1.)
    }

    #[test]
    fn non_axial_vector() {
        let v = vector(1., 2., 3.);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt())
    }

    #[test]
    fn negative_non_axial_vector() {
        let v = vector(-1., -2., -3.);
        assert_eq!(v.magnitude(), 14.0_f32.sqrt())
    }
}

#[cfg(test)]
mod normalize_tests {
    use super::*;

    #[test]
    fn axial_vector() {
        let v = vector(4., 0., 0.);
        assert_eq!(v.normalize(), vector(1., 0., 0.));
    }

    #[test]
    fn non_axial_vector() {
        let v = vector(1., 2., 3.);
        let rt14 = 14.0_f32.sqrt();
        assert_eq!(v.normalize(), vector(1. / rt14, 2. / rt14, 3. / rt14));
    }
}
