use crate::core::{utils::typedefs::Key, context::{ptype::PType, Context}};

use super::{trait_node::Node, slot::{SlotMap, Slot, SlotIter}};

pub struct ProceduralNode<T: PType>{
    pub name: String,
    pub procedure: Box<dyn Fn(Context<T>, Vec<Slot<T>>) -> Vec<Slot<T>>>,
    pub inputs: SlotMap<T>,
    pub outputs: SlotMap<T>
}

impl <T: PType> Node for ProceduralNode<T> {
    type SlotType = T;
    fn eval(&self, context: Context<T>, inputs: Vec<Slot<T>>) -> Vec<Slot<T>>  {
        (self.procedure)(context, inputs)
    }
    fn get_input_slots(&self) -> &SlotMap<Self::SlotType> {
        &self.inputs
    }
    fn get_output_slots(&self) -> &SlotMap<Self::SlotType> {
        &self.outputs
    }
}
