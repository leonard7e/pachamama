/*

*/

use super::{slot::Slot, node::Node};
use crate::core::{
    utils::typedefs::Key,
    context::Context
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

pub struct Graph <N: Node> {
    key_counter: Key,
    nodes: BTreeMap<Key,Arc<N>>,
    connections: BTreeSet<Connection>,
    input: Vec<Slot>,
    output: Vec<Slot>,
}

impl <N: Node> Graph<N> {
    fn key_step(&mut self) -> Key {
        let k = self.key_counter;
        self.key_counter += 1;
        k
    }
    fn validate(&self) -> Result<(), ()> {
        /* Outside modules should not bother about validation */
        unimplemented!()
    }

    pub fn insert(&mut self, node: N) -> Option<Key> {
        let k = self.key_step();
        self.nodes.insert(k, Arc::new(node));
        Some(k)
    }

    fn connect(&mut self, c: Connection) -> Result<(), &str> {
        // check if target nodes and target slots exist
        let from_slot_valid = self.nodes.get(&c.from)
            .map_or(false, |n|
                n.as_ref()
                .has_output_slot(&c.from_output) );

        let to_slot_valid = self.nodes.get(&c.to)
            .map_or(false, |n|
                n.as_ref()
                .has_output_slot(&c.to_input) );

        if from_slot_valid && to_slot_valid {
            self.connections.insert(c);
            Ok(())
        }
        else {
            return Err("Graph error: could not establish connection")
        }
    }
    fn eval(&self, context: Context, inputs: Vec<Slot>) -> Vec<Slot>{ unimplemented!() }
}
