/*
*/
use crate::core::{depsgraph::{trait_node::Node, slot::SlotMap}, context::ptype::PType};

pub struct NodeInput<T: PType>{
    pub inputs: SlotMap<T>,
}

impl <T: PType> Node for NodeInput<T> {
    type SlotType= T;
    fn eval(
        &self,
        context: crate::core::context::Context<Self::SlotType>,
        input: Vec<crate::core::depsgraph::slot::Slot<Self::SlotType>>,
    ) -> Vec<crate::core::depsgraph::slot::Slot<Self::SlotType>> {
        panic!("Input node does not support evaluation.")
    }
    fn get_input_slots(&self) -> &SlotMap<Self::SlotType> {
        panic!("Input node does not have input slots.")
    }
    fn get_output_slots(&self) -> &SlotMap<Self::SlotType> {
        &self.inputs
    }

}
