/*

*/
use evaluator::Evaluator;

use std::arc::Arc;
use std::rc::RefCell;

struct Node <E: Evaluator>{
    name: String,
};

impl Evaluator for Node {
    fn eval() {
        unimplemented!()
    }
}
