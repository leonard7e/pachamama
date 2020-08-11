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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::context::ptype::PValue;
    fn basic_procedure( ctx: Context, inputs: Vec<Slot>) -> Vec<Slot> {
        let mut output = Vec::new();
        if let PValue::PV_Scalar (x) = inputs[0] {
            if let PValue::PV_Scalar (y) = inputs[1] {
                output.push( PValue::PV_Scalar (x+y) );
                output.push( PValue::PV_Scalar (x*y) );
            }
        }
        output
    }

    #[test]
    fn test_apply_procedural_node() {
        let ctx = Context::new();
        let node = NodeProcedural {
            procedure: Box::new(basic_procedure),
            inputs: SlotTypes::new(),
            outputs: SlotTypes::new(),
        };
        let mut inputs = Vec::new();
        inputs.push(PValue::PV_Scalar(4.0));
        inputs.push(PValue::PV_Scalar(7.0));
        let outputs = node.eval(ctx, inputs);
        if let PValue::PV_Scalar(r0) =outputs[0] {
            assert_eq!(r0, 11.0f32)
        }
        if let PValue::PV_Scalar(r1) =outputs[1] {
            assert_eq!(r1, 28.0f32)
        }
    }
}
