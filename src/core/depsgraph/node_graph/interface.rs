/*
*/
use crate::core::{depsgraph::trait_node::{Node, SlotTypes}, context::ptype::PType};

pub struct Interface{
    pub params: SlotTypes,
}

impl Interface {
    pub fn new() -> Self {
        let slots = SlotTypes::new();
        Interface {
            params: slots
        }
    }
}

impl Node for Interface {
    fn eval(
        &self,
        context: crate::core::context::Context,
        input: Vec<crate::core::depsgraph::slot::Slot>,
    ) -> Vec<crate::core::depsgraph::slot::Slot> {
        panic!("Input node does not support evaluation.")
    }
    fn get_input_slot_types(&self) -> &SlotTypes {
        panic!("Input node does not have input slots.")
    }
    fn get_output_slot_types(&self) -> &SlotTypes {
        &self.inputs
    }

}