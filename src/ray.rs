use std::ops::Mul;

use crate::{
    matrix::Matrix,
    sphere::Sphere,
    tuple::{Point, Vector},
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

pub struct Intersections(Vec<Intersection>);

impl Intersections {
    pub fn hit(&self) -> Option<Intersection> {
        let mut xs = self.0.clone();
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        let index = xs.iter().position(|&i| i.t >= 0.0);
        if let Some(idx) = index {
            return Some(xs[idx]);
        }
        None
    }
}

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.direction * t
    }

    pub fn intersect(&self, sphere: Sphere) -> Vec<Intersection> {
        let ray2 = *self * sphere.transform.inverse();
        let sphere_to_ray = ray2.origin - sphere.center;

        let a = ray2.direction.dot(ray2.direction);
        let b = 2.0 * ray2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - sphere.radius * sphere.radius;

        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        vec![
            Intersection {
                t: (-b - discriminant.sqrt()) / (2.0 * a),
                object: sphere,
            },
            Intersection {
                t: (-b + discriminant.sqrt()) / (2.0 * a),
                object: sphere,
            },
        ]
    }
}

impl Mul<Matrix> for Ray {
    type Output = Ray;
    fn mul(self, rhs: Matrix) -> Self::Output {
        Ray::new(rhs * self.origin, rhs * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix_transformations::{scaling, translation},
        sphere::Sphere,
        tuple::{point, vector},
    };

    use super::*;

    fn sphere() -> Sphere {
        Sphere::new(point(0., 0., 0.), 1.0)
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let o = point(2., 3., 4.);
        let d = vector(1., 0., 0.);

        let ray = Ray::new(o, d);

        assert_eq!(ray.at(0.), point(2., 3., 4.));
        assert_eq!(ray.at(1.), point(3., 3., 4.));
        assert_eq!(ray.at(-1.), point(1., 3., 4.));
        assert_eq!(ray.at(2.5), point(4.5, 3., 4.));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let ray = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let ray = Ray::new(point(0., 1., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = sphere();
        let i1 = Intersection { t: 1., object: s };
        let i2 = Intersection { t: 2., object: s };

        let xs = Intersections(vec![i2, i1]);
        assert_eq!(xs.hit(), Some(i1));
    }

    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = sphere();
        let i1 = Intersection { t: -1., object: s };
        let i2 = Intersection { t: 1., object: s };

        let xs = Intersections(vec![i2, i1]);
        assert_eq!(xs.hit(), Some(i2));
    }

    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = sphere();
        let i1 = Intersection { t: -2., object: s };
        let i2 = Intersection { t: -1., object: s };

        let xs = Intersections(vec![i2, i1]);
        assert_eq!(xs.hit(), None);
    }

    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = sphere();
        let i1 = Intersection { t: 5., object: s };
        let i2 = Intersection { t: 7., object: s };
        let i3 = Intersection { t: -3., object: s };
        let i4 = Intersection { t: 2., object: s };

        let xs = Intersections(vec![i1, i2, i3, i4]);
        assert_eq!(xs.hit(), Some(i4));
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.set_transform(scaling(2., 2., 2.));

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.);
        assert_eq!(xs[1].t, 7.);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_rlay() {
        let r = Ray::new(point(0., 0., -5.), vector(0., 0., 1.));
        let mut s = sphere();
        s.set_transform(translation(5., 0., 0.));

        let xs = r.intersect(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn translating_a_ray() {
        let ray = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = translation(3., 4., 5.);
        let r2 = ray * m;
        assert_eq!(r2.origin, point(4., 6., 8.));
        assert_eq!(r2.direction, vector(0., 1., 0.));
    }

    #[test]
    fn scaling_a_ray() {
        let ray = Ray::new(point(1., 2., 3.), vector(0., 1., 0.));
        let m = scaling(2., 3., 4.);
        let r2 = ray * m;
        assert_eq!(r2.origin, point(2., 6., 12.));
        assert_eq!(r2.direction, vector(0., 3., 0.));
    }
}
