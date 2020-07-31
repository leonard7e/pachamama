/*
    Slot types. Sketching ideas.
*/

/*
    This is how data is represented in pachamama. PType is a shortcut for Pachamama Type.
*/

/*
    Type descriptor.
*/

use crate::core::utils::typedefs::Scalar;

pub enum PType {
    String(String),
    Integer(i32),
    Float(Scalar),
    Vector2([Scalar; 2]),
    Vector3([Scalar; 3]),
    Matrix3x3([[Scalar;3];3]),
    Matrix4x4([[Scalar;4];4]),
    //
    // Closure([PT_Descr], [PT_Descr]),
    // Structure([PT_Descr]),
}
