/*

*/

use crate::core::depsgraph::{
    slot::{Slot, SlotMap},
    trait_node::Node,
};
use crate::core::{
    context::{ptype::PType, Context},
    utils::typedefs::Key,
};

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

/*
    Please keep in mind, this graph is a directed graph. While data
    might flow from one node output to another node input, the graphs
    connections are managed the opposite direction. The root of the graph
    is the output node.
    Only nodes which do have a path to the output node are used in evaluation.
*/

pub struct Graph<T: PType, N: Node<SlotType=T>> {
    key_counter: Key,
    nodes: BTreeMap<Key, BTreeMap<Key, Arc<N>>>,
    inputs: SlotMap<T>,
    outputs: SlotMap<T>,
}
