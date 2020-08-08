/*
*/
use crate::core::{depsgraph::trait_node::{Node, SlotTypes}, context::ptype::PType};

pub struct NodeInput{
    pub inputs: SlotTypes,
}

impl Node for NodeInput {
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
