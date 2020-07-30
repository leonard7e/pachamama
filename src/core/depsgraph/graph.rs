/*

*/

use super::node::Node;
use crate::core::context::{Context, evaluator::Evaluator};

use std::{
    sync::atomic::AtomicUsize,
    collections::{BTreeMap, BTreeSet}
};

/*

*/

type Key = usize; // Graph node index



struct Connection {
    from: AtomicUsize,
    from_output: AtomicUsize,
    to: AtomicUsize,
    to_output: AtomicUsize,
}

struct Graph {
    key_counter: AtomicUsize,
    nodes: BTreeMap<AtomicUsize,Node>,
    connections: BTreeSet<Connection>,
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
    fn eval(&self, context: Context) { unimplemented!() }
}
