
pub trait VectorSpace {
    fn add(&self, v: Self) -> Self;
    fn inverse(self) -> Self;
    fn scale(&self, s: f32) -> Self;
}

/*
use std::ops::Add;
impl <V: VectorSpace> Add<V> for V where V: VectorSpace {
    type Output = Self;
    fn add (self, other:Self) -> Self {
        self.add(other)
    }
}
*/

// General tests for Vectors needed
