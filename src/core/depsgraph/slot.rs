/*

*/

use crate::core::{utils::typedefs::Key, context::ptype::PType};
use std::collections::BTreeMap;
use std::collections::btree_map;

pub struct Slot <T: PType>{
    name: String,
    ty: T
}



pub type SlotMap <T: PType> = BTreeMap<Key, Slot<T>>;
pub type SlotIter <'a, T: PType > = btree_map::Iter<'a, Key,Slot<T>>;
