/*

*/

use crate::core::context::ptype::PType;

pub struct Slot <T: PType>{
    name: String,
    ty: T
}
