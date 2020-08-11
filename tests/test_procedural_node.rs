use pachamama::core::context::{Context, ptype::PValue};
use pachamama::core::depsgraph::{slot::Slot,trait_node::Node};
use pachamama::core::depsgraph::node_procedural::*;

fn basic_procedure(ctx: Context, inputs: Vec<Slot>) -> Vec<Slot> {
    let mut output = Vec::new();
    if let PValue::PV_Scalar(x) = inputs[0] {
        if let PValue::PV_Scalar(y) = inputs[1] {
            output.push(PValue::PV_Scalar(x + y));
            output.push(PValue::PV_Scalar(x * y));
        }
    }
    output
}

#[test]
fn test_apply_node_procedural() {
    let ctx = Context::new();
    let node = NodeProcedural::new(Box::new(basic_procedure));
    let mut inputs = Vec::new();

    inputs.push(PValue::PV_Scalar(4.0));
    inputs.push(PValue::PV_Scalar(7.0));

    let outputs = node.eval(ctx, inputs);
    if let PValue::PV_Scalar(r0) = outputs[0] {
        assert_eq!(r0, 11.0f32)
    }
    if let PValue::PV_Scalar(r1) = outputs[1] {
        assert_eq!(r1, 28.0f32)
    }
}
