use std::vec::Vec;

enum FaceType {
    Triangle(usize, usize, usize),
    Quad(usize, usize, usize, usize),
}

struct StaticMesh <V>{
    vertices: Vec<V>,
    faces: Vec<FaceType>,
}

trait ToStaticMesh<V> {
    fn to_static_mesh(&self) -> StaticMesh<V>;
