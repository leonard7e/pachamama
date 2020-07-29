/*

*/

use node::Node;
use evaluator::Evaluator;

use std::collections::BTreeMap;

/*

*/

type Key usize; // Graph node index

struct Graph {
    key_counter: Key,
    nodes: BTreeMap<Key,Node>,
    // connections: BTreeMap<[Key; 2]>
}

impl Graph {
    fn validate(&self) -> Result<()> {
        /* Outside modules should not bother about validation */
        unimplemented!()
    }

    pub fn insert(&mut self, node: &mut Node) -> Option<Key> {
        /* Make sure self is still valid after insert node. */
        unimplemented!()
    }

}

impl Evaluator for Graph {
    fn eval(&self) { unimplemented!() }
}
