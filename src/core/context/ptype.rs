/*
    Slot types. Sketching ideas.
*/

/*
    This is how data is represented in pachamama. PType is a shortcut for Pachamama Type.
*/

/*
    Type descriptor.
*/

pub enum PType {
    String(String),
    Integer(i32),
    Float(f32),
    Vector2([f32; 2]),
    Vector3([f32; 3]),
    Matrix3x3([[f32;3];3]),
    Matrix4x4([[f32;4];4]),
    //
    // Closure([PT_Descr], [PT_Descr]),
    // Structure([PT_Descr]),
}
