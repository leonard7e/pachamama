use crate::core::linear_algebra::vectorspace::VectorSpace;

pub fn blend<T:VectorSpace>(a :T, b:T, x: f32) -> T {
    //Since we do have VectorSpace trait, we only need to define this function once.
    a.scale(x).add(b.scale(1.0-x))
}
