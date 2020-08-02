/*

*/

use crate::core::{context::{ptype::PType, Context}, utils::typedefs::Key};

use super::slot::{SlotIter, Slot};

pub trait Node <T: PType>{
    fn has_input_slot(&self, s: &Key) -> bool;
    fn has_output_slot(&self, s: &Key) -> bool;
    fn iter_output_slots(&self) -> Box<SlotIter<T>>;
    fn iter_input_slots(&self) -> Box<SlotIter<T>>;
    fn eval(&self, context: Context<T>, input: Vec<Slot<T>>) -> Vec<Slot<T>>;
}
