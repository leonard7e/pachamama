/*

*/

/*
    The node trait
*/

use crate::core::{
    context::{ptype::PType, Context},
    utils::typedefs::Key,
};

use super::slot::Slot;
use std::collections::BTreeMap;

pub type SlotTypes = BTreeMap<Key, PType>;

pub trait Node {
    fn eval(
        &self,
        context: Context,
        input: Vec<Slot>,
    ) -> Vec<Slot>;

    fn get_input_slot_types(&self) -> &SlotTypes;
    fn get_output_slot_types(&self) -> &SlotTypes;
}
