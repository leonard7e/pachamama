/*

*/

use crate::core::depsgraph::{
    slot::Slot,
    trait_node::{Node, SlotTypes},
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

pub struct Graph {
    key_counter: Key,
    nodes: BTreeMap<Key, Arc<dyn Node>>,
    ref_node_input: Key,
    ref_node_output: Key,
}

impl Node for Graph {
    fn eval(
        &self,
        context: Context,
        input: Vec<Slot>,
    ) -> Vec<Slot> {
        todo!()
    }
    fn get_input_slot_types(&self) -> &SlotTypes {
        let out_node = self.nodes[&self.ref_node_input].as_ref();
        out_node.get_output_slot_types()
    }
    fn get_output_slot_types(&self) -> &SlotTypes {
        let out_node = self.nodes[&self.ref_node_output].as_ref();
        out_node.get_input_slot_types()
    }
}
