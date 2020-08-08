/*

*/

/*
    The node trait
*/

use crate::core::{
    context::Context,
    utils::typedefs::Key,
};

use super::slot::Slot;
use std::collections::BTreeMap;

pub type SlotMap = BTreeMap<Key, Slot>;

pub trait Node {
    fn eval(
        &self,
        context: Context,
        input: Vec<Slot>,
    ) -> Vec<Slot>;

    fn get_input_slots(&self) -> &SlotMap;
    fn get_output_slots(&self) -> &SlotMap;
}
