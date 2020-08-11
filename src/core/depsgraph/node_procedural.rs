use crate::core::context::Context;

use super::{trait_node::{SlotTypes, Node}, slot::Slot};

pub type Procedure = Fn(Context, Vec<Slot>) -> Vec<Slot>;

pub struct NodeProcedural{
    procedure: Box<Procedure>,
    inputs: SlotTypes,
    outputs: SlotTypes,
}

impl NodeProcedural {
    pub fn new(p: Box<Procedure>) -> NodeProcedural {
        NodeProcedural {
            procedure: p,
            inputs: SlotTypes::new(),
            outputs: SlotTypes::new(),
        }
    }
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
