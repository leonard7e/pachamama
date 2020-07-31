/*

*/

use crate::core::depsgraph::slot::Slot;

use super::Context;

pub trait Evaluator {
    fn eval(&self, context: Context, input: Vec<Slot>) -> Vec<Slot>;
}
