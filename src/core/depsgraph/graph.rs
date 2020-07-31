/*

*/

use super::{slot::Slot, node::Node};
use crate::core::{
    utils::typedefs::Key,
    context::{Context, evaluator::Evaluator}
};

use std::{
    sync::{Arc},
    collections::{BTreeMap, BTreeSet}
};

/*

*/

struct Connection {
    from: Key,
    from_output: Key,
    to: Key,
    to_output: Key,
}

pub struct Graph {
    key_counter: Key,
    nodes: BTreeMap<Key,Arc<Node>>,
    connections: BTreeSet<Connection>,
    input: Vec<Slot>,
    output: Vec<Slot>,
}

impl Graph {
    fn validate(&self) -> Result<(), ()> {
        /* Outside modules should not bother about validation */
        unimplemented!()
    }

    pub fn insert(&mut self, node: &mut Node) -> Option<Key> {
        /* Make sure self is still valid after insert node. */
        unimplemented!()
    }
}

impl Evaluator for Graph {
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>{ unimplemented!() }
}
