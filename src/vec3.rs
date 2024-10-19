use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Vec3(pub [f32; 3]);
pub type Point3 = Vec3;

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self([x, y, z])
    }

    pub fn length_squared(&self) -> f32 {
        self[0] * self[0] + self[1] * self[1] + self[2] * self[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn dot(&self, rhs: &Self) -> f32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }

    pub fn cross(&self, rhs: &Self) -> Self {
        Vec3([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }

    pub fn unit_vector(&self) -> Self {
        *self / self.length()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3([-self[0], -self[1], -self[2]])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2]])
    }
}

impl AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self[0] += rhs[0];
        self[1] += rhs[1];
        self[2] += rhs[2];
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2]])
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3([self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2]])
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Self::Output {
        Vec3([self[0] * rhs, self[1] * rhs, self[2] * rhs])
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self[0] *= rhs;
        self[1] *= rhs;
        self[2] *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Self::Output {
        self * rhs.recip()
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= rhs.recip();
    }
}
