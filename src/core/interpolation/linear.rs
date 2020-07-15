use crate::core::linear_algebra::vectorspace::VectorSpace;


pub fn blend<T:VectorSpace>(a :T, b:T, x: f32) -> T {
    a.scale(x).add(b.scale(1.0-x))
}
