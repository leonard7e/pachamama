use crate::core::context::evaluator::Evaluator;
use crate::core::{utils::typedefs::Key, context::{ptype::PType, Context}};

use super::{node::Node, slot::Slot};

pub struct ProceduralNode{
    pub name: String,
    pub procedure: Box<dyn Fn(Context, Vec<Slot>) -> Vec<Slot>>,
    pub inputs: Vec<PType>,
    pub outputs: Vec<PType>
}

impl Node for ProceduralNode {
    fn has_input_slot(&self, s: Key) -> bool {
        unimplemented!()
    }
    fn has_output_slot(&self, s: Key) -> bool {
        unimplemented!()
    }
}

impl Evaluator for ProceduralNode {
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>  {
        (self.procedure)(context, inputs)
    }
}
