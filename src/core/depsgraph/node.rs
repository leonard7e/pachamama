/*

*/
use crate::core::context::evaluator::Evaluator;
use crate::core::context::{ptype::PType, Context};

use super::{graph::Graph, slot::Slot};

pub enum Node {
    Subgraph(Box<Graph>),
    Node(NodeRec)
}

pub struct NodeRec{
    name: String,
    procedure: Box<dyn Fn(Context, Vec<Slot>) -> Vec<Slot>>,
    inputs: Vec<PType>,
    outputs: Vec<PType>
}

// Typendescriptors sollten keine Arbeitsdaten enthalten

impl Evaluator for Node {
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>  {
        match self {
            Node::Subgraph(g) => g.eval(context, inputs),
            Node::Node(n) => (n.procedure)(context, inputs),
        }

    }
}
