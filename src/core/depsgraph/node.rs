/*

*/
use crate::core::context::evaluator::Evaluator;
use crate::core::context::Context;

use super::slot::Slot;

pub struct Node{
    name: String,
    inputs: Vec<Slot>,
    outputs: Vec<Slot>
}

impl Evaluator for Node {
    fn eval(&self, context: Context) {
        unimplemented!()
    }
}
