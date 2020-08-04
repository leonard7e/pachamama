/*

*/

/*
    The node trait
*/

use crate::core::{
    context::{ptype::PType, Context},
    utils::typedefs::Key,
};

use super::slot::{Slot, SlotMap};

pub trait Node {
    type SlotType: PType;
    fn eval(
        &self,
        context: Context<Self::SlotType>,
        input: Vec<Slot<Self::SlotType>>,
    ) -> Vec<Slot<Self::SlotType>>;

    fn get_input_slots(&self) -> &SlotMap<Self::SlotType>;
    fn get_output_slots(&self) -> &SlotMap<Self::SlotType>;
}
