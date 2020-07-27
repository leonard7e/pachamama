/*

*/

use node::Node;
use evaluator::Evaluator;

use std::collections::BTreeMap;

/*

*/

type Key usize; // Graph node index

struct Graph {
    gix_counter: Key,
    nodes: BTreeMap<Key,Node>,
    // connections: BTreeMap<[Key; 2]>
}

impl Evaluator for Graph {
    fn eval(&self) { unimplemented!() }
}


// fn validate(&self) { unimplemented!()}

// Any change could introduce a circular dependency.
// insert
