use crate::core::context::Context;

use super::{trait_node::{SlotTypes, Node}, slot::Slot};

pub struct NodeProcedural{
    pub procedure: Box<dyn Fn(Context, Vec<Slot>) -> Vec<Slot>>,
    inputs: SlotTypes,
    outputs: SlotTypes,
}

impl Node for NodeProcedural {
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>  {
        (self.procedure)(context, inputs)
    }
    fn get_input_slot_types(&self) -> &SlotTypes {
        &self.inputs
    }
    fn get_output_slot_types(&self) -> &SlotTypes {
        &self.outputs
    }
}
