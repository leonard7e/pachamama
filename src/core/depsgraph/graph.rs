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

#[derive(PartialOrd, PartialEq, Eq, Ord)]
struct Connection {
    from: Key,
    from_output: Key,
    to: Key,
    to_input: Key,
}

pub struct Graph {
    key_counter: Key,
    nodes: BTreeMap<Key,Arc<Node>>,
    connections: BTreeSet<Connection>,
    input: Vec<Slot>,
    output: Vec<Slot>,
}

impl Graph {
    fn key_step(&mut self) -> Key {
        let k = self.key_counter;
        self.key_counter += 1;
        k
    }
    fn validate(&self) -> Result<(), ()> {
        /* Outside modules should not bother about validation */
        unimplemented!()
    }

    pub fn insert(&mut self, node: Node) -> Option<Key> {
        let k = self.key_step();
        self.nodes.insert(k, Arc::new(node));
        Some(k)
    }

    fn connect(&mut self, c: Connection) -> () {
        self.connections.insert(c);
    }
}

impl Evaluator for Graph {
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>{ unimplemented!() }
}
