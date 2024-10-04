use crate::tuple::Point;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}
