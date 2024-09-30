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
    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }
}

#[cfg(test)]
mod test {
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
}
