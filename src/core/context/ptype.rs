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

pub enum PT_Descr {
    PT_String,
    PT_Integer,
    PT_Scalar,
    PT_Vector2,
    PT_Vector3,
    PT_Vector4,
    PT_Matrix3x3,
    PT_Matrix4x4,
    // P_Node
}

pub trait PType {
    fn pt_id() -> PT_Descr;
}


impl PType for i32 {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Integer }
}

impl PType for Scalar {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Scalar }
}

impl PType for [Scalar; 2] {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Vector2 }
}

impl PType for [Scalar; 3] {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Vector3 }
}

impl PType for [Scalar; 4] {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Vector4 }
}

impl PType for [[Scalar;3];3] {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Matrix3x3 }
}

impl PType for [[Scalar;4];4] {
    fn pt_id() -> PT_Descr { PT_Descr::PT_Matrix4x4 }
}
