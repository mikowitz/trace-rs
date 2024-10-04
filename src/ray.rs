use crate::{
    sphere::Sphere,
    tuple::{Point, Vector},
};

#[derive(Debug, PartialEq)]
pub struct Intersection {
    pub t: f32,
    pub object: Sphere,
}

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

    pub fn intersects(&self, sphere: Sphere) -> Vec<Intersection> {
        let sphere_to_ray = self.origin - sphere.center;

        let a = self.direction.dot(self.direction);
        let b = 2.0 * self.direction.dot(sphere_to_ray);
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

#[cfg(test)]
mod tests {
    use crate::{
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
        let xs = ray.intersects(s);

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
        let xs = ray.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let ray = Ray::new(point(0., 2., -5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersects(s);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let ray = Ray::new(point(0., 0., 0.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let ray = Ray::new(point(0., 0., 5.), vector(0., 0., 1.));
        let s = sphere();
        let xs = ray.intersects(s);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].object, s);
    }
}
