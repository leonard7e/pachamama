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

#[derive(PartialEq)]
pub enum PValue {
    PV_String(String),
    PV_Integer(i32),
    PV_Scalar(Scalar),
    PV_Vector2([Scalar; 2]),
    PV_Vector3([Scalar; 3]),
    PV_Vector4([Scalar; 4]),
    PV_Matrix3x3([[Scalar;3];3]),
    PV_Matrix4x4([[Scalar;4];4]),
}

#[derive(PartialEq, Eq)]
pub enum PType {
    PT_String,
    PT_Integer,
    PT_Scalar,
    PT_Vector2,
    PT_Vector3,
    PT_Vector4,
    PT_Matrix3x3,
    PT_Matrix4x4,
}

impl PValue {
    fn get_type(&self) -> PType {
        match self {
            PValue::PV_String(_) => PType::PT_String,
            PValue::PV_Integer(_) => PType::PT_Integer,
            PValue::PV_Scalar(_) => PType::PT_Scalar,
            PValue::PV_Vector2(_) => PType::PT_Vector2,
            PValue::PV_Vector3(_) => PType::PT_Vector3,
            PValue::PV_Vector4(_) => PType::PT_Vector4,
            PValue::PV_Matrix3x3(_) => PType::PT_Matrix3x3,
            PValue::PV_Matrix4x4(_) => PType::PT_Matrix4x4,
        }
    }
}

impl PType {
    fn type_match(&self, t:&PType) -> bool {
        self == t
    }

    fn type_name(&self) -> &str {
        match self {
            PType::PT_String => "String",
            PType::PT_Integer => "Integer",
            PType::PT_Scalar => "Scalar",
            PType::PT_Vector2 => "Vec2",
            PType::PT_Vector3 => "Vec3",
            PType::PT_Vector4 => "Vec4",
            PType::PT_Matrix3x3 => "Matrix3x3",
            PType::PT_Matrix4x4 => "Matrix4x4",
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn does_dyn_work_for_PType() {
        let v : Vec<Box<dyn PType>> = Vec::new();
        (&mut v).push(Box::new()(5i32 as dyn PType));
    }
}
*/
