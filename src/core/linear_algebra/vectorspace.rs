/*
    A Vector Space is defined by two operators.
    Vector Addition
    and scalar multiplication.

    We also need a neutral element for addition (zero vector).
    A neutral element of multiplication is already defined with 1.0
    or all vector fields. So, we do not need to define it here.
*/

pub trait VectorSpace {
    fn add(&self, v: Self) -> Self;
    fn scale(&self, s: f32) -> Self;
    fn zero() -> Self; // Neutral element for addition
}

// We could define an invert vector
pub fn invert<V>(v: V) -> V
where
    V: VectorSpace,
{
    v.scale(-1.0)
}

/*
IDea:
Introduce
derive 'Vector'
to speed up creating vector types.

use std::ops::Add;
impl <V: VectorSpace> Add<V> for V where V: VectorSpace {
    type Output = Self;
    fn add (self, other:Self) -> Self {
        self.add(other)
    }
}
*/

// General tests for Vectors needed
