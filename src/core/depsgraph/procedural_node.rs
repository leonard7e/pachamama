use crate::core::{utils::typedefs::Key, context::{ptype::PType, Context}};

use super::{node::Node, slot::{SlotMap, Slot, SlotIter}};

pub struct ProceduralNode<T: PType>{
    pub name: String,
    pub procedure: Box<dyn Fn(Context<T>, Vec<Slot<T>>) -> Vec<Slot<T>>>,
    pub inputs: SlotMap<T>,
    pub outputs: SlotMap<T>
}

impl <T:PType> Node<T> for ProceduralNode<T> {
    fn has_input_slot(&self, s: &Key) -> bool {
        unimplemented!()
    }

    fn has_output_slot(&self, s: &Key) -> bool {
        unimplemented!()
    }

    fn iter_output_slots(&self) -> Box<SlotIter<T>>
    {
        Box::new(self.outputs.iter())
    }

    fn iter_input_slots(&self) -> Box<SlotIter<T>>
    {
        Box::new(self.inputs.iter())
    }

    fn eval(&self, context: Context<T>, inputs: Vec<Slot<T>>) -> Vec<Slot<T>>  {
        (self.procedure)(context, inputs)
    }
}
