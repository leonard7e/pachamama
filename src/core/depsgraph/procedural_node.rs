use crate::core::{utils::typedefs::Key, context::{ptype::PType, Context}};

use super::{node::Node, slot::Slot};

pub struct ProceduralNode<T: PType>{
    pub name: String,
    pub procedure: Box<dyn Fn(Context<T>, Vec<Slot<T>>) -> Vec<Slot<T>>>,
    pub inputs: Vec<T>,
    pub outputs: Vec<T>
}

impl <T:PType> Node<T> for ProceduralNode<T> {
    fn has_input_slot(&self, s: &Key) -> bool {
        unimplemented!()
    }
    fn has_output_slot(&self, s: &Key) -> bool {
        unimplemented!()
    }
    fn eval(&self, context: Context<T>, inputs: Vec<Slot<T>>) -> Vec<Slot<T>>  {
        (self.procedure)(context, inputs)
    }
}
