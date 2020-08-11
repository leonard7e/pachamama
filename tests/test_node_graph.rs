use pachamama::core::{
    context::{ptype::PValue, Context},
    depsgraph::{
        node_procedural::NodeProcedural,
        slot::Slot,
        trait_node::{Node, SlotTypes},
    },
};

fn basic_procedure_1(ctx: Context, inputs: Vec<Slot>) -> Vec<Slot> {
    let mut output = Vec::new();
    if let PValue::PV_Scalar(x) = inputs[0] {
        if let PValue::PV_Scalar(y) = inputs[1] {
            output.push(PValue::PV_Scalar(x - y));
            output.push(PValue::PV_Scalar(x.powf(y)));
        }
    }
    output
}

fn basic_procedure_2(ctx: Context, inputs: Vec<Slot>) -> Vec<Slot> {
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
fn test_apply_node_graph() {
    let ctx = Context::new();

    let pnode1 = NodeProcedural::new(Box::new(basic_procedure_1));
    let pnode2 = NodeProcedural::new(Box::new(basic_procedure_2));

    let mut inputs = Vec::new();
    inputs.push(PValue::PV_Scalar(3.0));
    inputs.push(PValue::PV_Scalar(9.0));

    unimplemented!()
}
