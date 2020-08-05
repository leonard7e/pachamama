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

pub struct Graph<N: Node> {
    key_counter: Key,
    nodes: BTreeMap<Key, Arc<N>>,
    ref_node_input: Key,
    ref_node_output: Key,
}

impl <T: PType, N:Node<SlotType=T>> Node for Graph<N> {
    type SlotType = T;

    fn eval(
        &self,
        context: Context<Self::SlotType>,
        input: Vec<Slot<Self::SlotType>>,
    ) -> Vec<Slot<Self::SlotType>> {
        todo!()
    }
    fn get_input_slots(&self) -> &SlotMap<Self::SlotType> {
        let out_node = self.nodes[&self.ref_node_input].as_ref();
        out_node.get_output_slots()
    }
    fn get_output_slots(&self) -> &SlotMap<Self::SlotType> {
        let out_node = self.nodes[&self.ref_node_output].as_ref();
        out_node.get_input_slots()
    }
}
