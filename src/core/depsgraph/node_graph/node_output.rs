/*
*/
use crate::core::{depsgraph::trait_node::{Node, SlotTypes}, context::ptype::PType};

pub struct NodeOutput{
    pub inputs: SlotTypes,
}

impl Node for NodeOutput {
    fn eval(
        &self,
        context: crate::core::context::Context,
        input: Vec<crate::core::depsgraph::slot::Slot>,
    ) -> Vec<crate::core::depsgraph::slot::Slot> {
        panic!("Output node does not support evaluation.")
    }
    fn get_input_slot_types(&self) -> &SlotTypes {
        &self.inputs
    }
    fn get_output_slot_types(&self) -> &SlotTypes {
        panic!("Output node does not have output slots.")
    }
}
