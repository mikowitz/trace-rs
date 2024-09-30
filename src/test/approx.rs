#[allow(dead_code)]
pub trait Approx {
    fn approximate(&self, rhs: Self) -> bool;
}
